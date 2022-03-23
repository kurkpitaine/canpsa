use core::{fmt, time::Duration};

use crate::{Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 2-bit fragrance selection field,
    /// 1-bit fragrance diffuser enable flag,
    /// 2-bit fragrance intensity field,
    /// 3-bit fragrance cartridge type field.
    pub const FRAGRANCE: usize = 0;
}

/// Length of a x1d0 CAN frame.
pub const FRAME_LEN: usize = field::FRAGRANCE + 1;

/// Periodicity of a x1d0 CAN frame.
pub const PERIODICITY: Duration = Duration::from_millis(200);

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

    /// Return the fragrance selection field.
    #[inline]
    pub fn fragrance_selection(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::FRAGRANCE] & 0x03
    }

    /// Return the fragrance diffuser enable flag.
    #[inline]
    pub fn fragrance_diffuser_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FRAGRANCE] & 0x04 != 0
    }

    /// Return the fragrance intensity field.
    #[inline]
    pub fn fragrance_intensity(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::FRAGRANCE] & 0x18) >> 3
    }

    /// Return the fragrance cartridge type field.
    #[inline]
    pub fn fragrance_cartridge_type(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::FRAGRANCE] >> 5
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the fragrance selection field.
    #[inline]
    pub fn set_fragrance_selection(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::FRAGRANCE] & !0x03;
        let raw = raw | (value & 0x03);
        data[field::FRAGRANCE] = raw;
    }

    /// Set the the fragrance diffuser enable flag.
    #[inline]
    pub fn set_fragrance_diffuser_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FRAGRANCE];
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::FRAGRANCE] = raw;
    }

    /// Set the fragrance intensity field.
    #[inline]
    pub fn set_fragrance_intensity(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::FRAGRANCE] & !0x18;
        let raw = raw | ((value << 3) & 0x18);
        data[field::FRAGRANCE] = raw;
    }

    /// Set the fragrance cartridge type field.
    #[inline]
    pub fn set_fragrance_cartridge_type(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::FRAGRANCE] & !0xe0;
        let raw = raw | (value << 5);
        data[field::FRAGRANCE] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x1d0 ({})", err)?;
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

/// A high-level representation of a x1d0 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub fragrance_selection: u8,
    pub fragrance_diffuser_enable: bool,
    pub fragrance_intensity: u8,
    pub fragrance_cartridge_type: u8,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            fragrance_selection: frame.fragrance_selection(),
            fragrance_diffuser_enable: frame.fragrance_diffuser_enable(),
            fragrance_intensity: frame.fragrance_intensity(),
            fragrance_cartridge_type: frame.fragrance_cartridge_type(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x1d0 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_fragrance_selection(self.fragrance_selection);
        frame.set_fragrance_diffuser_enable(self.fragrance_diffuser_enable);
        frame.set_fragrance_intensity(self.fragrance_intensity);
        frame.set_fragrance_cartridge_type(self.fragrance_cartridge_type);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x1d0")?;
        writeln!(f, " fragrance_selection={}", self.fragrance_selection)?;
        writeln!(f, " fragrance_diffuser_enable={}", self.fragrance_diffuser_enable)?;
        writeln!(f, " fragrance_intensity={}", self.fragrance_intensity)?;
        writeln!(f, " fragrance_cartridge_type={}", self.fragrance_cartridge_type)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::Error;

    static REPR_FRAME_BYTES_1: [u8; 1] = [0xfd];
    static REPR_FRAME_BYTES_2: [u8; 1] = [0xc8];

    fn frame_1_repr() -> Repr {
        Repr {
            fragrance_selection: 1,
            fragrance_diffuser_enable: true,
            fragrance_intensity: 3,
            fragrance_cartridge_type: 7,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            fragrance_selection: 0,
            fragrance_diffuser_enable: false,
            fragrance_intensity: 1,
            fragrance_cartridge_type: 6,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.fragrance_selection(), 1);
        assert_eq!(frame.fragrance_diffuser_enable(), true);
        assert_eq!(frame.fragrance_intensity(), 3);
        assert_eq!(frame.fragrance_cartridge_type(), 7);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.fragrance_selection(), 0);
        assert_eq!(frame.fragrance_diffuser_enable(), false);
        assert_eq!(frame.fragrance_intensity(), 1);
        assert_eq!(frame.fragrance_cartridge_type(), 6);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 1];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_fragrance_selection(1);
        frame.set_fragrance_diffuser_enable(true);
        frame.set_fragrance_intensity(3);
        frame.set_fragrance_cartridge_type(7);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 1];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_fragrance_selection(0);
        frame.set_fragrance_diffuser_enable(false);
        frame.set_fragrance_intensity(1);
        frame.set_fragrance_cartridge_type(6);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 2] = [0x00, 0xff];
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
    fn test_repr_1_parse_valid() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        let repr = Repr::parse(&frame).unwrap();
        assert_eq!(repr, frame_1_repr());
    }

    #[test]
    fn test_repr_2_parse_valid() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        let repr = Repr::parse(&frame).unwrap();
        assert_eq!(repr, frame_2_repr());
    }

    #[test]
    fn test_basic_repr_1_emit() {
        let mut buf = [0u8; 1];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_basic_repr_2_emit() {
        let mut buf = [0u8; 1];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_2_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }
}
