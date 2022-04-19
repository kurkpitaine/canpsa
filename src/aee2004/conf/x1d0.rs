use core::{cmp::Ordering, fmt, time::Duration};

use crate::{
    vehicle::{
        ACAirDistributionPosition, ACAirIntakeMode, ACAirTemperature, ACFanMode, ACFanSpeed,
        ACModeRequest,
    },
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 3-bit A/C request field,
    /// 1-bit front A/C failure flag,
    /// 2-bit front A/C fan mode,
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
    /// 5-bit front left temperature field,
    /// 3-bit unknown.
    pub const AC_5: usize = 5;
    /// 5-bit front right temperature field,
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

    /// Return the A/C request field.
    #[inline]
    pub fn ac_request(&self) -> ACModeRequest {
        let data = self.buffer.as_ref();
        ACModeRequest::from(data[field::AC_0] & 0x07)
    }

    /// Return the front A/C failure flag.
    #[inline]
    pub fn front_ac_failure(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::AC_0] & 0x08 != 0
    }

    /// Return the front A/C fan mode.
    #[inline]
    pub fn front_ac_fan_mode(&self) -> ACFanMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::AC_0] & 0x30) >> 4;
        ACFanMode::from(raw)
    }

    /// Return the rear windshield demist enable flag.
    #[inline]
    pub fn rear_demist(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::AC_0] & 0x40 != 0
    }

    /// Return the A/C off mode request flag.
    #[inline]
    pub fn ac_off(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::AC_0] & 0x80 != 0
    }

    /// Return the fan failure flag.
    #[inline]
    pub fn fan_failure(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::AC_1] & 0x40 != 0
    }

    /// Return the cabin sensor failure flag.
    #[inline]
    pub fn cabin_sensor_failure(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::AC_1] & 0x80 != 0
    }

    /// Return the front fan speed field.
    #[inline]
    pub fn front_fan_speed(&self) -> ACFanSpeed {
        let data = self.buffer.as_ref();
        ACFanSpeed::from(data[field::AC_2] & 0x0f)
    }

    /// Return the front right air distribution position field.
    #[inline]
    pub fn front_right_distribution_position(&self) -> ACAirDistributionPosition {
        let data = self.buffer.as_ref();
        let raw = data[field::AC_3] & 0x0f;
        ACAirDistributionPosition::from(raw)
    }

    /// Return the front left air distribution position field.
    #[inline]
    pub fn front_left_distribution_position(&self) -> ACAirDistributionPosition {
        let data = self.buffer.as_ref();
        let raw = data[field::AC_3] >> 4;
        ACAirDistributionPosition::from(raw)
    }

    /// Return the air intake mode field.
    #[inline]
    pub fn air_intake_mode(&self) -> ACAirIntakeMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::AC_4] & 0x70) >> 4;
        ACAirIntakeMode::from(raw)
    }

    /// Return the restore mode flag.
    #[inline]
    pub fn restore_mode(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::AC_4] & 0x80 != 0
    }

    /// Return the front left temperature field.
    #[inline]
    pub fn front_left_temp(&self) -> ACAirTemperature {
        let data = self.buffer.as_ref();
        ACAirTemperature::from(data[field::AC_5] & 0x1f)
    }

    /// Return the front right temperature field.
    #[inline]
    pub fn front_right_temp(&self) -> ACAirTemperature {
        let data = self.buffer.as_ref();
        ACAirTemperature::from(data[field::AC_6] & 0x1f)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the A/C request field.
    #[inline]
    pub fn set_ac_request(&mut self, value: ACModeRequest) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_0] & !0x07;
        let raw = raw | (u8::from(value) & 0x07);
        data[field::AC_0] = raw;
    }

    /// Set the front A/C failure flag.
    #[inline]
    pub fn set_front_ac_failure(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_0];
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::AC_0] = raw;
    }

    /// Set the front A/C fan mode.
    #[inline]
    pub fn set_front_ac_fan_mode(&mut self, value: ACFanMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_0] & !0x30;
        let raw = raw | ((u8::from(value) << 4) & 0x30);
        data[field::AC_0] = raw;
    }

    /// Set the rear windshield demist enable flag.
    #[inline]
    pub fn set_rear_demist(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_0];
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::AC_0] = raw;
    }

    /// Set the A/C off mode request flag.
    #[inline]
    pub fn set_ac_off(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_0];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::AC_0] = raw;
    }

    /// Set the fan failure flag.
    #[inline]
    pub fn set_fan_failure(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_1];
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::AC_1] = raw;
    }

    /// Set the cabin sensor failure flag.
    #[inline]
    pub fn set_cabin_sensor_failure(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_1];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::AC_1] = raw;
    }

    /// Set the front fan speed value field.
    #[inline]
    pub fn set_front_fan_speed(&mut self, value: ACFanSpeed) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_2] & !0x0f;
        let raw = raw | (u8::from(value) & 0x0f);
        data[field::AC_2] = raw;
    }

    /// Set the front right air distribution position field.
    #[inline]
    pub fn set_front_right_distribution_position(&mut self, value: ACAirDistributionPosition) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_3] & !0x0f;
        let raw = raw | (u8::from(value) & 0x0f);
        data[field::AC_3] = raw;
    }

    /// Set the front left air distribution position field.
    #[inline]
    pub fn set_front_left_distribution_position(&mut self, value: ACAirDistributionPosition) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_3] & !0xf0;
        let raw = raw | (u8::from(value) << 4);
        data[field::AC_3] = raw;
    }

    /// Set the air intake mode value field.
    #[inline]
    pub fn set_air_intake_mode(&mut self, value: ACAirIntakeMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_4] & !0x70;
        let raw = raw | ((u8::from(value) << 4) & 0x70);
        data[field::AC_4] = raw;
    }

    /// Set the restore mode flag.
    #[inline]
    pub fn set_restore_mode(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_4];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::AC_4] = raw;
    }

    /// Set the front left temperature field.
    #[inline]
    pub fn set_front_left_temp(&mut self, value: ACAirTemperature) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_5] & !0x1f;
        let raw = raw | (u8::from(value) & 0x1f);
        data[field::AC_5] = raw;
    }

    /// Set the front right temperature field.
    #[inline]
    pub fn set_front_right_temp(&mut self, value: ACAirTemperature) {
        let data = self.buffer.as_mut();
        let raw = data[field::AC_6] & !0x1f;
        let raw = raw | (u8::from(value) & 0x1f);
        data[field::AC_6] = raw;
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
    pub ac_request: ACModeRequest,
    pub front_ac_failure: bool,
    pub front_ac_fan_mode: ACFanMode,
    pub rear_demist: bool,
    pub ac_off: bool,
    pub fan_failure: bool,
    pub cabin_sensor_failure: bool,
    pub front_fan_speed: ACFanSpeed,
    pub front_right_distribution_position: ACAirDistributionPosition,
    pub front_left_distribution_position: ACAirDistributionPosition,
    pub air_intake_mode: ACAirIntakeMode,
    pub restore_mode: bool,
    pub front_left_temp: ACAirTemperature,
    pub front_right_temp: ACAirTemperature,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            ac_request: frame.ac_request(),
            front_ac_failure: frame.front_ac_failure(),
            front_ac_fan_mode: frame.front_ac_fan_mode(),
            rear_demist: frame.rear_demist(),
            ac_off: frame.ac_off(),
            fan_failure: frame.fan_failure(),
            cabin_sensor_failure: frame.cabin_sensor_failure(),
            front_fan_speed: frame.front_fan_speed(),
            front_right_distribution_position: frame.front_right_distribution_position(),
            front_left_distribution_position: frame.front_left_distribution_position(),
            air_intake_mode: frame.air_intake_mode(),
            restore_mode: frame.restore_mode(),
            front_left_temp: frame.front_left_temp(),
            front_right_temp: frame.front_right_temp(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x1d0 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_ac_request(self.ac_request);
        frame.set_front_ac_failure(self.front_ac_failure);
        frame.set_front_ac_fan_mode(self.front_ac_fan_mode);
        frame.set_rear_demist(self.rear_demist);
        frame.set_ac_off(self.ac_off);
        frame.set_fan_failure(self.fan_failure);
        frame.set_cabin_sensor_failure(self.cabin_sensor_failure);
        frame.set_front_fan_speed(self.front_fan_speed);
        frame.set_front_right_distribution_position(self.front_right_distribution_position);
        frame.set_front_left_distribution_position(self.front_left_distribution_position);
        frame.set_air_intake_mode(self.air_intake_mode);
        frame.set_restore_mode(self.restore_mode);
        frame.set_front_left_temp(self.front_left_temp);
        frame.set_front_right_temp(self.front_right_temp);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x1d0")?;
        writeln!(f, " ac_request={}", self.ac_request)?;
        writeln!(f, " front_ac_failure={}", self.front_ac_failure)?;
        writeln!(f, " front_ac_fan_mode={}", self.front_ac_fan_mode)?;
        writeln!(f, " rear_demist={}", self.rear_demist)?;
        writeln!(f, " ac_off={}", self.ac_off)?;
        writeln!(f, " fan_failure={}", self.fan_failure)?;
        writeln!(f, " cabin_sensor_failure={}", self.cabin_sensor_failure)?;
        writeln!(f, " front_fan_speed={}", self.front_fan_speed)?;
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
        writeln!(f, " air_intake_mode={}", self.air_intake_mode)?;
        writeln!(f, " restore_mode={}", self.restore_mode)?;
        writeln!(f, " front_left_temp={}", self.front_left_temp)?;
        writeln!(f, " front_right_temp={}", self.front_right_temp)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        vehicle::{
            ACAirDistributionPosition, ACAirIntakeMode, ACAirTemperature, ACFanMode, ACFanSpeed,
            ACModeRequest,
        },
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 7] = [0x40, 0x40, 0x02, 0x40, 0x80, 0x03, 0x0a];
    static REPR_FRAME_BYTES_2: [u8; 7] = [0xb9, 0x80, 0x04, 0x86, 0x20, 0x13, 0x06];

    fn frame_1_repr() -> Repr {
        Repr {
            ac_request: ACModeRequest::AutoComfort,
            front_ac_failure: false,
            front_ac_fan_mode: ACFanMode::AutoComfort,
            rear_demist: true,
            ac_off: false,
            fan_failure: true,
            cabin_sensor_failure: false,
            front_fan_speed: ACFanSpeed::Speed3,
            front_right_distribution_position: ACAirDistributionPosition::AutoComfort,
            front_left_distribution_position: ACAirDistributionPosition::Demist,
            air_intake_mode: ACAirIntakeMode::AutoComfort,
            restore_mode: true,
            front_left_temp: ACAirTemperature::Sixteen,
            front_right_temp: ACAirTemperature::TwentyDotFive,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            ac_request: ACModeRequest::AutoDemist,
            front_ac_failure: true,
            front_ac_fan_mode: ACFanMode::AutoSoft,
            rear_demist: false,
            ac_off: true,
            fan_failure: false,
            cabin_sensor_failure: true,
            front_fan_speed: ACFanSpeed::Speed5,
            front_right_distribution_position: ACAirDistributionPosition::FootDemist,
            front_left_distribution_position: ACAirDistributionPosition::FootVentilationDemist,
            air_intake_mode: ACAirIntakeMode::ForcedOpen,
            restore_mode: false,
            front_left_temp: ACAirTemperature::TwentySix,
            front_right_temp: ACAirTemperature::EighteenDotFive,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.ac_request(), ACModeRequest::AutoComfort);
        assert_eq!(frame.front_ac_failure(), false);
        assert_eq!(frame.front_ac_fan_mode(), ACFanMode::AutoComfort);
        assert_eq!(frame.rear_demist(), true);
        assert_eq!(frame.ac_off(), false);
        assert_eq!(frame.fan_failure(), true);
        assert_eq!(frame.cabin_sensor_failure(), false);
        assert_eq!(frame.front_fan_speed(), ACFanSpeed::Speed3);
        assert_eq!(
            frame.front_right_distribution_position(),
            ACAirDistributionPosition::AutoComfort
        );
        assert_eq!(
            frame.front_left_distribution_position(),
            ACAirDistributionPosition::Demist
        );
        assert_eq!(frame.air_intake_mode(), ACAirIntakeMode::AutoComfort);
        assert_eq!(frame.restore_mode(), true);
        assert_eq!(frame.front_left_temp(), ACAirTemperature::Sixteen);
        assert_eq!(frame.front_right_temp(), ACAirTemperature::TwentyDotFive);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.ac_request(), ACModeRequest::AutoDemist);
        assert_eq!(frame.front_ac_failure(), true);
        assert_eq!(frame.front_ac_fan_mode(), ACFanMode::AutoSoft);
        assert_eq!(frame.rear_demist(), false);
        assert_eq!(frame.ac_off(), true);
        assert_eq!(frame.fan_failure(), false);
        assert_eq!(frame.cabin_sensor_failure(), true);
        assert_eq!(frame.front_fan_speed(), ACFanSpeed::Speed5);
        assert_eq!(
            frame.front_right_distribution_position(),
            ACAirDistributionPosition::FootDemist
        );
        assert_eq!(
            frame.front_left_distribution_position(),
            ACAirDistributionPosition::FootVentilationDemist
        );
        assert_eq!(frame.air_intake_mode(), ACAirIntakeMode::ForcedOpen);
        assert_eq!(frame.restore_mode(), false);
        assert_eq!(frame.front_left_temp(), ACAirTemperature::TwentySix);
        assert_eq!(frame.front_right_temp(), ACAirTemperature::EighteenDotFive);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_ac_request(ACModeRequest::AutoComfort);
        frame.set_front_ac_failure(false);
        frame.set_front_ac_fan_mode(ACFanMode::AutoComfort);
        frame.set_rear_demist(true);
        frame.set_ac_off(false);
        frame.set_fan_failure(true);
        frame.set_cabin_sensor_failure(false);
        frame.set_front_fan_speed(ACFanSpeed::Speed3);
        frame.set_front_right_distribution_position(ACAirDistributionPosition::AutoComfort);
        frame.set_front_left_distribution_position(ACAirDistributionPosition::Demist);
        frame.set_air_intake_mode(ACAirIntakeMode::AutoComfort);
        frame.set_restore_mode(true);
        frame.set_front_left_temp(ACAirTemperature::Sixteen);
        frame.set_front_right_temp(ACAirTemperature::TwentyDotFive);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_ac_request(ACModeRequest::AutoDemist);
        frame.set_front_ac_failure(true);
        frame.set_front_ac_fan_mode(ACFanMode::AutoSoft);
        frame.set_rear_demist(false);
        frame.set_ac_off(true);
        frame.set_fan_failure(false);
        frame.set_cabin_sensor_failure(true);
        frame.set_front_fan_speed(ACFanSpeed::Speed5);
        frame.set_front_right_distribution_position(ACAirDistributionPosition::FootDemist);
        frame
            .set_front_left_distribution_position(ACAirDistributionPosition::FootVentilationDemist);
        frame.set_air_intake_mode(ACAirIntakeMode::ForcedOpen);
        frame.set_restore_mode(false);
        frame.set_front_left_temp(ACAirTemperature::TwentySix);
        frame.set_front_right_temp(ACAirTemperature::EighteenDotFive);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 8] = [0x40, 0x40, 0x20, 0x40, 0x80, 0x03, 0x0a, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 6] = [0x40, 0x40, 0x20, 0x40, 0x80, 0x03];
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
