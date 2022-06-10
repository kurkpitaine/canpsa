use core::{cmp::Ordering, fmt, time::Duration};

use crate::{vehicle::BodyType, Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 1-bit fuel cap opened flag,
    /// 1-bit rear windscreen opened flag,
    /// 1-bit bonnet opened flag,
    /// 1-bit boot opened flag,
    /// 1-bit rear right door opened flag,
    /// 1-bit rear left door opened flag,
    /// 1-bit front right door opened flag,
    /// 1-bit front left door opened flag.
    pub const FLAGS_0: usize = 0;
    /// 6-bit empty,
    /// 1-bit spare wheel arm opened flag,
    /// 1-bit vehicle body type flag.
    pub const FLAGS_1: usize = 1;
}

/// Raw x220 CAN frame identifier.
pub const FRAME_ID: u16 = 0x220;
/// Length of a x220 CAN frame.
pub const FRAME_LEN: usize = field::FLAGS_1 + 1;

/// Periodicity of a x220 CAN frame.
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

    /// Return the opening state in byte B at index I.
    #[inline]
    pub fn read_opening_state<const B: usize, const I: u8>(&self) -> bool {
        let data = self.buffer.as_ref();
        (data[B] & (1u8 << I)) != 0
    }

    /// Return the vehicle body type flag.
    #[inline]
    pub fn vehicle_body_type(&self) -> BodyType {
        let data = self.buffer.as_ref();
        let raw = data[field::FLAGS_1] >> 7;
        BodyType::from(raw)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the opening state in byte B at index I.
    #[inline]
    pub fn write_opening_state<const B: usize, const I: u8>(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let mask = 1u8 << I;
        let raw = data[B];
        let raw = if value { raw | mask } else { raw & !mask };
        data[B] = raw;
    }

    /// Set the vehicle body type flag.
    #[inline]
    pub fn set_vehicle_body_type(&mut self, value: BodyType) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_1];
        let raw = raw | (u8::from(value) << 7);
        data[field::FLAGS_1] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x220 ({})", err)?;
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

/// A high-level representation of a x220 CAN frame.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub fuel_cap_opened: bool,
    pub rear_windscreen_opened: bool,
    pub bonnet_opened: bool,
    pub boot_opened: bool,
    pub rear_right_door_opened: bool,
    pub rear_left_door_opened: bool,
    pub front_right_door_opened: bool,
    pub front_left_door_opened: bool,
    pub spare_wheel_arm_opened: bool,
    pub vehicle_body_type: BodyType,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            fuel_cap_opened: frame.read_opening_state::<{ field::FLAGS_0 }, 0>(),
            rear_windscreen_opened: frame.read_opening_state::<{ field::FLAGS_0 }, 1>(),
            bonnet_opened: frame.read_opening_state::<{ field::FLAGS_0 }, 2>(),
            boot_opened: frame.read_opening_state::<{ field::FLAGS_0 }, 3>(),
            rear_right_door_opened: frame.read_opening_state::<{ field::FLAGS_0 }, 4>(),
            rear_left_door_opened: frame.read_opening_state::<{ field::FLAGS_0 }, 5>(),
            front_right_door_opened: frame.read_opening_state::<{ field::FLAGS_0 }, 6>(),
            front_left_door_opened: frame.read_opening_state::<{ field::FLAGS_0 }, 7>(),
            spare_wheel_arm_opened: frame.read_opening_state::<{ field::FLAGS_1 }, 6>(),
            vehicle_body_type: frame.vehicle_body_type(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x220 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.write_opening_state::<{ field::FLAGS_0 }, 0>(self.fuel_cap_opened);
        frame.write_opening_state::<{ field::FLAGS_0 }, 1>(self.rear_windscreen_opened);
        frame.write_opening_state::<{ field::FLAGS_0 }, 2>(self.bonnet_opened);
        frame.write_opening_state::<{ field::FLAGS_0 }, 3>(self.boot_opened);
        frame.write_opening_state::<{ field::FLAGS_0 }, 4>(self.rear_right_door_opened);
        frame.write_opening_state::<{ field::FLAGS_0 }, 5>(self.rear_left_door_opened);
        frame.write_opening_state::<{ field::FLAGS_0 }, 6>(self.front_right_door_opened);
        frame.write_opening_state::<{ field::FLAGS_0 }, 7>(self.front_left_door_opened);
        frame.write_opening_state::<{ field::FLAGS_1 }, 6>(self.spare_wheel_arm_opened);
        frame.set_vehicle_body_type(self.vehicle_body_type);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x220")?;
        writeln!(f, " fuel_cap_opened={}", self.fuel_cap_opened)?;
        writeln!(f, " rear_windscreen_opened={}", self.rear_windscreen_opened)?;
        writeln!(f, " bonnet_opened={}", self.bonnet_opened)?;
        writeln!(f, " boot_opened={}", self.boot_opened)?;
        writeln!(f, " rear_right_door_opened={}", self.rear_right_door_opened)?;
        writeln!(f, " rear_left_door_opened={}", self.rear_left_door_opened)?;
        writeln!(
            f,
            " front_right_door_opened={}",
            self.front_right_door_opened
        )?;
        writeln!(f, " front_left_door_opened={}", self.front_left_door_opened)?;
        writeln!(f, " spare_wheel_arm_opened={}", self.spare_wheel_arm_opened)?;
        writeln!(f, " vehicle_body_type={}", self.vehicle_body_type)
    }
}

