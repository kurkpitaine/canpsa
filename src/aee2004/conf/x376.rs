use core::{cmp::Ordering, fmt, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time};

use crate::{
    config::DisplayMode,
    Error, Result, YEAR_OFFSET,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    use crate::field::Field;
    /// 1-bit empty,
    /// 7-bit clock year field.
    pub const YEAR_CLOCK: usize = 0;
    /// 1-bit empty,
    /// 5-bit clock hour field,
    /// 6-bit clock day field,
    /// 4-bit clock month field.
    pub const MONTH_DAY_HOUR_CLOCK: Field = 1..3;
    /// 1-bit empty,
    /// 1-bit clock display mode flag,
    /// 6-bit clock minute field.
    pub const MINUTE_CLOCK_DISP_MODE: usize = 3;
}

/// Raw x376 CAN frame identifier.
pub const FRAME_ID: u16 = 0x376;
/// Length of a x376 CAN frame.
pub const FRAME_LEN: usize = field::MINUTE_CLOCK_DISP_MODE + 1;

/// Periodicity of a x376 CAN frame.
pub const PERIODICITY: Duration = Duration::from_millis(1000);

impl<T: AsRef<[u8]>> Frame<T> {
    /// Create a raw octet buffer with a CAN frame structure.
    #[inline]
    pub fn new_unchecked(buffer: T) -> Frame<T> {
        Frame { buffer }
    }

    /// Shorthand for a combination of [new_unchecked] and [check_len].
    ///
    /// [new_unchecked]: #method.new_unchecked
    /// [check_len]: #method.check_len
    #[inline]
    pub fn new_checked(buffer: T) -> Result<Frame<T>> {
        let packet = Self::new_unchecked(buffer);
        packet.check_len()?;
        Ok(packet)
    }

    /// Ensure that no accessor method will panic if called.
    /// Returns `Err(Error::Truncated)` if the buffer is too short.
    ///
    /// The result of this check is invalidated by calling [set_payload_len].
    ///
    /// [set_payload_len]: #method.set_payload_len
    #[inline]
    pub fn check_len(&self) -> Result<()> {
        let len = self.buffer.as_ref().len();
        match len.cmp(&FRAME_LEN) {
            Ordering::Less => Err(Error::Truncated),
            Ordering::Greater => Err(Error::Overlong),
            Ordering::Equal => Ok(()),
        }
    }

    /// Consume the frame, returning the underlying buffer.
    #[inline]
    pub fn into_inner(self) -> T {
        self.buffer
    }

    /// Return the frame length.
    #[inline]
    pub fn frame_len(&self) -> usize {
        FRAME_LEN
    }

    /// Return the year field.
    #[inline]
    pub fn year(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::YEAR_CLOCK] >> 1
    }

    /// Return the hour field.
    #[inline]
    pub fn hour(&self) -> u8 {
        let data = self.buffer.as_ref();
        let raw = (NetworkEndian::read_u16(&data[field::MONTH_DAY_HOUR_CLOCK]) & 0x003e) >> 1;
        raw as u8
    }

    /// Return the day field.
    #[inline]
    pub fn day(&self) -> u8 {
        let data = self.buffer.as_ref();
        let raw = (NetworkEndian::read_u16(&data[field::MONTH_DAY_HOUR_CLOCK]) & 0x0fc0) >> 6;
        raw as u8
    }

    /// Return the month field.
    #[inline]
    pub fn month(&self) -> u8 {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::MONTH_DAY_HOUR_CLOCK]) >> 12;
        raw as u8
    }

    /// Return the clock display mode field.
    #[inline]
    pub fn clock_display_mode(&self) -> DisplayMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::MINUTE_CLOCK_DISP_MODE] & 0x02) >> 1;
        DisplayMode::from(raw)
    }

    /// Return the minute field.
    #[inline]
    pub fn minute(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::MINUTE_CLOCK_DISP_MODE] >> 2
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the year field.
    #[inline]
    pub fn set_year(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::YEAR_CLOCK] & !0xfe;
        let raw = raw | (value << 1);
        data[field::YEAR_CLOCK] = raw
    }

    /// Set the hour field.
    #[inline]
    pub fn set_hour(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::MONTH_DAY_HOUR_CLOCK]) & !0x003e;
        let raw = raw | ((u16::from(value) << 1) & 0x003e);
        NetworkEndian::write_u16(&mut data[field::MONTH_DAY_HOUR_CLOCK], raw);
    }

    /// Set the day field.
    #[inline]
    pub fn set_day(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::MONTH_DAY_HOUR_CLOCK]) & !0x0fc0;
        let raw = raw | ((u16::from(value) << 6) & 0x0fc0);
        NetworkEndian::write_u16(&mut data[field::MONTH_DAY_HOUR_CLOCK], raw);
    }

    /// Set the month field.
    #[inline]
    pub fn set_month(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::MONTH_DAY_HOUR_CLOCK]) & !0xf000;
        let raw = raw | (u16::from(value) << 12);
        NetworkEndian::write_u16(&mut data[field::MONTH_DAY_HOUR_CLOCK], raw);
    }

    /// Set the clock display mode field.
    #[inline]
    pub fn set_clock_display_mode(&mut self, value: DisplayMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::MINUTE_CLOCK_DISP_MODE] & !0x02;
        let raw = raw | ((u8::from(value) << 1) & 0x02);
        data[field::MINUTE_CLOCK_DISP_MODE] = raw;
    }

    /// Set the minute field.
    #[inline]
    pub fn set_minute(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::MINUTE_CLOCK_DISP_MODE] & !0xfc;
        let raw = raw | (value << 2);
        data[field::MINUTE_CLOCK_DISP_MODE] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x376 ({})", err)?;
                Ok(())
            }
        }
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Frame<T> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_ref()
    }
}

