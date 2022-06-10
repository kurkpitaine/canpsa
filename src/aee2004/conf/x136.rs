use core::{cmp::Ordering, fmt, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};

use crate::{Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    use crate::field::Field;
    /// 14-bit AdBlue autonomy,
    /// 1-bit empty,
    /// 1-bit AdBlue autonomy display request.
    pub const FLAGS_ADBLUE_AUTONOMY: Field = 0..2;
}

/// Raw x136 CAN frame identifier.
pub const FRAME_ID: u16 = 0x136;
/// Length of a x136 CAN frame.
pub const FRAME_LEN: usize = field::FLAGS_ADBLUE_AUTONOMY.end;

/// Periodicity of a x136 CAN frame.
pub const PERIODICITY: Duration = Duration::from_millis(500);

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

    /// Return the Adblue autonomy field.
    #[inline]
    pub fn adblue_autonomy(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::FLAGS_ADBLUE_AUTONOMY]) & 0x3fff
    }

    /// Return the AdBlue autonomy display request field.
    #[inline]
    pub fn adblue_autonomy_display_request(&self) -> bool {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_ADBLUE_AUTONOMY]);
        raw & !0x7fff != 0
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the Adblue autonomy field.
    #[inline]
    pub fn set_adblue_autonomy(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_ADBLUE_AUTONOMY]);
        let raw = raw | (value & 0x3fff);
        NetworkEndian::write_u16(&mut data[field::FLAGS_ADBLUE_AUTONOMY], raw);
    }

    /// Set the AdBlue autonomy display request field.
    #[inline]
    pub fn set_adblue_autonomy_display_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_ADBLUE_AUTONOMY]);
        let raw = if value { raw | 0x8000 } else { raw & !0x8000 };
        NetworkEndian::write_u16(&mut data[field::FLAGS_ADBLUE_AUTONOMY], raw);
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x136 ({})", err)?;
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

/// A high-level representation of a x136 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub adblue_autonomy: u16,
    pub adblue_autonomy_display_request: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            adblue_autonomy: frame.adblue_autonomy(),
            adblue_autonomy_display_request: frame.adblue_autonomy_display_request(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x136 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_adblue_autonomy(self.adblue_autonomy);
        frame.set_adblue_autonomy_display_request(self.adblue_autonomy_display_request);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x136 adblue_autonomy={}", self.adblue_autonomy)?;
        writeln!(
            f,
            " adblue_autonomy_display_request={}",
            self.adblue_autonomy_display_request
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::Error;

    static REPR_FRAME_BYTES: [u8; 2] = [0x3f, 0xfe];

    fn frame_repr() -> Repr {
        Repr {
            adblue_autonomy: 16382,
            adblue_autonomy_display_request: false,
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.adblue_autonomy(), 0x3ffe);
        assert_eq!(frame.adblue_autonomy_display_request(), false);
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0u8; 2];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_adblue_autonomy(0x3ffe);
        frame.set_adblue_autonomy_display_request(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 3] = [0x3f, 0xfe, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 1] = [0x3f];
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
