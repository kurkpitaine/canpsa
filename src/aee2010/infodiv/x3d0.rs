use core::{cmp::Ordering, fmt, time::Duration};

use crate::{Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
3D0 ETAT_CLIM_AR_DISTRIBUTION_ARD_HS7_3D0
3D0 ETAT_CLIM_AR_DISTRIBUTION_ARG_HS7_3D0
3D0 ETAT_CLIM_AR_DMD_SIEGE_CHAUF_ARD_HS7_3D0
3D0 ETAT_CLIM_AR_DMD_SIEGE_CHAUF_ARG_HS7_3D0
3D0 ETAT_CLIM_AR_DMD_SIEGE_VENTIL_ARD_HS7_3D0
3D0 ETAT_CLIM_AR_DMD_SIEGE_VENTIL_ARG_HS7_3D0
3D0 ETAT_CLIM_AR_ETAT_REAR_HS7_3D0              // OK
3D0 ETAT_CLIM_AR_PULS_ARD_HS7_3D0
3D0 ETAT_CLIM_AR_PULS_ARG_HS7_3D0
3D0 ETAT_CLIM_AR_UB_ARD_HS7_3D0
3D0 ETAT_CLIM_AR_UB_ARG_HS7_3D0
3D0 ETAT_CLIM_AR_VAL_CONS_TEMP_ARD_HS7_3D0      // OK
3D0 ETAT_CLIM_AR_VAL_CONS_TEMP_ARG_HS7_3D0      // OK
*/

mod field {
    /// 8-bit unknown.
    pub const _AC_0: usize = 0;
    /// 5-bit rear left temperature value instruction field,
    /// 3-bit unknown.
    pub const AC_1: usize = 1;
    /// 5-bit rear right temperature value instruction field,
    /// 3-bit unknown.
    pub const AC_2: usize = 2;
    /// 2-bit rear A/C state field,
    /// 6-bit unknown.
    pub const AC_3: usize = 3;
    /// 8-bit unknown.
    pub const _AC_4: usize = 4;
    /// 8-bit unknown.
    pub const AC_5: usize = 5;
}

/// Length of a x3d0 CAN frame.
pub const FRAME_LEN: usize = field::AC_5 + 1;

/// Periodicity of a x3d0 CAN frame.
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

    /// Return the rear left temperature value instruction field.
    #[inline]
    pub fn rear_left_temp(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::AC_1] & 0x1f
    }

    /// Return the rear right temperature value instruction field.
    #[inline]
    pub fn rear_right_temp(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::AC_2] & 0x1f
    }

    /// Return the rear A/C state field.
    #[inline]
    pub fn rear_ac_state(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::AC_3] & 0x03
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the rear left temperature value instruction field.
    #[inline]
    pub fn set_rear_left_temp(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_1] & !0x1f;
        let raw = raw | (value & 0x1f);
        data[field::AC_1] = raw;
    }

    /// Set the rear right temperature value instruction field.
    #[inline]
    pub fn set_rear_right_temp(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_2] & !0x1f;
        let raw = raw | (value & 0x1f);
        data[field::AC_2] = raw;
    }

    /// Set the rear A/C state field.
    #[inline]
    pub fn set_rear_ac_state(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_3] & !0x03;
        let raw = raw | (value & 0x03);
        data[field::AC_3] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x3d0 ({})", err)?;
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

/// A high-level representation of a x3d0 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub rear_left_temp: u8,
    pub rear_right_temp: u8,
    pub rear_ac_state: u8,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            rear_left_temp: frame.rear_left_temp(),
            rear_right_temp: frame.rear_right_temp(),
            rear_ac_state: frame.rear_ac_state(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x3d0 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_rear_left_temp(self.rear_left_temp);
        frame.set_rear_right_temp(self.rear_right_temp);
        frame.set_rear_ac_state(self.rear_ac_state);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x3d0")?;
        writeln!(f, " rear_left_temp={}", self.rear_left_temp)?;
        writeln!(f, " rear_right_temp={}", self.rear_right_temp)?;
        writeln!(f, " rear_ac_state={}", self.rear_ac_state)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::Error;

    static REPR_FRAME_BYTES_1: [u8; 6] = [0x00, 0x14, 0x14, 0x02, 0x00, 0x00];
    static REPR_FRAME_BYTES_2: [u8; 6] = [0x00, 0x11, 0x13, 0x01, 0x00, 0x00];

    fn frame_1_repr() -> Repr {
        Repr {
            rear_left_temp: 20,
            rear_right_temp: 20,
            rear_ac_state: 2,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            rear_left_temp: 17,
            rear_right_temp: 19,
            rear_ac_state: 1,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.rear_left_temp(), 20);
        assert_eq!(frame.rear_right_temp(), 20);
        assert_eq!(frame.rear_ac_state(), 2);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.rear_left_temp(), 17);
        assert_eq!(frame.rear_right_temp(), 19);
        assert_eq!(frame.rear_ac_state(), 1);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0u8; 6];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_rear_left_temp(20);
        frame.set_rear_right_temp(20);
        frame.set_rear_ac_state(2);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0u8; 6];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_rear_left_temp(17);
        frame.set_rear_right_temp(19);
        frame.set_rear_ac_state(1);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 7] = [0x00, 0x14, 0x14, 0x02, 0x00, 0x00, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 5] = [0x00, 0x14, 0x14, 0x02, 0x00];
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
        let mut buf = [0u8; 6];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_basic_repr_2_emit() {
        let mut buf = [0u8; 6];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_2_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }
}
