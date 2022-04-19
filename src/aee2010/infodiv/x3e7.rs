use core::{cmp::Ordering, fmt, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};

use crate::{config::DisplayMode, Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
3E7 INFOS_MAINTENANCE_EV_AFF_MAINT_HS7_3E7              // OK
3E7 INFOS_MAINTENANCE_EV_K_MAINT_HS7_3E7                // OK
3E7 INFOS_MAINTENANCE_EV_NB_JOUR_MAINTENANCE_HS7_3E7    // OK
3E7 INFOS_MAINTENANCE_EV_PICTO_CLE_HS7_3E7              // OK
3E7 INFOS_MAINTENANCE_EV_SIGN_ECHEANCE_HS7_3E7
3E7 INFOS_MAINTENANCE_EV_SIGN_MAINT_HS7_3E7             // OK
3E7 INFOS_MAINTENANCE_EV_TYPE_MAINT_2_HS7_3E7
3E7 INFOS_MAINTENANCE_EV_TYPE_MAINT_HS7_3E7
*/

mod field {
    use crate::field::Field;
    /// 3-bit unknown,
    /// 1-bit maintenance remaining distance counter is negative flag,
    /// 1-bit maintenance remaining distance counter display mode flag,
    /// 1-bit maintenance wrench display mode flag,
    /// 2-bit unknown.
    pub const FLAGS: usize = 0;
    /// 3-bit unknown,
    /// 11-bit maintenance remaining time counter field,
    /// 2-bit unknown.
    pub const MAINTENANCE_DAYS: Field = 1..3;
    /// 16-bit maintenance remaining distance counter field.
    pub const MAINTENANCE_DISTANCE: Field = 3..5;
}

/// Length of a x3e7 CAN frame.
pub const FRAME_LEN: usize = field::MAINTENANCE_DISTANCE.end;

/// Periodicity of a x3e7 CAN frame.
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

    /// Return the maintenance remaining distance counter is negative flag.
    #[inline]
    pub fn distance_counter_negative(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS] & 0x08 != 0
    }

    /// Return the maintenance remaining distance counter display mode flag.
    #[inline]
    pub fn distance_counter_display_mode(&self) -> DisplayMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS] & 0x10) >> 4;
        DisplayMode::from(raw)
    }

    /// Return the maintenance wrench display mode flag.
    #[inline]
    pub fn maintenance_wrench_display_mode(&self) -> DisplayMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS] & 0x20) >> 5;
        DisplayMode::from(raw)
    }

    /// Return the maintenance remaining days counter field.
    #[inline]
    pub fn remaining_days(&self) -> u16 {
        let data = self.buffer.as_ref();
        (NetworkEndian::read_u16(&data[field::MAINTENANCE_DAYS]) & 0x3ff8) >> 3
    }

    /// Return the maintenance remaining distance counter field.
    #[inline]
    pub fn remaining_distance(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::MAINTENANCE_DISTANCE])
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the maintenance remaining distance counter is negative flag.
    #[inline]
    pub fn set_distance_counter_negative(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS];
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::FLAGS] = raw;
    }

    /// Set the maintenance remaining distance counter display mode flag.
    #[inline]
    pub fn set_distance_counter_display_mode(&mut self, value: DisplayMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS] & !0x10;
        let raw = raw | ((u8::from(value) << 4) & 0x10);
        data[field::FLAGS] = raw;
    }

    /// Set the maintenance wrench display mode flag.
    #[inline]
    pub fn set_maintenance_wrench_display_mode(&mut self, value: DisplayMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS] & !0x20;
        let raw = raw | ((u8::from(value) << 5) & 0x20);
        data[field::FLAGS] = raw;
    }

    /// Set the maintenance remaining days counter field.
    #[inline]
    pub fn set_remaining_days(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::MAINTENANCE_DAYS]) & !0x3ff8;
        let raw = raw | ((value << 3) & 0x3ff8);
        NetworkEndian::write_u16(&mut data[field::MAINTENANCE_DAYS], raw);
    }

    /// Set the maintenance remaining distance counter field.
    #[inline]
    pub fn set_remaining_distance(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::MAINTENANCE_DISTANCE], value);
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x3e7 ({})", err)?;
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

/// A high-level representation of a x3e7 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub distance_counter_display_mode: DisplayMode,
    pub maintenance_wrench_display_mode: DisplayMode,
    pub remaining_days: i32,
    pub remaining_distance: i32,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        let remaining_days = frame.remaining_days().into();
        let mut remaining_distance: i32 = frame.remaining_distance().into();

        if frame.distance_counter_negative() {
            remaining_distance = -remaining_distance;
        }

        Ok(Repr {
            distance_counter_display_mode: frame.distance_counter_display_mode(),
            maintenance_wrench_display_mode: frame.maintenance_wrench_display_mode(),
            remaining_days,
            remaining_distance: remaining_distance * 20,
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x3e7 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_distance_counter_negative(self.remaining_distance < 0);
        frame.set_distance_counter_display_mode(self.distance_counter_display_mode);
        frame.set_maintenance_wrench_display_mode(self.maintenance_wrench_display_mode);
        frame.set_remaining_days(self.remaining_days as u16);
        frame.set_remaining_distance((self.remaining_distance / 20) as u16);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x3e7")?;
        writeln!(
            f,
            " distance_counter_display_mode={}",
            self.distance_counter_display_mode
        )?;
        writeln!(
            f,
            " maintenance_wrench_display_mode={}",
            self.maintenance_wrench_display_mode
        )?;
        writeln!(f, " remaining_days={}", self.remaining_days)?;
        writeln!(f, " remaining_distance={}", self.remaining_distance)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};

    use crate::{config::DisplayMode, Error};

    static REPR_FRAME_BYTES: [u8; 5] = [0x00, 0x00, 0x50, 0x00, 0x01];

    fn frame_repr() -> Repr {
        Repr {
            distance_counter_display_mode: DisplayMode::Steady,
            maintenance_wrench_display_mode: DisplayMode::Steady,
            remaining_days: 10,
            remaining_distance: 20,
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.distance_counter_negative(), false);
        assert_eq!(frame.distance_counter_display_mode(), DisplayMode::Steady);
        assert_eq!(frame.maintenance_wrench_display_mode(), DisplayMode::Steady);
        assert_eq!(frame.remaining_days(), 10);
        assert_eq!(frame.remaining_distance(), 1);
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0u8; 5];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_distance_counter_negative(false);
        frame.set_distance_counter_display_mode(DisplayMode::Steady);
        frame.set_maintenance_wrench_display_mode(DisplayMode::Steady);
        frame.set_remaining_days(10);
        frame.set_remaining_distance(1);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 6] = [0x00, 0x00, 0x50, 0x00, 0x01, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 4] = [0x00, 0x00, 0x50, 0x00];
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
        let mut buf = [0u8; 5];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }
}
