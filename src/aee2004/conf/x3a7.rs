use core::{cmp::Ordering, fmt, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};

use crate::{
    config::{DisplayMode, MaintenanceDisplayMode, MaintenanceType},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    use crate::field::Field;
    /// 2-bit empty,
    /// 2-bit maintenance wrench display mode field (when NOT displaying maintenance counters),
    /// 2-bit maintenance wrench display mode field (when displaying maintenance counters),
    /// 1-bit empty,
    /// 1-bit maintenance type field.
    pub const FLAGS_1: usize = 0;
    /// 5-bit empty,
    /// 1-bit maintenance remaining distance counter display mode flag,
    /// 1-bit empty,
    /// 1-bit maintenance remaining distance counter is negative flag.
    pub const FLAGS_2: usize = 1;
    /// 5-bit empty,
    /// 1-bit maintenance remaining time counter display mode flag,
    /// 1-bit empty,
    /// 1-bit maintenance remaining time counter is negative flag.
    pub const FLAGS_3: usize = 2;
    /// 16-bit maintenance remaining distance counter field.
    pub const MAINTENANCE_DISTANCE: Field = 3..5;
    /// 16-bit maintenance remaining time counter field.
    pub const MAINTENANCE_DAYS: Field = 5..7;
    /// 8-bit maintenance counters display duration field.
    pub const DISP_DURATION: usize = 7;
}

/// Raw x3a7 CAN frame identifier.
pub const FRAME_ID: u16 = 0x3a7;
/// Length of a x3a7 CAN frame.
pub const FRAME_LEN: usize = field::DISP_DURATION + 1;

/// Periodicity of a x3a7 CAN frame.
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

    /// Return the maintenance wrench display mode field (when NOT displaying maintenance counters).
    #[inline]
    pub fn wrench_standard_display_mode(&self) -> MaintenanceDisplayMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_1] & 0x0c) >> 2;
        MaintenanceDisplayMode::from(raw)
    }

    /// Return the maintenance wrench display mode field (when displaying maintenance counters).
    #[inline]
    pub fn wrench_maintenance_display_mode(&self) -> MaintenanceDisplayMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_1] & 0x30) >> 4;
        MaintenanceDisplayMode::from(raw)
    }

    /// Return the maintenance type field.
    #[inline]
    pub fn maintenance_type(&self) -> MaintenanceType {
        let data = self.buffer.as_ref();
        let raw = data[field::FLAGS_1] >> 7;
        MaintenanceType::from(raw)
    }

    /// Return the maintenance remaining distance counter display mode flag.
    #[inline]
    pub fn distance_counter_display_mode(&self) -> DisplayMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_2] & 0x20) >> 5;
        DisplayMode::from(raw)
    }

    /// Return the maintenance remaining distance counter is negative flag.
    #[inline]
    pub fn distance_counter_negative(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_2] >> 7 != 0
    }

    /// Return the maintenance remaining time counter display mode flag.
    #[inline]
    pub fn time_counter_display_mode(&self) -> DisplayMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_3] & 0x20) >> 5;
        DisplayMode::from(raw)
    }

    /// Return the maintenance remaining time counter is negative flag.
    #[inline]
    pub fn time_counter_negative(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_3] >> 7 != 0
    }

    /// Return the maintenance remaining distance counter field.
    #[inline]
    pub fn remaining_distance(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::MAINTENANCE_DISTANCE])
    }

    /// Return the maintenance remaining days counter field.
    #[inline]
    pub fn remaining_days(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::MAINTENANCE_DAYS])
    }

    /// Return the maintenance counters display duration field.
    #[inline]
    pub fn display_duration(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::DISP_DURATION]
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the maintenance wrench display mode field (when NOT displaying maintenance counters).
    #[inline]
    pub fn set_wrench_standard_display_mode(&mut self, value: MaintenanceDisplayMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_1];
        let raw = raw | ((u8::from(value) << 2) & 0x0c);
        data[field::FLAGS_1] = raw;
    }

    /// Set the maintenance wrench display mode field (when displaying maintenance counters).
    #[inline]
    pub fn set_wrench_maintenance_display_mode(&mut self, value: MaintenanceDisplayMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_1];
        let raw = raw | ((u8::from(value) << 4) & 0x30);
        data[field::FLAGS_1] = raw;
    }

    /// Set the maintenance type field.
    #[inline]
    pub fn set_maintenance_type(&mut self, value: MaintenanceType) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_1];
        let raw = raw | (u8::from(value) << 7);
        data[field::FLAGS_1] = raw;
    }

    /// Set the maintenance remaining distance counter display mode flag.
    #[inline]
    pub fn set_distance_counter_display_mode(&mut self, value: DisplayMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_2] & !0x20;
        let raw = raw | ((u8::from(value) << 5) & 0x20);
        data[field::FLAGS_2] = raw;
    }

    /// Set the maintenance remaining distance counter is negative flag.
    #[inline]
    pub fn set_distance_counter_negative(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_2];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::FLAGS_2] = raw;
    }

    /// Set the maintenance remaining distance counter display mode flag.
    #[inline]
    pub fn set_time_counter_display_mode(&mut self, value: DisplayMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_3] & !0x20;
        let raw = raw | ((u8::from(value) << 5) & 0x20);
        data[field::FLAGS_3] = raw;
    }

    /// Set the maintenance remaining distance counter is negative flag.
    #[inline]
    pub fn set_time_counter_negative(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_3];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::FLAGS_3] = raw;
    }

    /// Set the maintenance remaining distance counter field.
    #[inline]
    pub fn set_remaining_distance(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::MAINTENANCE_DISTANCE], value);
    }

    /// Set the maintenance remaining days counter field.
    #[inline]
    pub fn set_remaining_days(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::MAINTENANCE_DAYS], value);
    }

    /// Set the maintenance counters display duration field.
    #[inline]
    pub fn set_display_duration(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::DISP_DURATION] = value;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x3a7 ({})", err)?;
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

