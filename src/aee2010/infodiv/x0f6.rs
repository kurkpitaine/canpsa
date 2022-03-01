use core::{fmt, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};

use crate::{
    vehicle::{BlinkersStatus, MainStatus, PowertrainStatus, SteeringWheelPosition, VsmConfigMode},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
0F6 DONNEES_VSM_LENTES_CDE_ECL_CONNECT_HS7_0F6
0F6 DONNEES_VSM_LENTES_ESSUYAGE_HS7_0F6            // OK
0F6 DONNEES_VSM_LENTES_ETAT_CLIGNOTANTS_HS7_0F6    // OK
0F6 DONNEES_VSM_LENTES_ETAT_GEN_HS7_0F6            // OK
0F6 DONNEES_VSM_LENTES_ETAT_GMP_HS7_0F6            // OK
0F6 DONNEES_VSM_LENTES_ETAT_MA_HS7_0F6             // OK
0F6 DONNEES_VSM_LENTES_ETAT_PRINCIP_SEV_HS7_0F6    // OK
0F6 DONNEES_VSM_LENTES_KM_TOTAL_HS7_0F6            // OK
0F6 DONNEES_VSM_LENTES_MDE_CFG_HS7_0F6             // OK
0F6 DONNEES_VSM_LENTES_PARC_USINE_HS7_0F6          // OK
0F6 DONNEES_VSM_LENTES_TEAU_HS7_0F6                // OK
0F6 DONNEES_VSM_LENTES_TEST_VOY_CMB_HS7_0F6        // OK
0F6 DONNEES_VSM_LENTES_T_EXT_FILT_HS7_0F6          // OK
0F6 DONNEES_VSM_LENTES_T_EXT_HS7_0F6               // Ok
0F6 DONNEES_VSM_LENTES_TYPE_DIR_HS7_0F6            // OK
*/

mod field {
    use crate::field::*;
    /// 2-bit powertrain status,
    /// 1-bit generator working flag,
    /// 2-bit vehicle main status,
    /// 1-bit factory park flag,
    /// 2-bit vehicle vehicle supervision module config mode.
    pub const STATES: usize = 0;
    /// 8-bit engine coolant temperature value, in celsius with a +40 offset.
    pub const COOLANT_TEMP: usize = 1;
    /// 24-bit odometer value, in 0.1 kilometers.
    pub const ODOMETER: Field = 2..5;
    /// 8-bit external temperature value, in 0.5 celsius with a +40 offset.
    pub const EXT_TEMP: usize = 5;
    /// 8-bit filtered external temperature value, in 0.5 celsius with a +40 offset.
    pub const EXT_TEMP_FILTERED: usize = 6;
    /// 2-bit blinkers status,
    /// 1-bit empty,
    /// 1-bit cluster lights test flag,
    /// 2-bit steering wheel position,
    /// 1-bit front wiping acknowledge flag,
    /// 1-bit reverse gear engaged flag.
    pub const FLAGS: usize = 7;
}

/// Length of a x0f6 CAN frame.
pub const FRAME_LEN: usize = field::FLAGS + 1;

/// Periodicity of a x0f6 CAN frame.
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

    /// Return the powertrain status field.
    #[inline]
    pub fn powertrain_status(&self) -> PowertrainStatus {
        let data = self.buffer.as_ref();
        let raw = data[field::STATES] & 0x03;
        PowertrainStatus::from(raw)
    }

    /// Return the generator working flag.
    #[inline]
    pub fn generator_working(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::STATES] & 0x04 != 0
    }

    /// Return the vehicle main status field.
    #[inline]
    pub fn vehicle_main_status(&self) -> MainStatus {
        let data = self.buffer.as_ref();
        let raw = (data[field::STATES] & 0x18) >> 3;
        MainStatus::from(raw)
    }

    /// Return the factory park flag.
    #[inline]
    pub fn factory_park(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::STATES] & 0x20 != 0
    }

    /// Return the vehicle supervision module config mode field.
    #[inline]
    pub fn vsm_config_mode(&self) -> VsmConfigMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::STATES] & 0xc0) >> 6;
        VsmConfigMode::from(raw)
    }

    /// Return the engine coolant temperature value, in celsius with a +40 offset.
    #[inline]
    pub fn coolant_temp(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::COOLANT_TEMP]
    }

    /// Return the odometer value, in 0.1 kilometers.
    #[inline]
    pub fn odometer(&self) -> u32 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u24(&data[field::ODOMETER])
    }

    /// Return the external temperature value, in 0.5 celsius with a +40 offset.
    #[inline]
    pub fn external_temp(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::EXT_TEMP]
    }

    /// Return the filtered external temperature value, in 0.5 celsius with a +40 offset.
    #[inline]
    pub fn external_temp_filtered(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::EXT_TEMP_FILTERED]
    }

    /// Return the blinkers status field.
    #[inline]
    pub fn blinkers_status(&self) -> BlinkersStatus {
        let data = self.buffer.as_ref();
        let raw = data[field::FLAGS] & 0x03;
        BlinkersStatus::from(raw)
    }

    /// Return the cluster lights test flag.
    #[inline]
    pub fn cluster_lights_test(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS] & 0x08 != 0
    }

    /// Return the steering wheel position field.
    #[inline]
    pub fn steering_wheel_pos(&self) -> SteeringWheelPosition {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS] & 0x30) >> 4;
        SteeringWheelPosition::from(raw)
    }

    /// Return the front wiping acknowledge flag.
    #[inline]
    pub fn front_wiping_ack(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS] & 0x40 != 0
    }

    /// Return the reverse gear engaged flag.
    #[inline]
    pub fn reverse_gear_engaged(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS] & 0x80 != 0
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the powertrain status field.
    #[inline]
    pub fn set_powertrain_status(&mut self, value: PowertrainStatus) {
        let data = self.buffer.as_mut();
        let raw = data[field::STATES] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::STATES] = raw;
    }

    /// Set the generator working flag.
    #[inline]
    pub fn set_generator_working(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::STATES] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::STATES] = raw;
    }

    /// Set the vehicle main status field.
    #[inline]
    pub fn set_vehicle_main_status(&mut self, value: MainStatus) {
        let data = self.buffer.as_mut();
        let raw = data[field::STATES] & !0x18;
        let raw = raw | ((u8::from(value) << 3) & 0x18);
        data[field::STATES] = raw;
    }

    /// Set the factory park flag.
    #[inline]
    pub fn set_factory_park(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::STATES] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::STATES] = raw;
    }

    /// Set the vehicle supervision module config mode field.
    #[inline]
    pub fn set_vsm_config_mode(&mut self, value: VsmConfigMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::STATES] & !0xc0;
        let raw = raw | ((u8::from(value) << 6) & 0xc0);
        data[field::STATES] = raw;
    }

    /// Set the engine coolant temperature value, in celsius with a +40 offset.
    #[inline]
    pub fn set_coolant_temp(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::COOLANT_TEMP] = value;
    }

    /// Set the odometer value, in 0.1 kilometers.
    #[inline]
    pub fn set_odometer(&mut self, value: u32) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u24(&mut data[field::ODOMETER], value);
    }

    /// Set the external temperature value, in 0.5 celsius with a +40 offset.
    #[inline]
    pub fn set_external_temp(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::EXT_TEMP] = value;
    }

    /// Set the filtered external temperature value, in 0.5 celsius with a +40 offset.
    #[inline]
    pub fn set_external_temp_filtered(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::EXT_TEMP_FILTERED] = value;
    }

    /// Set the blinkers status field.
    #[inline]
    pub fn set_blinkers_status(&mut self, value: BlinkersStatus) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::FLAGS] = raw;
    }

    /// Set the cluster lights test flag.
    #[inline]
    pub fn set_cluster_lights_test(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::FLAGS] = raw;
    }

    /// Set the steering wheel position field.
    #[inline]
    pub fn set_steering_wheel_pos(&mut self, value: SteeringWheelPosition) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS] & !0x30;
        let raw = raw | ((u8::from(value) << 4) & 0x30);
        data[field::FLAGS] = raw;
    }

    /// Set the front wiping acknowledge flag.
    #[inline]
    pub fn set_front_wiping_ack(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::FLAGS] = raw;
    }

    /// Set the reverse gear engaged flag.
    #[inline]
    pub fn set_reverse_gear_engaged(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::FLAGS] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x0f6 ({})", err)?;
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