/// A high-level representation of a x376 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub clock_disp_mode: DisplayMode,
    pub utc_datetime: OffsetDateTime,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        // month values:
        //  - 0x00 and 0x0d are not used
        //  - 0x0e means unavailable
        //  - 0x0f means invalid value.

        let date = Date::from_calendar_date(
            YEAR_OFFSET + (frame.year() as i32),
            Month::try_from(frame.month()).map_err(|_| Error::Illegal)?,
            frame.day(),
        )
        .map_err(|_| Error::Illegal)?;

        let time = Time::from_hms(frame.hour(), frame.minute(), 0).map_err(|_| Error::Illegal)?;
        let date_time = PrimitiveDateTime::new(date, time);
        let utc_datetime = OffsetDateTime::from_unix_timestamp(0).map_err(|_| Error::Illegal)?;

        Ok(Repr {
            clock_disp_mode: frame.clock_display_mode(),
            utc_datetime: utc_datetime.replace_date_time(date_time),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x376 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        let can_year = self.utc_datetime.year() - YEAR_OFFSET;
        frame.set_year(can_year as u8);
        frame.set_month(self.utc_datetime.month().into());
        frame.set_clock_display_mode(self.clock_disp_mode);
        frame.set_day(self.utc_datetime.day());
        frame.set_hour(self.utc_datetime.hour());
        frame.set_minute(self.utc_datetime.minute());
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, " clock_disp_mode={}", self.clock_disp_mode)?;
        writeln!(f, " utc_datetime={}", self.utc_datetime)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        config::DisplayMode,
        Error,
    };

    use time::macros::datetime;

    static REPR_FRAME_BYTES: [u8; 4] = [0x2c, 0x63, 0xd4, 0x62];

    fn frame_repr() -> Repr {
        Repr {
            clock_disp_mode: DisplayMode::Blinking,
            utc_datetime: datetime!(2022-06-15 10:24 UTC),
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.year(), 22);
        assert_eq!(frame.hour(), 10);
        assert_eq!(frame.day(), 15);
        assert_eq!(frame.month(), 6);
        assert_eq!(frame.clock_display_mode(), DisplayMode::Blinking);
        assert_eq!(frame.minute(), 24);
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0u8; 4];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_year(22);
        frame.set_hour(10);
        frame.set_day(15);
        frame.set_month(6);
        frame.set_clock_display_mode(DisplayMode::Blinking);
        frame.set_minute(24);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 5] = [0x2c, 0x63, 0xd4, 0x62, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 3] = [0x2c, 0x63, 0xd4];
        assert_eq!(Frame::new_checked(&bytes).unwrap_err(), Error::Truncated);
    }

    #[test]
    fn test_repr_parse_valid() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        let repr = Repr::parse(&frame).unwrap();
        assert_eq!(repr, frame_repr());
    }

    #[test]
    fn test_basic_repr_emit() {
        let mut buf = [0u8; 4];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }
}
