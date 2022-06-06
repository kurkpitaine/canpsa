use core::{cmp::Ordering, fmt, time::Duration};

use crate::{Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
2AD CDE_IHM_CLIM_AIRQ_PURIF_AQI_INT_LEVEL_HS7_2AD
2AD CDE_IHM_CLIM_AUTOR_RESTORE_HS7_2AD
2AD CDE_IHM_CLIM_CABIN_AIR_PURIFIER_STATE_HS7_2AD
2AD CDE_IHM_CLIM_CLEAN_CABIN_FILTER_STATE_HS7_2AD
2AD CDE_IHM_CLIM_CLEAN_CABIN_STATE_HS7_2AD
2AD CDE_IHM_CLIM_CMD_IONIZER_HS7_2AD
2AD CDE_IHM_CLIM_CMD_LED_HEATING_STRWHL_HS7_2AD
2AD CDE_IHM_CLIM_CONS_ENTREE_AIR_HS7_2AD
2AD CDE_IHM_CLIM_CONS_PULSEUR_ARG_HS7_2AD
2AD CDE_IHM_CLIM_CONS_PULSEUR_AVANT_HS7_2AD
2AD CDE_IHM_CLIM_CONS_TEMP_CENT_HS7_2AD             // OK
2AD CDE_IHM_CLIM_ETAT_ELEC_IHM_CLIM_HS7_2AD
2AD CDE_IHM_CLIM_EXTINCTION_LCD_CONDA_HS7_2AD
2AD CDE_IHM_CLIM_IONIZER_STATE_HS7_2AD
2AD CDE_IHM_CLIM_LEDLTG_QUICKSTRTCMFT_HS7_2AD
2AD CDE_IHM_CLIM_PRESENCE_IHM_AR_CENTRAL_HS7_2AD
2AD CDE_IHM_CLIM_PRESENCE_PBC_HS7_2AD
2AD CDE_IHM_CLIM_PRESENCE_SIEGES_CLIM_AR_HS7_2AD
2AD CDE_IHM_CLIM_PRESENCE_SIEGES_CLIM_HS7_2AD
2AD CDE_IHM_CLIM_P_SHORTCUT_CLOSE_CMFT_HS7_2AD
2AD CDE_IHM_CLIM_PULS_ARG_MOINS_HS7_2AD
2AD CDE_IHM_CLIM_PULS_ARG_PLUS_HS7_2AD
2AD CDE_IHM_CLIM_PULS_AV_MOINS_HS7_2AD
2AD CDE_IHM_CLIM_PULS_AV_PLUS_HS7_2AD
2AD CDE_IHM_CLIM_TYPE_CLIM_HS7_2AD
2AD CDE_IHM_CLIM_VOY_LUCH_HS7_2AD
2AD CDE_IHM_CLIM_VOY_PBC_HS7_2AD
*/

mod field {
    /// 2-bit unknown,
    /// 3-bit central temperature instruction value field,
    /// 3-bit unknown.
    pub const AC_0: usize = 0;
    /// 8-bit unknown.
    pub const _AC_1: usize = 1;
    /// 8-bit unknown.
    pub const _AC_2: usize = 2;
    /// 8-bit unknown.
    pub const _AC_3: usize = 3;
    /// 8-bit unknown.
    pub const _AC_4: usize = 4;
    /// 8-bit unknown.
    pub const _AC_5: usize = 5;
    /// 8-bit unknown.
    pub const _AC_6: usize = 6;
    /// 8-bit unknown.
    pub const AC_7: usize = 7;
}

/// Length of a x2ad CAN frame.
pub const FRAME_LEN: usize = field::AC_7 + 1;

/// Periodicity of a x2ad CAN frame.
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

    /// Return the central temperature instruction value field.
    #[inline]
    pub fn central_temperature(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::AC_0] & 0x1c) >> 2
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the central temperature instruction value  field.
    #[inline]
    pub fn set_central_temperature(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_0] & !0x1c;
        let raw = raw | ((value << 2) & 0x1c);
        data[field::AC_0] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x2ad ({})", err)?;
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

/// A high-level representation of a x2ad CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub central_temperature: u8,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            central_temperature: frame.central_temperature(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x2ad CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_central_temperature(self.central_temperature);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x2ad")?;
        writeln!(f, " central_temperature={}", self.central_temperature)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::Error;

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    fn frame_1_repr() -> Repr {
        Repr {
            central_temperature: 7,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            central_temperature: 5,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.central_temperature(), 7);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.central_temperature(), 5);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_central_temperature(7);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_central_temperature(5);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
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