/// A high-level representation of a x3a7 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub wrench_standard_display_mode: MaintenanceDisplayMode,
    pub wrench_maintenance_display_mode: MaintenanceDisplayMode,
    pub maintenance_type: MaintenanceType,
    pub distance_counter_display_mode: DisplayMode,
    pub time_counter_display_mode: DisplayMode,
    pub remaining_distance: i32,
    pub remaining_days: i32,
    pub display_duration: Duration,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        let mut remaining_days: i32 = frame.remaining_days().into();
        let mut remaining_distance: i32 = frame.remaining_distance().into();

        if frame.distance_counter_negative() {
            remaining_distance = -remaining_distance;
        }

        if frame.time_counter_negative() {
            remaining_days = -remaining_days;
        }

        Ok(Repr {
            wrench_standard_display_mode: frame.wrench_standard_display_mode(),
            wrench_maintenance_display_mode: frame.wrench_maintenance_display_mode(),
            maintenance_type: frame.maintenance_type(),
            distance_counter_display_mode: frame.distance_counter_display_mode(),
            time_counter_display_mode: frame.time_counter_display_mode(),
            remaining_distance: remaining_distance * 20,
            remaining_days,
            display_duration: Duration::from_secs(frame.display_duration().into()),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x3a7 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_wrench_standard_display_mode(self.wrench_standard_display_mode);
        frame.set_wrench_maintenance_display_mode(self.wrench_maintenance_display_mode);
        frame.set_maintenance_type(self.maintenance_type);
        frame.set_distance_counter_negative(self.remaining_distance < 0);
        frame.set_distance_counter_display_mode(self.distance_counter_display_mode);
        frame.set_time_counter_negative(self.remaining_days < 0);
        frame.set_time_counter_display_mode(self.time_counter_display_mode);
        frame.set_remaining_distance((self.remaining_distance / 20) as u16);
        frame.set_remaining_days(self.remaining_days as u16);
        frame.set_display_duration(self.display_duration.as_secs() as u8);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x3a7")?;
        writeln!(
            f,
            " wrench_standard_display_mode={}",
            self.wrench_standard_display_mode
        )?;
        writeln!(
            f,
            " wrench_maintenance_display_mode={}",
            self.wrench_maintenance_display_mode
        )?;
        writeln!(f, " maintenance_type={}", self.maintenance_type)?;
        writeln!(
            f,
            " distance_counter_display_mode={}",
            self.distance_counter_display_mode
        )?;
        writeln!(
            f,
            " time_counter_display_mode={}",
            self.time_counter_display_mode
        )?;
        writeln!(f, " remaining_distance={}", self.remaining_distance)?;
        writeln!(f, " remaining_days={}", self.remaining_days)?;
        writeln!(f, " display_duration={}", self.display_duration.as_secs())
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use core::time::Duration;

    use crate::{
        config::{DisplayMode, MaintenanceDisplayMode, MaintenanceType},
        Error,
    };

    static REPR_FRAME_BYTES: [u8; 8] = [0x20, 0x20, 0x20, 0x00, 0x05, 0x00, 0x0a, 0x14];

    fn frame_repr() -> Repr {
        Repr {
            wrench_standard_display_mode: MaintenanceDisplayMode::Off,
            wrench_maintenance_display_mode: MaintenanceDisplayMode::Blinking,
            maintenance_type: MaintenanceType::Distance,
            distance_counter_display_mode: DisplayMode::Blinking,
            time_counter_display_mode: DisplayMode::Blinking,
            remaining_distance: 100,
            remaining_days: 10,
            display_duration: Duration::from_secs(20),
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(
            frame.wrench_standard_display_mode(),
            MaintenanceDisplayMode::Off
        );
        assert_eq!(
            frame.wrench_maintenance_display_mode(),
            MaintenanceDisplayMode::Blinking
        );
        assert_eq!(frame.maintenance_type(), MaintenanceType::Distance);
        assert_eq!(frame.distance_counter_negative(), false);
        assert_eq!(frame.distance_counter_display_mode(), DisplayMode::Blinking);
        assert_eq!(frame.time_counter_negative(), false);
        assert_eq!(frame.time_counter_display_mode(), DisplayMode::Blinking);
        assert_eq!(frame.remaining_distance(), 5);
        assert_eq!(frame.remaining_days(), 10);
        assert_eq!(frame.display_duration(), 20);
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_wrench_standard_display_mode(MaintenanceDisplayMode::Off);
        frame.set_wrench_maintenance_display_mode(MaintenanceDisplayMode::Blinking);
        frame.set_maintenance_type(MaintenanceType::Distance);
        frame.set_distance_counter_negative(false);
        frame.set_distance_counter_display_mode(DisplayMode::Blinking);
        frame.set_time_counter_negative(false);
        frame.set_time_counter_display_mode(DisplayMode::Blinking);
        frame.set_remaining_distance(5);
        frame.set_remaining_days(10);
        frame.set_display_duration(20);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x20, 0x20, 0x20, 0x00, 0x05, 0x00, 0x0a, 0x14, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x20, 0x20, 0x20, 0x00, 0x05, 0x00, 0x0a];
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