#[cfg(test)]
mod test {
    use super::{field, Frame, Repr};

    use crate::{vehicle::BodyType, Error};

    static REPR_FRAME_BYTES_1: [u8; 2] = [0x55, 0x40];
    static REPR_FRAME_BYTES_2: [u8; 2] = [0xaa, 0x80];

    fn frame_1_repr() -> Repr {
        Repr {
            fuel_cap_opened: true,
            rear_windscreen_opened: false,
            bonnet_opened: true,
            boot_opened: false,
            rear_right_door_opened: true,
            rear_left_door_opened: false,
            front_right_door_opened: true,
            front_left_door_opened: false,
            spare_wheel_arm_opened: true,
            vehicle_body_type: BodyType::FiveDoors,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            fuel_cap_opened: false,
            rear_windscreen_opened: true,
            bonnet_opened: false,
            boot_opened: true,
            rear_right_door_opened: false,
            rear_left_door_opened: true,
            front_right_door_opened: false,
            front_left_door_opened: true,
            spare_wheel_arm_opened: false,
            vehicle_body_type: BodyType::ThreeDoors,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 0>(), true);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 1>(), false);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 2>(), true);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 3>(), false);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 4>(), true);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 5>(), false);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 6>(), true);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 7>(), false);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_1 }, 6>(), true);
        assert_eq!(frame.vehicle_body_type(), BodyType::FiveDoors);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 0>(), false);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 1>(), true);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 2>(), false);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 3>(), true);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 4>(), false);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 5>(), true);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 6>(), false);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_0 }, 7>(), true);
        assert_eq!(frame.read_opening_state::<{ field::FLAGS_1 }, 6>(), false);
        assert_eq!(frame.vehicle_body_type(), BodyType::ThreeDoors);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0u8; 2];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.write_opening_state::<{ field::FLAGS_0 }, 0>(true);
        frame.write_opening_state::<{ field::FLAGS_0 }, 1>(false);
        frame.write_opening_state::<{ field::FLAGS_0 }, 2>(true);
        frame.write_opening_state::<{ field::FLAGS_0 }, 3>(false);
        frame.write_opening_state::<{ field::FLAGS_0 }, 4>(true);
        frame.write_opening_state::<{ field::FLAGS_0 }, 5>(false);
        frame.write_opening_state::<{ field::FLAGS_0 }, 6>(true);
        frame.write_opening_state::<{ field::FLAGS_0 }, 7>(false);
        frame.write_opening_state::<{ field::FLAGS_1 }, 6>(true);
        frame.set_vehicle_body_type(BodyType::FiveDoors);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0u8; 2];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.write_opening_state::<{ field::FLAGS_0 }, 0>(false);
        frame.write_opening_state::<{ field::FLAGS_0 }, 1>(true);
        frame.write_opening_state::<{ field::FLAGS_0 }, 2>(false);
        frame.write_opening_state::<{ field::FLAGS_0 }, 3>(true);
        frame.write_opening_state::<{ field::FLAGS_0 }, 4>(false);
        frame.write_opening_state::<{ field::FLAGS_0 }, 5>(true);
        frame.write_opening_state::<{ field::FLAGS_0 }, 6>(false);
        frame.write_opening_state::<{ field::FLAGS_0 }, 7>(true);
        frame.write_opening_state::<{ field::FLAGS_1 }, 6>(false);
        frame.set_vehicle_body_type(BodyType::ThreeDoors);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 3] = [0x55, 0x40, 0x00];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 1] = [0x55];
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
        let mut buf = [0u8; 2];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_basic_repr_2_emit() {
        let mut buf = [0u8; 2];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_2_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }
}
