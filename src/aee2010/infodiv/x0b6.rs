use core::{cmp::Ordering, fmt, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};

use crate::{vehicle::SpeedValidity, Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
0B6 DONNEES_VSM_RAPIDES_CONSO_HS7_0B6           // OK
0B6 DONNEES_VSM_RAPIDES_DIST_HS7_0B6            // OK
0B6 DONNEES_VSM_RAPIDES_SECU_VITESSE_HS7_0B6    // OK
0B6 DONNEES_VSM_RAPIDES_SECU_VITV_HS7_0B6       // OK
0B6 DONNEES_VSM_RAPIDES_VITM_HS7_0B6            // OK
0B6 DONNEES_VSM_RAPIDES_VITV_HS7_0B6            // OK
*/

mod field {
    use crate::field::*;
    /// 16-bit engine revolution per minute in 0.125 rpm units.
    pub const ENGINE_RPM: Field = 0..2;
    /// 16-bit vehicle immediate speed measured on the driving wheels, in 0.01 km/h.
    pub const VEHICLE_SPD: Field = 2..4;
    /// 16-bit odometer value since start of vehicle incremented at each distance top, in cm.
    pub const ODOMETER: Field = 4..6;
    /// 16-bit fuel consumption since start of vehicle, in mm3.
    pub const FUEL_CONSUMPTION: usize = 6;
    /// 3-bit empty,
    /// 4-bit vehicle speed value validity field,
    /// 1-bit vehicle immediate speed value validity flag.
    pub const VALIDITY: usize = 7;
}

/// Raw x0b6 CAN frame identifier.
pub const FRAME_ID: u16 = 0x0b6;
/// Length of a x0b6 CAN frame.
pub const FRAME_LEN: usize = field::VALIDITY + 1;

/// Periodicity of a x0b6 CAN frame.
pub const PERIODICITY: Duration = Duration::from_millis(50);

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

    /// Return the engine revolution per minute field in 0.1 rpm units.
    #[inline]
    pub fn engine_rpm(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::ENGINE_RPM])
    }

    /// Return the vehicle immediate speed measured on the driving wheels field, in 0.01 km/h.
    #[inline]
    pub fn vehicle_immediate_speed(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::VEHICLE_SPD])
    }

    /// Return the odometer value since start of vehicle field, incremented at each distance top.
    #[inline]
    pub fn trip_odometer(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::ODOMETER])
    }

    /// Return the fuel consumption since start of vehicle field.
    #[inline]
    pub fn trip_fuel_consumption(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::FUEL_CONSUMPTION]
    }

    /// Return the vehicle speed value validity field.
    #[inline]
    pub fn speed_validity(&self) -> SpeedValidity {
        let data = self.buffer.as_ref();
        let raw = (data[field::VALIDITY] & 0x78) >> 3;
        SpeedValidity::from(raw)
    }

    /// Return the vehicle immediate speed value validity flag.
    #[inline]
    pub fn immediate_speed_validity(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::VALIDITY] & 0x80 != 0
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the engine revolution per minute field in 0.1 rpm units.
    #[inline]
    pub fn set_engine_rpm(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::ENGINE_RPM], value);
    }

    /// Set the vehicle immediate speed measured on the driving wheels field, in 0.01 km/h.
    #[inline]
    pub fn set_vehicle_immediate_speed(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::VEHICLE_SPD], value);
    }

    /// Set the odometer value since start of vehicle field, incremented at each distance top.
    #[inline]
    pub fn set_trip_odometer(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::ODOMETER], value);
    }

    /// Set the fuel consumption since start of vehicle field.
    #[inline]
    pub fn set_trip_fuel_consumption(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::FUEL_CONSUMPTION] = value;
    }

    /// Set the vehicle speed value validity field.
    #[inline]
    pub fn set_speed_validity(&mut self, value: SpeedValidity) {
        let data = self.buffer.as_mut();
        let raw = data[field::VALIDITY] & !0x78;
        let raw = raw | ((u8::from(value) << 3) & 0x78);
        data[field::VALIDITY] = raw;
    }

    /// Set the vehicle immediate speed value validity flag.
    #[inline]
    pub fn set_immediate_speed_validity(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::VALIDITY];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::VALIDITY] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x0b6 ({})", err)?;
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

