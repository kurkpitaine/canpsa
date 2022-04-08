use core::{fmt, time::Duration};

use crate::{Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 3-bit A/C request field,
    /// 1-bit front A/C failure flag,
    /// 2-bit front fan mode,
    /// 1-bit rear windshield demist enable flag,
    /// 1-bit A/C off mode request flag.
    pub const AC_0: usize = 0;
    /// 6-bit unknown,
    /// 1-bit fan failure flag,
    /// 1-bit cabin sensor failure flag.
    pub const AC_1: usize = 1;
    /// 4-bit front fan speed field,
    /// 4-bit unknown.
    pub const AC_2: usize = 2;
    /// 4-bit front right air distribution position field,
    /// 4-bit front left air distribution position field.
    pub const AC_3: usize = 3;
    /// 4-bit unknown,
    /// 3-bit A/C air intake mode field,
    /// 1-bit restore mode flag.
    pub const AC_4: usize = 4;
    /// 5-bit driver temperature value instruction field,
    /// 3-bit unknown.
    pub const AC_5: usize = 5;
    /// 5-bit passenger temperature value instruction field,
    /// 3-bit unknown.
    pub const AC_6: usize = 6;
}

/// Length of a x1d0 CAN frame.
pub const FRAME_LEN: usize = field::AC_6 + 1;

