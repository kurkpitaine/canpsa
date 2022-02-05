use core::fmt;

use crate::{config::UnderInflationDetectionSystem, Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 1-bit daytime running lamps option presence flag,
    /// 1-bit automatic headlamps option presence flag,
    /// 1-bit mood lighting option presence flag,
    /// 1-bit blind spot monitoring option presence flag,
    /// 1-bit adaptive lamps option presence flag.
    /// 1-bit welcome lighting option presence flag,
    /// 1-bit motorway lighting option presence flag,
    /// 1-bit configuration menu information global availability flag.
    pub const OPT_0: usize = 0;
    /// 1-bit selective unlocking option presence flag,
    /// 1-bit key selective unlocking option presence flag,
    /// 1-bit boot selective unlocking option presence flag,
    /// 1-bit motorized tailgate option presence flag,
    /// 1-bit welcome function option presence flag,
    /// 1-bit follow-me-home option presence flag,
    /// 1-bit rear wiper in reverse gear option presence flag,
    /// 1-bit parking sensors inhibition option presence flag.
    pub const OPT_1: usize = 1;
    /// 1-bit empty,
    /// 1-bit extended traffic sign recognition option presence flag,
    /// 1-bit 'IMA' option presence flag,
    /// 1-bit sound harmony option presence flag,
    /// 1-bit automatic electrical parking brake application presence option flag,
    /// 1-bit configurable button/key option presence flag,
    /// 1-bit cruise-control option presence flag,
    /// 1-bit Seat belt not fastened / unfastened warning lamps presence flag.
    pub const OPT_2: usize = 2;
    /// 3-bit under-inflation detection option system type,
    /// 1-bit gear efficiency indicator presence flag,
    /// 1-bit cruise-control memorized speeds setting menu option presence flag,
    /// 1-bit collision alert sensibility setting menu option presence flag,
    /// 1-bit automatic emergency braking option presence flag,
    /// 1-bit under-inflation detection reset menu option presence flag.
    pub const OPT_3: usize = 3;
    /// 1-bit hands-free tailgate automatic locking menu option presence flag,
    /// 1-bit empty,
    /// 1-bit hands-free tailgate option presence flag,
    /// 1-bit speed limit recognition option presence flag,
    /// 1-bit radiator grill lamps option presence flag (maybe anti-fog lights),
    /// 1-bit 'CFC' option presence flag,
    /// 2-bit empty.
    pub const OPT_4: usize = 4;
    /// 1-bit 'IRV' option presence flag (maybe InfraRed Vision),
    /// 4-bit empty,
    /// 1-bit automatic main beam option presence flag,
    /// 1-bit 'ECS' option presence flag,
    /// 1-bit driver alert assist option presence flag.
    pub const OPT_5: usize = 5;
}