/// A high-level representation of a x0f6 CAN frame.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub powertrain_status: PowertrainStatus,
    pub generator_working: bool,
    pub vehicle_main_status: MainStatus,
    pub factory_park_enabled: bool,
    pub vsm_config_mode: VsmConfigMode,
    pub coolant_temperature: f32,
    pub odometer: f32,
    pub external_temperature: f32,
    pub external_temperature_filtered: f32,
    pub blinkers_status: BlinkersStatus,
    pub cluster_lights_test: bool,
    pub steering_wheel_position: SteeringWheelPosition,
    pub front_wiping_acknowledge: bool,
    pub reverse_gear_engaged: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            powertrain_status: frame.powertrain_status(),
            generator_working: frame.generator_working(),
            vehicle_main_status: frame.vehicle_main_status(),
            factory_park_enabled: frame.factory_park(),
            vsm_config_mode: frame.vsm_config_mode(),
            coolant_temperature: frame.coolant_temp() as f32 - 40.0,
            odometer: (frame.odometer() as f32 / 10.0),
            external_temperature: (frame.external_temp() as f32 / 2.0) - 40.0,
            external_temperature_filtered: (frame.external_temp_filtered() as f32 / 2.0) - 40.0,
            blinkers_status: frame.blinkers_status(),
            cluster_lights_test: frame.cluster_lights_test(),
            steering_wheel_position: frame.steering_wheel_pos(),
            front_wiping_acknowledge: frame.front_wiping_ack(),
            reverse_gear_engaged: frame.reverse_gear_engaged(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x0f6 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_powertrain_status(self.powertrain_status);
        frame.set_generator_working(self.generator_working);
        frame.set_vehicle_main_status(self.vehicle_main_status);
        frame.set_factory_park(self.factory_park_enabled);
        frame.set_vsm_config_mode(self.vsm_config_mode);
        frame.set_coolant_temp((self.coolant_temperature + 40.0) as u8);
        frame.set_odometer((self.odometer * 10.0) as u32);
        frame.set_external_temp(((self.external_temperature + 40.0) * 2.0) as u8);
        frame.set_external_temp_filtered(((self.external_temperature_filtered + 40.0) * 2.0) as u8);
        frame.set_blinkers_status(self.blinkers_status);
        frame.set_cluster_lights_test(self.cluster_lights_test);
        frame.set_steering_wheel_pos(self.steering_wheel_position);
        frame.set_front_wiping_ack(self.front_wiping_acknowledge);
        frame.set_reverse_gear_engaged(self.reverse_gear_engaged);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x0f6 powertrain_status={}", self.powertrain_status)?;
        writeln!(f, " generator_working={}", self.generator_working)?;
        writeln!(f, " vehicle_main_status={}", self.vehicle_main_status)?;
        writeln!(f, " factory_park_enabled={}", self.factory_park_enabled)?;
        writeln!(f, " vsm_config_mode={}", self.vsm_config_mode)?;
        writeln!(f, " coolant_temperature={}", self.coolant_temperature)?;
        writeln!(f, " odometer={}", self.odometer)?;
        writeln!(f, " external_temperature={}", self.external_temperature)?;
        writeln!(
            f,
            " external_temperature_filtered={}",
            self.external_temperature_filtered
        )?;
        writeln!(f, " blinkers_status={}", self.blinkers_status)?;
        writeln!(f, " cluster_lights_test={}", self.cluster_lights_test)?;
        writeln!(
            f,
            " steering_wheel_position={}",
            self.steering_wheel_position
        )?;
        writeln!(
            f,
            " front_wiping_acknowledge={}",
            self.front_wiping_acknowledge
        )?;
        writeln!(f, " reverse_gear_engaged={}", self.reverse_gear_engaged)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        vehicle::{
            BlinkersStatus, MainStatus, PowertrainStatus, SteeringWheelPosition, VsmConfigMode,
        },
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x80, 0x3c, 0x12, 0x99, 0x36, 0x9b, 0x9b, 0x20];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0x8e, 0x69, 0x11, 0x75, 0x46, 0x8c, 0x8c, 0x20];

    fn frame_1_repr() -> Repr {
        Repr {
            powertrain_status: PowertrainStatus::Stopped,
            generator_working: false,
            vehicle_main_status: MainStatus::Off,
            factory_park_enabled: false,
            vsm_config_mode: VsmConfigMode::Customer,
            coolant_temperature: 20.0,
            odometer: 121887.0,
            external_temperature: 37.5,
            external_temperature_filtered: 37.5,
            blinkers_status: BlinkersStatus::Off,
            cluster_lights_test: false,
            steering_wheel_position: SteeringWheelPosition::Left,
            front_wiping_acknowledge: false,
            reverse_gear_engaged: false,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            powertrain_status: PowertrainStatus::Running,
            generator_working: true,
            vehicle_main_status: MainStatus::On,
            factory_park_enabled: false,
            vsm_config_mode: VsmConfigMode::Customer,
            coolant_temperature: 65.0,
            odometer: 114413.4,
            external_temperature: 30.0,
            external_temperature_filtered: 30.0,
            blinkers_status: BlinkersStatus::Off,
            cluster_lights_test: false,
            steering_wheel_position: SteeringWheelPosition::Left,
            front_wiping_acknowledge: false,
            reverse_gear_engaged: false,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.powertrain_status(), PowertrainStatus::Stopped);
        assert_eq!(frame.generator_working(), false);
        assert_eq!(frame.vehicle_main_status(), MainStatus::Off);
        assert_eq!(frame.factory_park(), false);
        assert_eq!(frame.vsm_config_mode(), VsmConfigMode::Customer);
        assert_eq!(frame.coolant_temp(), 60);
        assert_eq!(frame.odometer(), 1218870);
        assert_eq!(frame.external_temp(), 155);
        assert_eq!(frame.external_temp_filtered(), 155);
        assert_eq!(frame.blinkers_status(), BlinkersStatus::Off);
        assert_eq!(frame.cluster_lights_test(), false);
        assert_eq!(frame.steering_wheel_pos(), SteeringWheelPosition::Left);
        assert_eq!(frame.front_wiping_ack(), false);
        assert_eq!(frame.reverse_gear_engaged(), false);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.powertrain_status(), PowertrainStatus::Running);
        assert_eq!(frame.generator_working(), true);
        assert_eq!(frame.vehicle_main_status(), MainStatus::On);
        assert_eq!(frame.factory_park(), false);
        assert_eq!(frame.vsm_config_mode(), VsmConfigMode::Customer);
        assert_eq!(frame.coolant_temp(), 105);
        assert_eq!(frame.odometer(), 1144134);
        assert_eq!(frame.external_temp(), 140);
        assert_eq!(frame.external_temp_filtered(), 140);
        assert_eq!(frame.blinkers_status(), BlinkersStatus::Off);
        assert_eq!(frame.cluster_lights_test(), false);
        assert_eq!(frame.steering_wheel_pos(), SteeringWheelPosition::Left);
        assert_eq!(frame.front_wiping_ack(), false);
        assert_eq!(frame.reverse_gear_engaged(), false);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_powertrain_status(PowertrainStatus::Stopped);
        frame.set_generator_working(false);
        frame.set_vehicle_main_status(MainStatus::Off);
        frame.set_factory_park(false);
        frame.set_vsm_config_mode(VsmConfigMode::Customer);
        frame.set_coolant_temp(60);
        frame.set_odometer(1218870);
        frame.set_external_temp(155);
        frame.set_external_temp_filtered(155);
        frame.set_blinkers_status(BlinkersStatus::Off);
        frame.set_cluster_lights_test(false);
        frame.set_steering_wheel_pos(SteeringWheelPosition::Left);
        frame.set_front_wiping_ack(false);
        frame.set_reverse_gear_engaged(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_powertrain_status(PowertrainStatus::Running);
        frame.set_generator_working(true);
        frame.set_vehicle_main_status(MainStatus::On);
        frame.set_factory_park(false);
        frame.set_vsm_config_mode(VsmConfigMode::Customer);
        frame.set_coolant_temp(105);
        frame.set_odometer(1144134);
        frame.set_external_temp(140);
        frame.set_external_temp_filtered(140);
        frame.set_blinkers_status(BlinkersStatus::Off);
        frame.set_cluster_lights_test(false);
        frame.set_steering_wheel_pos(SteeringWheelPosition::Left);
        frame.set_front_wiping_ack(false);
        frame.set_reverse_gear_engaged(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x80, 0x3c, 0x12, 0x99, 0x36, 0x73, 0x73, 0x20, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x80, 0x3c, 0x12, 0x99, 0x36, 0x73, 0x73];
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
