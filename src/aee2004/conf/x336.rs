use core::fmt;

use heapless::String;

use crate::{Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 8-bit WMI first char.
    pub const WMI_1: usize = 0;
    /// 8-bit WMI second char.
    pub const WMI_2: usize = 1;
    /// 8-bit WMI third char.
    pub const WMI_3: usize = 2;
}

/// Length of a x336 CAN frame.
pub const FRAME_LEN: usize = field::WMI_3 + 1;

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

    /// Return the WMI first char.
    #[inline]
    pub fn wmi_first_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::WMI_1].into()
    }

    /// Return the WMI second char.
    #[inline]
    pub fn wmi_second_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::WMI_2].into()
    }

    /// Return the WMI third char.
    #[inline]
    pub fn wmi_third_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::WMI_3].into()
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the WMI first char.
    #[inline]
    pub fn set_wmi_first_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::WMI_1] = value as u8;
    }

    /// Set the WMI second char.
    #[inline]
    pub fn set_wmi_second_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::WMI_2] = value as u8;
    }

    /// Set the WMI third char.
    #[inline]
    pub fn set_wmi_third_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::WMI_3] = value as u8;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x336 ({})", err)?;
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

/// A high-level representation of a x336 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub wmi: String<3>,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        let mut wmi: String<3> = String::new();
        wmi.push(frame.wmi_first_char()).unwrap();
        wmi.push(frame.wmi_second_char()).unwrap();
        wmi.push(frame.wmi_third_char()).unwrap();

        Ok(Repr { wmi })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x336 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        let mut wmi = self.wmi.clone();
        frame.set_wmi_third_char(wmi.pop().unwrap());
        frame.set_wmi_second_char(wmi.pop().unwrap());
        frame.set_wmi_first_char(wmi.pop().unwrap());
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x336 wmi={}", self.wmi)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use heapless::String;

    use crate::Error;

    static REPR_FRAME_BYTES: [u8; 3] = [0x56, 0x46, 0x37];

    fn frame_repr() -> Repr {
        Repr {
            wmi: String::from("VF7"),
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.wmi_first_char(), 'V');
        assert_eq!(frame.wmi_second_char(), 'F');
        assert_eq!(frame.wmi_third_char(), '7');
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0u8; 3];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_wmi_first_char('V');
        frame.set_wmi_second_char('F');
        frame.set_wmi_third_char('7');

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 4] = [0x56, 0x46, 0x37, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 2] = [0x56, 0x46];
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
        let mut buf = [0u8; 3];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }
}
