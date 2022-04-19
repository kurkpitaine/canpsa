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
    /// 1-bit navigation/vocal command push button state flag,
    /// 2-bit empty,
    /// 1-bit trip computer push button state flag,
    /// 2-bit empty,
    /// 1-bit remaining fuel range data validity flag,
    /// 1-bit fuel consumption data validity flag.
    pub const FLAGS: usize = 0;
    /// 16-bit instant fuel consumption in 0.1 liter/100 km.
    pub const INSTANT_CONSUMPTION: Field = 1..3;
    /// 16-bit remaining fuel range in kilometers unit.
    pub const FUEL_RANGE: Field = 3..5;
    /// 16-bit remaining trip distance in kilometers unit.
    pub const REM_TRIP_DIST: Field = 5..7;
}

/// Length of a x221 CAN frame.
pub const FRAME_LEN: usize = field::REM_TRIP_DIST.end;

/// Periodicity of a x221 CAN frame.
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

    /// Return the navigation/vocal command push button state flag.
    #[inline]
    pub fn nav_vocal_command_push_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        (data[field::FLAGS] & 0x01) != 0
    }

    /// Return the trip computer push button state flag.
    #[inline]
    pub fn trip_computer_push_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        (data[field::FLAGS] & 0x08) != 0
    }

    /// Return the fuel autonomy data validity flag.
    #[inline]
    pub fn fuel_autonomy_data_valid(&self) -> bool {
        let data = self.buffer.as_ref();
        (data[field::FLAGS] & 0x40) == 0
    }

    /// Return the fuel consumption data validity flag.
    #[inline]
    pub fn fuel_consumption_data_valid(&self) -> bool {
        let data = self.buffer.as_ref();
        (data[field::FLAGS] & 0x80) == 0
    }

    /// Return the instant fuel consumption in 0.1 liter/100 km.
    #[inline]
    pub fn instant_fuel_consumption(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::INSTANT_CONSUMPTION])
    }

    /// Return the remaining fuel range in kilometers unit.
    #[inline]
    pub fn remaining_fuel_range(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::FUEL_RANGE])
    }

    /// Return the remaining trip distance in kilometers unit.
    #[inline]
    pub fn remaining_trip_distance(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::REM_TRIP_DIST])
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the preconditioning setting reset flag.
    #[inline]
    pub fn set_nav_vocal_command_push_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS];
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::FLAGS] = raw;
    }

    /// Set the trip computer push button state flag.
    #[inline]
    pub fn set_trip_computer_push_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS];
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::FLAGS] = raw;
    }

    /// Set the fuel autonomy data validity flag.
    #[inline]
    pub fn set_fuel_autonomy_data_valid(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS];
        let raw = if !value { raw | 0x40 } else { raw & !0x40 };
        data[field::FLAGS] = raw;
    }

    /// Set the fuel consumption data validity flag.
    #[inline]
    pub fn set_fuel_consumption_data_valid(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS];
        let raw = if !value { raw | 0x80 } else { raw & !0x80 };
        data[field::FLAGS] = raw;
    }

    /// Set the instant fuel consumption in 0.1 liter/100 km.
    #[inline]
    pub fn set_instant_fuel_consumption(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::INSTANT_CONSUMPTION], value);
    }

    /// Set the remaining fuel range in kilometers unit.
    #[inline]
    pub fn set_remaining_fuel_range(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::FUEL_RANGE], value);
    }

    /// Set the remaining trip distance in kilometers unit.
    #[inline]
    pub fn set_remaining_trip_distance(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::REM_TRIP_DIST], value);
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x221 ({})", err)?;
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