/// A high-level representation of a x0b6 CAN frame.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    #[cfg(feature = "float")]
    pub engine_rpm: f32,
    #[cfg(not(feature = "float"))]
    pub engine_rpm: u16,
    #[cfg(feature = "float")]
    pub vehicle_immediate_speed: f32,
    #[cfg(not(feature = "float"))]
    pub vehicle_immediate_speed: u16,
    pub trip_odometer: u16,
    pub trip_fuel_consumption: u8,
    pub speed_validity: SpeedValidity,
    pub immediate_speed_validity: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            #[cfg(feature = "float")]
            engine_rpm: frame.engine_rpm() as f32 / 10.0,
            #[cfg(not(feature = "float"))]
            engine_rpm: frame.engine_rpm(),
            #[cfg(feature = "float")]
            vehicle_immediate_speed: frame.vehicle_immediate_speed() as f32 / 100.0,
            #[cfg(not(feature = "float"))]
            vehicle_immediate_speed: frame.vehicle_immediate_speed(),
            trip_odometer: frame.trip_odometer(),
            trip_fuel_consumption: frame.trip_fuel_consumption(),
            speed_validity: frame.speed_validity(),
            immediate_speed_validity: frame.immediate_speed_validity(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x0b6 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        #[cfg(feature = "float")]
        frame.set_engine_rpm((self.engine_rpm * 10.0) as u16);
        #[cfg(not(feature = "float"))]
        frame.set_engine_rpm(self.engine_rpm);
        #[cfg(feature = "float")]
        frame.set_vehicle_immediate_speed((self.vehicle_immediate_speed * 100.0) as u16);
        #[cfg(not(feature = "float"))]
        frame.set_vehicle_immediate_speed(self.vehicle_immediate_speed);
        frame.set_trip_odometer(self.trip_odometer);
        frame.set_trip_fuel_consumption(self.trip_fuel_consumption);
        frame.set_speed_validity(self.speed_validity);
        frame.set_immediate_speed_validity(self.immediate_speed_validity);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x0b6 engine_rpm={}", self.engine_rpm)?;
        writeln!(
            f,
            " vehicle_immediate_speed={}",
            self.vehicle_immediate_speed
        )?;
        writeln!(f, " trip_odometer={}", self.trip_odometer)?;
        writeln!(f, " trip_fuel_consumption={}", self.trip_fuel_consumption)?;
        writeln!(f, " speed_validity={}", self.speed_validity)?;
        writeln!(
            f,
            " immediate_speed_validity={}",
            self.immediate_speed_validity
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{vehicle::SpeedValidity, Error};

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x18, 0xa7, 0x00, 0x00, 0x00, 0x00, 0x42, 0xd0];

    fn frame_1_repr() -> Repr {
        Repr {
            engine_rpm: 631.1,
            vehicle_immediate_speed: 0.0,
            trip_odometer: 0,
            trip_fuel_consumption: 66,
            speed_validity: SpeedValidity::Valid,
            immediate_speed_validity: true,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.engine_rpm(), 0x18a7);
        assert_eq!(frame.vehicle_immediate_speed(), 0);
        assert_eq!(frame.trip_odometer(), 0);
        assert_eq!(frame.trip_fuel_consumption(), 0x42);
        assert_eq!(frame.speed_validity(), SpeedValidity::Valid);
        assert_eq!(frame.immediate_speed_validity(), true);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_engine_rpm(0x18a7);
        frame.set_vehicle_immediate_speed(0);
        frame.set_vehicle_immediate_speed(0);
        frame.set_trip_odometer(0);
        frame.set_trip_fuel_consumption(0x42);
        frame.set_speed_validity(SpeedValidity::Valid);
        frame.set_immediate_speed_validity(true);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x18, 0xa7, 0x00, 0x00, 0x00, 0x00, 0x42, 0xd0, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x18, 0xa7, 0x00, 0x00, 0x00, 0x00, 0x42];
        assert_eq!(Frame::new_checked(&bytes).unwrap_err(), Error::Truncated);
    }

    #[test]
    fn test_repr_1_parse_valid() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        let repr = Repr::parse(&frame).unwrap();
        assert_eq!(repr, frame_1_repr());
    }

    #[test]
    fn test_basic_repr_1_emit() {
        let mut buf = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }
}
