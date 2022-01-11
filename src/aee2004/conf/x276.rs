use core::{convert::TryFrom, fmt, time::Duration};
use time::{Date, Month, PrimitiveDateTime, Time};

use crate::Frame;
use crate::{Error, Result, YEAR_OFFSET};

mod field {
    pub const YEAR: usize = 0;
    pub const MONTH: usize = 1;
    pub const DAY: usize = 2;
    pub const HOUR: usize = 3;
    pub const MINUTE: usize = 4;
    pub const UNKNOWN_1: usize = 5;
    pub const UNKNOWN_2: usize = 6;
}

/// Length of a x276 CAN frame.
pub const FRAME_LEN: usize = field::UNKNOWN_2 + 1;

/// Periodicity of a x276 CAN frame.
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

    /// Return the year field.
    #[inline]
    pub fn year(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::YEAR]
    }

    /// Return the month field.
    #[inline]
    pub fn month(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::MONTH]
    }

    /// Return the day field.
    #[inline]
    pub fn day(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::DAY]
    }

    /// Return the hour field.
    #[inline]
    pub fn hour(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::HOUR]
    }

    /// Return the minute field.
    #[inline]
    pub fn minute(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::MINUTE]
    }

    /// Return the first unknown field.
    #[inline]
    pub fn unknown_1(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::UNKNOWN_1]
    }

    /// Return the second unknown field.
    #[inline]
    pub fn unknown_2(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::UNKNOWN_2]
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the year field.
    #[inline]
    pub fn set_year(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::YEAR] = value
    }

    /// Set the month field.
    #[inline]
    pub fn set_month(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::MONTH] = value;
    }

    /// Set the day field.
    #[inline]
    pub fn set_day(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::DAY] = value;
    }

    /// Set the hour field.
    #[inline]
    pub fn set_hour(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::HOUR] = value;
    }

    /// Set the minute field.
    #[inline]
    pub fn set_minute(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::MINUTE] = value;
    }

    /// Set the first unknown field.
    #[inline]
    pub fn set_unknown_1(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::UNKNOWN_1] = value;
    }

    /// Set the second unknown field.
    #[inline]
    pub fn set_unknown_2(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::UNKNOWN_2] = value;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x276 ({})", err)?;
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

/// A high-level representation of a x276 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
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
            Err(Error::Illegal)
        } else {
            Ok(Repr {
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

    /// Emit a high-level representation into a x276 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        let can_year = self.utc_datetime.year() - YEAR_OFFSET;
        frame.set_year(can_year as u8);
        frame.set_month(self.utc_datetime.month().into());
        frame.set_day(self.utc_datetime.day());
        frame.set_hour(self.utc_datetime.hour());
        frame.set_minute(self.utc_datetime.minute());
        frame.set_unknown_1(0x3f); // Seems to be fixed to this value
        frame.set_unknown_2(0xfe); // Seems to be fixed to this value
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x276 datetime={}", self.utc_datetime)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::Error;

    use time::macros::datetime;

    static REPR_FRAME_BYTES: [u8; 7] = [0x96, 0x01, 0x0a, 0x0f, 0x1d, 0x3f, 0xfe];

    fn frame_repr() -> Repr {
        Repr {
            utc_datetime: datetime!(2022-01-10 15:29),
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.year(), 0x96);
        assert_eq!(frame.month(), 0x01);
        assert_eq!(frame.day(), 0x0a);
        assert_eq!(frame.hour(), 0x0f);
        assert_eq!(frame.minute(), 0x1d);
        assert_eq!(frame.unknown_1(), 0x3f);
        assert_eq!(frame.unknown_2(), 0xfe);
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0xff; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_year(0x96);
        frame.set_month(0x01);
        frame.set_day(0x0a);
        frame.set_hour(0x0f);
        frame.set_minute(0x1d);
        frame.set_unknown_1(0x3f);
        frame.set_unknown_2(0xfe);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 8] = [0x96, 0x01, 0x0a, 0x0f, 0x1d, 0x3f, 0xfe, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 5] = [0x96, 0x01, 0x0a, 0x0f, 0x1d];
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
        let mut buf = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }
}