/// A high-level representation of a x221 CAN frame.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub nav_vocal_command_push_button_state: bool,
    pub trip_computer_push_button_state: bool,
    pub fuel_autonomy_data_valid: bool,
    pub fuel_consumption_data_valid: bool,
    #[cfg(feature = "float")]
    pub instant_fuel_consumption: f32,
    #[cfg(not(feature = "float"))]
    pub instant_fuel_consumption: u16,
    pub remaining_fuel_range: u16,
    pub remaining_trip_distance: u16,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            nav_vocal_command_push_button_state: frame.nav_vocal_command_push_button_state(),
            trip_computer_push_button_state: frame.trip_computer_push_button_state(),
            fuel_autonomy_data_valid: frame.fuel_autonomy_data_valid(),
            fuel_consumption_data_valid: frame.fuel_consumption_data_valid(),
            #[cfg(feature = "float")]
            instant_fuel_consumption: frame.instant_fuel_consumption() as f32 / 10.0,
            #[cfg(not(feature = "float"))]
            instant_fuel_consumption: frame.instant_fuel_consumption(),
            remaining_fuel_range: frame.remaining_fuel_range(),
            remaining_trip_distance: frame.remaining_trip_distance(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x221 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_nav_vocal_command_push_button_state(self.nav_vocal_command_push_button_state);
        frame.set_trip_computer_push_button_state(self.trip_computer_push_button_state);
        frame.set_fuel_autonomy_data_valid(self.fuel_autonomy_data_valid);
        frame.set_fuel_consumption_data_valid(self.fuel_consumption_data_valid);
        #[cfg(feature = "float")]
        frame.set_instant_fuel_consumption((self.instant_fuel_consumption * 10.0) as u16);
        #[cfg(not(feature = "float"))]
        frame.set_instant_fuel_consumption(self.instant_fuel_consumption);
        frame.set_remaining_fuel_range(self.remaining_fuel_range);
        frame.set_remaining_trip_distance(self.remaining_trip_distance);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x221")?;
        writeln!(
            f,
            " nav_vocal_command_push_button_state={}",
            self.nav_vocal_command_push_button_state
        )?;
        writeln!(
            f,
            " trip_computer_push_button_state={}",
            self.trip_computer_push_button_state
        )?;
        writeln!(
            f,
            " fuel_autonomy_data_valid={}",
            self.fuel_autonomy_data_valid
        )?;
        writeln!(
            f,
            " fuel_consumption_data_valid={}",
            self.fuel_consumption_data_valid
        )?;
        writeln!(
            f,
            " instant_fuel_consumption={}",
            self.instant_fuel_consumption
        )?;
        writeln!(f, " remaining_fuel_range={}", self.remaining_fuel_range)?;
        writeln!(
            f,
            " remaining_trip_distance={}",
            self.remaining_trip_distance
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};

    use crate::Error;

    static REPR_FRAME_BYTES_1: [u8; 7] = [0x81, 0x00, 0x00, 0x00, 0xb9, 0x00, 0x00];
    static REPR_FRAME_BYTES_2: [u8; 7] = [0x48, 0x00, 0x7f, 0x04, 0x01, 0x00, 0xfa];

    fn frame_1_repr() -> Repr {
        Repr {
            nav_vocal_command_push_button_state: true,
            trip_computer_push_button_state: false,
            fuel_autonomy_data_valid: true,
            fuel_consumption_data_valid: false,
            instant_fuel_consumption: 0.0,
            remaining_fuel_range: 185,
            remaining_trip_distance: 0,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            nav_vocal_command_push_button_state: false,
            trip_computer_push_button_state: true,
            fuel_autonomy_data_valid: false,
            fuel_consumption_data_valid: true,
            instant_fuel_consumption: 12.7,
            remaining_fuel_range: 1025,
            remaining_trip_distance: 250,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.nav_vocal_command_push_button_state(), true);
        assert_eq!(frame.trip_computer_push_button_state(), false);
        assert_eq!(frame.fuel_autonomy_data_valid(), true);
        assert_eq!(frame.fuel_consumption_data_valid(), false);
        assert_eq!(frame.instant_fuel_consumption(), 0);
        assert_eq!(frame.remaining_fuel_range(), 185);
        assert_eq!(frame.remaining_trip_distance(), 0);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.nav_vocal_command_push_button_state(), false);
        assert_eq!(frame.trip_computer_push_button_state(), true);
        assert_eq!(frame.fuel_autonomy_data_valid(), false);
        assert_eq!(frame.fuel_consumption_data_valid(), true);
        assert_eq!(frame.instant_fuel_consumption(), 127);
        assert_eq!(frame.remaining_fuel_range(), 1025);
        assert_eq!(frame.remaining_trip_distance(), 250);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_nav_vocal_command_push_button_state(true);
        frame.set_trip_computer_push_button_state(false);
        frame.set_fuel_autonomy_data_valid(true);
        frame.set_fuel_consumption_data_valid(false);
        frame.set_instant_fuel_consumption(0);
        frame.set_remaining_fuel_range(185);
        frame.set_remaining_trip_distance(0);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_nav_vocal_command_push_button_state(false);
        frame.set_trip_computer_push_button_state(true);
        frame.set_fuel_autonomy_data_valid(false);
        frame.set_fuel_consumption_data_valid(true);
        frame.set_instant_fuel_consumption(127);
        frame.set_remaining_fuel_range(1025);
        frame.set_remaining_trip_distance(250);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 8] = [0x41, 0x00, 0x00, 0x00, 0xb9, 0x00, 0x00, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 6] = [0x41, 0x00, 0x00, 0x00, 0xb9, 0x00];
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
        let mut buf = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_basic_repr_2_emit() {
        let mut buf = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_2_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }
}
