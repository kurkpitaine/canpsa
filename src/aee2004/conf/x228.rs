use core::fmt;

use time::Time;

use crate::{Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 5-bit clock hour, 3-bit empty.
    pub const HOUR: usize = 0;
    /// 6-bit clock minute, 2-bit empty.
    pub const MINUTE: usize = 1;
}

/// Length of a x228 CAN frame.
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
    /// Set the hour field.
    #[inline]
    pub fn set_hour(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::HOUR] & !0x1f;
        let raw = raw | (value & 0x1f);
        data[field::HOUR] = raw;
    }

    /// Set the minute field.
    #[inline]
    pub fn set_minute(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::MINUTE] & !0x3f;
        let raw = raw | (value & 0x3f);
        data[field::MINUTE] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x228 ({})", err)?;
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

/// A high-level representation of a x228 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    time: Time,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        if frame.hour() > 23 || frame.minute() > 59 {
            Err(Error::Invalid)
        } else {
            Ok(Repr {
                time: Time::from_hms(frame.hour(), frame.minute(), 0).unwrap(),
            })
        }
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x228 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_hour(self.time.hour());
        frame.set_minute(self.time.minute());
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x228 time={}", self.time)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::Error;

    use time::Time;

    static REPR_FRAME_BYTES: [u8; 2] = [0x10, 0x2e];

    fn frame_repr() -> Repr {
        Repr {
            time: Time::from_hms(16, 46, 0).unwrap()
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.hour(), 16);
        assert_eq!(frame.minute(), 46);
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0x00; 2];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_hour(16);
        frame.set_minute(46);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 3] = [0x10, 0x2e, 0x80];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 1] = [0x10];
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
        let mut buf = [0u8; 2];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }
}
