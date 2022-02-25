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
    /// 8-bit VDS first char.
    pub const VDS_1: usize = 0;
    /// 8-bit VDS second char.
    pub const VDS_2: usize = 1;
    /// 8-bit VDS third char.
    pub const VDS_3: usize = 2;
    /// 8-bit VDS fourth char.
    pub const VDS_4: usize = 3;
    /// 8-bit VDS fifth char.
    pub const VDS_5: usize = 4;
    /// 8-bit VDS sixth char.
    pub const VDS_6: usize = 5;
}

/// Length of a x3b6 CAN frame.
pub const FRAME_LEN: usize = field::VDS_6 + 1;

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

    /// Return the VDS first char.
    #[inline]
    pub fn vds_first_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VDS_1].into()
    }

    /// Return the VDS second char.
    #[inline]
    pub fn vds_second_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VDS_2].into()
    }

    /// Return the VDS third char.
    #[inline]
    pub fn vds_third_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VDS_3].into()
    }

    /// Return the VDS fourth char.
    #[inline]
    pub fn vds_fourth_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VDS_4].into()
    }

    /// Return the VDS fifth char.
    #[inline]
    pub fn vds_fifth_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VDS_5].into()
    }

    /// Return the VDS sixth char.
    #[inline]
    pub fn vds_sixth_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VDS_6].into()
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the VDS first char.
    #[inline]
    pub fn set_vds_first_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VDS_1] = value as u8;
    }

    /// Set the VDS second char.
    #[inline]
    pub fn set_vds_second_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VDS_2] = value as u8;
    }

    /// Set the VDS third char.
    #[inline]
    pub fn set_vds_third_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VDS_3] = value as u8;
    }

    /// Set the VDS fourth char.
    #[inline]
    pub fn set_vds_fourth_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VDS_4] = value as u8;
    }

    /// Set the VDS fifth char.
    #[inline]
    pub fn set_vds_fifth_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VDS_5] = value as u8;
    }

    /// Set the VDS sixth char.
    #[inline]
    pub fn set_vds_sixth_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VDS_6] = value as u8;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x3b6 ({})", err)?;
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

/// A high-level representation of a x3b6 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub vds: String<6>,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        let mut vds: String<6> = String::new();
        vds.push(frame.vds_first_char()).unwrap();
        vds.push(frame.vds_second_char()).unwrap();
        vds.push(frame.vds_third_char()).unwrap();
        vds.push(frame.vds_fourth_char()).unwrap();
        vds.push(frame.vds_fifth_char()).unwrap();
        vds.push(frame.vds_sixth_char()).unwrap();

        Ok(Repr { vds })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x3b6 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        let mut vds = self.vds.clone();
        frame.set_vds_sixth_char(vds.pop().unwrap());
        frame.set_vds_fifth_char(vds.pop().unwrap());
        frame.set_vds_fourth_char(vds.pop().unwrap());
        frame.set_vds_third_char(vds.pop().unwrap());
        frame.set_vds_second_char(vds.pop().unwrap());
        frame.set_vds_first_char(vds.pop().unwrap());
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x3b6 vds={}", self.vds)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use heapless::String;

    use crate::Error;

    static REPR_FRAME_BYTES: [u8; 6] = [0x53, 0x41, 0x39, 0x48, 0x52, 0x38];

    fn frame_repr() -> Repr {
        Repr {
            vds: String::from("SA9HR8"),
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.vds_first_char(), 'S');
        assert_eq!(frame.vds_second_char(), 'A');
        assert_eq!(frame.vds_third_char(), '9');
        assert_eq!(frame.vds_fourth_char(), 'H');
        assert_eq!(frame.vds_fifth_char(), 'R');
        assert_eq!(frame.vds_sixth_char(), '8');
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0u8; 6];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_vds_first_char('S');
        frame.set_vds_second_char('A');
        frame.set_vds_third_char('9');
        frame.set_vds_fourth_char('H');
        frame.set_vds_fifth_char('R');
        frame.set_vds_sixth_char('8');

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 7] = [0x83, 0x65, 0x57, 0x72, 0x82, 0x56, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 5] = [0x83, 0x65, 0x57, 0x72, 0x82];
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
        let mut buf = [0u8; 6];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }
}
