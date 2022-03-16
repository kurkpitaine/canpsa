use core::fmt;

use crate::{
    vehicle::{ACRecirculationState, FuelType, PushButtonLedState},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 2-bit sport suspension LED state,
    /// 2-bit child lock LED state,
    /// 2-bit ESP LED state,
    /// 2-bit parking sensors LED state.
    pub const LED_0: usize = 0;
    /// 2-bit A/C ON LED state,
    /// 2-bit rear windshield demist LED state,
    /// 2-bit lane centering LED state,
    /// 2-bit electrical parking brake LED state.
    pub const LED_1: usize = 1;
    /// 2-bit blind spot monitoring LED state,
    /// 2-bit A/C air recirculation state,
    /// 2-bit fuel engine type information field,
    /// 2-bit Stop & Start 1 LED state.
    pub const LED_2: usize = 2;
    /// 2-bit adaptive cruise-control LED state,
    /// 1-bit preconditioning setting reset flag,
    /// 1-bit preconditioning setting request flag,
    /// 1-bit A/C air recirculation request flag,
    /// 1-bit empty,
    /// 2-bit over-speed LED state.
    pub const LED_3: usize = 3;
    /// 6-bit unknown,
    /// 2-bit Stop & Start 2 LED state.
    pub const LED_4: usize = 4;
}

