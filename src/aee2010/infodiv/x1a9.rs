use core::{cmp::Ordering, fmt, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};

use crate::{
    vehicle::{AutomaticParkingMode, CruiseControlCustomSettingPosition},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
1A9 DEMANDES_IVI_APPUI_PUSH_DETECTE_HS7_1A9         // NOPE
1A9 DEMANDES_IVI_APP_URG_HS7_1A9                    // NOPE
1A9 DEMANDES_IVI_CGT_XVV_CONS_MEMO_CLOSE_HS7_1A9    // OK
1A9 DEMANDES_IVI_DEFAUT_AFF_ARC_HS7_1A9             // OK
1A9 DEMANDES_IVI_DMD_CHG_ET_SCP_SEC_HS7_1A9         // OK
1A9 DEMANDES_IVI_DMD_XVV_CONS_VIT_PROG_HS7_1A9      // OK
1A9 DEMANDES_IVI_ENT_CONS_ARTIV_MOINS_HS7_1A9       // OK
1A9 DEMANDES_IVI_ENT_CONS_ARTIV_PLUS_HS7_1A9        // OK
1A9 DEMANDES_IVI_ENT_PUSH_AAS_HS7_1A9               // OK
1A9 DEMANDES_IVI_ENT_PUSH_ACTIV_AVP_HS7_1A9         // OK
1A9 DEMANDES_IVI_ENT_PUSH_ARTIV_HS7_1A9             // OK
1A9 DEMANDES_IVI_ENT_PUSH_AVP_AR_HS7_1A9            // OK
1A9 DEMANDES_IVI_ENT_PUSH_AVP_AV_HS7_1A9            // OK
1A9 DEMANDES_IVI_ENT_PUSH_AVP_PANO_HS7_1A9          // OK
1A9 DEMANDES_IVI_ENT_PUSH_CAFR_HS7_1A9              // OK
1A9 DEMANDES_IVI_ENT_PUSH_CHECK_TACT_HS7_1A9        // OK
1A9 DEMANDES_IVI_ENT_PUSH_DSGI_HS7_1A9              // OK
1A9 DEMANDES_IVI_ENT_PUSH_MPD_HS7_1A9               // OK
1A9 DEMANDES_IVI_ENT_PUSH_SAM_HS7_1A9               // OK
1A9 DEMANDES_IVI_ENT_PUSH_STL_HS7_1A9               // OK
1A9 DEMANDES_IVI_ENT_PUSH_STT_HS7_1A9               // OK
1A9 DEMANDES_IVI_ENT_PUSH_VUE_VPARK_HS7_1A9         // OK
1A9 DEMANDES_IVI_ETAT_BP_DARK_HS7_1A9               // OK
1A9 DEMANDES_IVI_NIV_LUM_TACT_HS7_1A9               // OK
1A9 DEMANDES_IVI_PHASE_VIE_BTEL_HS7_1A9             // OK
1A9 DEMANDES_IVI_POINT_MESS_INTERACTIF_HS7_1A9      // OK
1A9 DEMANDES_IVI_RAZ_CUMT1_DDES_EMF_HS7_1A9         // OK
1A9 DEMANDES_IVI_RAZ_CUMT2_DDES_EMF_HS7_1A9         // OK
1A9 DEMANDES_IVI_SEL_MENU_CPK_HS7_1A9               // OK
1A9 DEMANDES_IVI_S_FCT_TELE_HS7_1A9                 // OK
1A9 DEMANDES_IVI_STOP_CHECK_EMF_HS7_1A9             // OK
1A9 DEMANDES_IVI_XVV_CONS_VIT_PROG_HS7_1A9          // OK
1A9 DEMANDES_IVI_XVV_POS_C_VIT_PROG_HS7_1A9         // OK
1A9 DEMANDES_IVI_XVV_VALID_C_VIT_PROG_HS7_1A9       // OK
 */

mod field {
    use crate::field::Field;
    /// 1-bit trip computer secondary trip reset request,
    /// 1-bit trip computer primary trip reset request,
    /// 1-bit adaptive cruise-control push button state,
    /// 2-bit automatic parking mode selection,
    /// 1-bit telematics state,
    /// 1-bit empty,
    /// 1-bit black panel function state.
    pub const REQ_0: usize = 0;
    /// 15-bit interactive message.
    /// 1-bit MFD stop check request.
    pub const INTERACTIVE_MSG_STOP_CHK: Field = 1..3;
    /// 1-bit cruise-control custom speed memorization request,
    /// 1-bit available space measurement push button state,
    /// 1-bit parking sensors push button state,
    /// 1-bit automatic main beam push button state,
    /// 1-bit lane centering push button state,
    /// 1-bit blind spot monitoring push button state,
    /// 1-bit adaptive cruise-control '+' push button state,
    /// 1-bit adaptive cruise-control '-' push button state,
    pub const REQ_1: usize = 3;
    /// 8-bit cruise-control speed instruction value field.
    pub const CC_SPD: usize = 4;
    /// 1-bit indirect under-inflation push button state,
    /// 1-bit empty,
    /// 1-bit automatic parking state change request flag,
    /// 1-bit collision alert failure display request flag,
    /// 3-bit cruise-control speed setting instruction position field,
    /// 1-bit empty.
    pub const REQ_2: usize = 5;
    /// 1-bit fault check request flag,
    /// 4-bit telematic screen lighting level value field,
    /// 2-bit telematic unit life state field,
    /// 1-bit Stop & Start push button state flag.
    pub const REQ_3: usize = 6;
    /// 3-bit 'visiopark' visual parking assistance push button state field,
    /// 1-bit cruise-control speed instruction value request flag,
    /// 1-bit visual parking assistance panoramic view push button state flag,
    /// 1-bit front visual parking assistance push button state flag,
    /// 1-bit rear visual parking assistance push button state flag,
    /// 1-bit visual parking assistance activation request flag.
    pub const REQ_4: usize = 7;
}

/// Raw x1a9 CAN frame identifier.
pub const FRAME_ID: u16 = 0x1a9;
/// Length of a x1a9 CAN frame.
pub const FRAME_LEN: usize = field::REQ_4 + 1;

/// Periodicity of a x1a9 CAN frame.
pub const PERIODICITY: Duration = Duration::from_millis(200);

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

    /// Return the trip computer secondary trip reset request flag.
    #[inline]
    pub fn trip_computer_secondary_trip_reset_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x01 != 0
    }

    /// Return the trip computer primary trip reset request flag.
    #[inline]
    pub fn trip_computer_primary_trip_reset_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x02 != 0
    }

    /// Return the adaptive cruise-control push button state flag.
    #[inline]
    pub fn adaptive_cruise_control_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x04 != 0
    }

    /// Return the automatic parking mode selection field.
    #[inline]
    pub fn auto_parking_mode(&self) -> AutomaticParkingMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::REQ_0] & 0x18) >> 3;
        AutomaticParkingMode::from(raw)
    }

    /// Return the telematics enabled flag.
    #[inline]
    pub fn telematics_enabled(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x20 != 0
    }

    /// Return the black panel function state flag.
    #[inline]
    pub fn black_panel_enabled(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x80 != 0
    }

    /// Return the interactive message field.
    #[inline]
    pub fn interactive_message(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::INTERACTIVE_MSG_STOP_CHK]) & 0x7fff
    }

    /// Return the MFD stop check request field.
    #[inline]
    pub fn stop_check_request(&self) -> bool {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::INTERACTIVE_MSG_STOP_CHK]);
        raw & !0x7fff != 0
    }

    /// Return the cruise-control custom speed memorization request flag.
    #[inline]
    pub fn cruise_control_custom_speed_mem_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x01 != 0
    }

    /// Return the available space measurement push button state flag.
    #[inline]
    pub fn available_space_measurement_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x02 != 0
    }

    /// Return the parking sensors push button state flag.
    #[inline]
    pub fn parking_sensors_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x04 != 0
    }

    /// Return the automatic main beam push button state flag.
    #[inline]
    pub fn auto_main_beam_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x08 != 0
    }

    /// Return the lane centering push button state flag.
    #[inline]
    pub fn lane_centering_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x10 != 0
    }

    /// Return the blind spot monitoring push button state flag.
    #[inline]
    pub fn blind_spot_monitoring_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x20 != 0
    }

    /// Return the adaptive cruise-control '+' push button state flag.
    #[inline]
    pub fn adaptive_cruise_control_plus_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x40 != 0
    }

    /// Return the adaptive cruise-control '-' push button state flag.
    #[inline]
    pub fn adaptive_cruise_control_minus_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x80 != 0
    }

    /// Return the cruise-control speed instruction value field.
    #[inline]
    pub fn cruise_control_speed_instruction(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::CC_SPD]
    }

    /// Return the indirect under-inflation push button state flag.
    #[inline]
    pub fn indirect_under_inflation_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_2] & 0x01 != 0
    }

    /// Return the automatic parking state change request flag.
    #[inline]
    pub fn auto_parking_state_change_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_2] & 0x04 != 0
    }

    /// Return the collision alert failure display request flag.
    #[inline]
    pub fn collision_alert_failure_display_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_2] & 0x08 != 0
    }

    /// Return the cruise-control speed setting instruction position field.
    #[inline]
    pub fn cruise_control_spd_setting_instruction_pos(&self) -> CruiseControlCustomSettingPosition {
        let data = self.buffer.as_ref();
        let raw = (data[field::REQ_2] & 0x70) >> 4;
        CruiseControlCustomSettingPosition::from(raw)
    }

    /// Return the fault check request flag.
    #[inline]
    pub fn fault_check_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_3] & 0x01 != 0
    }

    /// Return the telematic screen lighting level value field.
    #[inline]
    pub fn telematic_screen_lighting_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::REQ_3] & 0x1e) >> 1
    }

    /// Return the telematic unit life state field.
    #[inline]
    pub fn telematic_unit_life_state(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::REQ_3] & 0x60) >> 5
    }

    /// Return the Stop & Start push button state flag.
    #[inline]
    pub fn stop_start_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_3] & 0x80 != 0
    }

    /// Return the 'visiopark' visual parking assistance push button state field.
    #[inline]
    pub fn visual_parking_assistance_button_state(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::REQ_4] & 0x07
    }

    /// Return the cruise-control speed instruction value request flag.
    #[inline]
    pub fn cruise_control_spd_instruction_val_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_4] & 0x08 != 0
    }

    /// Return the visual parking assistance panoramic view push button state flag.
    #[inline]
    pub fn visual_parking_assistance_panoramic_view_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_4] & 0x10 != 0
    }

    /// Return the front visual parking assistance push button state flag.
    #[inline]
    pub fn front_visual_parking_assistance_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_4] & 0x20 != 0
    }

    /// Return the rear visual parking assistance push button state flag.
    #[inline]
    pub fn rear_visual_parking_assistance_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_4] & 0x40 != 0
    }

    /// Return the visual parking assistance activation request flag.
    #[inline]
    pub fn visual_parking_assistance_activation_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_4] & 0x80 != 0
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
                write!(f, "x1a9 ({})", err)?;
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

/// A high-level representation of a x1a9 CAN frame.
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

    /// Emit a high-level representation into a x1a9 CAN frame.
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
            "x1a9 trip_computer_secondary_trip_reset_request={}",
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
