use core::{fmt, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};

use crate::{
    vehicle::{
        AdaptiveCruiseControlState, SpeedRegulationFunctionState, SpeedRegulationMode,
        SpeedRegulationSettingPage,
    },
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
228 ACC_XVV_IHM_ETAT_ACC_STOP_LIGHT_REQUEST_HS7_228
228 ACC_XVV_IHM_ETAT_AUTO_HOLD_ACC_HS7_228
228 ACC_XVV_IHM_ETAT_CONS_TIV_ACC_HS7_228               // OK
228 ACC_XVV_IHM_ETAT_DMD_MEM_CSV_HS7_228                // OK
228 ACC_XVV_IHM_ETAT_DMD_PAGE_CONS_XVV_HS7_228          // OK
228 ACC_XVV_IHM_ETAT_ETAT_FONCT_LVV_RVV_HS7_228         // OK
228 ACC_XVV_IHM_ETAT_ETAT_IHM_RVVI_HS7_228              // OK
228 ACC_XVV_IHM_ETAT_FONCT_ACT_LVV_RVV_HS7_228          // OK
228 ACC_XVV_IHM_ETAT_P_INFO_XVV_INCIT_PLV_HS7_228       // OK
228 ACC_XVV_IHM_ETAT_PRESENCE_CIBLE_HS7_228
228 ACC_XVV_IHM_ETAT_REGL_CONS_TIV_ACC_HS7_228
228 ACC_XVV_IHM_ETAT_TENT_ACT_LVV_RVV_HS7_228           // OK
228 ACC_XVV_IHM_ETAT_TIV_ACC_HS7_228
228 ACC_XVV_IHM_ETAT_VIT_CONS_LVV_RVV_HS7_228           // OK
228 ACC_XVV_IHM_ETAT_VIT_CONS_RVVI_AJUST_HS7_228        // OK
228 ACC_XVV_IHM_ETAT_XVV_APPEL_REGL_TIV_HS7_228
228 ACC_XVV_IHM_ETAT_XVV_DISPONIBLE_HS7_228             // OK
228 ACC_XVV_IHM_ETAT_XVV_REGL_CONS_VIT_HS7_228          // OK
 */

mod field {
    use crate::field::Field;
    /// 16-bit cruise-control/speed-limiter/acc speed setting field.
    pub const SPD_INST: Field = 0..2;
    /// 1-bit speed setting adjustment in progress flag,
    /// 1-bit unknown,
    /// 1-bit cruise-control/speed-limiter/acc try enable flag,
    /// 3-bit cruise-control/speed-limiter/acc activated mode state field,
    /// 2-bit cruise-control/speed-limiter/acc activated mode field.
    pub const XVV_1: usize = 2;
    /// 7-bit unknown
    /// 1-bit cruise-control/speed-limiter/acc availability flag.
    pub const XVV_2: usize = 3;
    /// 6-bit adaptive cruise-control time setting field,
    /// 2-bit cruise-control/speed-limiter/acc setting page request field.
    pub const XVV_3: usize = 4;
    /// 16-bit adaptive cruise-control adjusted speed setting field.
    pub const ACC_ADJ_SPD: Field = 5..7;
    /// 1-bit cruise-control/speed-limiter/acc speed setting from traffic sign recognition allowed flag,
    /// 1-bit memorize speed setting request flag,
    /// 2-bit unknown,
    /// 4-bit adaptive cruise-control displayed state field.
    pub const XVV_4: usize = 7;
}

/// Length of a x228 CAN frame.
pub const FRAME_LEN: usize = field::XVV_4 + 1;

/// Periodicity of a x228 CAN frame.
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

    /// Return the cruise-control/speed-limiter/acc speed setting field.
    #[inline]
    pub fn speed_setting(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::SPD_INST])
    }

    /// Return the speed setting adjustment in progress flag.
    #[inline]
    pub fn speed_setting_adjustment_in_progress(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::XVV_1] & 0x01 != 0
    }

    /// Return the cruise-control/speed-limiter/acc try enable flag.
    #[inline]
    pub fn try_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::XVV_1] & 0x04 != 0
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the trip computer secondary trip reset request flag.
    #[inline]
    pub fn set_trip_computer_secondary_trip_reset_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0];
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::REQ_0] = raw;
    }

    /// Set the trip computer primary trip reset request flag.
    #[inline]
    pub fn set_trip_computer_primary_trip_reset_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0];
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::REQ_0] = raw;
    }

    /// Set the adaptive cruise-control push button state flag.
    #[inline]
    pub fn set_adaptive_cruise_control_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0];
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::REQ_0] = raw;
    }

    /// Set the automatic parking mode selection field.
    #[inline]
    pub fn set_auto_parking_mode(&mut self, value: AutomaticParkingMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0] & !0x18;
        let raw = raw | ((u8::from(value) << 3) & 0x18);
        data[field::REQ_0] = raw;
    }

    /// Set the telematics enabled flag.
    #[inline]
    pub fn set_telematics_enabled(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0];
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::REQ_0] = raw;
    }

    /// Set the black panel function state flag.
    #[inline]
    pub fn set_black_panel_enabled(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::REQ_0] = raw;
    }

    /// Set the interactive message field.
    #[inline]
    pub fn set_interactive_message(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::INTERACTIVE_MSG_STOP_CHK]);
        let raw = raw | (value & 0x7fff);
        NetworkEndian::write_u16(&mut data[field::INTERACTIVE_MSG_STOP_CHK], raw);
    }

    /// Set the MFD stop check request field.
    #[inline]
    pub fn set_stop_check_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::INTERACTIVE_MSG_STOP_CHK]);
        let raw = if value { raw | 0x8000 } else { raw & !0x8000 };
        NetworkEndian::write_u16(&mut data[field::INTERACTIVE_MSG_STOP_CHK], raw);
    }

    /// Set the cruise-control custom speed memorization request field.
    #[inline]
    pub fn set_cruise_control_custom_speed_mem_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1];
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::REQ_1] = raw;
    }

    /// Set the available space measurement push button state flag.
    #[inline]
    pub fn set_available_space_measurement_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1];
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::REQ_1] = raw;
    }

    /// Set the parking sensors push button state flag.
    #[inline]
    pub fn set_parking_sensors_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1];
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::REQ_1] = raw;
    }

    /// Set the automatic main beam push button state flag.
    #[inline]
    pub fn set_auto_main_beam_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1];
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::REQ_1] = raw;
    }

    /// Set the lane centering push button state flag.
    #[inline]
    pub fn set_lane_centering_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1];
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::REQ_1] = raw;
    }

    /// Set the blind spot monitoring push button state flag.
    #[inline]
    pub fn set_blind_spot_monitoring_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1];
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::REQ_1] = raw;
    }

    /// Set the adaptive cruise-control '+' push button state flag.
    #[inline]
    pub fn set_adaptive_cruise_control_plus_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1];
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::REQ_1] = raw;
    }

    /// Set the adaptive cruise-control '-' push button state flag.
    #[inline]
    pub fn set_adaptive_cruise_control_minus_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::REQ_1] = raw;
    }

    /// Set the cruise-control speed instruction value field.
    #[inline]
    pub fn set_cruise_control_speed_instruction(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::CC_SPD] = value;
    }

    /// Set the indirect under-inflation push button state flag.
    #[inline]
    pub fn set_indirect_under_inflation_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_2];
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::REQ_2] = raw;
    }

    /// Set the automatic parking state change request flag.
    #[inline]
    pub fn set_auto_parking_state_change_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_2];
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::REQ_2] = raw;
    }

    /// Set the collision alert failure display request flag.
    #[inline]
    pub fn set_collision_alert_failure_display_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_2];
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::REQ_2] = raw;
    }

    /// Set the cruise-control speed setting instruction position field.
    #[inline]
    pub fn set_cruise_control_spd_setting_instruction_pos(
        &mut self,
        value: CruiseControlCustomSettingPosition,
    ) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_2];
        let raw = raw | ((u8::from(value) << 4) & 0x70);
        data[field::REQ_2] = raw;
    }

    /// Set the fault check request flag.
    #[inline]
    pub fn set_fault_check_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_3];
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::REQ_3] = raw;
    }

    /// Set the telematic screen lighting level value field.
    #[inline]
    pub fn set_telematic_screen_lighting_level(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_3] & !0x1e;
        let raw = raw | ((value << 1) & 0x1e);
        data[field::REQ_3] = raw;
    }

    /// Set the telematic unit life state field.
    #[inline]
    pub fn set_telematic_unit_life_state(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_3] & !0x60;
        let raw = raw | ((value << 5) & 0x60);
        data[field::REQ_3] = raw;
    }

    /// Set the indirect under-inflation detection reset request flag.
    #[inline]
    pub fn set_stop_start_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_3];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::REQ_3] = raw;
    }

    /// Set the 'visiopark' visual parking assistance push button state field.
    #[inline]
    pub fn set_visual_parking_assistance_button_state(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_4] & !0x07;
        let raw = raw | (value & 0x07);
        data[field::REQ_4] = raw;
    }

    /// Set the cruise-control speed instruction value request flag.
    #[inline]
    pub fn set_cruise_control_spd_instruction_val_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_4];
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::REQ_4] = raw;
    }

    /// Set the visual parking assistance panoramic view push button state flag.
    #[inline]
    pub fn set_visual_parking_assistance_panoramic_view_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_4];
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::REQ_4] = raw;
    }

    /// Set the front visual parking assistance push button state flag.
    #[inline]
    pub fn set_front_visual_parking_assistance_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_4];
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::REQ_4] = raw;
    }

    /// Set the rear visual parking assistance push button state flag.
    #[inline]
    pub fn set_rear_visual_parking_assistance_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_4];
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::REQ_4] = raw;
    }

    /// Set the visual parking assistance activation request flag.
    #[inline]
    pub fn set_visual_parking_assistance_activation_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_4];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::REQ_4] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x228 ({})", err)?;
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