/// Length of a x227 CAN frame.
pub const FRAME_LEN: usize = field::LED_4 + 1;

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

    /// Return the sport suspension LED state.
    #[inline]
    pub fn sport_suspension_led_state(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from(data[field::LED_0] & 0x03)
    }

    /// Return the child lock LED state.
    #[inline]
    pub fn child_lock_led_state(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from((data[field::LED_0] & 0x0c) >> 2)
    }

    /// Return the ESP LED state.
    #[inline]
    pub fn esp_led_state(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from((data[field::LED_0] & 0x30) >> 4)
    }

    /// Return the parking sensors LED state.
    #[inline]
    pub fn parking_sensors_led_state(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from(data[field::LED_0] >> 6)
    }

    /// Return the A/C ON LED state.
    #[inline]
    pub fn ac_on_led_state(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from(data[field::LED_1] & 0x03)
    }

    /// Return the rear windshield demist LED state.
    #[inline]
    pub fn rear_windshield_demist_led_state(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from((data[field::LED_1] & 0x0c) >> 2)
    }

    /// Return the lane centering LED state.
    #[inline]
    pub fn lane_centering_led_state(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from((data[field::LED_1] & 0x30) >> 4)
    }

    /// Return the electrical parking brake LED state.
    #[inline]
    pub fn electrical_parking_brake_led_state(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from(data[field::LED_1] >> 6)
    }

    /// Return the blind spot monitoring LED state.
    #[inline]
    pub fn blind_spot_monitoring_led_state(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from(data[field::LED_2] & 0x03)
    }

    /// Return the A/C air recirculation state.
    #[inline]
    pub fn ac_recirculation_state(&self) -> ACRecirculationState {
        let data = self.buffer.as_ref();
        ACRecirculationState::from((data[field::LED_2] & 0x0c) >> 2)
    }

    /// Return the fuel engine type information field.
    #[inline]
    pub fn fuel_type(&self) -> FuelType {
        let data = self.buffer.as_ref();
        FuelType::from((data[field::LED_2] & 0x30) >> 4)
    }

    /// Return the Stop & Start 1 LED state.
    #[inline]
    pub fn stop_start_1(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from(data[field::LED_2] >> 6)
    }

    /// Return the adaptive cruise-control LED state.
    #[inline]
    pub fn adaptive_cruise_control_led_state(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from(data[field::LED_3] & 0x03)
    }

    /// Return the preconditioning setting reset flag.
    #[inline]
    pub fn preconditioning_reset(&self) -> bool {
        let data = self.buffer.as_ref();
        (data[field::LED_3] & 0x04) != 0
    }

    /// Return the preconditioning setting request flag.
    #[inline]
    pub fn preconditioning_request(&self) -> bool {
        let data = self.buffer.as_ref();
        (data[field::LED_3] & 0x08) != 0
    }

    /// Return the A/C air recirculation request flag.
    #[inline]
    pub fn ac_recirculation_state_request(&self) -> bool {
        let data = self.buffer.as_ref();
        (data[field::LED_3] & 0x10) != 0
    }

    /// Return the over-speed LED state.
    #[inline]
    pub fn over_speed_led_state(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from(data[field::LED_3] >> 6)
    }

    /// Return the Stop & Start 2 LED state.
    #[inline]
    pub fn stop_start_2(&self) -> PushButtonLedState {
        let data = self.buffer.as_ref();
        PushButtonLedState::from(data[field::LED_4] >> 6)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the sport suspension LED state.
    #[inline]
    pub fn set_sport_suspension_led_state(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_0] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::LED_0] = raw;
    }

    /// Set the child lock LED state.
    #[inline]
    pub fn set_child_lock_led_state(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_0] & !0x0c;
        let raw = raw | ((u8::from(value) << 2) & 0x0c);
        data[field::LED_0] = raw;
    }

    /// Set the ESP LED state.
    #[inline]
    pub fn set_esp_led_state(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_0] & !0x30;
        let raw = raw | ((u8::from(value) << 4) & 0x30);
        data[field::LED_0] = raw;
    }

    /// Set the parking sensors LED state.
    #[inline]
    pub fn set_parking_sensors_led_state(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_0] & !0xc0;
        let raw = raw | (u8::from(value) << 6);
        data[field::LED_0] = raw;
    }

    /// Set the A/C ON LED state.
    #[inline]
    pub fn set_ac_on_led_state(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_1] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::LED_1] = raw;
    }

    /// Set the rear windshield demist LED state.
    #[inline]
    pub fn set_rear_windshield_demist_led_state(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_1] & !0x0c;
        let raw = raw | ((u8::from(value) << 2) & 0x0c);
        data[field::LED_1] = raw;
    }

    /// Set the lane centering LED state.
    #[inline]
    pub fn set_lane_centering_led_state(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_1] & !0x30;
        let raw = raw | ((u8::from(value) << 4) & 0x30);
        data[field::LED_1] = raw;
    }

    /// Set the electrical parking brake LED state.
    #[inline]
    pub fn set_electrical_parking_brake_led_state(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_1] & !0xc0;
        let raw = raw | (u8::from(value) << 6);
        data[field::LED_1] = raw;
    }

    /// Set the blind spot monitoring LED state.
    #[inline]
    pub fn set_blind_spot_monitoring_led_state(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_2] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::LED_2] = raw;
    }

    /// Set the A/C air recirculation state.
    #[inline]
    pub fn set_ac_recirculation_state(&mut self, value: ACRecirculationState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_2] & !0x0c;
        let raw = raw | ((u8::from(value) << 2) & 0x0c);
        data[field::LED_2] = raw;
    }

    /// Set the fuel engine type information field.
    #[inline]
    pub fn set_fuel_type(&mut self, value: FuelType) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_2] & !0x30;
        let raw = raw | ((u8::from(value) << 4) & 0x30);
        data[field::LED_2] = raw;
    }

    /// Set the Stop & Start 1 LED state.
    #[inline]
    pub fn set_stop_start_1(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_2] & !0xc0;
        let raw = raw | (u8::from(value) << 6);
        data[field::LED_2] = raw;
    }

    /// Set the adaptive cruise-control LED state.
    #[inline]
    pub fn set_adaptive_cruise_control_led_state(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_3] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::LED_3] = raw;
    }

    /// Set the preconditioning setting reset flag.
    #[inline]
    pub fn set_preconditioning_reset(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_3];
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::LED_3] = raw;
    }

    /// Set the preconditioning setting request flag.
    #[inline]
    pub fn set_preconditioning_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_3];
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::LED_3] = raw;
    }

    /// Set the A/C air recirculation request flag.
    #[inline]
    pub fn set_ac_recirculation_state_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_3];
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::LED_3] = raw;
    }

    /// Set the over-speed LED state.
    #[inline]
    pub fn set_over_speed_led_state(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_3] & !0xc0;
        let raw = raw | (u8::from(value) << 6);
        data[field::LED_3] = raw;
    }

    /// Set the Stop & Start 2 LED state.
    #[inline]
    pub fn set_stop_start_2(&mut self, value: PushButtonLedState) {
        let data = self.buffer.as_mut();
        let raw = data[field::LED_4] & !0xc0;
        let raw = raw | (u8::from(value) << 6);
        data[field::LED_4] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x227 ({})", err)?;
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

/// A high-level representation of a x227 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub sport_suspension_led_state: PushButtonLedState,
    pub child_lock_led_state: PushButtonLedState,
    pub esp_led_state: PushButtonLedState,
    pub parking_sensors_led_state: PushButtonLedState,
    pub ac_on_led_state: PushButtonLedState,
    pub rear_windshield_demist_led_state: PushButtonLedState,
    pub lane_centering_led_state: PushButtonLedState,
    pub electrical_parking_brake_led_state: PushButtonLedState,
    pub blind_spot_monitoring_led_state: PushButtonLedState,
    pub ac_recirculation_state: ACRecirculationState,
    pub fuel_type: FuelType,
    pub stop_start_1: PushButtonLedState,
    pub adaptive_cruise_control_led_state: PushButtonLedState,
    pub preconditioning_reset: bool,
    pub preconditioning_request: bool,
    pub ac_recirculation_state_request: bool,
    pub over_speed_led_state: PushButtonLedState,
    pub stop_start_2: PushButtonLedState,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            sport_suspension_led_state: frame.sport_suspension_led_state(),
            child_lock_led_state: frame.child_lock_led_state(),
            esp_led_state: frame.esp_led_state(),
            parking_sensors_led_state: frame.parking_sensors_led_state(),
            ac_on_led_state: frame.ac_on_led_state(),
            rear_windshield_demist_led_state: frame.rear_windshield_demist_led_state(),
            lane_centering_led_state: frame.lane_centering_led_state(),
            electrical_parking_brake_led_state: frame.electrical_parking_brake_led_state(),
            blind_spot_monitoring_led_state: frame.blind_spot_monitoring_led_state(),
            ac_recirculation_state: frame.ac_recirculation_state(),
            fuel_type: frame.fuel_type(),
            stop_start_1: frame.stop_start_1(),
            adaptive_cruise_control_led_state: frame.adaptive_cruise_control_led_state(),
            preconditioning_reset: frame.preconditioning_reset(),
            preconditioning_request: frame.preconditioning_request(),
            ac_recirculation_state_request: frame.ac_recirculation_state_request(),
            over_speed_led_state: frame.over_speed_led_state(),
            stop_start_2: frame.stop_start_2(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x227 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_sport_suspension_led_state(self.sport_suspension_led_state);
        frame.set_child_lock_led_state(self.child_lock_led_state);
        frame.set_esp_led_state(self.esp_led_state);
        frame.set_parking_sensors_led_state(self.parking_sensors_led_state);
        frame.set_ac_on_led_state(self.ac_on_led_state);
        frame.set_rear_windshield_demist_led_state(self.rear_windshield_demist_led_state);
        frame.set_lane_centering_led_state(self.lane_centering_led_state);
        frame.set_electrical_parking_brake_led_state(self.electrical_parking_brake_led_state);
        frame.set_blind_spot_monitoring_led_state(self.blind_spot_monitoring_led_state);
        frame.set_ac_recirculation_state(self.ac_recirculation_state);
        frame.set_fuel_type(self.fuel_type);
        frame.set_stop_start_1(self.stop_start_1);
        frame.set_adaptive_cruise_control_led_state(self.adaptive_cruise_control_led_state);
        frame.set_preconditioning_reset(self.preconditioning_reset);
        frame.set_preconditioning_request(self.preconditioning_request);
        frame.set_ac_recirculation_state_request(self.ac_recirculation_state_request);
        frame.set_over_speed_led_state(self.over_speed_led_state);
        frame.set_stop_start_2(self.stop_start_2);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x227")?;
        writeln!(
            f,
            " sport_suspension_led_state={}",
            self.sport_suspension_led_state
        )?;
        writeln!(f, " child_lock_led_state={}", self.child_lock_led_state)?;
        writeln!(f, " esp_led_state={}", self.esp_led_state)?;
        writeln!(
            f,
            " parking_sensors_led_state={}",
            self.parking_sensors_led_state
        )?;
        writeln!(f, " ac_on_led_state={}", self.ac_on_led_state)?;
        writeln!(
            f,
            " rear_windshield_demist_led_state={}",
            self.rear_windshield_demist_led_state
        )?;
        writeln!(
            f,
            " lane_centering_led_state={}",
            self.lane_centering_led_state
        )?;
        writeln!(
            f,
            " electrical_parking_brake_led_state={}",
            self.electrical_parking_brake_led_state
        )?;
        writeln!(
            f,
            " blind_spot_monitoring_led_state={}",
            self.blind_spot_monitoring_led_state
        )?;
        writeln!(f, " ac_recirculation_state={}", self.ac_recirculation_state)?;
        writeln!(f, " fuel_type={}", self.fuel_type)?;
        writeln!(f, " stop_start_1={}", self.stop_start_1)?;
        writeln!(
            f,
            " adaptive_cruise_control_led_state={}",
            self.adaptive_cruise_control_led_state
        )?;
        writeln!(f, " preconditioning_reset={}", self.preconditioning_reset)?;
        writeln!(
            f,
            " preconditioning_request={}",
            self.preconditioning_request
        )?;
        writeln!(
            f,
            " ac_recirculation_state_request={}",
            self.ac_recirculation_state_request
        )?;
        writeln!(f, " over_speed_led_state={}", self.over_speed_led_state)?;
        writeln!(f, " stop_start_2={}", self.stop_start_2)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};

    use crate::{
        vehicle::{ACRecirculationState, FuelType, PushButtonLedState},
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 5] = [0x24, 0x49, 0x02, 0x95, 0x00];
    static REPR_FRAME_BYTES_2: [u8; 5] = [0x92, 0x24, 0x9d, 0x48, 0x80];

    fn frame_1_repr() -> Repr {
        Repr {
            sport_suspension_led_state: PushButtonLedState::Off,
            child_lock_led_state: PushButtonLedState::Steady,
            esp_led_state: PushButtonLedState::Blinking,
            parking_sensors_led_state: PushButtonLedState::Off,
            ac_on_led_state: PushButtonLedState::Steady,
            rear_windshield_demist_led_state: PushButtonLedState::Blinking,
            lane_centering_led_state: PushButtonLedState::Off,
            electrical_parking_brake_led_state: PushButtonLedState::Steady,
            blind_spot_monitoring_led_state: PushButtonLedState::Blinking,
            ac_recirculation_state: ACRecirculationState::ExteriorAir,
            fuel_type: FuelType::Petrol,
            stop_start_1: PushButtonLedState::Off,
            adaptive_cruise_control_led_state: PushButtonLedState::Steady,
            preconditioning_reset: true,
            preconditioning_request: false,
            ac_recirculation_state_request: true,
            over_speed_led_state: PushButtonLedState::Blinking,
            stop_start_2: PushButtonLedState::Off,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            sport_suspension_led_state: PushButtonLedState::Blinking,
            child_lock_led_state: PushButtonLedState::Off,
            esp_led_state: PushButtonLedState::Steady,
            parking_sensors_led_state: PushButtonLedState::Blinking,
            ac_on_led_state: PushButtonLedState::Off,
            rear_windshield_demist_led_state: PushButtonLedState::Steady,
            lane_centering_led_state: PushButtonLedState::Blinking,
            electrical_parking_brake_led_state: PushButtonLedState::Off,
            blind_spot_monitoring_led_state: PushButtonLedState::Steady,
            ac_recirculation_state: ACRecirculationState::Stopped,
            fuel_type: FuelType::Diesel,
            stop_start_1: PushButtonLedState::Blinking,
            adaptive_cruise_control_led_state: PushButtonLedState::Off,
            preconditioning_reset: false,
            preconditioning_request: true,
            ac_recirculation_state_request: false,
            over_speed_led_state: PushButtonLedState::Steady,
            stop_start_2: PushButtonLedState::Blinking,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.sport_suspension_led_state(), PushButtonLedState::Off);
        assert_eq!(frame.child_lock_led_state(), PushButtonLedState::Steady);
        assert_eq!(frame.esp_led_state(), PushButtonLedState::Blinking);
        assert_eq!(frame.parking_sensors_led_state(), PushButtonLedState::Off);
        assert_eq!(frame.ac_on_led_state(), PushButtonLedState::Steady);
        assert_eq!(
            frame.rear_windshield_demist_led_state(),
            PushButtonLedState::Blinking
        );
        assert_eq!(frame.lane_centering_led_state(), PushButtonLedState::Off);
        assert_eq!(
            frame.electrical_parking_brake_led_state(),
            PushButtonLedState::Steady
        );
        assert_eq!(
            frame.blind_spot_monitoring_led_state(),
            PushButtonLedState::Blinking
        );
        assert_eq!(
            frame.ac_recirculation_state(),
            ACRecirculationState::ExteriorAir
        );
        assert_eq!(frame.fuel_type(), FuelType::Petrol);
        assert_eq!(frame.stop_start_1(), PushButtonLedState::Off);
        assert_eq!(
            frame.adaptive_cruise_control_led_state(),
            PushButtonLedState::Steady
        );
        assert_eq!(frame.preconditioning_reset(), true);
        assert_eq!(frame.preconditioning_request(), false);
        assert_eq!(frame.ac_recirculation_state_request(), true);
        assert_eq!(frame.over_speed_led_state(), PushButtonLedState::Blinking);
        assert_eq!(frame.stop_start_2(), PushButtonLedState::Off);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(
            frame.sport_suspension_led_state(),
            PushButtonLedState::Blinking
        );
        assert_eq!(frame.child_lock_led_state(), PushButtonLedState::Off);
        assert_eq!(frame.esp_led_state(), PushButtonLedState::Steady);
        assert_eq!(
            frame.parking_sensors_led_state(),
            PushButtonLedState::Blinking
        );
        assert_eq!(frame.ac_on_led_state(), PushButtonLedState::Off);
        assert_eq!(
            frame.rear_windshield_demist_led_state(),
            PushButtonLedState::Steady
        );
        assert_eq!(
            frame.lane_centering_led_state(),
            PushButtonLedState::Blinking
        );
        assert_eq!(
            frame.electrical_parking_brake_led_state(),
            PushButtonLedState::Off
        );
        assert_eq!(
            frame.blind_spot_monitoring_led_state(),
            PushButtonLedState::Steady
        );
        assert_eq!(
            frame.ac_recirculation_state(),
            ACRecirculationState::Stopped
        );
        assert_eq!(frame.fuel_type(), FuelType::Diesel);
        assert_eq!(frame.stop_start_1(), PushButtonLedState::Blinking);
        assert_eq!(
            frame.adaptive_cruise_control_led_state(),
            PushButtonLedState::Off
        );
        assert_eq!(frame.preconditioning_reset(), false);
        assert_eq!(frame.preconditioning_request(), true);
        assert_eq!(frame.ac_recirculation_state_request(), false);
        assert_eq!(frame.over_speed_led_state(), PushButtonLedState::Steady);
        assert_eq!(frame.stop_start_2(), PushButtonLedState::Blinking);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0u8; 5];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_sport_suspension_led_state(PushButtonLedState::Off);
        frame.set_child_lock_led_state(PushButtonLedState::Steady);
        frame.set_esp_led_state(PushButtonLedState::Blinking);
        frame.set_parking_sensors_led_state(PushButtonLedState::Off);
        frame.set_ac_on_led_state(PushButtonLedState::Steady);
        frame.set_rear_windshield_demist_led_state(PushButtonLedState::Blinking);
        frame.set_lane_centering_led_state(PushButtonLedState::Off);
        frame.set_electrical_parking_brake_led_state(PushButtonLedState::Steady);
        frame.set_blind_spot_monitoring_led_state(PushButtonLedState::Blinking);
        frame.set_ac_recirculation_state(ACRecirculationState::ExteriorAir);
        frame.set_fuel_type(FuelType::Petrol);
        frame.set_stop_start_1(PushButtonLedState::Off);
        frame.set_adaptive_cruise_control_led_state(PushButtonLedState::Steady);
        frame.set_preconditioning_reset(true);
        frame.set_preconditioning_request(false);
        frame.set_ac_recirculation_state_request(true);
        frame.set_over_speed_led_state(PushButtonLedState::Blinking);
        frame.set_stop_start_2(PushButtonLedState::Off);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0u8; 5];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_sport_suspension_led_state(PushButtonLedState::Blinking);
        frame.set_child_lock_led_state(PushButtonLedState::Off);
        frame.set_esp_led_state(PushButtonLedState::Steady);
        frame.set_parking_sensors_led_state(PushButtonLedState::Blinking);
        frame.set_ac_on_led_state(PushButtonLedState::Off);
        frame.set_rear_windshield_demist_led_state(PushButtonLedState::Steady);
        frame.set_lane_centering_led_state(PushButtonLedState::Blinking);
        frame.set_electrical_parking_brake_led_state(PushButtonLedState::Off);
        frame.set_blind_spot_monitoring_led_state(PushButtonLedState::Steady);
        frame.set_ac_recirculation_state(ACRecirculationState::Stopped);
        frame.set_fuel_type(FuelType::Diesel);
        frame.set_stop_start_1(PushButtonLedState::Blinking);
        frame.set_adaptive_cruise_control_led_state(PushButtonLedState::Off);
        frame.set_preconditioning_reset(false);
        frame.set_preconditioning_request(true);
        frame.set_ac_recirculation_state_request(false);
        frame.set_over_speed_led_state(PushButtonLedState::Steady);
        frame.set_stop_start_2(PushButtonLedState::Blinking);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 6] = [0x24, 0x49, 0x02, 0x54, 0x00, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 4] = [0x24, 0x49, 0x02, 0x54];
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
        let mut buf = [0u8; 5];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_basic_repr_2_emit() {
        let mut buf = [0u8; 5];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_2_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }
}
