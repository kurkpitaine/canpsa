use core::{fmt, time::Duration};

use heapless::String;

use crate::{Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 8-bit VIS first char.
    pub const VIS_1: usize = 0;
    /// 8-bit VIS second char.
    pub const VIS_2: usize = 1;
    /// 8-bit VIS third char.
    pub const VIS_3: usize = 2;
    /// 8-bit VIS fourth char.
    pub const VIS_4: usize = 3;
    /// 8-bit VIS fifth char.
    pub const VIS_5: usize = 4;
    /// 8-bit VIS sixth char.
    pub const VIS_6: usize = 5;
    /// 8-bit VIS seventh char.
    pub const VIS_7: usize = 6;
    /// 8-bit VIS eighth char.
    pub const VIS_8: usize = 7;
}

/// Length of a x2b6 CAN frame.
pub const FRAME_LEN: usize = field::VIS_8 + 1;

/// Periodicity of a x2b6 CAN frame.
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

    /// Return the VIS first char.
    #[inline]
    pub fn vis_first_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VIS_1].into()
    }

    /// Return the VIS second char.
    #[inline]
    pub fn vis_second_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VIS_2].into()
    }

    /// Return the VIS third char.
    #[inline]
    pub fn vis_third_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VIS_3].into()
    }

    /// Return the VIS fourth char.
    #[inline]
    pub fn vis_fourth_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VIS_4].into()
    }

    /// Return the VIS fifth char.
    #[inline]
    pub fn vis_fifth_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VIS_5].into()
    }

    /// Return the VIS sixth char.
    #[inline]
    pub fn vis_sixth_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VIS_6].into()
    }

    /// Return the VIS seventh char.
    #[inline]
    pub fn vis_seventh_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VIS_7].into()
    }

    /// Return the VIS eighth char.
    #[inline]
    pub fn vis_eighth_char(&self) -> char {
        let data = self.buffer.as_ref();
        data[field::VIS_8].into()
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the VIS first char.
    #[inline]
    pub fn set_vis_first_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VIS_1] = value as u8;
    }

    /// Set the VIS second char.
    #[inline]
    pub fn set_vis_second_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VIS_2] = value as u8;
    }

    /// Set the VIS third char.
    #[inline]
    pub fn set_vis_third_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VIS_3] = value as u8;
    }

    /// Set the VIS fourth char.
    #[inline]
    pub fn set_vis_fourth_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VIS_4] = value as u8;
    }

    /// Set the VIS fifth char.
    #[inline]
    pub fn set_vis_fifth_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VIS_5] = value as u8;
    }

    /// Set the VIS sixth char.
    #[inline]
    pub fn set_vis_sixth_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VIS_6] = value as u8;
    }

    /// Set the VIS seventh char.
    #[inline]
    pub fn set_vis_seventh_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VIS_7] = value as u8;
    }

    /// Set the VIS eighth char.
    #[inline]
    pub fn set_vis_eighth_char(&mut self, value: char) {
        let data = self.buffer.as_mut();
        data[field::VIS_8] = value as u8;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x2b6 ({})", err)?;
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

/// A high-level representation of a x2b6 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub vis: String<8>,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        let mut vis: String<8> = String::new();
        vis.push(frame.vis_first_char()).unwrap();
        vis.push(frame.vis_second_char()).unwrap();
        vis.push(frame.vis_third_char()).unwrap();
        vis.push(frame.vis_fourth_char()).unwrap();
        vis.push(frame.vis_fifth_char()).unwrap();
        vis.push(frame.vis_sixth_char()).unwrap();
        vis.push(frame.vis_seventh_char()).unwrap();
        vis.push(frame.vis_eighth_char()).unwrap();

        Ok(Repr { vis })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x2b6 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        let mut vis = self.vis.clone();
        frame.set_vis_eighth_char(vis.pop().unwrap());
        frame.set_vis_seventh_char(vis.pop().unwrap());
        frame.set_vis_sixth_char(vis.pop().unwrap());
        frame.set_vis_fifth_char(vis.pop().unwrap());
        frame.set_vis_fourth_char(vis.pop().unwrap());
        frame.set_vis_third_char(vis.pop().unwrap());
        frame.set_vis_second_char(vis.pop().unwrap());
        frame.set_vis_first_char(vis.pop().unwrap());
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x2b6 vis={}", self.vis)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use heapless::String;

    use crate::Error;

    static REPR_FRAME_BYTES: [u8; 8] = [0x37, 0x34, 0x37, 0x38, 0x30, 0x32, 0x34, 0x38];

    fn frame_repr() -> Repr {
        Repr {
            vis: String::from("74780248"),
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.vis_first_char(), '7');
        assert_eq!(frame.vis_second_char(), '4');
        assert_eq!(frame.vis_third_char(), '7');
        assert_eq!(frame.vis_fourth_char(), '8');
        assert_eq!(frame.vis_fifth_char(), '0');
        assert_eq!(frame.vis_sixth_char(), '2');
        assert_eq!(frame.vis_seventh_char(), '4');
        assert_eq!(frame.vis_eighth_char(), '8');
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_vis_first_char('7');
        frame.set_vis_second_char('4');
        frame.set_vis_third_char('7');
        frame.set_vis_fourth_char('8');
        frame.set_vis_fifth_char('0');
        frame.set_vis_sixth_char('2');
        frame.set_vis_seventh_char('4');
        frame.set_vis_eighth_char('8');

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x37, 0x34, 0x37, 0x38, 0x30, 0x32, 0x34, 0x38, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x37, 0x34, 0x37, 0x38, 0x30, 0x32, 0x34];
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
        let mut buf = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }
}
