use core::{fmt, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};

use crate::{Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
2A1 INFOS_TRAJET1_ODB_CONSO_MOY_GPL_T1_HS7_2A1
2A1 INFOS_TRAJET1_ODB_CONSO_TRAJET1_HS7_2A1         // OK
2A1 INFOS_TRAJET1_ODB_DISTANCE_TRAJET1_HS7_2A1      // OK
2A1 INFOS_TRAJET1_ODB_VITESSE_MOYENNE_T1_HS7_2A1    // OK
*/

mod field {
    use crate::field::Field;
    /// 8-bit trip average speed in kilometer unit.
    pub const AVG_SPD: usize = 0;
    /// 16-bit trip distance in kilometer unit.
    pub const DISTANCE: Field = 1..3;
    /// 16-bit trip average fuel consumption in 0.1 liter/100 km.
    pub const AVG_CONSUMPTION: Field = 3..5;
    /// 16-bit reserved.
    pub const RES: Field = 5..7;
}

/// Length of a x2a1 CAN frame.
pub const FRAME_LEN: usize = field::RES.end;

/// Periodicity of a x2a1 CAN frame.
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

    /// Return the trip average speed in kilometer unit.
    #[inline]
    pub fn average_speed(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::AVG_SPD]
    }

    /// Return the trip distance in kilometer unit.
    #[inline]
    pub fn distance(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::DISTANCE])
    }

    /// Return the trip average fuel consumption in 0.1 liter/100 km.
    #[inline]
    pub fn average_consumption(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::AVG_CONSUMPTION])
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the trip average speed in kilometer unit.
    #[inline]
    pub fn set_average_speed(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::AVG_SPD] = value;
    }

    /// Set the trip distance in kilometer unit.
    #[inline]
    pub fn set_distance(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::DISTANCE], value);
    }

    /// Set the trip average fuel consumption in 0.1 liter/100 km.
    #[inline]
    pub fn set_average_consumption(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::AVG_CONSUMPTION], value);
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x2a1 ({})", err)?;
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

/// A high-level representation of a x2a1 CAN frame.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub average_speed: u8,
    pub distance: u16,
    pub average_consumption: f32,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            average_speed: frame.average_speed(),
            distance: frame.distance(),
            average_consumption: frame.average_consumption() as f32 / 10.0,
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x2a1 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_average_speed(self.average_speed);
        frame.set_distance(self.distance);
        frame.set_average_consumption((self.average_consumption * 10.0) as u16);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x2a1")?;
        writeln!(f, " average_speed={}", self.average_speed)?;
        writeln!(f, " distance={}", self.distance)?;
        writeln!(f, " average_consumption={}", self.average_consumption)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};

    use crate::Error;

    static REPR_FRAME_BYTES: [u8; 7] = [0x1d, 0x03, 0xe3, 0x00, 0x6b, 0x00, 0x00];

    fn frame_repr() -> Repr {
        Repr {
            average_speed: 29,
            distance: 995,
            average_consumption: 10.7,
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.average_speed(), 29);
        assert_eq!(frame.distance(), 995);
        assert_eq!(frame.average_consumption(), 107);
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_average_speed(29);
        frame.set_distance(995);
        frame.set_average_consumption(107);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 8] = [0x1d, 0x03, 0xe3, 0x00, 0x6b, 0x00, 0x00, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 6] = [0x1d, 0x03, 0xe3, 0x00, 0x6b, 0x00];
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
        let mut buf = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }
}