/// A high-level representation of a x228 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub trip_computer_secondary_trip_reset_request: bool,
    pub trip_computer_primary_trip_reset_request: bool,
    pub adaptive_cruise_control_button_state: bool,
    pub automatic_parking_mode: AutomaticParkingMode,
    pub telematics_enabled: bool,
    pub black_panel_enabled: bool,
    pub interactive_message: u16,
    pub stop_check_request: bool,
    pub cruise_control_custom_speed_memorization_request: bool,
    pub available_space_measurement_button_state: bool,
    pub parking_sensors_button_state: bool,
    pub auto_main_beam_button_state: bool,
    pub lane_centering_button_state: bool,
    pub blind_spot_monitoring_button_state: bool,
    pub adaptive_cruise_control_plus_button_state: bool,
    pub adaptive_cruise_control_minus_button_state: bool,
    pub cruise_control_speed_instruction: u8,
    pub indirect_under_inflation_button_state: bool,
    pub automatic_parking_state_change_request: bool,
    pub collision_alert_failure_display_request: bool,
    pub cruise_control_speed_setting_instruction_position: CruiseControlCustomSettingPosition,
    pub fault_check_request: bool,
    pub telematic_screen_lighting_level: u8,
    pub telematic_unit_life_state: u8,
    pub stop_start_button_state: bool,
    pub visual_parking_assistance_button_state: u8,
    pub cruise_control_speed_instruction_value_request: bool,
    pub visual_parking_assistance_panoramic_view_button_state: bool,
    pub front_visual_parking_assistance_button_state: bool,
    pub rear_visual_parking_assistance_button_state: bool,
    pub visual_parking_assistance_activation_request: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            trip_computer_secondary_trip_reset_request: frame
                .trip_computer_secondary_trip_reset_request(),
            trip_computer_primary_trip_reset_request: frame
                .trip_computer_primary_trip_reset_request(),
            adaptive_cruise_control_button_state: frame.adaptive_cruise_control_button_state(),
            automatic_parking_mode: frame.auto_parking_mode(),
            telematics_enabled: frame.telematics_enabled(),
            black_panel_enabled: frame.black_panel_enabled(),
            interactive_message: frame.interactive_message(),
            stop_check_request: frame.stop_check_request(),
            cruise_control_custom_speed_memorization_request: frame
                .cruise_control_custom_speed_mem_request(),
            available_space_measurement_button_state: frame
                .available_space_measurement_button_state(),
            parking_sensors_button_state: frame.parking_sensors_button_state(),
            auto_main_beam_button_state: frame.auto_main_beam_button_state(),
            lane_centering_button_state: frame.lane_centering_button_state(),
            blind_spot_monitoring_button_state: frame.blind_spot_monitoring_button_state(),
            adaptive_cruise_control_plus_button_state: frame
                .adaptive_cruise_control_plus_button_state(),
            adaptive_cruise_control_minus_button_state: frame
                .adaptive_cruise_control_minus_button_state(),
            cruise_control_speed_instruction: frame.cruise_control_speed_instruction(),
            indirect_under_inflation_button_state: frame.indirect_under_inflation_button_state(),
            automatic_parking_state_change_request: frame.auto_parking_state_change_request(),
            collision_alert_failure_display_request: frame
                .collision_alert_failure_display_request(),
            cruise_control_speed_setting_instruction_position: frame
                .cruise_control_spd_setting_instruction_pos(),
            fault_check_request: frame.fault_check_request(),
            telematic_screen_lighting_level: frame.telematic_screen_lighting_level(),
            telematic_unit_life_state: frame.telematic_unit_life_state(),
            stop_start_button_state: frame.stop_start_button_state(),
            visual_parking_assistance_button_state: frame.visual_parking_assistance_button_state(),
            cruise_control_speed_instruction_value_request: frame
                .cruise_control_spd_instruction_val_request(),
            visual_parking_assistance_panoramic_view_button_state: frame
                .visual_parking_assistance_panoramic_view_button_state(),
            front_visual_parking_assistance_button_state: frame
                .front_visual_parking_assistance_button_state(),
            rear_visual_parking_assistance_button_state: frame
                .rear_visual_parking_assistance_button_state(),
            visual_parking_assistance_activation_request: frame
                .visual_parking_assistance_activation_request(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x228 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_trip_computer_secondary_trip_reset_request(
            self.trip_computer_secondary_trip_reset_request,
        );
        frame.set_trip_computer_primary_trip_reset_request(
            self.trip_computer_primary_trip_reset_request,
        );
        frame.set_adaptive_cruise_control_button_state(self.adaptive_cruise_control_button_state);
        frame.set_auto_parking_mode(self.automatic_parking_mode);
        frame.set_telematics_enabled(self.telematics_enabled);
        frame.set_black_panel_enabled(self.black_panel_enabled);
        frame.set_interactive_message(self.interactive_message);
        frame.set_stop_check_request(self.stop_check_request);
        frame.set_cruise_control_custom_speed_mem_request(
            self.cruise_control_custom_speed_memorization_request,
        );
        frame.set_available_space_measurement_button_state(
            self.available_space_measurement_button_state,
        );
        frame.set_parking_sensors_button_state(self.parking_sensors_button_state);
        frame.set_auto_main_beam_button_state(self.auto_main_beam_button_state);
        frame.set_lane_centering_button_state(self.lane_centering_button_state);
        frame.set_blind_spot_monitoring_button_state(self.blind_spot_monitoring_button_state);
        frame.set_adaptive_cruise_control_plus_button_state(
            self.adaptive_cruise_control_plus_button_state,
        );
        frame.set_adaptive_cruise_control_minus_button_state(
            self.adaptive_cruise_control_minus_button_state,
        );
        frame.set_cruise_control_speed_instruction(self.cruise_control_speed_instruction);
        frame.set_indirect_under_inflation_button_state(self.indirect_under_inflation_button_state);
        frame.set_auto_parking_state_change_request(self.automatic_parking_state_change_request);
        frame.set_collision_alert_failure_display_request(
            self.collision_alert_failure_display_request,
        );
        frame.set_cruise_control_spd_setting_instruction_pos(
            self.cruise_control_speed_setting_instruction_position,
        );
        frame.set_fault_check_request(self.fault_check_request);
        frame.set_telematic_screen_lighting_level(self.telematic_screen_lighting_level);
        frame.set_telematic_unit_life_state(self.telematic_unit_life_state);
        frame.set_stop_start_button_state(self.stop_start_button_state);
        frame.set_visual_parking_assistance_button_state(
            self.visual_parking_assistance_button_state,
        );
        frame.set_cruise_control_spd_instruction_val_request(
            self.cruise_control_speed_instruction_value_request,
        );
        frame.set_visual_parking_assistance_panoramic_view_button_state(
            self.visual_parking_assistance_panoramic_view_button_state,
        );
        frame.set_front_visual_parking_assistance_button_state(
            self.front_visual_parking_assistance_button_state,
        );
        frame.set_rear_visual_parking_assistance_button_state(
            self.rear_visual_parking_assistance_button_state,
        );
        frame.set_visual_parking_assistance_activation_request(
            self.visual_parking_assistance_activation_request,
        );
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "x228 trip_computer_secondary_trip_reset_request={}",
            self.trip_computer_secondary_trip_reset_request
        )?;
        writeln!(
            f,
            " trip_computer_primary_trip_reset_request={}",
            self.trip_computer_primary_trip_reset_request
        )?;
        writeln!(
            f,
            " adaptive_cruise_control_button_state={}",
            self.adaptive_cruise_control_button_state
        )?;
        writeln!(f, " automatic_parking_mode={}", self.automatic_parking_mode)?;
        writeln!(f, " telematics_enabled={}", self.telematics_enabled)?;
        writeln!(f, " black_panel_enabled={}", self.black_panel_enabled)?;
        writeln!(f, " interactive_message={}", self.interactive_message)?;
        writeln!(f, " stop_check_request={}", self.stop_check_request)?;
        writeln!(
            f,
            " cruise_control_custom_speed_memorization_request={}",
            self.cruise_control_custom_speed_memorization_request
        )?;
        writeln!(
            f,
            " available_space_measurement_button_state={}",
            self.available_space_measurement_button_state
        )?;
        writeln!(
            f,
            " parking_sensors_button_state={}",
            self.parking_sensors_button_state
        )?;
        writeln!(
            f,
            " auto_main_beam_button_state={}",
            self.auto_main_beam_button_state
        )?;
        writeln!(
            f,
            " lane_centering_button_state={}",
            self.lane_centering_button_state
        )?;
        writeln!(
            f,
            " blind_spot_monitoring_button_state={}",
            self.blind_spot_monitoring_button_state
        )?;
        writeln!(
            f,
            " adaptive_cruise_control_plus_button_state={}",
            self.adaptive_cruise_control_plus_button_state
        )?;
        writeln!(
            f,
            " adaptive_cruise_control_minus_button_state={}",
            self.adaptive_cruise_control_minus_button_state
        )?;
        writeln!(
            f,
            " cruise_control_speed_instruction={}",
            self.cruise_control_speed_instruction
        )?;
        writeln!(
            f,
            " indirect_under_inflation_button_state={}",
            self.indirect_under_inflation_button_state
        )?;
        writeln!(
            f,
            " automatic_parking_state_change_request={}",
            self.automatic_parking_state_change_request
        )?;
        writeln!(
            f,
            " collision_alert_failure_display_request={}",
            self.collision_alert_failure_display_request
        )?;
        writeln!(
            f,
            " cruise_control_speed_setting_instruction_position={}",
            self.cruise_control_speed_setting_instruction_position
        )?;
        writeln!(f, " fault_check_request={}", self.fault_check_request)?;
        writeln!(
            f,
            " telematic_screen_lighting_level={}",
            self.telematic_screen_lighting_level
        )?;
        writeln!(
            f,
            " telematic_unit_life_state={}",
            self.telematic_unit_life_state
        )?;
        writeln!(
            f,
            " stop_start_button_state={}",
            self.stop_start_button_state
        )?;
        writeln!(
            f,
            " visual_parking_assistance_button_state={}",
            self.visual_parking_assistance_button_state
        )?;
        writeln!(
            f,
            " cruise_control_speed_instruction_value_request={}",
            self.cruise_control_speed_instruction_value_request
        )?;
        writeln!(
            f,
            " visual_parking_assistance_panoramic_view_button_state={}",
            self.visual_parking_assistance_panoramic_view_button_state
        )?;
        writeln!(
            f,
            " front_visual_parking_assistance_button_state={}",
            self.front_visual_parking_assistance_button_state
        )?;
        writeln!(
            f,
            " rear_visual_parking_assistance_button_state={}",
            self.rear_visual_parking_assistance_button_state
        )?;
        writeln!(
            f,
            " visual_parking_assistance_activation_request={}",
            self.visual_parking_assistance_activation_request
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        vehicle::{AutomaticParkingMode, CruiseControlCustomSettingPosition},
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x85, 0x00, 0x00, 0x55, 0x00, 0x14, 0x01, 0x50];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0x2a, 0x80, 0x21, 0xaa, 0x33, 0x69, 0xd0, 0xab];

    fn frame_1_repr() -> Repr {
        Repr {
            trip_computer_secondary_trip_reset_request: true,
            trip_computer_primary_trip_reset_request: false,
            adaptive_cruise_control_button_state: true,
            automatic_parking_mode: AutomaticParkingMode::SCP6,
            telematics_enabled: false,
            black_panel_enabled: true,
            interactive_message: 0,
            stop_check_request: false,
            cruise_control_custom_speed_memorization_request: true,
            available_space_measurement_button_state: false,
            parking_sensors_button_state: true,
            auto_main_beam_button_state: false,
            lane_centering_button_state: true,
            blind_spot_monitoring_button_state: false,
            adaptive_cruise_control_plus_button_state: true,
            adaptive_cruise_control_minus_button_state: false,
            cruise_control_speed_instruction: 0,
            indirect_under_inflation_button_state: false,
            automatic_parking_state_change_request: true,
            collision_alert_failure_display_request: false,
            cruise_control_speed_setting_instruction_position:
                CruiseControlCustomSettingPosition::Position1,
            fault_check_request: true,
            telematic_screen_lighting_level: 0,
            telematic_unit_life_state: 0,
            stop_start_button_state: false,
            visual_parking_assistance_button_state: 0,
            cruise_control_speed_instruction_value_request: false,
            visual_parking_assistance_panoramic_view_button_state: true,
            front_visual_parking_assistance_button_state: false,
            rear_visual_parking_assistance_button_state: true,
            visual_parking_assistance_activation_request: false,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            trip_computer_secondary_trip_reset_request: false,
            trip_computer_primary_trip_reset_request: true,
            adaptive_cruise_control_button_state: false,
            automatic_parking_mode: AutomaticParkingMode::SCP9,
            telematics_enabled: true,
            black_panel_enabled: false,
            interactive_message: 33,
            stop_check_request: true,
            cruise_control_custom_speed_memorization_request: false,
            available_space_measurement_button_state: true,
            parking_sensors_button_state: false,
            auto_main_beam_button_state: true,
            lane_centering_button_state: false,
            blind_spot_monitoring_button_state: true,
            adaptive_cruise_control_plus_button_state: false,
            adaptive_cruise_control_minus_button_state: true,
            cruise_control_speed_instruction: 51,
            indirect_under_inflation_button_state: true,
            automatic_parking_state_change_request: false,
            collision_alert_failure_display_request: true,
            cruise_control_speed_setting_instruction_position:
                CruiseControlCustomSettingPosition::Position6,
            fault_check_request: false,
            telematic_screen_lighting_level: 8,
            telematic_unit_life_state: 2,
            stop_start_button_state: true,
            visual_parking_assistance_button_state: 3,
            cruise_control_speed_instruction_value_request: true,
            visual_parking_assistance_panoramic_view_button_state: false,
            front_visual_parking_assistance_button_state: true,
            rear_visual_parking_assistance_button_state: false,
            visual_parking_assistance_activation_request: true,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.trip_computer_secondary_trip_reset_request(), true);
        assert_eq!(frame.trip_computer_primary_trip_reset_request(), false);
        assert_eq!(frame.adaptive_cruise_control_button_state(), true);
        assert_eq!(frame.auto_parking_mode(), AutomaticParkingMode::SCP6);
        assert_eq!(frame.telematics_enabled(), false);
        assert_eq!(frame.black_panel_enabled(), true);
        assert_eq!(frame.interactive_message(), 0);
        assert_eq!(frame.stop_check_request(), false);
        assert_eq!(frame.cruise_control_custom_speed_mem_request(), true);
        assert_eq!(frame.available_space_measurement_button_state(), false);
        assert_eq!(frame.parking_sensors_button_state(), true);
        assert_eq!(frame.auto_main_beam_button_state(), false);
        assert_eq!(frame.lane_centering_button_state(), true);
        assert_eq!(frame.blind_spot_monitoring_button_state(), false);
        assert_eq!(frame.adaptive_cruise_control_plus_button_state(), true);
        assert_eq!(frame.adaptive_cruise_control_minus_button_state(), false);
        assert_eq!(frame.cruise_control_speed_instruction(), 0);
        assert_eq!(frame.indirect_under_inflation_button_state(), false);
        assert_eq!(frame.auto_parking_state_change_request(), true);
        assert_eq!(frame.collision_alert_failure_display_request(), false);
        assert_eq!(
            frame.cruise_control_spd_setting_instruction_pos(),
            CruiseControlCustomSettingPosition::Position1
        );
        assert_eq!(frame.fault_check_request(), true);
        assert_eq!(frame.telematic_screen_lighting_level(), 0);
        assert_eq!(frame.telematic_unit_life_state(), 0);
        assert_eq!(frame.stop_start_button_state(), false);
        assert_eq!(frame.visual_parking_assistance_button_state(), 0);
        assert_eq!(frame.cruise_control_spd_instruction_val_request(), false);
        assert_eq!(
            frame.visual_parking_assistance_panoramic_view_button_state(),
            true
        );
        assert_eq!(frame.front_visual_parking_assistance_button_state(), false);
        assert_eq!(frame.rear_visual_parking_assistance_button_state(), true);
        assert_eq!(frame.visual_parking_assistance_activation_request(), false);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.trip_computer_secondary_trip_reset_request(), false);
        assert_eq!(frame.trip_computer_primary_trip_reset_request(), true);
        assert_eq!(frame.adaptive_cruise_control_button_state(), false);
        assert_eq!(frame.auto_parking_mode(), AutomaticParkingMode::SCP9);
        assert_eq!(frame.telematics_enabled(), true);
        assert_eq!(frame.black_panel_enabled(), false);
        assert_eq!(frame.interactive_message(), 33);
        assert_eq!(frame.stop_check_request(), true);
        assert_eq!(frame.cruise_control_custom_speed_mem_request(), false);
        assert_eq!(frame.available_space_measurement_button_state(), true);
        assert_eq!(frame.parking_sensors_button_state(), false);
        assert_eq!(frame.auto_main_beam_button_state(), true);
        assert_eq!(frame.lane_centering_button_state(), false);
        assert_eq!(frame.blind_spot_monitoring_button_state(), true);
        assert_eq!(frame.adaptive_cruise_control_plus_button_state(), false);
        assert_eq!(frame.adaptive_cruise_control_minus_button_state(), true);
        assert_eq!(frame.cruise_control_speed_instruction(), 51);
        assert_eq!(frame.indirect_under_inflation_button_state(), true);
        assert_eq!(frame.auto_parking_state_change_request(), false);
        assert_eq!(frame.collision_alert_failure_display_request(), true);
        assert_eq!(
            frame.cruise_control_spd_setting_instruction_pos(),
            CruiseControlCustomSettingPosition::Position6
        );
        assert_eq!(frame.fault_check_request(), false);
        assert_eq!(frame.telematic_screen_lighting_level(), 8);
        assert_eq!(frame.telematic_unit_life_state(), 2);
        assert_eq!(frame.stop_start_button_state(), true);
        assert_eq!(frame.visual_parking_assistance_button_state(), 3);
        assert_eq!(frame.cruise_control_spd_instruction_val_request(), true);
        assert_eq!(
            frame.visual_parking_assistance_panoramic_view_button_state(),
            false
        );
        assert_eq!(frame.front_visual_parking_assistance_button_state(), true);
        assert_eq!(frame.rear_visual_parking_assistance_button_state(), false);
        assert_eq!(frame.visual_parking_assistance_activation_request(), true);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_trip_computer_secondary_trip_reset_request(true);
        frame.set_trip_computer_primary_trip_reset_request(false);
        frame.set_adaptive_cruise_control_button_state(true);
        frame.set_auto_parking_mode(AutomaticParkingMode::SCP6);
        frame.set_telematics_enabled(false);
        frame.set_black_panel_enabled(true);
        frame.set_interactive_message(0);
        frame.set_stop_check_request(false);
        frame.set_cruise_control_custom_speed_mem_request(true);
        frame.set_available_space_measurement_button_state(false);
        frame.set_parking_sensors_button_state(true);
        frame.set_auto_main_beam_button_state(false);
        frame.set_lane_centering_button_state(true);
        frame.set_blind_spot_monitoring_button_state(false);
        frame.set_adaptive_cruise_control_plus_button_state(true);
        frame.set_adaptive_cruise_control_minus_button_state(false);
        frame.set_cruise_control_speed_instruction(0);
        frame.set_indirect_under_inflation_button_state(false);
        frame.set_auto_parking_state_change_request(true);
        frame.set_collision_alert_failure_display_request(false);
        frame.set_cruise_control_spd_setting_instruction_pos(
            CruiseControlCustomSettingPosition::Position1,
        );
        frame.set_fault_check_request(true);
        frame.set_telematic_screen_lighting_level(0);
        frame.set_telematic_unit_life_state(0);
        frame.set_stop_start_button_state(false);
        frame.set_visual_parking_assistance_button_state(0);
        frame.set_cruise_control_spd_instruction_val_request(false);
        frame.set_visual_parking_assistance_panoramic_view_button_state(true);
        frame.set_front_visual_parking_assistance_button_state(false);
        frame.set_rear_visual_parking_assistance_button_state(true);
        frame.set_visual_parking_assistance_activation_request(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_trip_computer_secondary_trip_reset_request(false);
        frame.set_trip_computer_primary_trip_reset_request(true);
        frame.set_adaptive_cruise_control_button_state(false);
        frame.set_auto_parking_mode(AutomaticParkingMode::SCP9);
        frame.set_telematics_enabled(true);
        frame.set_black_panel_enabled(false);
        frame.set_interactive_message(33);
        frame.set_stop_check_request(true);
        frame.set_cruise_control_custom_speed_mem_request(false);
        frame.set_available_space_measurement_button_state(true);
        frame.set_parking_sensors_button_state(false);
        frame.set_auto_main_beam_button_state(true);
        frame.set_lane_centering_button_state(false);
        frame.set_blind_spot_monitoring_button_state(true);
        frame.set_adaptive_cruise_control_plus_button_state(false);
        frame.set_adaptive_cruise_control_minus_button_state(true);
        frame.set_cruise_control_speed_instruction(51);
        frame.set_indirect_under_inflation_button_state(true);
        frame.set_auto_parking_state_change_request(false);
        frame.set_collision_alert_failure_display_request(true);
        frame.set_cruise_control_spd_setting_instruction_pos(
            CruiseControlCustomSettingPosition::Position6,
        );
        frame.set_fault_check_request(false);
        frame.set_telematic_screen_lighting_level(8);
        frame.set_telematic_unit_life_state(2);
        frame.set_stop_start_button_state(true);
        frame.set_visual_parking_assistance_button_state(3);
        frame.set_cruise_control_spd_instruction_val_request(true);
        frame.set_visual_parking_assistance_panoramic_view_button_state(false);
        frame.set_front_visual_parking_assistance_button_state(true);
        frame.set_rear_visual_parking_assistance_button_state(false);
        frame.set_visual_parking_assistance_activation_request(true);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x85, 0x00, 0x00, 0x55, 0x00, 0x14, 0x01, 0x50, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x85, 0x00, 0x00, 0x55, 0x00, 0x14, 0x01];
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