/// Periodicity of a x1d0 CAN frame.
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

    /// Return the 'typage' field.
    #[inline]
    pub fn typage(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::AC_0] & 0x03
    }

    /// Return the A/C request field.
    #[inline]
    pub fn ac_request(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::AC_0] & 0x0c) >> 2
    }

    /// Return the front left temperature value instruction field.
    #[inline]
    pub fn front_left_temp(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::AC_3] & 0x1f
    }

    /// Return the mono temperature mode flag.
    #[inline]
    pub fn mono_temp(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::AC_3] & 0x40 != 0
    }

    /// Return the A/C max request flag.
    #[inline]
    pub fn ac_max(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::AC_3] & 0x80 != 0
    }

    /// Return the front right temperature value instruction field.
    #[inline]
    pub fn front_right_temp(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::AC_4] & 0x1f
    }

    /// Return the front left seat ventilation request field.
    #[inline]
    pub fn front_left_seat_ventilation(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::AC_4] & 0x60) >> 5
    }

    /// Return the front fan speed value field.
    #[inline]
    pub fn front_fan_speed(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::AC_5] & 0x0f
    }

    /// Return the air intake mode value field.
    #[inline]
    pub fn air_intake_mode(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::AC_5] & 0x70) >> 4
    }

    /// Return the air quality system enable flag.
    #[inline]
    pub fn air_quality_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::AC_5] & 0x80 != 0
    }

    /// Return the front right air distribution position value field.
    #[inline]
    pub fn front_right_distribution_position(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::AC_6] & 0x0f
    }

    /// Return the front left air distribution position value field.
    #[inline]
    pub fn front_left_distribution_position(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::AC_6] >> 4
    }

    /// Return the front right seat ventilation request field.
    #[inline]
    pub fn front_right_seat_ventilation(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::AC_7] & 0x06) >> 1
    }

    /// Return the front left seat heating value request field.
    #[inline]
    pub fn front_left_seat_heating(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::AC_7] & 0x18) >> 3
    }

    /// Return the front right seat heating value request field.
    #[inline]
    pub fn front_right_seat_heating(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::AC_7] & 0x60) >> 5
    }

    /// Return the energy saver mode enable flag.
    #[inline]
    pub fn energy_saver_mode_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::AC_7] & 0x80 != 0
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the typage field.
    #[inline]
    pub fn set_typage(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_0] & !0x03;
        let raw = raw | (value & 0x03);
        data[field::AC_0] = raw;
    }

    /// Set the A/C request field.
    #[inline]
    pub fn set_ac_request(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_0] & !0x0c;
        let raw = raw | ((value << 2) & 0x0c);
        data[field::AC_0] = raw;
    }

    /// Set the front left temperature value instruction field.
    #[inline]
    pub fn set_front_left_temp(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_3] & !0x1f;
        let raw = raw | (value & 0x1f);
        data[field::AC_3] = raw;
    }

    /// Set the mono temperature mode flag.
    #[inline]
    pub fn set_mono_temp(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_3];
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::AC_3] = raw;
    }

    /// Set the A/C max request flag.
    #[inline]
    pub fn set_ac_max(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_3];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::AC_3] = raw;
    }

    /// Set the front right temperature value instruction field.
    #[inline]
    pub fn set_front_right_temp(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_4] & !0x1f;
        let raw = raw | (value & 0x1f);
        data[field::AC_4] = raw;
    }

    /// Set the front left seat ventilation request field.
    #[inline]
    pub fn set_front_left_seat_ventilation(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_4] & !0x60;
        let raw = raw | ((value << 5) & 0x60);
        data[field::AC_4] = raw;
    }

    /// Set the front fan speed value field.
    #[inline]
    pub fn set_front_fan_speed(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_5] & !0x0f;
        let raw = raw | (value & 0x0f);
        data[field::AC_5] = raw;
    }

    /// Set the air intake mode value field.
    #[inline]
    pub fn set_air_intake_mode(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_5] & !0x70;
        let raw = raw | ((value << 4) & 0x70);
        data[field::AC_5] = raw;
    }

    /// Set the air quality system enable flag.
    #[inline]
    pub fn set_air_quality_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_5];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::AC_5] = raw;
    }

    /// Set the front right air distribution position value field.
    #[inline]
    pub fn set_front_right_distribution_position(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_6] & !0x0f;
        let raw = raw | (value & 0x0f);
        data[field::AC_6] = raw;
    }

    /// Set the front left air distribution position value field.
    #[inline]
    pub fn set_front_left_distribution_position(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_6] & !0xf0;
        let raw = raw | (value << 4);
        data[field::AC_6] = raw;
    }

    /// Set the front right seat ventilation request field.
    #[inline]
    pub fn set_front_right_seat_ventilation(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_7] & !0x06;
        let raw = raw | ((value << 1) & 0x06);
        data[field::AC_7] = raw;
    }

    /// Set the front left seat heating value request field.
    #[inline]
    pub fn set_front_left_seat_heating(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_7] & !0x18;
        let raw = raw | ((value << 3) & 0x18);
        data[field::AC_7] = raw;
    }

    /// Set the front right seat heating value request field.
    #[inline]
    pub fn set_front_right_seat_heating(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_7] & !0x60;
        let raw = raw | ((value << 5) & 0x60);
        data[field::AC_7] = raw;
    }

    /// Set the energy saver mode enable flag.
    #[inline]
    pub fn set_energy_saver_mode_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_7];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::AC_7] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x1d0 ({})", err)?;
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

