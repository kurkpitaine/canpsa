use core::fmt;

use byteorder::{ByteOrder, NetworkEndian};

use crate::{Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
1A8 GESTION_VITESSE_DMD_COULEUR_TACHY_HY_HS7_1A8
1A8 GESTION_VITESSE_ODO_PARTIEL_HS7_1A8             // OK
1A8 GESTION_VITESSE_XVV_BUTEE_ATTEINTE_HS7_1A8      // OK
1A8 GESTION_VITESSE_XVV_BUTEE_INF_HS7_1A8
1A8 GESTION_VITESSE_XVV_BUTEE_SUP_HS7_1A8
1A8 GESTION_VITESSE_XVV_PREPROG_ACTIF_HS7_1A8       // OK
*/

mod field {
    use crate::field::Field;
    /// 1-bit limit reached flag,
    /// 1-bit pre-programming state flag,
    /// 6-bit unknown.
    pub const FLAGS: usize = 0;
    /// 32-bit unknown.
    pub const UNKNOWN: Field = 1..5;
    /// 24-bit partial odometer field, in 0.1 kilometers units.
    pub const ODOMETER: Field = 5..8;
}

/// Length of a x1a8 CAN frame.
pub const FRAME_LEN: usize = field::ODOMETER.end;

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

    /// Return the limit reached flag.
    #[inline]
    pub fn limit_reached(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS] & 0x01 != 0
    }

    /// Return the pre-programming state flag.
    #[inline]
    pub fn pre_programming_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS] & 0x02 != 0
    }

    /// Return the partial odometer field, in 0.1 kilometers units.
    #[inline]
    pub fn partial_odometer(&self) -> u32 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u24(&data[field::ODOMETER])
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the limit reached flag.
    #[inline]
    pub fn set_limit_reached(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS];
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::FLAGS] = raw;
    }

    /// Set the pre-programming state flag.
    #[inline]
    pub fn set_pre_programming_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS];
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::FLAGS] = raw;
    }

    /// Set the partial odometer field, in 0.1 kilometers units.
    #[inline]
    pub fn set_partial_odometer(&mut self, value: u32) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u24(&mut data[field::ODOMETER], value);
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x1a8 ({})", err)?;
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

/// A high-level representation of a x1a8 CAN frame.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub limit_reached: bool,
    pub pre_programming_state: bool,
    pub partial_odometer: f32,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            limit_reached: frame.limit_reached(),
            pre_programming_state: frame.pre_programming_state(),
            partial_odometer: (frame.partial_odometer() as f32 / 10.0),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x1a8 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_limit_reached(self.limit_reached);
        frame.set_pre_programming_state(self.pre_programming_state);
        frame.set_partial_odometer((self.partial_odometer * 10.0) as u32);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x1a8")?;
        write!(f, " limit_reached={}", self.limit_reached)?;
        write!(f, " pre_programming_state={}", self.pre_programming_state)?;
        write!(f, " partial_odometer={}", self.partial_odometer)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};

    use crate::Error;

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x84];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0xb6];

    fn frame_1_repr() -> Repr {
        Repr {
            limit_reached: true,
            pre_programming_state: false,
            partial_odometer: 653.2,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            limit_reached: false,
            pre_programming_state: true,
            partial_odometer: 325.4,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.limit_reached(), true);
        assert_eq!(frame.pre_programming_state(), false);
        assert_eq!(frame.partial_odometer(), 6532);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.limit_reached(), false);
        assert_eq!(frame.pre_programming_state(), true);
        assert_eq!(frame.partial_odometer(), 3254);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_limit_reached(true);
        frame.set_pre_programming_state(false);
        frame.set_partial_odometer(6532);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_limit_reached(false);
        frame.set_pre_programming_state(true);
        frame.set_partial_odometer(3254);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x44, 0x00, 0x82, 0x00, 0x00, 0x00, 0x19, 0x84, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x44, 0x00, 0x82, 0x00, 0x00, 0x00, 0x19];
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
        let mut buf = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_basic_repr_2_emit() {
        let mut buf = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_2_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }
}
