use core::fmt;
use time::{Date, Month, PrimitiveDateTime, Time};

use crate::{config::ClockFormat, Error, Result, YEAR_OFFSET};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 7-bit clock year, 1-bit clock format
    pub const YEAR_CLK_FMT: usize = 0;
    /// 4-bit clock month, 4-bit empty
    pub const MONTH: usize = 1;
    /// 6-bit clock day, 2-bit empty
    pub const DAY: usize = 2;
    /// 5-bit clock hour, 3-bit empty
    pub const HOUR: usize = 3;
    /// 6-bit clock minute, 2-bit empty
    pub const MINUTE: usize = 4;
}

/// Length of a x39b CAN frame.
pub const FRAME_LEN: usize = field::MINUTE + 1;

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
        if len < (FRAME_LEN) {
            Err(Error::Truncated)
        } else if len > (FRAME_LEN) {
            Err(Error::Overlong)
        } else {
            Ok(())
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

    /// Return the clock format field.
    #[inline]
    pub fn clock_format(&self) -> ClockFormat {
        let data = self.buffer.as_ref();
        let raw = (data[field::YEAR_CLK_FMT] & 0x80) >> 7;
        ClockFormat::from(raw)
    }

    /// Return the year field.
    #[inline]
    pub fn year(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::YEAR_CLK_FMT] & 0x7f
    }

    /// Return the month field.
    #[inline]
    pub fn month(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::MONTH] & 0x0f
    }

    /// Return the day field.
    #[inline]
    pub fn day(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::DAY] & 0x3f
    }

    /// Return the hour field.
    #[inline]
    pub fn hour(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::HOUR] & 0x1f
    }

    /// Return the minute field.
    #[inline]
    pub fn minute(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::MINUTE] & 0x3f
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the clock format field.
    #[inline]
    pub fn set_clock_format(&mut self, value: ClockFormat) {
        let data = self.buffer.as_mut();
        let raw = data[field::YEAR_CLK_FMT] & !0x80;
        let raw = raw | (u8::from(value) << 7);
        data[field::YEAR_CLK_FMT] = raw;
    }

    /// Set the year field.
    #[inline]
    pub fn set_year(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::YEAR_CLK_FMT] & !0x7f;
        let raw = raw | (value & 0x7f);
        data[field::YEAR_CLK_FMT] = raw
    }

    /// Set the month field.
    #[inline]
    pub fn set_month(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::MONTH] & !0x0f;
        let raw = raw | value & 0x0f;
        data[field::MONTH] = raw;
    }

    /// Set the day field.
    #[inline]
    pub fn set_day(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::DAY] & !0x3f;
        let raw = raw | value & 0x3f;
        data[field::DAY] = raw;
    }

    /// Set the hour field.
    #[inline]
    pub fn set_hour(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::HOUR] & !0x1f;
        let raw = raw | value & 0x1f;
        data[field::HOUR] = raw;
    }

    /// Set the minute field.
    #[inline]
    pub fn set_minute(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::MINUTE] & !0x3f;
        let raw = raw | value & 0x3f;
        data[field::MINUTE] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x39b ({})", err)?;
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

/// A high-level representation of a x39b CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    clock_format: ClockFormat,
    utc_datetime: PrimitiveDateTime,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        if frame.month() < 1
            || frame.month() > 12
            || frame.day() < 1
            || frame.day() > 31
            || frame.hour() > 23
            || frame.minute() > 59
        {
            Err(Error::Invalid)
        } else {
            Ok(Repr {
                clock_format: frame.clock_format(),
                utc_datetime: PrimitiveDateTime::new(
                    Date::from_calendar_date(
                        YEAR_OFFSET + (frame.year() as i32),
                        Month::try_from(frame.month()).unwrap(),
                        frame.day(),
                    )
                    .unwrap(),
                    Time::from_hms(frame.hour(), frame.minute(), 0).unwrap(),
                ),
            })
        }
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x39b CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        let can_year = self.utc_datetime.year() - YEAR_OFFSET;
        frame.set_clock_format(self.clock_format);
        frame.set_year(can_year as u8);
        frame.set_month(self.utc_datetime.month().into());
        frame.set_day(self.utc_datetime.day());
        frame.set_hour(self.utc_datetime.hour());
        frame.set_minute(self.utc_datetime.minute());
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "x39b utc_datetime={} clock_format={}",
            self.utc_datetime, self.clock_format
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{config::ClockFormat, Error};

    use time::macros::datetime;

    static REPR_FRAME_BYTES: [u8; 5] = [0x96, 0x01, 0x0a, 0x0f, 0x1d];

    fn frame_repr() -> Repr {
        Repr {
            clock_format: ClockFormat::H24,
            utc_datetime: datetime!(2022-01-10 15:29),
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.clock_format(), ClockFormat::H24);
        assert_eq!(frame.year(), 0x16);
        assert_eq!(frame.month(), 0x01);
        assert_eq!(frame.day(), 0x0a);
        assert_eq!(frame.hour(), 0x0f);
        assert_eq!(frame.minute(), 0x1d);
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0u8; 5];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_clock_format(ClockFormat::H24);
        frame.set_year(0x16);
        frame.set_month(0x01);
        frame.set_day(0x0a);
        frame.set_hour(0x0f);
        frame.set_minute(0x1d);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 6] = [0x96, 0x01, 0x0a, 0x0f, 0x1d, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 4] = [0x96, 0x01, 0x0a, 0x0f];
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
        let mut buf = [0u8; 5];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }
}
