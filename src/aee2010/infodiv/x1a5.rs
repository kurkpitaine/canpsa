use core::{cmp::Ordering, fmt};

use crate::{vehicle::VolumeLevelOrigin, Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 5-bit audio volume level field,
    /// 3-bit audio volume level origin.
    pub const VOLUME: usize = 0;
}

/// Raw x1a5 CAN frame identifier.
pub const FRAME_ID: u16 = 0x1a5;
/// Length of a x1a5 CAN frame.
pub const FRAME_LEN: usize = field::VOLUME + 1;

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

    /// Return the audio volume level field.
    #[inline]
    pub fn volume_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::VOLUME] & 0x1f
    }

    /// Return the audio volume level origin.
    #[inline]
    pub fn volume_level_origin(&self) -> VolumeLevelOrigin {
        let data = self.buffer.as_ref();
        VolumeLevelOrigin::from(data[field::VOLUME] >> 5)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the audio volume level field.
    #[inline]
    pub fn set_volume_level(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::VOLUME] & !0x1f;
        let raw = raw | (value & 0x1f);
        data[field::VOLUME] = raw;
    }

    /// Set the audio volume level origin.
    #[inline]
    pub fn set_volume_level_origin(&mut self, value: VolumeLevelOrigin) {
        let data = self.buffer.as_mut();
        let raw = data[field::VOLUME] & !0xe0;
        let raw = raw | ((u8::from(value) << 5) & 0xe0);
        data[field::VOLUME] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x1a5 ({})", err)?;
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

/// A high-level representation of a x1a5 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub volume: u8,
    pub origin: VolumeLevelOrigin,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            volume: frame.volume_level(),
            origin: frame.volume_level_origin(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x1a5 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_volume_level(self.volume);
        frame.set_volume_level_origin(self.origin);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x1a5 volume={}", self.volume)?;
        writeln!(f, " origin={}", self.origin)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};

    use crate::{vehicle::VolumeLevelOrigin, Error};

    static REPR_FRAME_BYTES: [u8; 1] = [0x8a];

    fn frame_repr() -> Repr {
        Repr {
            volume: 10,
            origin: VolumeLevelOrigin::ThermalProtection,
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.volume_level(), 10);
        assert_eq!(
            frame.volume_level_origin(),
            VolumeLevelOrigin::ThermalProtection
        );
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0u8; 1];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_volume_level(10);
        frame.set_volume_level_origin(VolumeLevelOrigin::ThermalProtection);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 2] = [0x8a, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 0] = [];
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
        let mut buf = [0u8; 1];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }
}
