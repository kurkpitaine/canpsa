use core::{cmp::Ordering, fmt, time::Duration};

use crate::{config::UnderInflationDetectionSystem, Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
361 VSM_INF_CFG_AAS_INHIB_HS7_361                       // OK
361 VSM_INF_CFG_AFF_MENU_ARC_SENS_HS7_361               // OK
361 VSM_INF_CFG_AFF_MENU_CLIM_PRECOND_HS7_361
361 VSM_INF_CFG_AFF_MENU_CMD_VTH_HS7_361
361 VSM_INF_CFG_AFF_MENU_DRIVEPLUS_FUNCTION_HS7_361
361 VSM_INF_CFG_AFF_MENU_ECLI_PPC_BLOC2_HS7_361
361 VSM_INF_CFG_AFF_MENU_ECLX_WELCOME_HS7_361
361 VSM_INF_CFG_AFF_MENU_GAV_BUZZER_HS7_361
361 VSM_INF_CFG_AFF_MENU_RCTA_HS7_361
361 VSM_INF_CFG_AFF_MENU_RTAB_RECHARGE_HS7_361
361 VSM_INF_CFG_AFF_MENU_VIT_XVV_HS7_361                // OK
361 VSM_INF_CFG_DISPO_INFO_MENU_HS7_361                 // OK
361 VSM_INF_CFG_DMD_INHIB_WLC_HS7_361
361 VSM_INF_CFG_ECL_ADAPT_O_HS7_361                     // OK
361 VSM_INF_CFG_ESSUI_MAR_HS7_361                       // OK
361 VSM_INF_CFG_FARC_FA_HS7_361                         // OK
361 VSM_INF_CFG_FEUX_DIURN_O_HS7_361                    // OK
361 VSM_INF_CFG_FOLLOW_HS7_361                          // OK
361 VSM_INF_CFG_PHARE_AUTO_HS7_361                      // OK
361 VSM_INF_CFG_PRES_ACCUEIL_HS7_361                    // OK
361 VSM_INF_CFG_PRES_AMBIANCE_HS7_361                   // OK
361 VSM_INF_CFG_PRES_AUTOROUTE_HS7_361                  // OK
361 VSM_INF_CFG_PRES_BAA_LOCK_HS7_361                   // OK
361 VSM_INF_CFG_PRES_BOIT_TNB_HS7_361                   // OK
361 VSM_INF_CFG_PRES_CFC_HS7_361                        // OK
361 VSM_INF_CFG_PRES_DAA_ACTIV_HS7_361                  // OK
361 VSM_INF_CFG_PRES_DAE_4WD_HS7_361
361 VSM_INF_CFG_PRES_DAE_HS7_361
361 VSM_INF_CFG_PRES_DSG_HS7_361                        // OK
361 VSM_INF_CFG_PRES_DSG_IND_HS7_361                    // OK
361 VSM_INF_CFG_PRES_ECL_CALAND_HS7_361                 // OK
361 VSM_INF_CFG_PRES_ECL_DECONDA_HS7_361                // OK
361 VSM_INF_CFG_PRES_ECLI_PPC_BLOC_HS7_361
361 VSM_INF_CFG_PRES_ECLX_AFS_HS7_361
361 VSM_INF_CFG_PRES_ECLX_ARS_HS7_361
361 VSM_INF_CFG_PRES_ECLX_ECL_CAFR_HS7_361              // OK
361 VSM_INF_CFG_PRES_ECS_MODE_HS7_361                   // OK
361 VSM_INF_CFG_PRES_ETSR_HS7_361                       // OK
361 VSM_INF_CFG_PRES_GAV_AMLA_HS7_361
361 VSM_INF_CFG_PRES_HARMONIE_SON_HS7_361               // OK
361 VSM_INF_CFG_PRES_ILV_ILV_HS7_361                    // OK
361 VSM_INF_CFG_PRES_IMA_HS7_361                        // OK
361 VSM_INF_CFG_PRES_INVIO_ADSD_HS7_361
361 VSM_INF_CFG_PRES_IRC_HS7_361                        // OK
361 VSM_INF_CFG_PRES_MOT_VOL_HS7_361                    // OK
361 VSM_INF_CFG_PRES_PPC_ANIM_HS7_361
361 VSM_INF_CFG_PRES_PPC_HS7_361
361 VSM_INF_CFG_PRES_PRIVACY_MODE_HS7_361
361 VSM_INF_CFG_PRES_SAM_HS7_361                        // OK
361 VSM_INF_CFG_PRES_SER_FSE_AUTO_HS7_361               // OK
361 VSM_INF_CFG_PRES_TCFG_HS7_361                       // OK
361 VSM_INF_CFG_PRES_USER_PROFIL_HS7_361
361 VSM_INF_CFG_PRES_VAM_BAA_HS7_361                    // OK
361 VSM_INF_CFG_PRES_VTOR_IRV_HS7_361                   // OK
361 VSM_INF_CFG_PRES_XVV_HS7_361                        // OK
361 VSM_INF_CFG_SELEC_OUV_AR_HS7_361                    // OK
361 VSM_INF_CFG_SELEC_OUV_CAB_HS7_361                   // OK
361 VSM_INF_CFG_SELEC_OUV_CLE_HS7_361                   // OK
*/

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
    /// 1-bit mirrors tilting in reverse option presence flag,
    /// 1-bit sound harmony option presence flag,
    /// 1-bit automatic electrical parking brake application presence option flag,
    /// 1-bit configurable button/key option presence flag,
    /// 1-bit cruise-control custom limits option presence flag,
    /// 1-bit seat belt not fastened / unfastened warning lamps presence flag.
    pub const OPT_2: usize = 2;
    /// 3-bit under-inflation detection option system type,
    /// 1-bit gear efficiency indicator presence flag,
    /// 1-bit cruise-control custom limits setting menu option presence flag,
    /// 1-bit collision alert sensibility setting menu option presence flag,
    /// 1-bit automatic emergency braking option presence flag,
    /// 1-bit under-inflation detection reset menu option presence flag.
    pub const OPT_3: usize = 3;
    /// 1-bit hands-free tailgate automatic locking menu option presence flag,
    /// 1-bit empty,
    /// 1-bit hands-free tailgate option presence flag,
    /// 1-bit speed limit recognition option presence flag,
    /// 1-bit radiator grill lamps option presence flag (maybe anti-fog lights?),
    /// 1-bit 'CFC' option presence flag,
    /// 2-bit empty.
    pub const OPT_4: usize = 4;
    /// 1-bit automatic mirrors folding inhibition option presence flag,
    /// 4-bit empty,
    /// 1-bit automatic main beam option presence flag,
    /// 1-bit electric child lock security option presence flag,
    /// 1-bit driver alert assist option presence flag.
    pub const OPT_5: usize = 5;
}