/// Length of a x361 CAN frame.
pub const FRAME_LEN: usize = field::OPT_5 + 1;

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

    /// Return the daytime running lamps option presence flag.
    #[inline]
    pub fn daytime_running_lamps_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_0] & 0x01 != 0
    }

    /// Return the automatic headlamps option presence flag.
    #[inline]
    pub fn automatic_headlamps_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_0] & 0x02 != 0
    }

    /// Return the mood lighting option presence flag.
    #[inline]
    pub fn mood_lighting_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_0] & 0x04 != 0
    }

    /// Return the blind spot monitoring option presence flag.
    #[inline]
    pub fn blind_spot_monitoring_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_0] & 0x08 != 0
    }

    /// Return the adaptive lamps option presence flag.
    #[inline]
    pub fn adaptive_lamps_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_0] & 0x10 != 0
    }

    /// Return the welcome lighting option presence flag.
    #[inline]
    pub fn welcome_lighting_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_0] & 0x20 != 0
    }

    /// Return the motorway lighting option presence flag.
    #[inline]
    pub fn motorway_lighting_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_0] & 0x40 != 0
    }

    /// Return the configuration menu information global availability flag.
    #[inline]
    pub fn config_menu_information_availability(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_0] & 0x80 != 0
    }

    /// Return the selective unlocking option presence flag.
    #[inline]
    pub fn selective_unlocking_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x01 != 0
    }

    /// Return the key selective unlocking option presence flag.
    #[inline]
    pub fn key_selective_unlocking_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x02 != 0
    }

    /// Return the boot selective unlocking option presence flag.
    #[inline]
    pub fn boot_selective_unlocking_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x04 != 0
    }

    /// Return the motorized tailgate option presence flag.
    #[inline]
    pub fn motorized_tailgate_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x08 != 0
    }

    /// Return the welcome function option presence flag.
    #[inline]
    pub fn welcome_function_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x10 != 0
    }

    /// Return the follow-me-home option presence flag.
    #[inline]
    pub fn follow_me_home_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x20 != 0
    }

    /// Return the rear wiper in reverse gear option presence flag.
    #[inline]
    pub fn rear_wiper_in_reverse_gear_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x40 != 0
    }

    /// Return the parking sensors inhibition option presence flag.
    #[inline]
    pub fn park_sensors_inhibition_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x80 != 0
    }

    /// Return the extended traffic sign recognition option presence flag.
    #[inline]
    pub fn extended_traffic_sign_recognition_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x02 != 0
    }

    /// Return the 'IMA' option presence flag.
    #[inline]
    pub fn ima_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x04 != 0
    }

    /// Return the sound harmony option presence flag.
    #[inline]
    pub fn sound_harmony_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x08 != 0
    }

    /// Return the automatic electrical parking brake application option presence flag.
    #[inline]
    pub fn auto_elec_parking_brake_application_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x10 != 0
    }

    /// Return the configurable button/key option presence flag.
    #[inline]
    pub fn configurable_key_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x20 != 0
    }

    /// Return the cruise-control option presence flag.
    #[inline]
    pub fn cruise_control_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x40 != 0
    }

    /// Return the seat belt not fastened / unfastened warning lamps presence flag.
    #[inline]
    pub fn seat_belt_status_lamps_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x80 != 0
    }

    /// Return the under-inflation detection option system type field.
    #[inline]
    pub fn under_inflation_detection(&self) -> UnderInflationDetectionSystem {
        let data = self.buffer.as_ref();
        let raw = data[field::OPT_3] & 0x07;
        UnderInflationDetectionSystem::from(raw)
    }

    /// Return the gear efficiency indicator option presence flag.
    #[inline]
    pub fn gear_efficiency_indicator_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x08 != 0
    }

    /// Return the cruise-control memorized speeds setting menu option presence flag.
    #[inline]
    pub fn cruise_control_memorized_speeds_menu_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x10 != 0
    }

    /// Return the collision alert sensibility setting menu option presence flag.
    #[inline]
    pub fn collision_alert_sensibility_menu_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x20 != 0
    }

    /// Return the automatic emergency braking option presence flag.
    #[inline]
    pub fn automatic_emergency_braking_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x40 != 0
    }

    /// Return the under-inflation detection reset menu option presence flag.
    #[inline]
    pub fn under_inflation_detection_reset_menu_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x80 != 0
    }

    /// Return the hands-free tailgate automatic locking menu option presence flag.
    #[inline]
    pub fn hands_free_tailgate_auto_lock_menu_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x01 != 0
    }

    /// Return the hands-free tailgate option presence flag.
    #[inline]
    pub fn hands_free_tailgate_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x04 != 0
    }

    /// Return the speed limit recognition option presence flag.
    #[inline]
    pub fn speed_limit_recognition_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x08 != 0
    }

    /// Return the radiator grill lamps option presence flag (maybe anti-fog lights).
    #[inline]
    pub fn radiator_grill_lamps_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x10 != 0
    }

    /// Return the 'CFC' option presence flag.
    #[inline]
    pub fn cfc_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x20 != 0
    }

    /// Return the 'IRV' option presence flag (maybe InfraRed Vision).
    #[inline]
    pub fn irv_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x01 != 0
    }

    /// Return the automatic main beam option presence flag.
    #[inline]
    pub fn automatic_main_beam_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x20 != 0
    }

    /// Return the 'ECS' option presence flag.
    #[inline]
    pub fn ecs_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x40 != 0
    }

    /// Return the driver alert assist option presence flag.
    #[inline]
    pub fn driver_alert_assist_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x80 != 0
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the daytime running lamps option presence flag.
    #[inline]
    pub fn set_daytime_running_lamps_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_0] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_0] = raw;
    }

    /// Set the automatic headlamps option presence flag.
    #[inline]
    pub fn set_automatic_headlamps_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_0] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_0] = raw;
    }

    /// Set the mood lighting option presence flag.
    #[inline]
    pub fn set_mood_lighting_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_0] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_0] = raw;
    }

    /// Set the blind spot monitoring option presence flag.
    #[inline]
    pub fn set_blind_spot_monitoring_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_0] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_0] = raw;
    }

    /// Set the adaptive lamps option presence flag.
    #[inline]
    pub fn set_adaptive_lamps_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_0] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_0] = raw;
    }

    /// Set the welcome lighting option presence flag.
    #[inline]
    pub fn set_welcome_lighting_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_0] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_0] = raw;
    }

    /// Set the motorway lighting option presence flag.
    #[inline]
    pub fn set_motorway_lighting_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_0] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_0] = raw;
    }

    /// Set the configuration menu information global availability flag.
    #[inline]
    pub fn set_config_menu_information_availability(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_0] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_0] = raw;
    }

    /// Set the selective unlocking option presence flag.
    #[inline]
    pub fn set_selective_unlocking_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_1] = raw;
    }

    /// Set the key selective unlocking option presence flag.
    #[inline]
    pub fn set_key_selective_unlocking_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_1] = raw;
    }

    /// Set the boot selective unlocking option presence flag.
    #[inline]
    pub fn set_boot_selective_unlocking_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_1] = raw;
    }

    /// Set the motorized tailgate option presence flag.
    #[inline]
    pub fn set_motorized_tailgate_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_1] = raw;
    }

    /// Set the welcome function option presence flag.
    #[inline]
    pub fn set_welcome_function_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_1] = raw;
    }

    /// Set the follow-me-home option presence flag.
    #[inline]
    pub fn set_follow_me_home_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_1] = raw;
    }

    /// Set the rear wiper in reverse gear option presence flag.
    #[inline]
    pub fn set_rear_wiper_in_reverse_gear_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_1] = raw;
    }

    /// Set the parking sensors inhibition option presence flag.
    #[inline]
    pub fn set_park_sensors_inhibition_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_1] = raw;
    }

    /// Set the extended traffic sign recognition option presence flag.
    #[inline]
    pub fn set_extended_traffic_sign_recognition_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_2] = raw;
    }

    /// Set the 'IMA' option presence flag.
    #[inline]
    pub fn set_ima_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_2] = raw;
    }

    /// Set the sound harmony option presence flag.
    #[inline]
    pub fn set_sound_harmony_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_2] = raw;
    }

    /// Set the automatic electrical parking brake application option presence flag.
    #[inline]
    pub fn set_auto_elec_parking_brake_application_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_2] = raw;
    }

    /// Set the configurable button/key option presence flag.
    #[inline]
    pub fn set_configurable_key_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_2] = raw;
    }

    /// Set the cruise-control option presence flag.
    #[inline]
    pub fn set_cruise_control_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_2] = raw;
    }

    /// Set the seat belt not fastened / unfastened warning lamps presence flag.
    #[inline]
    pub fn set_seat_belt_status_lamps_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_2] = raw;
    }

    /// Set the under-inflation detection option system type field.
    #[inline]
    pub fn set_under_inflation_detection(&mut self, value: UnderInflationDetectionSystem) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x07;
        let raw = raw | (u8::from(value) & 0x07);
        data[field::OPT_3] = raw;
    }

    /// Set the gear efficiency indicator option presence flag.
    #[inline]
    pub fn set_gear_efficiency_indicator_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_3] = raw;
    }

    /// Set the cruise-control memorized speeds setting menu option presence flag.
    #[inline]
    pub fn set_cruise_control_memorized_speeds_menu_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_3] = raw;
    }

    /// Set the collision alert sensibility setting menu option presence flag.
    #[inline]
    pub fn set_collision_alert_sensibility_menu_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_3] = raw;
    }

    /// Set the automatic emergency braking option presence flag.
    #[inline]
    pub fn set_automatic_emergency_braking_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_3] = raw;
    }

    /// Set the under-inflation detection reset menu option presence flag.
    #[inline]
    pub fn set_under_inflation_detection_reset_menu_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_3] = raw;
    }

    /// Set the hands-free tailgate automatic locking menu option presence flag.
    #[inline]
    pub fn set_hands_free_tailgate_auto_lock_menu_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_4] = raw;
    }

    /// Set the hands-free tailgate option presence flag.
    #[inline]
    pub fn set_hands_free_tailgate_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_4] = raw;
    }

    /// Set the speed limit recognition option presence flag.
    #[inline]
    pub fn set_speed_limit_recognition_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_4] = raw;
    }

    /// Set the radiator grill lamps option presence flag (maybe anti-fog lights).
    #[inline]
    pub fn set_radiator_grill_lamps_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_4] = raw;
    }

    /// Set the 'CFC' option presence flag.
    #[inline]
    pub fn set_cfc_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_4] = raw;
    }

    /// Set the 'IRV' option presence flag (maybe InfraRed Vision).
    #[inline]
    pub fn set_irv_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_5] = raw;
    }

    /// Set the automatic main beam option presence flag.
    #[inline]
    pub fn set_automatic_main_beam_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_5] = raw;
    }

    /// Set the 'ECS' option presence flag.
    #[inline]
    pub fn set_ecs_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_5] = raw;
    }

    /// Set the driver alert assist option presence flag.
    #[inline]
    pub fn set_driver_alert_assist_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_5] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x361 ({})", err)?;
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