/// A high-level representation of a x1d0 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub typage: u8,
    pub ac_request: u8,
    pub front_left_temperature: u8,
    pub mono_temperature: bool,
    pub ac_max: bool,
    pub front_right_temperature: u8,
    pub front_left_seat_ventilation: u8,
    pub front_fan_speed: u8,
    pub air_intake_mode: u8,
    pub air_quality_enabled: bool,
    pub front_right_distribution_position: u8,
    pub front_left_distribution_position: u8,
    pub front_right_seat_ventilation: u8,
    pub front_left_seat_heating: u8,
    pub front_right_seat_heating: u8,
    pub energy_saver_mode_enabled: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            typage: frame.typage(),
            ac_request: frame.ac_request(),
            front_left_temperature: frame.front_left_temp(),
            mono_temperature: frame.mono_temp(),
            ac_max: frame.ac_max(),
            front_right_temperature: frame.front_right_temp(),
            front_left_seat_ventilation: frame.front_left_seat_ventilation(),
            front_fan_speed: frame.front_fan_speed(),
            air_intake_mode: frame.air_intake_mode(),
            air_quality_enabled: frame.air_quality_enable(),
            front_right_distribution_position: frame.front_right_distribution_position(),
            front_left_distribution_position: frame.front_left_distribution_position(),
            front_right_seat_ventilation: frame.front_right_seat_ventilation(),
            front_left_seat_heating: frame.front_left_seat_heating(),
            front_right_seat_heating: frame.front_right_seat_heating(),
            energy_saver_mode_enabled: frame.energy_saver_mode_enable(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x1d0 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_typage(self.typage);
        frame.set_ac_request(self.ac_request);
        frame.set_front_left_temp(self.front_left_temperature);
        frame.set_mono_temp(self.mono_temperature);
        frame.set_ac_max(self.ac_max);
        frame.set_front_right_temp(self.front_right_temperature);
        frame.set_front_left_seat_ventilation(self.front_left_seat_ventilation);
        frame.set_front_fan_speed(self.front_fan_speed);
        frame.set_air_intake_mode(self.air_intake_mode);
        frame.set_air_quality_enable(self.air_quality_enabled);
        frame.set_front_right_distribution_position(self.front_right_distribution_position);
        frame.set_front_left_distribution_position(self.front_left_distribution_position);
        frame.set_front_right_seat_ventilation(self.front_right_seat_ventilation);
        frame.set_front_left_seat_heating(self.front_left_seat_heating);
        frame.set_front_right_seat_heating(self.front_right_seat_heating);
        frame.set_energy_saver_mode_enable(self.energy_saver_mode_enabled);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x1d0")?;
        writeln!(f, " typage={}", self.typage)?;
        writeln!(f, " ac_request={}", self.ac_request)?;
        writeln!(f, " front_left_temperature={}", self.front_left_temperature)?;
        writeln!(f, " mono_temperature={}", self.mono_temperature)?;
        writeln!(f, " ac_max={}", self.ac_max)?;
        writeln!(
            f,
            " front_right_temperature={}",
            self.front_right_temperature
        )?;
        writeln!(
            f,
            " front_left_seat_ventilation={}",
            self.front_left_seat_ventilation
        )?;
        writeln!(f, " front_fan_speed={}", self.front_fan_speed)?;
        writeln!(f, " air_intake_mode={}", self.air_intake_mode)?;
        writeln!(f, " air_quality_enabled={}", self.air_quality_enabled)?;
        writeln!(
            f,
            " front_right_distribution_position={}",
            self.front_right_distribution_position
        )?;
        writeln!(
            f,
            " front_left_distribution_position={}",
            self.front_left_distribution_position
        )?;
        writeln!(
            f,
            " front_right_seat_ventilation={}",
            self.front_right_seat_ventilation
        )?;
        writeln!(
            f,
            " front_left_seat_heating={}",
            self.front_left_seat_heating
        )?;
        writeln!(
            f,
            " front_right_seat_heating={}",
            self.front_right_seat_heating
        )?;
        writeln!(
            f,
            " energy_saver_mode_enabled={}",
            self.energy_saver_mode_enabled
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::Error;

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x09, 0x00, 0x00, 0x94, 0x14, 0x25, 0x32, 0xc8];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0x03, 0x00, 0x00, 0x53, 0x31, 0x83, 0x10, 0x1c];

    fn frame_1_repr() -> Repr {
        Repr {
            typage: 1,
            ac_request: 2,
            front_left_temperature: 20,
            mono_temperature: false,
            ac_max: true,
            front_right_temperature: 20,
            front_left_seat_ventilation: 0,
            front_fan_speed: 5,
            air_intake_mode: 2,
            air_quality_enabled: false,
            front_right_distribution_position: 2,
            front_left_distribution_position: 3,
            front_right_seat_ventilation: 0,
            front_left_seat_heating: 1,
            front_right_seat_heating: 2,
            energy_saver_mode_enabled: true,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            typage: 3,
            ac_request: 0,
            front_left_temperature: 19,
            mono_temperature: true,
            ac_max: false,
            front_right_temperature: 17,
            front_left_seat_ventilation: 1,
            front_fan_speed: 3,
            air_intake_mode: 0,
            air_quality_enabled: true,
            front_right_distribution_position: 0,
            front_left_distribution_position: 1,
            front_right_seat_ventilation: 2,
            front_left_seat_heating: 3,
            front_right_seat_heating: 0,
            energy_saver_mode_enabled: false,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.typage(), 1);
        assert_eq!(frame.ac_request(), 2);
        assert_eq!(frame.front_left_temp(), 20);
        assert_eq!(frame.mono_temp(), false);
        assert_eq!(frame.ac_max(), true);
        assert_eq!(frame.front_right_temp(), 20);
        assert_eq!(frame.front_left_seat_ventilation(), 0);
        assert_eq!(frame.front_fan_speed(), 5);
        assert_eq!(frame.air_intake_mode(), 2);
        assert_eq!(frame.air_quality_enable(), false);
        assert_eq!(frame.front_right_distribution_position(), 2);
        assert_eq!(frame.front_left_distribution_position(), 3);
        assert_eq!(frame.front_right_seat_ventilation(), 0);
        assert_eq!(frame.front_left_seat_heating(), 1);
        assert_eq!(frame.front_right_seat_heating(), 2);
        assert_eq!(frame.energy_saver_mode_enable(), true);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.typage(), 3);
        assert_eq!(frame.ac_request(), 0);
        assert_eq!(frame.front_left_temp(), 19);
        assert_eq!(frame.mono_temp(), true);
        assert_eq!(frame.ac_max(), false);
        assert_eq!(frame.front_right_temp(), 17);
        assert_eq!(frame.front_left_seat_ventilation(), 1);
        assert_eq!(frame.front_fan_speed(), 3);
        assert_eq!(frame.air_intake_mode(), 0);
        assert_eq!(frame.air_quality_enable(), true);
        assert_eq!(frame.front_right_distribution_position(), 0);
        assert_eq!(frame.front_left_distribution_position(), 1);
        assert_eq!(frame.front_right_seat_ventilation(), 2);
        assert_eq!(frame.front_left_seat_heating(), 3);
        assert_eq!(frame.front_right_seat_heating(), 0);
        assert_eq!(frame.energy_saver_mode_enable(), false);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_typage(1);
        frame.set_ac_request(2);
        frame.set_front_left_temp(20);
        frame.set_mono_temp(false);
        frame.set_ac_max(true);
        frame.set_front_right_temp(20);
        frame.set_front_left_seat_ventilation(0);
        frame.set_front_fan_speed(5);
        frame.set_air_intake_mode(2);
        frame.set_air_quality_enable(false);
        frame.set_front_right_distribution_position(2);
        frame.set_front_left_distribution_position(3);
        frame.set_front_right_seat_ventilation(0);
        frame.set_front_left_seat_heating(1);
        frame.set_front_right_seat_heating(2);
        frame.set_energy_saver_mode_enable(true);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_typage(3);
        frame.set_ac_request(0);
        frame.set_front_left_temp(19);
        frame.set_mono_temp(true);
        frame.set_ac_max(false);
        frame.set_front_right_temp(17);
        frame.set_front_left_seat_ventilation(1);
        frame.set_front_fan_speed(3);
        frame.set_air_intake_mode(0);
        frame.set_air_quality_enable(true);
        frame.set_front_right_distribution_position(0);
        frame.set_front_left_distribution_position(1);
        frame.set_front_right_seat_ventilation(2);
        frame.set_front_left_seat_heating(3);
        frame.set_front_right_seat_heating(0);
        frame.set_energy_saver_mode_enable(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x09, 0x00, 0x00, 0x94, 0x14, 0x25, 0x32, 0xc8, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x09, 0x00, 0x00, 0x94, 0x14, 0x25, 0x32];
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