/// Length of a x361 CAN frame.
pub const FRAME_LEN: usize = field::OPT_5 + 1;

/// Periodicity of a x361 CAN frame.
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

    /// Return the mirrors tilting in reverse option presence flag.
    #[inline]
    pub fn mirror_tilt_in_reverse_presence(&self) -> bool {
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

    /// Return the cruise-control custom limits option presence flag.
    #[inline]
    pub fn cruise_control_custom_limits_presence(&self) -> bool {
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

    /// Return the cruise-control custom limits setting menu option presence flag.
    #[inline]
    pub fn cruise_control_custom_limits_menu_presence(&self) -> bool {
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

    /// Return the automatic mirrors folding option presence flag.
    #[inline]
    pub fn auto_mirrors_folding_inhibit_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x01 != 0
    }

    /// Return the automatic main beam option presence flag.
    #[inline]
    pub fn automatic_main_beam_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x20 != 0
    }

    /// Return the electric child lock security option presence flag.
    #[inline]
    pub fn electric_child_security_presence(&self) -> bool {
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

    /// Set the mirrors tilting in reverse option presence flag.
    #[inline]
    pub fn set_mirror_tilt_in_reverse_presence(&mut self, value: bool) {
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

    /// Set the cruise-control custom limits option presence flag.
    #[inline]
    pub fn set_cruise_control_custom_limits_presence(&mut self, value: bool) {
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

    /// Set the cruise-control custom limits setting menu option presence flag.
    #[inline]
    pub fn set_cruise_control_custom_limits_menu_presence(&mut self, value: bool) {
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

    /// Set the automatic mirrors folding inhibition option presence flag.
    #[inline]
    pub fn set_auto_mirrors_folding_inhibit_presence(&mut self, value: bool) {
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

    /// Set the electric child lock security option presence flag.
    #[inline]
    pub fn set_electric_child_security_presence(&mut self, value: bool) {
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
    pub daytime_running_lamps_present: bool,
    pub automatic_headlamps_present: bool,
    pub mood_lighting_present: bool,
    pub blind_spot_monitoring_present: bool,
    pub adaptive_lamps_present: bool,
    pub welcome_lighting_present: bool,
    pub motorway_lighting_present: bool,
    pub config_menu_info_available: bool,
    pub selective_unlocking_present: bool,
    pub key_selective_unlocking_present: bool,
    pub boot_selective_unlocking_present: bool,
    pub motorized_tailgate_present: bool,
    pub welcome_function_present: bool,
    pub follow_me_home_present: bool,
    pub rear_wiper_in_reverse_gear_present: bool,
    pub parking_sensors_inhibition_present: bool,
    pub extended_traffic_sign_recognition_present: bool,
    pub mirror_tilt_in_reverse_present: bool,
    pub sound_harmony_present: bool,
    pub automatic_electric_parking_brake_application_present: bool,
    pub configurable_key_present: bool,
    pub cruise_control_custom_limits_present: bool,
    pub seat_belt_status_lamps_present: bool,
    pub under_inflation_detection: UnderInflationDetectionSystem,
    pub gear_efficiency_indicator_present: bool,
    pub cruise_control_custom_limits_menu_present: bool,
    pub collision_alert_sensibility_menu_present: bool,
    pub automatic_emergency_braking_present: bool,
    pub under_inflation_detection_reset_menu_present: bool,
    pub hands_free_tailgate_auto_lock_menu_present: bool,
    pub hands_free_tailgate_present: bool,
    pub speed_limit_recognition_present: bool,
    pub radiator_grill_lamps_present: bool,
    pub cfc_present: bool,
    pub automatic_mirrors_folding_inhibit_present: bool,
    pub automatic_main_beam_present: bool,
    pub electric_child_security_present: bool,
    pub driver_alert_assist_present: bool,
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
            mirror_tilt_in_reverse_present: frame.mirror_tilt_in_reverse_presence(),
            sound_harmony_present: frame.sound_harmony_presence(),
            automatic_electric_parking_brake_application_present: frame
                .auto_elec_parking_brake_application_presence(),
            configurable_key_present: frame.configurable_key_presence(),
            cruise_control_custom_limits_present: frame.cruise_control_custom_limits_presence(),
            seat_belt_status_lamps_present: frame.seat_belt_status_lamps_presence(),
            under_inflation_detection: frame.under_inflation_detection(),
            gear_efficiency_indicator_present: frame.gear_efficiency_indicator_presence(),
            cruise_control_custom_limits_menu_present: frame
                .cruise_control_custom_limits_menu_presence(),
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
            automatic_mirrors_folding_inhibit_present: frame
                .auto_mirrors_folding_inhibit_presence(),
            automatic_main_beam_present: frame.automatic_main_beam_presence(),
            electric_child_security_present: frame.electric_child_security_presence(),
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
        frame.set_mirror_tilt_in_reverse_presence(self.mirror_tilt_in_reverse_present);
        frame.set_sound_harmony_presence(self.sound_harmony_present);
        frame.set_auto_elec_parking_brake_application_presence(
            self.automatic_electric_parking_brake_application_present,
        );
        frame.set_configurable_key_presence(self.configurable_key_present);
        frame.set_cruise_control_custom_limits_presence(self.cruise_control_custom_limits_present);
        frame.set_seat_belt_status_lamps_presence(self.seat_belt_status_lamps_present);
        frame.set_under_inflation_detection(self.under_inflation_detection);
        frame.set_gear_efficiency_indicator_presence(self.gear_efficiency_indicator_present);
        frame.set_cruise_control_custom_limits_menu_presence(
            self.cruise_control_custom_limits_menu_present,
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
        frame.set_auto_mirrors_folding_inhibit_presence(
            self.automatic_mirrors_folding_inhibit_present,
        );
        frame.set_automatic_main_beam_presence(self.automatic_main_beam_present);
        frame.set_electric_child_security_presence(self.electric_child_security_present);
        frame.set_driver_alert_assist_presence(self.driver_alert_assist_present);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "x361 daytime_running_lamps_present={}",
            self.daytime_running_lamps_present
        )?;
        writeln!(
            f,
            " automatic_headlamps_present={}",
            self.automatic_headlamps_present
        )?;
        writeln!(f, " mood_lighting_present={}", self.mood_lighting_present)?;
        writeln!(
            f,
            " blind_spot_monitoring_present={}",
            self.blind_spot_monitoring_present
        )?;
        writeln!(f, " adaptive_lamps_present={}", self.adaptive_lamps_present)?;
        writeln!(
            f,
            " welcome_lighting_present={}",
            self.welcome_lighting_present
        )?;
        writeln!(
            f,
            " motorway_lighting_present={}",
            self.motorway_lighting_present
        )?;
        writeln!(
            f,
            " config_menu_info_available={}",
            self.config_menu_info_available
        )?;
        writeln!(
            f,
            " selective_unlocking_present={}",
            self.selective_unlocking_present
        )?;
        writeln!(
            f,
            " key_selective_unlocking_present={}",
            self.key_selective_unlocking_present
        )?;
        writeln!(
            f,
            " boot_selective_unlocking_present={}",
            self.boot_selective_unlocking_present
        )?;
        writeln!(
            f,
            " motorized_tailgate_present={}",
            self.motorized_tailgate_present
        )?;
        writeln!(
            f,
            " welcome_function_present={}",
            self.welcome_function_present
        )?;
        writeln!(f, " follow_me_home_present={}", self.follow_me_home_present)?;
        writeln!(
            f,
            " rear_wiper_in_reverse_gear_present={}",
            self.rear_wiper_in_reverse_gear_present
        )?;
        writeln!(
            f,
            " parking_sensors_inhibition_present={}",
            self.parking_sensors_inhibition_present
        )?;
        writeln!(
            f,
            " extended_traffic_sign_recognition_present={}",
            self.extended_traffic_sign_recognition_present
        )?;
        writeln!(
            f,
            " mirror_tilt_in_reverse_present={}",
            self.mirror_tilt_in_reverse_present
        )?;
        writeln!(f, " sound_harmony_present={}", self.sound_harmony_present)?;
        writeln!(
            f,
            " automatic_electric_parking_brake_application_present={}",
            self.automatic_electric_parking_brake_application_present
        )?;
        writeln!(
            f,
            " configurable_key_present={}",
            self.configurable_key_present
        )?;
        writeln!(
            f,
            "cruise_control_custom_limits_present={}",
            self.cruise_control_custom_limits_present
        )?;
        writeln!(
            f,
            " seat_belt_status_lamps_present={}",
            self.seat_belt_status_lamps_present
        )?;
        writeln!(
            f,
            " under_inflation_detection={}",
            self.under_inflation_detection
        )?;
        writeln!(
            f,
            " gear_efficiency_indicator_present={}",
            self.gear_efficiency_indicator_present
        )?;
        writeln!(
            f,
            " cruise_control_custom_limits_menu_present={}",
            self.cruise_control_custom_limits_menu_present
        )?;
        writeln!(
            f,
            " collision_alert_sensibility_menu_present={}",
            self.collision_alert_sensibility_menu_present
        )?;
        writeln!(
            f,
            " automatic_emergency_braking_present={}",
            self.automatic_emergency_braking_present
        )?;
        writeln!(
            f,
            " under_inflation_detection_reset_menu_present={}",
            self.under_inflation_detection_reset_menu_present
        )?;
        writeln!(
            f,
            " hands_free_tailgate_auto_lock_menu_present={}",
            self.hands_free_tailgate_auto_lock_menu_present
        )?;
        writeln!(
            f,
            " hands_free_tailgate_present={}",
            self.hands_free_tailgate_present
        )?;
        writeln!(
            f,
            " speed_limit_recognition_present={}",
            self.speed_limit_recognition_present
        )?;
        writeln!(
            f,
            " radiator_grill_lamps_present={}",
            self.radiator_grill_lamps_present
        )?;
        writeln!(f, " 'CFC' present={}", self.cfc_present)?;
        writeln!(
            f,
            " automatic_mirrors_folding_inhibit_present={}",
            self.automatic_mirrors_folding_inhibit_present
        )?;
        writeln!(
            f,
            " automatic_main_beam_present={}",
            self.automatic_main_beam_present
        )?;
        writeln!(
            f,
            " electric_child_security_presence present={}",
            self.electric_child_security_present
        )?;
        writeln!(
            f,
            " driver_alert_assist_present={}",
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
            mirror_tilt_in_reverse_present: true,
            sound_harmony_present: false,
            automatic_electric_parking_brake_application_present: true,
            configurable_key_present: false,
            cruise_control_custom_limits_present: true,
            seat_belt_status_lamps_present: false,
            under_inflation_detection: UnderInflationDetectionSystem::Indirect,
            gear_efficiency_indicator_present: false,
            cruise_control_custom_limits_menu_present: true,
            collision_alert_sensibility_menu_present: false,
            automatic_emergency_braking_present: true,
            under_inflation_detection_reset_menu_present: false,
            hands_free_tailgate_auto_lock_menu_present: true,
            hands_free_tailgate_present: true,
            speed_limit_recognition_present: false,
            radiator_grill_lamps_present: true,
            cfc_present: false,
            automatic_mirrors_folding_inhibit_present: true,
            automatic_main_beam_present: false,
            electric_child_security_present: true,
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
            mirror_tilt_in_reverse_present: false,
            sound_harmony_present: true,
            automatic_electric_parking_brake_application_present: false,
            configurable_key_present: true,
            cruise_control_custom_limits_present: false,
            seat_belt_status_lamps_present: true,
            under_inflation_detection: UnderInflationDetectionSystem::DirectWithoutAbsolutePressure,
            gear_efficiency_indicator_present: true,
            cruise_control_custom_limits_menu_present: false,
            collision_alert_sensibility_menu_present: true,
            automatic_emergency_braking_present: false,
            under_inflation_detection_reset_menu_present: true,
            hands_free_tailgate_auto_lock_menu_present: false,
            hands_free_tailgate_present: false,
            speed_limit_recognition_present: true,
            radiator_grill_lamps_present: false,
            cfc_present: true,
            automatic_mirrors_folding_inhibit_present: false,
            automatic_main_beam_present: true,
            electric_child_security_present: false,
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
        assert_eq!(frame.mirror_tilt_in_reverse_presence(), true);
        assert_eq!(frame.sound_harmony_presence(), false);
        assert_eq!(frame.auto_elec_parking_brake_application_presence(), true);
        assert_eq!(frame.configurable_key_presence(), false);
        assert_eq!(frame.cruise_control_custom_limits_presence(), true);
        assert_eq!(frame.seat_belt_status_lamps_presence(), false);
        assert_eq!(
            frame.under_inflation_detection(),
            UnderInflationDetectionSystem::Indirect
        );
        assert_eq!(frame.gear_efficiency_indicator_presence(), false);
        assert_eq!(frame.cruise_control_custom_limits_menu_presence(), true);
        assert_eq!(frame.collision_alert_sensibility_menu_presence(), false);
        assert_eq!(frame.automatic_emergency_braking_presence(), true);
        assert_eq!(frame.under_inflation_detection_reset_menu_presence(), false);
        assert_eq!(frame.hands_free_tailgate_auto_lock_menu_presence(), true);
        assert_eq!(frame.hands_free_tailgate_presence(), true);
        assert_eq!(frame.speed_limit_recognition_presence(), false);
        assert_eq!(frame.radiator_grill_lamps_presence(), true);
        assert_eq!(frame.cfc_presence(), false);
        assert_eq!(frame.auto_mirrors_folding_inhibit_presence(), true);
        assert_eq!(frame.automatic_main_beam_presence(), false);
        assert_eq!(frame.electric_child_security_presence(), true);
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
        assert_eq!(frame.mirror_tilt_in_reverse_presence(), false);
        assert_eq!(frame.sound_harmony_presence(), true);
        assert_eq!(frame.auto_elec_parking_brake_application_presence(), false);
        assert_eq!(frame.configurable_key_presence(), true);
        assert_eq!(frame.cruise_control_custom_limits_presence(), false);
        assert_eq!(frame.seat_belt_status_lamps_presence(), true);
        assert_eq!(
            frame.under_inflation_detection(),
            UnderInflationDetectionSystem::DirectWithoutAbsolutePressure
        );
        assert_eq!(frame.gear_efficiency_indicator_presence(), true);
        assert_eq!(frame.cruise_control_custom_limits_menu_presence(), false);
        assert_eq!(frame.collision_alert_sensibility_menu_presence(), true);
        assert_eq!(frame.automatic_emergency_braking_presence(), false);
        assert_eq!(frame.under_inflation_detection_reset_menu_presence(), true);
        assert_eq!(frame.hands_free_tailgate_auto_lock_menu_presence(), false);
        assert_eq!(frame.hands_free_tailgate_presence(), false);
        assert_eq!(frame.speed_limit_recognition_presence(), true);
        assert_eq!(frame.radiator_grill_lamps_presence(), false);
        assert_eq!(frame.cfc_presence(), true);
        assert_eq!(frame.auto_mirrors_folding_inhibit_presence(), false);
        assert_eq!(frame.automatic_main_beam_presence(), true);
        assert_eq!(frame.electric_child_security_presence(), false);
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
        frame.set_mirror_tilt_in_reverse_presence(true);
        frame.set_sound_harmony_presence(false);
        frame.set_auto_elec_parking_brake_application_presence(true);
        frame.set_configurable_key_presence(false);
        frame.set_cruise_control_custom_limits_presence(true);
        frame.set_seat_belt_status_lamps_presence(false);
        frame.set_under_inflation_detection(UnderInflationDetectionSystem::Indirect);
        frame.set_gear_efficiency_indicator_presence(false);
        frame.set_cruise_control_custom_limits_menu_presence(true);
        frame.set_collision_alert_sensibility_menu_presence(false);
        frame.set_automatic_emergency_braking_presence(true);
        frame.set_under_inflation_detection_reset_menu_presence(false);
        frame.set_hands_free_tailgate_auto_lock_menu_presence(true);
        frame.set_hands_free_tailgate_presence(true);
        frame.set_speed_limit_recognition_presence(false);
        frame.set_radiator_grill_lamps_presence(true);
        frame.set_cfc_presence(false);
        frame.set_auto_mirrors_folding_inhibit_presence(true);
        frame.set_automatic_main_beam_presence(false);
        frame.set_electric_child_security_presence(true);
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
        frame.set_mirror_tilt_in_reverse_presence(false);
        frame.set_sound_harmony_presence(true);
        frame.set_auto_elec_parking_brake_application_presence(false);
        frame.set_configurable_key_presence(true);
        frame.set_cruise_control_custom_limits_presence(false);
        frame.set_seat_belt_status_lamps_presence(true);
        frame.set_under_inflation_detection(
            UnderInflationDetectionSystem::DirectWithoutAbsolutePressure,
        );
        frame.set_gear_efficiency_indicator_presence(true);
        frame.set_cruise_control_custom_limits_menu_presence(false);
        frame.set_collision_alert_sensibility_menu_presence(true);
        frame.set_automatic_emergency_braking_presence(false);
        frame.set_under_inflation_detection_reset_menu_presence(true);
        frame.set_hands_free_tailgate_auto_lock_menu_presence(false);
        frame.set_hands_free_tailgate_presence(false);
        frame.set_speed_limit_recognition_presence(true);
        frame.set_radiator_grill_lamps_presence(false);
        frame.set_cfc_presence(true);
        frame.set_auto_mirrors_folding_inhibit_presence(false);
        frame.set_automatic_main_beam_presence(true);
        frame.set_electric_child_security_presence(false);
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