/// A high-level representation of a x361 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    daytime_running_lamps_present: bool,
    automatic_headlamps_present: bool,
    mood_lighting_present: bool,
    blind_spot_monitoring_present: bool,
    adaptive_lamps_present: bool,
    welcome_lighting_present: bool,
    motorway_lighting_present: bool,
    config_menu_info_available: bool,
    selective_unlocking_present: bool,
    key_selective_unlocking_present: bool,
    boot_selective_unlocking_present: bool,
    motorized_tailgate_present: bool,
    welcome_function_present: bool,
    follow_me_home_present: bool,
    rear_wiper_in_reverse_gear_present: bool,
    parking_sensors_inhibition_present: bool,
    extended_traffic_sign_recognition_present: bool,
    ima_present: bool,
    sound_harmony_present: bool,
    automatic_electric_parking_brake_application_present: bool,
    configurable_key_present: bool,
    cruise_control_present: bool,
    seat_belt_status_lamps_present: bool,
    under_inflation_detection: UnderInflationDetectionSystem,
    gear_efficiency_indicator_present: bool,
    cruise_control_memorized_speeds_menu_present: bool,
    collision_alert_sensibility_menu_present: bool,
    automatic_emergency_braking_present: bool,
    under_inflation_detection_reset_menu_present: bool,
    hands_free_tailgate_auto_lock_menu_present: bool,
    hands_free_tailgate_present: bool,
    speed_limit_recognition_present: bool,
    radiator_grill_lamps_present: bool,
    cfc_present: bool,
    irv_present: bool,
    automatic_main_beam_present: bool,
    ecs_present: bool,
    driver_alert_assist_present: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            daytime_running_lamps_present: frame.daytime_running_lamps_presence(),
            automatic_headlamps_present: frame.automatic_headlamps_presence(),
            mood_lighting_present: frame.mood_lighting_presence(),
            blind_spot_monitoring_present: frame.blind_spot_monitoring_presence(),
            adaptive_lamps_present: frame.adaptive_lamps_presence(),
            welcome_lighting_present: frame.welcome_lighting_presence(),
            motorway_lighting_present: frame.motorway_lighting_presence(),
            config_menu_info_available: frame.config_menu_information_availability(),
            selective_unlocking_present: frame.selective_unlocking_presence(),
            key_selective_unlocking_present: frame.key_selective_unlocking_presence(),
            boot_selective_unlocking_present: frame.boot_selective_unlocking_presence(),
            motorized_tailgate_present: frame.motorized_tailgate_presence(),
            welcome_function_present: frame.welcome_function_presence(),
            follow_me_home_present: frame.follow_me_home_presence(),
            rear_wiper_in_reverse_gear_present: frame.rear_wiper_in_reverse_gear_presence(),
            parking_sensors_inhibition_present: frame.park_sensors_inhibition_presence(),
            extended_traffic_sign_recognition_present: frame
                .extended_traffic_sign_recognition_presence(),
            ima_present: frame.ima_presence(),
            sound_harmony_present: frame.sound_harmony_presence(),
            automatic_electric_parking_brake_application_present: frame
                .auto_elec_parking_brake_application_presence(),
            configurable_key_present: frame.configurable_key_presence(),
            cruise_control_present: frame.cruise_control_presence(),
            seat_belt_status_lamps_present: frame.seat_belt_status_lamps_presence(),
            under_inflation_detection: frame.under_inflation_detection(),
            gear_efficiency_indicator_present: frame.gear_efficiency_indicator_presence(),
            cruise_control_memorized_speeds_menu_present: frame
                .cruise_control_memorized_speeds_menu_presence(),
            collision_alert_sensibility_menu_present: frame
                .collision_alert_sensibility_menu_presence(),
            automatic_emergency_braking_present: frame.automatic_emergency_braking_presence(),
            under_inflation_detection_reset_menu_present: frame
                .under_inflation_detection_reset_menu_presence(),
            hands_free_tailgate_auto_lock_menu_present: frame
                .hands_free_tailgate_auto_lock_menu_presence(),
            hands_free_tailgate_present: frame.hands_free_tailgate_presence(),
            speed_limit_recognition_present: frame.speed_limit_recognition_presence(),
            radiator_grill_lamps_present: frame.radiator_grill_lamps_presence(),
            cfc_present: frame.cfc_presence(),
            irv_present: frame.irv_presence(),
            automatic_main_beam_present: frame.automatic_main_beam_presence(),
            ecs_present: frame.ecs_presence(),
            driver_alert_assist_present: frame.driver_alert_assist_presence(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x361 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_daytime_running_lamps_presence(self.daytime_running_lamps_present);
        frame.set_automatic_headlamps_presence(self.automatic_headlamps_present);
        frame.set_mood_lighting_presence(self.mood_lighting_present);
        frame.set_blind_spot_monitoring_presence(self.blind_spot_monitoring_present);
        frame.set_adaptive_lamps_presence(self.adaptive_lamps_present);
        frame.set_welcome_lighting_presence(self.welcome_lighting_present);
        frame.set_motorway_lighting_presence(self.motorway_lighting_present);
        frame.set_config_menu_information_availability(self.config_menu_info_available);
        frame.set_selective_unlocking_presence(self.selective_unlocking_present);
        frame.set_key_selective_unlocking_presence(self.key_selective_unlocking_present);
        frame.set_boot_selective_unlocking_presence(self.boot_selective_unlocking_present);
        frame.set_motorized_tailgate_presence(self.motorized_tailgate_present);
        frame.set_welcome_function_presence(self.welcome_function_present);
        frame.set_follow_me_home_presence(self.follow_me_home_present);
        frame.set_rear_wiper_in_reverse_gear_presence(self.rear_wiper_in_reverse_gear_present);
        frame.set_park_sensors_inhibition_presence(self.parking_sensors_inhibition_present);
        frame.set_extended_traffic_sign_recognition_presence(
            self.extended_traffic_sign_recognition_present,
        );
        frame.set_ima_presence(self.ima_present);
        frame.set_sound_harmony_presence(self.sound_harmony_present);
        frame.set_auto_elec_parking_brake_application_presence(
            self.automatic_electric_parking_brake_application_present,
        );
        frame.set_configurable_key_presence(self.configurable_key_present);
        frame.set_cruise_control_presence(self.cruise_control_present);
        frame.set_seat_belt_status_lamps_presence(self.seat_belt_status_lamps_present);
        frame.set_under_inflation_detection(self.under_inflation_detection);
        frame.set_gear_efficiency_indicator_presence(self.gear_efficiency_indicator_present);
        frame.set_cruise_control_memorized_speeds_menu_presence(
            self.cruise_control_memorized_speeds_menu_present,
        );
        frame.set_collision_alert_sensibility_menu_presence(
            self.collision_alert_sensibility_menu_present,
        );
        frame.set_automatic_emergency_braking_presence(self.automatic_emergency_braking_present);
        frame.set_under_inflation_detection_reset_menu_presence(
            self.under_inflation_detection_reset_menu_present,
        );
        frame.set_hands_free_tailgate_auto_lock_menu_presence(
            self.hands_free_tailgate_auto_lock_menu_present,
        );
        frame.set_hands_free_tailgate_presence(self.hands_free_tailgate_present);
        frame.set_speed_limit_recognition_presence(self.speed_limit_recognition_present);
        frame.set_radiator_grill_lamps_presence(self.radiator_grill_lamps_present);
        frame.set_cfc_presence(self.cfc_present);
        frame.set_irv_presence(self.irv_present);
        frame.set_automatic_main_beam_presence(self.automatic_main_beam_present);
        frame.set_ecs_presence(self.ecs_present);
        frame.set_driver_alert_assist_presence(self.driver_alert_assist_present);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "daytime running lamps present={}",
            self.daytime_running_lamps_present
        )?;
        write!(
            f,
            "automatic headlamps present={}",
            self.automatic_headlamps_present
        )?;
        write!(f, "mood lighting present={}", self.mood_lighting_present)?;
        write!(
            f,
            "blind spot monitoring present={}",
            self.blind_spot_monitoring_present
        )?;
        write!(f, "adaptive lamps present={}", self.adaptive_lamps_present)?;
        write!(
            f,
            "welcome lighting present={}",
            self.welcome_lighting_present
        )?;
        write!(
            f,
            "motorway lighting present={}",
            self.motorway_lighting_present
        )?;
        write!(
            f,
            "configuration menu information available={}",
            self.config_menu_info_available
        )?;
        write!(
            f,
            "selective unlocking present={}",
            self.selective_unlocking_present
        )?;
        write!(
            f,
            "key selective unlocking present={}",
            self.key_selective_unlocking_present
        )?;
        write!(
            f,
            "boot selective unlocking present={}",
            self.boot_selective_unlocking_present
        )?;
        write!(
            f,
            "motorized tailgate present={}",
            self.motorized_tailgate_present
        )?;
        write!(
            f,
            "welcome function present={}",
            self.welcome_function_present
        )?;
        write!(f, "follow-me-home present={}", self.follow_me_home_present)?;
        write!(
            f,
            "rear wiper in reverse gear present={}",
            self.rear_wiper_in_reverse_gear_present
        )?;
        write!(
            f,
            "parking sensors inhibition present={}",
            self.parking_sensors_inhibition_present
        )?;
        write!(
            f,
            "extended traffic sign recognition present={}",
            self.extended_traffic_sign_recognition_present
        )?;
        write!(f, "'IMA' present={}", self.ima_present)?;
        write!(f, "sound harmony present={}", self.sound_harmony_present)?;
        write!(
            f,
            "automatic electric parking brake application present={}",
            self.automatic_electric_parking_brake_application_present
        )?;
        write!(
            f,
            "configurable key present={}",
            self.configurable_key_present
        )?;
        write!(f, "cruise-control present={}", self.cruise_control_present)?;
        write!(
            f,
            "seat belt status lamps present={}",
            self.seat_belt_status_lamps_present
        )?;
        write!(
            f,
            "under inflation detection={}",
            self.under_inflation_detection
        )?;
        write!(
            f,
            "gear efficiency indicator present={}",
            self.gear_efficiency_indicator_present
        )?;
        write!(
            f,
            "cruise-control memorized speeds menu present={}",
            self.cruise_control_memorized_speeds_menu_present
        )?;
        write!(
            f,
            "collision alert sensibility menu present={}",
            self.collision_alert_sensibility_menu_present
        )?;
        write!(
            f,
            "automatic emergency braking present={}",
            self.automatic_emergency_braking_present
        )?;
        write!(
            f,
            "under inflation detection reset menu present={}",
            self.under_inflation_detection_reset_menu_present
        )?;
        write!(
            f,
            "hands free tailgate auto lock menu present={}",
            self.hands_free_tailgate_auto_lock_menu_present
        )?;
        write!(
            f,
            "hands free tailgate present={}",
            self.hands_free_tailgate_present
        )?;
        write!(
            f,
            "speed limit recognition present={}",
            self.speed_limit_recognition_present
        )?;
        write!(
            f,
            "radiator grill lamps present={}",
            self.radiator_grill_lamps_present
        )?;
        write!(f, "'CFC' present={}", self.cfc_present)?;
        write!(f, "'IRV' present={}", self.irv_present)?;
        write!(
            f,
            "automatic main beam present={}",
            self.automatic_main_beam_present
        )?;
        write!(f, "'ECS' present={}", self.ecs_present)?;
        write!(
            f,
            "driver alert assist present={}",
            self.driver_alert_assist_present
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{config::UnderInflationDetectionSystem, Error};

    static REPR_FRAME_BYTES_1: [u8; 6] = [0x55, 0x55, 0x54, 0x53, 0x15, 0x41];
    static REPR_FRAME_BYTES_2: [u8; 6] = [0xaa, 0xaa, 0xaa, 0xaa, 0x28, 0xa0];

    fn frame_1_repr() -> Repr {
        Repr {
            daytime_running_lamps_present: true,
            automatic_headlamps_present: false,
            mood_lighting_present: true,
            blind_spot_monitoring_present: false,
            adaptive_lamps_present: true,
            welcome_lighting_present: false,
            motorway_lighting_present: true,
            config_menu_info_available: false,
            selective_unlocking_present: true,
            key_selective_unlocking_present: false,
            boot_selective_unlocking_present: true,
            motorized_tailgate_present: false,
            welcome_function_present: true,
            follow_me_home_present: false,
            rear_wiper_in_reverse_gear_present: true,
            parking_sensors_inhibition_present: false,
            extended_traffic_sign_recognition_present: false,
            ima_present: true,
            sound_harmony_present: false,
            automatic_electric_parking_brake_application_present: true,
            configurable_key_present: false,
            cruise_control_present: true,
            seat_belt_status_lamps_present: false,
            under_inflation_detection: UnderInflationDetectionSystem::Indirect,
            gear_efficiency_indicator_present: false,
            cruise_control_memorized_speeds_menu_present: true,
            collision_alert_sensibility_menu_present: false,
            automatic_emergency_braking_present: true,
            under_inflation_detection_reset_menu_present: false,
            hands_free_tailgate_auto_lock_menu_present: true,
            hands_free_tailgate_present: true,
            speed_limit_recognition_present: false,
            radiator_grill_lamps_present: true,
            cfc_present: false,
            irv_present: true,
            automatic_main_beam_present: false,
            ecs_present: true,
            driver_alert_assist_present: false,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            daytime_running_lamps_present: false,
            automatic_headlamps_present: true,
            mood_lighting_present: false,
            blind_spot_monitoring_present: true,
            adaptive_lamps_present: false,
            welcome_lighting_present: true,
            motorway_lighting_present: false,
            config_menu_info_available: true,
            selective_unlocking_present: false,
            key_selective_unlocking_present: true,
            boot_selective_unlocking_present: false,
            motorized_tailgate_present: true,
            welcome_function_present: false,
            follow_me_home_present: true,
            rear_wiper_in_reverse_gear_present: false,
            parking_sensors_inhibition_present: true,
            extended_traffic_sign_recognition_present: true,
            ima_present: false,
            sound_harmony_present: true,
            automatic_electric_parking_brake_application_present: false,
            configurable_key_present: true,
            cruise_control_present: false,
            seat_belt_status_lamps_present: true,
            under_inflation_detection: UnderInflationDetectionSystem::DirectWithoutAbsolutePressure,
            gear_efficiency_indicator_present: true,
            cruise_control_memorized_speeds_menu_present: false,
            collision_alert_sensibility_menu_present: true,
            automatic_emergency_braking_present: false,
            under_inflation_detection_reset_menu_present: true,
            hands_free_tailgate_auto_lock_menu_present: false,
            hands_free_tailgate_present: false,
            speed_limit_recognition_present: true,
            radiator_grill_lamps_present: false,
            cfc_present: true,
            irv_present: false,
            automatic_main_beam_present: true,
            ecs_present: false,
            driver_alert_assist_present: true,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.daytime_running_lamps_presence(), true);
        assert_eq!(frame.automatic_headlamps_presence(), false);
        assert_eq!(frame.mood_lighting_presence(), true);
        assert_eq!(frame.blind_spot_monitoring_presence(), false);
        assert_eq!(frame.adaptive_lamps_presence(), true);
        assert_eq!(frame.welcome_lighting_presence(), false);
        assert_eq!(frame.motorway_lighting_presence(), true);
        assert_eq!(frame.config_menu_information_availability(), false);
        assert_eq!(frame.selective_unlocking_presence(), true);
        assert_eq!(frame.key_selective_unlocking_presence(), false);
        assert_eq!(frame.boot_selective_unlocking_presence(), true);
        assert_eq!(frame.motorized_tailgate_presence(), false);
        assert_eq!(frame.welcome_function_presence(), true);
        assert_eq!(frame.follow_me_home_presence(), false);
        assert_eq!(frame.rear_wiper_in_reverse_gear_presence(), true);
        assert_eq!(frame.park_sensors_inhibition_presence(), false);
        assert_eq!(frame.extended_traffic_sign_recognition_presence(), false);
        assert_eq!(frame.ima_presence(), true);
        assert_eq!(frame.sound_harmony_presence(), false);
        assert_eq!(frame.auto_elec_parking_brake_application_presence(), true);
        assert_eq!(frame.configurable_key_presence(), false);
        assert_eq!(frame.cruise_control_presence(), true);
        assert_eq!(frame.seat_belt_status_lamps_presence(), false);
        assert_eq!(
            frame.under_inflation_detection(),
            UnderInflationDetectionSystem::Indirect
        );
        assert_eq!(frame.gear_efficiency_indicator_presence(), false);
        assert_eq!(frame.cruise_control_memorized_speeds_menu_presence(), true);
        assert_eq!(frame.collision_alert_sensibility_menu_presence(), false);
        assert_eq!(frame.automatic_emergency_braking_presence(), true);
        assert_eq!(frame.under_inflation_detection_reset_menu_presence(), false);
        assert_eq!(frame.hands_free_tailgate_auto_lock_menu_presence(), true);
        assert_eq!(frame.hands_free_tailgate_presence(), true);
        assert_eq!(frame.speed_limit_recognition_presence(), false);
        assert_eq!(frame.radiator_grill_lamps_presence(), true);
        assert_eq!(frame.cfc_presence(), false);
        assert_eq!(frame.irv_presence(), true);
        assert_eq!(frame.automatic_main_beam_presence(), false);
        assert_eq!(frame.ecs_presence(), true);
        assert_eq!(frame.driver_alert_assist_presence(), false);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.daytime_running_lamps_presence(), false);
        assert_eq!(frame.automatic_headlamps_presence(), true);
        assert_eq!(frame.mood_lighting_presence(), false);
        assert_eq!(frame.blind_spot_monitoring_presence(), true);
        assert_eq!(frame.adaptive_lamps_presence(), false);
        assert_eq!(frame.welcome_lighting_presence(), true);
        assert_eq!(frame.motorway_lighting_presence(), false);
        assert_eq!(frame.config_menu_information_availability(), true);
        assert_eq!(frame.selective_unlocking_presence(), false);
        assert_eq!(frame.key_selective_unlocking_presence(), true);
        assert_eq!(frame.boot_selective_unlocking_presence(), false);
        assert_eq!(frame.motorized_tailgate_presence(), true);
        assert_eq!(frame.welcome_function_presence(), false);
        assert_eq!(frame.follow_me_home_presence(), true);
        assert_eq!(frame.rear_wiper_in_reverse_gear_presence(), false);
        assert_eq!(frame.park_sensors_inhibition_presence(), true);
        assert_eq!(frame.extended_traffic_sign_recognition_presence(), true);
        assert_eq!(frame.ima_presence(), false);
        assert_eq!(frame.sound_harmony_presence(), true);
        assert_eq!(frame.auto_elec_parking_brake_application_presence(), false);
        assert_eq!(frame.configurable_key_presence(), true);
        assert_eq!(frame.cruise_control_presence(), false);
        assert_eq!(frame.seat_belt_status_lamps_presence(), true);
        assert_eq!(
            frame.under_inflation_detection(),
            UnderInflationDetectionSystem::DirectWithoutAbsolutePressure
        );
        assert_eq!(frame.gear_efficiency_indicator_presence(), true);
        assert_eq!(frame.cruise_control_memorized_speeds_menu_presence(), false);
        assert_eq!(frame.collision_alert_sensibility_menu_presence(), true);
        assert_eq!(frame.automatic_emergency_braking_presence(), false);
        assert_eq!(frame.under_inflation_detection_reset_menu_presence(), true);
        assert_eq!(frame.hands_free_tailgate_auto_lock_menu_presence(), false);
        assert_eq!(frame.hands_free_tailgate_presence(), false);
        assert_eq!(frame.speed_limit_recognition_presence(), true);
        assert_eq!(frame.radiator_grill_lamps_presence(), false);
        assert_eq!(frame.cfc_presence(), true);
        assert_eq!(frame.irv_presence(), false);
        assert_eq!(frame.automatic_main_beam_presence(), true);
        assert_eq!(frame.ecs_presence(), false);
        assert_eq!(frame.driver_alert_assist_presence(), true);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 6];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_daytime_running_lamps_presence(true);
        frame.set_automatic_headlamps_presence(false);
        frame.set_mood_lighting_presence(true);
        frame.set_blind_spot_monitoring_presence(false);
        frame.set_adaptive_lamps_presence(true);
        frame.set_welcome_lighting_presence(false);
        frame.set_motorway_lighting_presence(true);
        frame.set_config_menu_information_availability(false);
        frame.set_selective_unlocking_presence(true);
        frame.set_key_selective_unlocking_presence(false);
        frame.set_boot_selective_unlocking_presence(true);
        frame.set_motorized_tailgate_presence(false);
        frame.set_welcome_function_presence(true);
        frame.set_follow_me_home_presence(false);
        frame.set_rear_wiper_in_reverse_gear_presence(true);
        frame.set_park_sensors_inhibition_presence(false);
        frame.set_extended_traffic_sign_recognition_presence(false);
        frame.set_ima_presence(true);
        frame.set_sound_harmony_presence(false);
        frame.set_auto_elec_parking_brake_application_presence(true);
        frame.set_configurable_key_presence(false);
        frame.set_cruise_control_presence(true);
        frame.set_seat_belt_status_lamps_presence(false);
        frame.set_under_inflation_detection(UnderInflationDetectionSystem::Indirect);
        frame.set_gear_efficiency_indicator_presence(false);
        frame.set_cruise_control_memorized_speeds_menu_presence(true);
        frame.set_collision_alert_sensibility_menu_presence(false);
        frame.set_automatic_emergency_braking_presence(true);
        frame.set_under_inflation_detection_reset_menu_presence(false);
        frame.set_hands_free_tailgate_auto_lock_menu_presence(true);
        frame.set_hands_free_tailgate_presence(true);
        frame.set_speed_limit_recognition_presence(false);
        frame.set_radiator_grill_lamps_presence(true);
        frame.set_cfc_presence(false);
        frame.set_irv_presence(true);
        frame.set_automatic_main_beam_presence(false);
        frame.set_ecs_presence(true);
        frame.set_driver_alert_assist_presence(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 6];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_daytime_running_lamps_presence(false);
        frame.set_automatic_headlamps_presence(true);
        frame.set_mood_lighting_presence(false);
        frame.set_blind_spot_monitoring_presence(true);
        frame.set_adaptive_lamps_presence(false);
        frame.set_welcome_lighting_presence(true);
        frame.set_motorway_lighting_presence(false);
        frame.set_config_menu_information_availability(true);
        frame.set_selective_unlocking_presence(false);
        frame.set_key_selective_unlocking_presence(true);
        frame.set_boot_selective_unlocking_presence(false);
        frame.set_motorized_tailgate_presence(true);
        frame.set_welcome_function_presence(false);
        frame.set_follow_me_home_presence(true);
        frame.set_rear_wiper_in_reverse_gear_presence(false);
        frame.set_park_sensors_inhibition_presence(true);
        frame.set_extended_traffic_sign_recognition_presence(true);
        frame.set_ima_presence(false);
        frame.set_sound_harmony_presence(true);
        frame.set_auto_elec_parking_brake_application_presence(false);
        frame.set_configurable_key_presence(true);
        frame.set_cruise_control_presence(false);
        frame.set_seat_belt_status_lamps_presence(true);
        frame.set_under_inflation_detection(
            UnderInflationDetectionSystem::DirectWithoutAbsolutePressure,
        );
        frame.set_gear_efficiency_indicator_presence(true);
        frame.set_cruise_control_memorized_speeds_menu_presence(false);
        frame.set_collision_alert_sensibility_menu_presence(true);
        frame.set_automatic_emergency_braking_presence(false);
        frame.set_under_inflation_detection_reset_menu_presence(true);
        frame.set_hands_free_tailgate_auto_lock_menu_presence(false);
        frame.set_hands_free_tailgate_presence(false);
        frame.set_speed_limit_recognition_presence(true);
        frame.set_radiator_grill_lamps_presence(false);
        frame.set_cfc_presence(true);
        frame.set_irv_presence(false);
        frame.set_automatic_main_beam_presence(true);
        frame.set_ecs_presence(false);
        frame.set_driver_alert_assist_presence(true);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 7] = [0x01, 0x00, 0x12, 0xe0, 0x30, 0x34, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 5] = [0x3f, 0x3f, 0x3f, 0x3f, 0x3f];
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
        let mut buf = [0u8; 6];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_basic_repr_2_emit() {
        let mut buf = [0u8; 6];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_2_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }
}
