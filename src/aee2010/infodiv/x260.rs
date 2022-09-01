use core::{cmp::Ordering, fmt, time::Duration};

use crate::{
    config::{
        CollisionAlertSensibilityLevel, ConfigurableKeyAction2010, ConsumptionUnit, DistanceUnit,
        Language, LightingDuration2010, MoodLightingLevel, SoundHarmony, TemperatureUnit,
        VolumeUnit,
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
260 VSM_INF_PROFILS_AAS_STATUS_HS7_260                  // OK
260 VSM_INF_PROFILS_ACCUEIL_COND_HS7_260                // OK
260 VSM_INF_PROFILS_ARC_SENS_HS7_260                    // OK
260 VSM_INF_PROFILS_ARC_SENS_NIV_HS7_260                // OK
260 VSM_INF_PROFILS_DISPO_PARAM_HS7_260                 // OK
260 VSM_INF_PROFILS_DISPO_UNITES_LANGUE_HS7_260         // OK
260 VSM_INF_PROFILS_ECL_ADAPT_HS7_260                   // OK
260 VSM_INF_PROFILS_ECLAI_AMBI_HS7_260                  // OK
260 VSM_INF_PROFILS_ECLAIRAGE_ACCOM_HS7_260             // OK
260 VSM_INF_PROFILS_ECLAIRAGE_AUTO_HS7_260              // OK
260 VSM_INF_PROFILS_ECL_AUTOROUTE_HS7_260               // OK
260 VSM_INF_PROFILS_ECL_DECONDA_HS7_260                 // OK
260 VSM_INF_PROFILS_ESSUI_VIT_MAR_HS7_260               // OK
260 VSM_INF_PROFILS_FCT_ECL_CALAND_HS7_260              // OK
260 VSM_INF_PROFILS_FCT_ECLX_AFS_HS7_260
260 VSM_INF_PROFILS_FCT_ECLX_ARS_HS7_260
260 VSM_INF_PROFILS_FCT_FEUX_DIURN_O_HS7_260            // OK
260 VSM_INF_PROFILS_FCT_MENU_BAA_LOCK_HS7_260           // OK
260 VSM_INF_PROFILS_FCT_MENU_DAA_ACTIV_HS7_260          // OK
260 VSM_INF_PROFILS_FCT_MENU_ECLX_ECL_CAFR_HS7_260      // OK
260 VSM_INF_PROFILS_FCT_MENU_ECS_MODE_HS7_260           // OK
260 VSM_INF_PROFILS_FCT_MENU_GAV_AMLA_HS7_260
260 VSM_INF_PROFILS_FCT_MENU_ILV_ETSR_HS7_260           // OK
260 VSM_INF_PROFILS_FCT_MENU_ILV_ILV_HS7_260            // OK
260 VSM_INF_PROFILS_FCT_MENU_TYPAGE_DAE_4WD_HS7_260
260 VSM_INF_PROFILS_FCT_MENU_TYPAGE_DAE_HS7_260
260 VSM_INF_PROFILS_FCT_MENU_USER_PROFIL_HS7_260
260 VSM_INF_PROFILS_FCT_MENU_VAM_BAA_HS7_260            // OK
260 VSM_INF_PROFILS_FCT_MOT_VOL_AR_HS7_260              // OK
260 VSM_INF_PROFILS_FCT_TCFG_HS7_260                    // OK
260 VSM_INF_PROFILS_FCT_VTOR_IRV_HS7_260                // OK
260 VSM_INF_PROFILS_HARMONIE_SON_HS7_260                // OK
260 VSM_INF_PROFILS_IMA_STATUS_HS7_260                  // OK
260 VSM_INF_PROFILS_LANGUE_VHL_HS7_260                  // OK
260 VSM_INF_PROFILS_NIV_AMBIANCE_HS7_260                // OK
260 VSM_INF_PROFILS_REINIT_DSG_STATUS_HS7_260           // OK
260 VSM_INF_PROFILS_SAM_STATUS_HS7_260                  // OK
260 VSM_INF_PROFILS_SELEC_ARRIERE_HS7_260               // OK
260 VSM_INF_PROFILS_SELEC_CABINE_HS7_260                // OK
260 VSM_INF_PROFILS_SELEC_FARC_FA_HS7_260               // OK
260 VSM_INF_PROFILS_SELEC_OUV_PLIP_HS7_260              // OK
260 VSM_INF_PROFILS_SER_FSE_AUTO_HS7_260                // OK
260 VSM_INF_PROFILS_TEMPO_ECL_DECONDA_HS7_260           // OK
260 VSM_INF_PROFILS_TEMPO_EXT_PHARE_HS7_260             // OK
260 VSM_INF_PROFILS_UNITE_CONSO_HS7_260                 // OK
260 VSM_INF_PROFILS_UNITE_DISTANCE_HS7_260              // OK
260 VSM_INF_PROFILS_UNITE_TEMPERATURE_HS7_260           // OK
260 VSM_INF_PROFILS_UNITE_VOLUME_HS7_260                // OK
*/

mod field {
    /// 1-bit consumption unit field,
    /// 1-bit distance unit field,
    /// 5-bit display language field,
    /// 1-bit units and language parameters validity flag.
    pub const OPT_0: usize = 0;
    /// 2-bit sound harmony field,
    /// 1-bit parameters validity flag,
    /// 3-bit mood lighting level field,
    /// 1-bit temperature unit field,
    /// 1-bit volume unit field.
    pub const OPT_1: usize = 1;
    /// 1-bit mood lighting enable flag,
    /// 1-bit daytime running lamps enable flag,
    /// 1-bit adaptive lamps enable flag,
    /// 1-bit welcome function enable flag,
    /// 1-bit boot selective unlocking enable flag,
    /// 1-bit selective unlocking enable flag.
    /// 1-bit key selective unlocking enable flag,
    /// 1-bit automatic electrical parking brake application enable flag.
    pub const OPT_2: usize = 2;
    /// 1-bit automatic headlamps enable flag,
    /// 2-bit welcome lighting duration field,
    /// 1-bit welcome lighting enable flag,
    /// 1-bit motorway lighting enable flag,
    /// 2-bit follow-me-home lighting duration field,
    /// 1-bit follow-me-home enable field.
    pub const OPT_3: usize = 3;
    /// 4-bit configurable button/key mode field,
    /// 1-bit motorized tailgate enable flag,
    /// 1-bit rear wiper in reverse gear enable flag,
    /// 1-bit blind spot monitoring enable field,
    /// 1-bit parking sensors enable field.
    pub const OPT_4: usize = 4;
    /// 2-bit empty,
    /// 1-bit mirrors tilting in reverse gear enable flag,
    /// 1-bit indirect under inflation detection reset status flag,
    /// 1-bit automatic emergency braking enable flag,
    /// 2-bit collision alert sensibility level field,
    /// 1-bit collision alert enable flag.
    pub const OPT_5: usize = 5;
    /// 1-bit hands-free tailgate enable flag,
    /// 1-bit speed limit recognition enable flag,
    /// 1-bit radiator grill lamps option presence flag (maybe anti-fog lights?),
    /// 1-bit automatic main beam enable flag,
    /// 1-bit driver alert assist enable flag,
    /// 1-bit hands-free tailgate automatic locking enable flag,
    /// 1-bit extended traffic sign recognition enable flag,
    /// 1-bit electric child lock security enable flag.
    pub const OPT_6: usize = 6;
    /// 3-bit empty,
    /// 1-bit automatic mirrors folding inhibition enable flag,
    /// 4-bit empty.
    pub const OPT_7: usize = 7;
}

/// Raw x260 CAN frame identifier.
pub const FRAME_ID: u16 = 0x260;
/// Length of a x260 CAN frame.
pub const FRAME_LEN: usize = field::OPT_7 + 1;

/// Periodicity of a x260 CAN frame.
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

    /// Return the consumption unit field.
    #[inline]
    pub fn consumption_unit(&self) -> ConsumptionUnit {
        let data = self.buffer.as_ref();
        let raw = data[field::OPT_0] & 0x01;
        ConsumptionUnit::from(raw)
    }

    /// Return the distance unit field.
    #[inline]
    pub fn distance_unit(&self) -> DistanceUnit {
        let data = self.buffer.as_ref();
        let raw = (data[field::OPT_0] & 0x02) >> 1;
        DistanceUnit::from(raw)
    }

    /// Return the language field.
    #[inline]
    pub fn language(&self) -> Language {
        let data = self.buffer.as_ref();
        let raw = (data[field::OPT_0] & 0x7c) >> 2;
        Language::from(raw)
    }

    /// Return the units and language parameters validity flag.
    #[inline]
    pub fn units_language_parameters_validity(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_0] & 0x80 != 0
    }

    /// Return the sound harmony field.
    #[inline]
    pub fn sound_harmony(&self) -> SoundHarmony {
        let data = self.buffer.as_ref();
        let raw = data[field::OPT_1] & 0x03;
        SoundHarmony::from(raw)
    }

    /// Return the parameters validity flag.
    #[inline]
    pub fn parameters_validity(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x04 != 0
    }

    /// Return the mood lighting level field.
    #[inline]
    pub fn mood_lighting_level(&self) -> MoodLightingLevel {
        let data = self.buffer.as_ref();
        let raw = (data[field::OPT_1] & 0x38) >> 3;
        MoodLightingLevel::from(raw)
    }

    /// Return the temperature unit field.
    #[inline]
    pub fn temperature_unit(&self) -> TemperatureUnit {
        let data = self.buffer.as_ref();
        let raw = (data[field::OPT_1] & 0x40) >> 6;
        TemperatureUnit::from(raw)
    }

    /// Return the volume unit field.
    #[inline]
    pub fn volume_unit(&self) -> VolumeUnit {
        let data = self.buffer.as_ref();
        let raw = (data[field::OPT_1] & 0x80) >> 7;
        VolumeUnit::from(raw)
    }

    /// Return the mood lighting enable flag.
    #[inline]
    pub fn mood_lighting_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x01 != 0
    }

    /// Return the daytime running lamps enable flag.
    #[inline]
    pub fn daytime_running_lamps_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x02 != 0
    }

    /// Return the adaptive lamps enable flag.
    #[inline]
    pub fn adaptive_lamps_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x04 != 0
    }

    /// Return the welcome function enable flag.
    #[inline]
    pub fn welcome_function_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x08 != 0
    }

    /// Return the boot selective unlocking enable flag.
    #[inline]
    pub fn boot_selective_unlocking_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x10 != 0
    }

    /// Return the selective unlocking enable flag.
    #[inline]
    pub fn selective_unlocking_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x20 != 0
    }

    /// Return the key selective unlocking enable flag.
    #[inline]
    pub fn key_selective_unlocking_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x40 != 0
    }

    /// Return the automatic electrical parking brake application enable flag.
    #[inline]
    pub fn auto_elec_parking_brake_application_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x80 != 0
    }

    /// Return the automatic headlamps enable flag.
    #[inline]
    pub fn automatic_headlamps_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x01 != 0
    }

    /// Return the welcome lighting duration field.
    #[inline]
    pub fn welcome_lighting_duration(&self) -> LightingDuration2010 {
        let data = self.buffer.as_ref();
        let raw = (data[field::OPT_3] & 0x06) >> 1;
        LightingDuration2010::from(raw)
    }

    /// Return the welcome lighting enable flag.
    #[inline]
    pub fn welcome_lighting_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x08 != 0
    }

    /// Return the motorway lighting enable flag.
    #[inline]
    pub fn motorway_lighting_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x10 != 0
    }

    /// Return the follow-me-home lighting duration field.
    #[inline]
    pub fn follow_me_home_lighting_duration(&self) -> LightingDuration2010 {
        let data = self.buffer.as_ref();
        let raw = (data[field::OPT_3] & 0x60) >> 5;
        LightingDuration2010::from(raw)
    }

    /// Return the follow-me-home enable flag.
    #[inline]
    pub fn follow_me_home_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x80 != 0
    }

    /// Return the configurable button/key mode field.
    #[inline]
    pub fn configurable_key_mode(&self) -> ConfigurableKeyAction2010 {
        let data = self.buffer.as_ref();
        let raw = data[field::OPT_4] & 0x0f;
        ConfigurableKeyAction2010::from(raw)
    }

    /// Return the motorized tailgate enable flag.
    #[inline]
    pub fn motorized_tailgate_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x10 != 0
    }

    /// Return the rear wiper in reverse gear enable flag.
    #[inline]
    pub fn rear_wiper_in_reverse_gear_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x20 != 0
    }

    /// Return the blind spot monitoring enable flag.
    #[inline]
    pub fn blind_spot_monitoring_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x40 != 0
    }

    /// Return the parking sensors enable flag.
    #[inline]
    pub fn park_sensors_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x80 != 0
    }

    /// Return the mirrors tilting in reverse gear enable flag.
    #[inline]
    pub fn mirrors_tilting_in_reverse_gear_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x04 != 0
    }

    /// Return the indirect under-inflation detection reset status flag.
    #[inline]
    pub fn indirect_under_inflation_reset_status(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x08 != 0
    }

    /// Return the automatic emergency braking enable flag.
    #[inline]
    pub fn automatic_emergency_braking_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x10 != 0
    }

    /// Return the collision alert sensibility level field.
    #[inline]
    pub fn collision_alert_sensibility_level(&self) -> CollisionAlertSensibilityLevel {
        let data = self.buffer.as_ref();
        let raw = (data[field::OPT_5] & 0x60) >> 5;
        CollisionAlertSensibilityLevel::from(raw)
    }

    /// Return the collision alert enable flag.
    #[inline]
    pub fn collision_alert_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x80 != 0
    }

    /// Return the hands-free tailgate enable flag.
    #[inline]
    pub fn hands_free_tailgate_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x01 != 0
    }

    /// Return the speed limit recognition enable flag.
    #[inline]
    pub fn speed_limit_recognition_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x02 != 0
    }

    /// Return the radiator grill lamps enable flag (maybe anti-fog lights?).
    #[inline]
    pub fn radiator_grill_lamps_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x04 != 0
    }

    /// Return the automatic main beam enable flag.
    #[inline]
    pub fn automatic_main_beam_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x08 != 0
    }

    /// Return the driver alert assist enable flag.
    #[inline]
    pub fn driver_alert_assist_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x10 != 0
    }

    /// Return the hands-free tailgate automatic locking enable flag.
    #[inline]
    pub fn hands_free_tailgate_auto_lock_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x20 != 0
    }

    /// Return the extended traffic sign recognition enable flag.
    #[inline]
    pub fn extended_traffic_sign_recognition_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x40 != 0
    }

    /// Return the electric child lock security enable flag.
    #[inline]
    pub fn electric_child_security_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x80 != 0
    }

    /// Return automatic mirrors folding inhibit enable flag.
    #[inline]
    pub fn auto_mirrors_folding_inhibit(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_7] & 0x08 != 0
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the consumption unit field.
    #[inline]
    pub fn set_consumption_unit(&mut self, value: ConsumptionUnit) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_0] & !0x01;
        let raw = raw | u8::from(value);
        data[field::OPT_0] = raw;
    }

    /// Set the distance unit field.
    #[inline]
    pub fn set_distance_unit(&mut self, value: DistanceUnit) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_0] & !0x02;
        let raw = raw | (u8::from(value) << 1);
        data[field::OPT_0] = raw;
    }

    /// Set the language field.
    #[inline]
    pub fn set_language(&mut self, value: Language) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_0] & !0x7c;
        let raw = raw | ((u8::from(value) << 2) & 0x7c);
        data[field::OPT_0] = raw;
    }

    /// Set the units and language parameters validity flag.
    #[inline]
    pub fn set_units_language_parameters_validity(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_0];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_0] = raw;
    }

    /// Set the sound harmony field.
    #[inline]
    pub fn set_sound_harmony(&mut self, value: SoundHarmony) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::OPT_1] = raw;
    }

    /// Set the parameters validity flag.
    #[inline]
    pub fn set_parameters_validity(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1];
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_1] = raw;
    }

    /// Set the mood lighting level field.
    #[inline]
    pub fn set_mood_lighting_level(&mut self, value: MoodLightingLevel) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x38;
        let raw = raw | ((u8::from(value) << 3) & 0x38);
        data[field::OPT_1] = raw;
    }

    /// Set the temperature unit field.
    #[inline]
    pub fn set_temperature_unit(&mut self, value: TemperatureUnit) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x40;
        let raw = raw | ((u8::from(value) << 6) & 0x40);
        data[field::OPT_1] = raw;
    }

    /// Set the volume unit field.
    #[inline]
    pub fn set_volume_unit(&mut self, value: VolumeUnit) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x80;
        let raw = raw | (u8::from(value) << 7);
        data[field::OPT_1] = raw;
    }

    /// Set the mood lighting enable flag.
    #[inline]
    pub fn set_mood_lighting_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_2] = raw;
    }

    /// Set the daytime running lamps enable flag.
    #[inline]
    pub fn set_daytime_running_lamps_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_2] = raw;
    }

    /// Set the adaptive lamps enable flag.
    #[inline]
    pub fn set_adaptive_lamps_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_2] = raw;
    }

    /// Set the welcome function enable flag.
    #[inline]
    pub fn set_welcome_function_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_2] = raw;
    }

    /// Set the bool selective unlocking enable flag.
    #[inline]
    pub fn set_boot_selective_unlocking_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_2] = raw;
    }

    /// Set the selective unlocking enable flag.
    #[inline]
    pub fn set_selective_unlocking_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_2] = raw;
    }

    /// Set the key selective unlocking enable flag.
    #[inline]
    pub fn set_key_selective_unlocking_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_2] = raw;
    }

    /// Set the automatic electrical parking brake application enable flag.
    #[inline]
    pub fn set_auto_elec_parking_brake_application_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_2] = raw;
    }

    /// Set the automatic headlamps enable flag.
    #[inline]
    pub fn set_automatic_headlamps_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_3] = raw;
    }

    /// Set the welcome lighting duration field.
    #[inline]
    pub fn set_welcome_lighting_duration(&mut self, value: LightingDuration2010) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x06;
        let raw = raw | ((u8::from(value) << 1) & 0x06);
        data[field::OPT_3] = raw;
    }

    /// Set the welcome lighting enable flag.
    #[inline]
    pub fn set_welcome_lighting_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_3] = raw;
    }

    /// Set the motorway lighting enable flag.
    #[inline]
    pub fn set_motorway_lighting_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_3] = raw;
    }

    /// Set the follow-me-home lighting duration field.
    #[inline]
    pub fn set_follow_me_home_lighting_duration(&mut self, value: LightingDuration2010) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x60;
        let raw = raw | ((u8::from(value) << 5) & 0x60);
        data[field::OPT_3] = raw;
    }

    /// Set the follow-me-home enable flag.
    #[inline]
    pub fn set_follow_me_home_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_3] = raw;
    }

    /// Set the locking mode on 'COE' enable flag.
    #[inline]
    pub fn set_locking_mode_on_coe_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_1] = raw;
    }

    /// Set the automatic door locking when leaving enable flag.
    #[inline]
    pub fn set_auto_door_locking_when_leaving_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_1] = raw;
    }

    /// Set the boot permanent locking enable flag.
    #[inline]
    pub fn set_boot_permanent_locking_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_1] = raw;
    }

    /// Set the automatic door locking when driving enable flag.
    #[inline]
    pub fn set_auto_door_locking_when_driving_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_1] = raw;
    }

    /// Set the configurable button/key enable flag.
    #[inline]
    pub fn set_configurable_key_mode(&mut self, value: ConfigurableKeyAction2010) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x0f;
        let raw = raw | (u8::from(value) & 0x0f);
        data[field::OPT_4] = raw;
    }

    /// Set the motorized tailgate enable flag.
    #[inline]
    pub fn set_motorized_tailgate_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_4] = raw;
    }

    /// Set the rear wiper in reverse gear enable flag.
    #[inline]
    pub fn set_rear_wiper_in_reverse_gear_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_4] = raw;
    }

    /// Set the blind spot monitoring enable flag.
    #[inline]
    pub fn set_blind_spot_monitoring_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_4] = raw;
    }

    /// Set the parking sensors enable flag.
    #[inline]
    pub fn set_park_sensors_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_4] = raw;
    }

    /// Set the mirrors tilting in reverse gear enable flag.
    #[inline]
    pub fn set_mirrors_tilting_in_reverse_gear_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_5] = raw;
    }

    /// Set the indirect under-inflation detection reset status flag.
    #[inline]
    pub fn set_indirect_under_inflation_reset_status(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_5] = raw;
    }

    /// Set the automatic emergency braking enable flag.
    #[inline]
    pub fn set_automatic_emergency_braking_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_5] = raw;
    }

    /// Set the collision alert sensibility level field.
    #[inline]
    pub fn set_collision_alert_sensibility_level(&mut self, value: CollisionAlertSensibilityLevel) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x60;
        let raw = raw | ((u8::from(value) << 5) & 0x60);
        data[field::OPT_5] = raw;
    }

    /// Set the collision alert enable flag.
    #[inline]
    pub fn set_collision_alert_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_5] = raw;
    }

    /// Set the hands-free tailgate enable flag.
    #[inline]
    pub fn set_hands_free_tailgate_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_6] = raw;
    }

    /// Set the speed limit recognition enable flag.
    #[inline]
    pub fn set_speed_limit_recognition_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_6] = raw;
    }

    /// Set the radiator grill lamps enable flag (maybe anti-fog lights).
    #[inline]
    pub fn set_radiator_grill_lamps_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_6] = raw;
    }

    /// Set the automatic main beam enable flag.
    #[inline]
    pub fn set_automatic_main_beam_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_6] = raw;
    }

    /// Set the driver alert assist enable flag.
    #[inline]
    pub fn set_driver_alert_assist_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_6] = raw;
    }

    /// Set the hands-free tailgate automatic locking enable flag.
    #[inline]
    pub fn set_hands_free_tailgate_auto_lock_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_6] = raw;
    }

    /// Set the extended traffic sign recognition enable flag.
    #[inline]
    pub fn set_extended_traffic_sign_recognition_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_6] = raw;
    }

    /// Set the electric child lock security enable flag.
    #[inline]
    pub fn set_electric_child_security_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_6] = raw;
    }

    /// Set the automatic mirrors folding inhibit enable flag.
    #[inline]
    pub fn set_auto_mirrors_folding_inhibit(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_7] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_7] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x260 ({})", err)?;
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

/// A high-level representation of a x260 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub consumption_unit: ConsumptionUnit,
    pub distance_unit: DistanceUnit,
    pub language: Language,
    pub units_language_parameters_validity: bool,
    pub sound_harmony: SoundHarmony,
    pub parameters_validity: bool,
    pub mood_lighting_level: MoodLightingLevel,
    pub temperature_unit: TemperatureUnit,
    pub volume_unit: VolumeUnit,
    pub mood_lighting_enabled: bool,
    pub daytime_running_lamps_enabled: bool,
    pub adaptive_lamps_enabled: bool,
    pub welcome_function_enabled: bool,
    pub boot_selective_unlocking_enabled: bool,
    pub selective_unlocking_enabled: bool,
    pub key_selective_unlocking_enabled: bool,
    pub automatic_elec_parking_brake_application_enabled: bool,
    pub automatic_headlamps_enabled: bool,
    pub welcome_lighting_duration: LightingDuration2010,
    pub welcome_lighting_enabled: bool,
    pub motorway_lighting_enabled: bool,
    pub follow_me_home_lighting_duration: LightingDuration2010,
    pub follow_me_home_enabled: bool,
    pub configurable_key_mode: ConfigurableKeyAction2010,
    pub motorized_tailgate_enabled: bool,
    pub rear_wiper_in_reverse_gear_enabled: bool,
    pub blind_spot_monitoring_enabled: bool,
    pub park_sensors_enabled: bool,
    pub mirrors_tilting_in_reverse_gear_enabled: bool,
    pub indirect_under_inflation_reset_status: bool,
    pub automatic_emergency_braking_enabled: bool,
    pub collision_alert_sensibility_level: CollisionAlertSensibilityLevel,
    pub collision_alert_enabled: bool,
    pub hands_free_tailgate_enabled: bool,
    pub speed_limit_recognition_enabled: bool,
    pub radiator_grill_lamps_enabled: bool,
    pub automatic_main_beam_enabled: bool,
    pub driver_alert_assist_enabled: bool,
    pub hands_free_tailgate_auto_lock_enabled: bool,
    pub extended_traffic_sign_recognition_enabled: bool,
    pub electric_child_security_enabled: bool,
    pub auto_mirrors_folding_inhibit: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            consumption_unit: frame.consumption_unit(),
            distance_unit: frame.distance_unit(),
            language: frame.language(),
            units_language_parameters_validity: frame.units_language_parameters_validity(),
            sound_harmony: frame.sound_harmony(),
            parameters_validity: frame.parameters_validity(),
            mood_lighting_level: frame.mood_lighting_level(),
            temperature_unit: frame.temperature_unit(),
            volume_unit: frame.volume_unit(),
            mood_lighting_enabled: frame.mood_lighting_enable(),
            daytime_running_lamps_enabled: frame.daytime_running_lamps_enable(),
            adaptive_lamps_enabled: frame.adaptive_lamps_enable(),
            welcome_function_enabled: frame.welcome_function_enable(),
            boot_selective_unlocking_enabled: frame.boot_selective_unlocking_enable(),
            selective_unlocking_enabled: frame.selective_unlocking_enable(),
            key_selective_unlocking_enabled: frame.key_selective_unlocking_enable(),
            automatic_elec_parking_brake_application_enabled: frame
                .auto_elec_parking_brake_application_enable(),
            automatic_headlamps_enabled: frame.automatic_headlamps_enable(),
            welcome_lighting_duration: frame.welcome_lighting_duration(),
            welcome_lighting_enabled: frame.welcome_lighting_enable(),
            motorway_lighting_enabled: frame.motorway_lighting_enable(),
            follow_me_home_lighting_duration: frame.follow_me_home_lighting_duration(),
            follow_me_home_enabled: frame.follow_me_home_enable(),
            configurable_key_mode: frame.configurable_key_mode(),
            motorized_tailgate_enabled: frame.motorized_tailgate_enable(),
            rear_wiper_in_reverse_gear_enabled: frame.rear_wiper_in_reverse_gear_enable(),
            blind_spot_monitoring_enabled: frame.blind_spot_monitoring_enable(),
            park_sensors_enabled: frame.park_sensors_enable(),
            mirrors_tilting_in_reverse_gear_enabled: frame.mirrors_tilting_in_reverse_gear_enable(),
            indirect_under_inflation_reset_status: frame.indirect_under_inflation_reset_status(),
            automatic_emergency_braking_enabled: frame.automatic_emergency_braking_enable(),
            collision_alert_sensibility_level: frame.collision_alert_sensibility_level(),
            collision_alert_enabled: frame.collision_alert_enable(),
            hands_free_tailgate_enabled: frame.hands_free_tailgate_enable(),
            speed_limit_recognition_enabled: frame.speed_limit_recognition_enable(),
            radiator_grill_lamps_enabled: frame.radiator_grill_lamps_enable(),
            automatic_main_beam_enabled: frame.automatic_main_beam_enable(),
            driver_alert_assist_enabled: frame.driver_alert_assist_enable(),
            hands_free_tailgate_auto_lock_enabled: frame.hands_free_tailgate_auto_lock_enable(),
            extended_traffic_sign_recognition_enabled: frame
                .extended_traffic_sign_recognition_enable(),
            electric_child_security_enabled: frame.electric_child_security_enable(),
            auto_mirrors_folding_inhibit: frame.auto_mirrors_folding_inhibit(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x260 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_consumption_unit(self.consumption_unit);
        frame.set_distance_unit(self.distance_unit);
        frame.set_language(self.language);
        frame.set_units_language_parameters_validity(self.units_language_parameters_validity);
        frame.set_sound_harmony(self.sound_harmony);
        frame.set_parameters_validity(self.parameters_validity);
        frame.set_mood_lighting_level(self.mood_lighting_level);
        frame.set_temperature_unit(self.temperature_unit);
        frame.set_volume_unit(self.volume_unit);
        frame.set_mood_lighting_enable(self.mood_lighting_enabled);
        frame.set_daytime_running_lamps_enable(self.daytime_running_lamps_enabled);
        frame.set_adaptive_lamps_enable(self.adaptive_lamps_enabled);
        frame.set_welcome_function_enable(self.welcome_function_enabled);
        frame.set_boot_selective_unlocking_enable(self.boot_selective_unlocking_enabled);
        frame.set_selective_unlocking_enable(self.selective_unlocking_enabled);
        frame.set_key_selective_unlocking_enable(self.key_selective_unlocking_enabled);
        frame.set_auto_elec_parking_brake_application_enable(
            self.automatic_elec_parking_brake_application_enabled,
        );
        frame.set_automatic_headlamps_enable(self.automatic_headlamps_enabled);
        frame.set_welcome_lighting_duration(self.welcome_lighting_duration);
        frame.set_welcome_lighting_enable(self.welcome_lighting_enabled);
        frame.set_motorway_lighting_enable(self.motorway_lighting_enabled);
        frame.set_follow_me_home_lighting_duration(self.follow_me_home_lighting_duration);
        frame.set_follow_me_home_enable(self.follow_me_home_enabled);
        frame.set_configurable_key_mode(self.configurable_key_mode);
        frame.set_motorized_tailgate_enable(self.motorized_tailgate_enabled);
        frame.set_rear_wiper_in_reverse_gear_enable(self.rear_wiper_in_reverse_gear_enabled);
        frame.set_blind_spot_monitoring_enable(self.blind_spot_monitoring_enabled);
        frame.set_park_sensors_enable(self.park_sensors_enabled);
        frame.set_mirrors_tilting_in_reverse_gear_enable(
            self.mirrors_tilting_in_reverse_gear_enabled,
        );
        frame.set_indirect_under_inflation_reset_status(self.indirect_under_inflation_reset_status);
        frame.set_automatic_emergency_braking_enable(self.automatic_emergency_braking_enabled);
        frame.set_collision_alert_sensibility_level(self.collision_alert_sensibility_level);
        frame.set_collision_alert_enable(self.collision_alert_enabled);
        frame.set_hands_free_tailgate_enable(self.hands_free_tailgate_enabled);
        frame.set_speed_limit_recognition_enable(self.speed_limit_recognition_enabled);
        frame.set_radiator_grill_lamps_enable(self.radiator_grill_lamps_enabled);
        frame.set_automatic_main_beam_enable(self.automatic_main_beam_enabled);
        frame.set_driver_alert_assist_enable(self.driver_alert_assist_enabled);
        frame.set_hands_free_tailgate_auto_lock_enable(self.hands_free_tailgate_auto_lock_enabled);
        frame.set_extended_traffic_sign_recognition_enable(
            self.extended_traffic_sign_recognition_enabled,
        );
        frame.set_electric_child_security_enable(self.electric_child_security_enabled);
        frame.set_auto_mirrors_folding_inhibit(self.auto_mirrors_folding_inhibit);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x260 consumption_unit={}", self.consumption_unit)?;
        writeln!(f, " distance_unit={}", self.distance_unit)?;
        writeln!(f, " language={}", self.language)?;
        writeln!(
            f,
            " units_language_parameters_validity={}",
            self.units_language_parameters_validity
        )?;
        writeln!(f, " sound_harmony={}", self.sound_harmony)?;
        writeln!(f, " parameters_validity={}", self.parameters_validity)?;
        writeln!(f, " mood_lighting_level={}", self.mood_lighting_level)?;
        writeln!(f, " temperature_unit={}", self.temperature_unit)?;
        writeln!(f, " volume_unit={}", self.volume_unit)?;
        writeln!(f, " mood_lighting_enabled={}", self.mood_lighting_enabled)?;
        writeln!(
            f,
            " daytime_running_lamps_enabled={}",
            self.daytime_running_lamps_enabled
        )?;
        writeln!(f, " adaptive_lamps_enabled={}", self.adaptive_lamps_enabled)?;
        writeln!(
            f,
            " welcome_function_enabled={}",
            self.welcome_function_enabled
        )?;
        writeln!(
            f,
            " boot_selective_unlocking_enabled={}",
            self.boot_selective_unlocking_enabled
        )?;
        writeln!(
            f,
            " selective_unlocking_enabled={}",
            self.selective_unlocking_enabled
        )?;
        writeln!(
            f,
            " key_selective_unlocking_enabled={}",
            self.key_selective_unlocking_enabled
        )?;
        writeln!(
            f,
            " automatic_elec_parking_brake_application_enabled={}",
            self.automatic_elec_parking_brake_application_enabled
        )?;
        writeln!(
            f,
            " automatic_headlamps_enabled={}",
            self.automatic_headlamps_enabled
        )?;
        writeln!(
            f,
            " welcome_lighting_duration={}",
            self.welcome_lighting_duration
        )?;
        writeln!(
            f,
            " welcome_lighting_enabled={}",
            self.welcome_lighting_enabled
        )?;
        writeln!(
            f,
            " motorway_lighting_enabled={}",
            self.motorway_lighting_enabled
        )?;
        writeln!(
            f,
            " follow_me_home_lighting_duration={}",
            self.follow_me_home_lighting_duration
        )?;
        writeln!(f, " follow_me_home_enabled={}", self.follow_me_home_enabled)?;
        writeln!(f, " configurable_key_mode={}", self.configurable_key_mode)?;
        writeln!(
            f,
            " motorized_tailgate_enabled={}",
            self.motorized_tailgate_enabled
        )?;
        writeln!(
            f,
            " rear_wiper_in_reverse_gear_enabled={}",
            self.rear_wiper_in_reverse_gear_enabled
        )?;
        writeln!(
            f,
            " blind_spot_monitoring_enabled={}",
            self.blind_spot_monitoring_enabled
        )?;
        writeln!(f, " park_sensors_enabled={}", self.park_sensors_enabled)?;
        writeln!(
            f,
            " mirrors_tilting_in_reverse_gear_enabled={}",
            self.mirrors_tilting_in_reverse_gear_enabled
        )?;
        writeln!(
            f,
            " indirect_under_inflation_reset_status={}",
            self.indirect_under_inflation_reset_status
        )?;
        writeln!(
            f,
            " automatic_emergency_braking_enabled={}",
            self.automatic_emergency_braking_enabled
        )?;
        writeln!(
            f,
            " collision_alert_sensibility_level={}",
            self.collision_alert_sensibility_level
        )?;
        writeln!(
            f,
            " collision_alert_enabled={}",
            self.collision_alert_enabled
        )?;
        writeln!(
            f,
            " hands_free_tailgate_enabled={}",
            self.hands_free_tailgate_enabled
        )?;
        writeln!(
            f,
            " speed_limit_recognition_enabled={}",
            self.speed_limit_recognition_enabled
        )?;
        writeln!(
            f,
            " radiator_grill_lamps_enabled={}",
            self.radiator_grill_lamps_enabled
        )?;
        writeln!(
            f,
            " automatic_main_beam_enabled={}",
            self.automatic_main_beam_enabled
        )?;
        writeln!(
            f,
            " driver_alert_assist_enabled={}",
            self.driver_alert_assist_enabled
        )?;
        writeln!(
            f,
            " hands_free_tailgate_auto_lock_enabled={}",
            self.hands_free_tailgate_auto_lock_enabled
        )?;
        writeln!(
            f,
            " extended_traffic_sign_recognition_enabled={}",
            self.extended_traffic_sign_recognition_enabled
        )?;
        writeln!(
            f,
            " electric_child_security_enabled={}",
            self.electric_child_security_enabled
        )?;
        writeln!(
            f,
            " auto_mirrors_folding_inhibit={}",
            self.auto_mirrors_folding_inhibit
        )
    }
}

impl From<&crate::aee2004::conf::x260::Repr> for Repr {
    fn from(repr_2004: &crate::aee2004::conf::x260::Repr) -> Self {
        Repr {
            consumption_unit: ConsumptionUnit::VolumePerDistance, // No equivalent on AEE2004.
            distance_unit: DistanceUnit::Kilometer,               // No equivalent on AEE2004.
            language: Language::English,                          // No equivalent on AEE2004.
            units_language_parameters_validity: true,             // No equivalent on AEE2004.
            sound_harmony: SoundHarmony::Harmony1,                // No equivalent on AEE2004.
            parameters_validity: repr_2004.parameters_validity,
            mood_lighting_level: MoodLightingLevel::Level3, // No equivalent on AEE2004.
            temperature_unit: TemperatureUnit::Celsius,     // No equivalent on AEE2004.
            volume_unit: VolumeUnit::Liter,                 // No equivalent on AEE2004.
            mood_lighting_enabled: repr_2004.mood_lighting_enabled,
            daytime_running_lamps_enabled: repr_2004.daytime_running_lamps_enabled,
            adaptive_lamps_enabled: repr_2004.adaptive_lamps_enabled,
            welcome_function_enabled: repr_2004.welcome_function_enabled,
            boot_selective_unlocking_enabled: repr_2004.boot_permanent_locking_enabled, // No such thing on AEE2010.
            selective_unlocking_enabled: repr_2004.selective_unlocking_enabled,
            key_selective_unlocking_enabled: repr_2004.auto_door_locking_when_leaving_enabled,
            automatic_elec_parking_brake_application_enabled: repr_2004
                .auto_elec_parking_brake_application_enabled,
            automatic_headlamps_enabled: repr_2004.automatic_headlamps_enabled,
            welcome_lighting_duration: LightingDuration2010::FifteenSeconds, // No equivalent on AEE2004.
            welcome_lighting_enabled: false,
            motorway_lighting_enabled: repr_2004.motorway_lighting_enabled,
            follow_me_home_lighting_duration: repr_2004.follow_me_home_lighting_duration.into(),
            follow_me_home_enabled: repr_2004.follow_me_home_enabled,
            configurable_key_mode: repr_2004.configurable_key_mode.into(),
            motorized_tailgate_enabled: false, // No equivalent on AEE2004.
            rear_wiper_in_reverse_gear_enabled: repr_2004.rear_wiper_in_reverse_gear_enabled,
            blind_spot_monitoring_enabled: false, // No equivalent on AEE2004.
            park_sensors_enabled: repr_2004.park_sensors_status > 0,
            mirrors_tilting_in_reverse_gear_enabled: repr_2004
                .mirrors_tilting_in_reverse_gear_enabled,
            indirect_under_inflation_reset_status: false, // No equivalent on AEE2004.
            automatic_emergency_braking_enabled: true, // FARC is equivalent on AEE2004, but not possible to disable.
            collision_alert_sensibility_level: CollisionAlertSensibilityLevel::Normal, // No equivalent on AEE2004.
            collision_alert_enabled: false, // No equivalent on AEE2004.
            hands_free_tailgate_enabled: false, // No equivalent on AEE2004.
            speed_limit_recognition_enabled: false, // No equivalent on AEE2004.
            radiator_grill_lamps_enabled: false, // No equivalent on AEE2004.
            automatic_main_beam_enabled: false, // No equivalent on AEE2004.
            driver_alert_assist_enabled: false, // No equivalent on AEE2004.
            hands_free_tailgate_auto_lock_enabled: false, // No equivalent on AEE2004.
            extended_traffic_sign_recognition_enabled: false, // No equivalent on AEE2004.
            electric_child_security_enabled: false, // No equivalent on AEE2004.
            auto_mirrors_folding_inhibit: false, // No equivalent on AEE2004.
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        config::{
            CollisionAlertSensibilityLevel, ConfigurableKeyAction2010, ConsumptionUnit,
            DistanceUnit, Language, LightingDuration2010, MoodLightingLevel, SoundHarmony,
            TemperatureUnit, VolumeUnit,
        },
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x01, 0x00, 0xab, 0xaa, 0xa3, 0xa8, 0xaa, 0x00];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0x86, 0xef, 0x54, 0x55, 0x50, 0x74, 0x55, 0x08];

    fn frame_1_repr() -> Repr {
        Repr {
            consumption_unit: ConsumptionUnit::DistancePerVolume,
            distance_unit: DistanceUnit::Kilometer,
            language: Language::French,
            units_language_parameters_validity: false,
            sound_harmony: SoundHarmony::Harmony1,
            parameters_validity: false,
            mood_lighting_level: MoodLightingLevel::Level1,
            temperature_unit: TemperatureUnit::Celsius,
            volume_unit: VolumeUnit::Liter,
            mood_lighting_enabled: true,
            daytime_running_lamps_enabled: true,
            adaptive_lamps_enabled: false,
            welcome_function_enabled: true,
            boot_selective_unlocking_enabled: false,
            selective_unlocking_enabled: true,
            key_selective_unlocking_enabled: false,
            automatic_elec_parking_brake_application_enabled: true,
            automatic_headlamps_enabled: false,
            welcome_lighting_duration: LightingDuration2010::ThirtySeconds,
            welcome_lighting_enabled: true,
            motorway_lighting_enabled: false,
            follow_me_home_lighting_duration: LightingDuration2010::ThirtySeconds,
            follow_me_home_enabled: true,
            configurable_key_mode: ConfigurableKeyAction2010::ClusterCustomization,
            motorized_tailgate_enabled: false,
            rear_wiper_in_reverse_gear_enabled: true,
            blind_spot_monitoring_enabled: false,
            park_sensors_enabled: true,
            mirrors_tilting_in_reverse_gear_enabled: false,
            indirect_under_inflation_reset_status: true,
            automatic_emergency_braking_enabled: false,
            collision_alert_sensibility_level: CollisionAlertSensibilityLevel::Close,
            collision_alert_enabled: true,
            hands_free_tailgate_enabled: false,
            speed_limit_recognition_enabled: true,
            radiator_grill_lamps_enabled: false,
            automatic_main_beam_enabled: true,
            driver_alert_assist_enabled: false,
            hands_free_tailgate_auto_lock_enabled: true,
            extended_traffic_sign_recognition_enabled: false,
            electric_child_security_enabled: true,
            auto_mirrors_folding_inhibit: false,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            consumption_unit: ConsumptionUnit::VolumePerDistance,
            distance_unit: DistanceUnit::Mile,
            language: Language::English,
            units_language_parameters_validity: true,
            sound_harmony: SoundHarmony::Harmony4,
            parameters_validity: true,
            mood_lighting_level: MoodLightingLevel::Level5,
            temperature_unit: TemperatureUnit::Fahrenheit,
            volume_unit: VolumeUnit::Gallon,
            mood_lighting_enabled: false,
            daytime_running_lamps_enabled: false,
            adaptive_lamps_enabled: true,
            welcome_function_enabled: false,
            boot_selective_unlocking_enabled: true,
            selective_unlocking_enabled: false,
            key_selective_unlocking_enabled: true,
            automatic_elec_parking_brake_application_enabled: false,
            automatic_headlamps_enabled: true,
            welcome_lighting_duration: LightingDuration2010::SixtySeconds,
            welcome_lighting_enabled: false,
            motorway_lighting_enabled: true,
            follow_me_home_lighting_duration: LightingDuration2010::SixtySeconds,
            follow_me_home_enabled: false,
            configurable_key_mode: ConfigurableKeyAction2010::CeilingLight,
            motorized_tailgate_enabled: true,
            rear_wiper_in_reverse_gear_enabled: false,
            blind_spot_monitoring_enabled: true,
            park_sensors_enabled: false,
            mirrors_tilting_in_reverse_gear_enabled: true,
            indirect_under_inflation_reset_status: false,
            automatic_emergency_braking_enabled: true,
            collision_alert_sensibility_level: CollisionAlertSensibilityLevel::Distant,
            collision_alert_enabled: false,
            hands_free_tailgate_enabled: true,
            speed_limit_recognition_enabled: false,
            radiator_grill_lamps_enabled: true,
            automatic_main_beam_enabled: false,
            driver_alert_assist_enabled: true,
            hands_free_tailgate_auto_lock_enabled: false,
            extended_traffic_sign_recognition_enabled: true,
            electric_child_security_enabled: false,
            auto_mirrors_folding_inhibit: true,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.consumption_unit(), ConsumptionUnit::DistancePerVolume);
        assert_eq!(frame.distance_unit(), DistanceUnit::Kilometer);
        assert_eq!(frame.language(), Language::French);
        assert_eq!(frame.units_language_parameters_validity(), false);
        assert_eq!(frame.sound_harmony(), SoundHarmony::Harmony1);
        assert_eq!(frame.parameters_validity(), false);
        assert_eq!(frame.mood_lighting_level(), MoodLightingLevel::Level1);
        assert_eq!(frame.temperature_unit(), TemperatureUnit::Celsius);
        assert_eq!(frame.volume_unit(), VolumeUnit::Liter);
        assert_eq!(frame.mood_lighting_enable(), true);
        assert_eq!(frame.daytime_running_lamps_enable(), true);
        assert_eq!(frame.adaptive_lamps_enable(), false);
        assert_eq!(frame.welcome_function_enable(), true);
        assert_eq!(frame.boot_selective_unlocking_enable(), false);
        assert_eq!(frame.selective_unlocking_enable(), true);
        assert_eq!(frame.key_selective_unlocking_enable(), false);
        assert_eq!(frame.auto_elec_parking_brake_application_enable(), true);
        assert_eq!(frame.automatic_headlamps_enable(), false);
        assert_eq!(
            frame.welcome_lighting_duration(),
            LightingDuration2010::ThirtySeconds
        );
        assert_eq!(frame.welcome_lighting_enable(), true);
        assert_eq!(frame.motorway_lighting_enable(), false);
        assert_eq!(
            frame.follow_me_home_lighting_duration(),
            LightingDuration2010::ThirtySeconds
        );
        assert_eq!(frame.follow_me_home_enable(), true);
        assert_eq!(
            frame.configurable_key_mode(),
            ConfigurableKeyAction2010::ClusterCustomization
        );
        assert_eq!(frame.motorized_tailgate_enable(), false);
        assert_eq!(frame.rear_wiper_in_reverse_gear_enable(), true);
        assert_eq!(frame.blind_spot_monitoring_enable(), false);
        assert_eq!(frame.park_sensors_enable(), true);
        assert_eq!(frame.mirrors_tilting_in_reverse_gear_enable(), false);
        assert_eq!(frame.indirect_under_inflation_reset_status(), true);
        assert_eq!(frame.automatic_emergency_braking_enable(), false);
        assert_eq!(
            frame.collision_alert_sensibility_level(),
            CollisionAlertSensibilityLevel::Close
        );
        assert_eq!(frame.collision_alert_enable(), true);
        assert_eq!(frame.hands_free_tailgate_enable(), false);
        assert_eq!(frame.speed_limit_recognition_enable(), true);
        assert_eq!(frame.radiator_grill_lamps_enable(), false);
        assert_eq!(frame.automatic_main_beam_enable(), true);
        assert_eq!(frame.driver_alert_assist_enable(), false);
        assert_eq!(frame.hands_free_tailgate_auto_lock_enable(), true);
        assert_eq!(frame.extended_traffic_sign_recognition_enable(), false);
        assert_eq!(frame.electric_child_security_enable(), true);
        assert_eq!(frame.auto_mirrors_folding_inhibit(), false);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.consumption_unit(), ConsumptionUnit::VolumePerDistance);
        assert_eq!(frame.distance_unit(), DistanceUnit::Mile);
        assert_eq!(frame.language(), Language::English);
        assert_eq!(frame.units_language_parameters_validity(), true);
        assert_eq!(frame.sound_harmony(), SoundHarmony::Harmony4);
        assert_eq!(frame.parameters_validity(), true);
        assert_eq!(frame.mood_lighting_level(), MoodLightingLevel::Level5);
        assert_eq!(frame.temperature_unit(), TemperatureUnit::Fahrenheit);
        assert_eq!(frame.volume_unit(), VolumeUnit::Gallon);
        assert_eq!(frame.mood_lighting_enable(), false);
        assert_eq!(frame.daytime_running_lamps_enable(), false);
        assert_eq!(frame.adaptive_lamps_enable(), true);
        assert_eq!(frame.welcome_function_enable(), false);
        assert_eq!(frame.boot_selective_unlocking_enable(), true);
        assert_eq!(frame.selective_unlocking_enable(), false);
        assert_eq!(frame.key_selective_unlocking_enable(), true);
        assert_eq!(frame.auto_elec_parking_brake_application_enable(), false);
        assert_eq!(frame.automatic_headlamps_enable(), true);
        assert_eq!(
            frame.welcome_lighting_duration(),
            LightingDuration2010::SixtySeconds
        );
        assert_eq!(frame.welcome_lighting_enable(), false);
        assert_eq!(frame.motorway_lighting_enable(), true);
        assert_eq!(
            frame.follow_me_home_lighting_duration(),
            LightingDuration2010::SixtySeconds
        );
        assert_eq!(frame.follow_me_home_enable(), false);
        assert_eq!(
            frame.configurable_key_mode(),
            ConfigurableKeyAction2010::CeilingLight
        );
        assert_eq!(frame.motorized_tailgate_enable(), true);
        assert_eq!(frame.rear_wiper_in_reverse_gear_enable(), false);
        assert_eq!(frame.blind_spot_monitoring_enable(), true);
        assert_eq!(frame.park_sensors_enable(), false);
        assert_eq!(frame.mirrors_tilting_in_reverse_gear_enable(), true);
        assert_eq!(frame.indirect_under_inflation_reset_status(), false);
        assert_eq!(frame.automatic_emergency_braking_enable(), true);
        assert_eq!(
            frame.collision_alert_sensibility_level(),
            CollisionAlertSensibilityLevel::Distant
        );
        assert_eq!(frame.collision_alert_enable(), false);
        assert_eq!(frame.hands_free_tailgate_enable(), true);
        assert_eq!(frame.speed_limit_recognition_enable(), false);
        assert_eq!(frame.radiator_grill_lamps_enable(), true);
        assert_eq!(frame.automatic_main_beam_enable(), false);
        assert_eq!(frame.driver_alert_assist_enable(), true);
        assert_eq!(frame.hands_free_tailgate_auto_lock_enable(), false);
        assert_eq!(frame.extended_traffic_sign_recognition_enable(), true);
        assert_eq!(frame.electric_child_security_enable(), false);
        assert_eq!(frame.auto_mirrors_folding_inhibit(), true);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_consumption_unit(ConsumptionUnit::DistancePerVolume);
        frame.set_distance_unit(DistanceUnit::Kilometer);
        frame.set_language(Language::French);
        frame.set_units_language_parameters_validity(false);
        frame.set_sound_harmony(SoundHarmony::Harmony1);
        frame.set_parameters_validity(false);
        frame.set_mood_lighting_level(MoodLightingLevel::Level1);
        frame.set_temperature_unit(TemperatureUnit::Celsius);
        frame.set_volume_unit(VolumeUnit::Liter);
        frame.set_mood_lighting_enable(true);
        frame.set_daytime_running_lamps_enable(true);
        frame.set_adaptive_lamps_enable(false);
        frame.set_welcome_function_enable(true);
        frame.set_boot_selective_unlocking_enable(false);
        frame.set_selective_unlocking_enable(true);
        frame.set_key_selective_unlocking_enable(false);
        frame.set_auto_elec_parking_brake_application_enable(true);
        frame.set_automatic_headlamps_enable(false);
        frame.set_welcome_lighting_duration(LightingDuration2010::ThirtySeconds);
        frame.set_welcome_lighting_enable(true);
        frame.set_motorway_lighting_enable(false);
        frame.set_follow_me_home_lighting_duration(LightingDuration2010::ThirtySeconds);
        frame.set_follow_me_home_enable(true);
        frame.set_configurable_key_mode(ConfigurableKeyAction2010::ClusterCustomization);
        frame.set_motorized_tailgate_enable(false);
        frame.set_rear_wiper_in_reverse_gear_enable(true);
        frame.set_blind_spot_monitoring_enable(false);
        frame.set_park_sensors_enable(true);
        frame.set_mirrors_tilting_in_reverse_gear_enable(false);
        frame.set_indirect_under_inflation_reset_status(true);
        frame.set_automatic_emergency_braking_enable(false);
        frame.set_collision_alert_sensibility_level(CollisionAlertSensibilityLevel::Close);
        frame.set_collision_alert_enable(true);
        frame.set_hands_free_tailgate_enable(false);
        frame.set_speed_limit_recognition_enable(true);
        frame.set_radiator_grill_lamps_enable(false);
        frame.set_automatic_main_beam_enable(true);
        frame.set_driver_alert_assist_enable(false);
        frame.set_hands_free_tailgate_auto_lock_enable(true);
        frame.set_extended_traffic_sign_recognition_enable(false);
        frame.set_electric_child_security_enable(true);
        frame.set_auto_mirrors_folding_inhibit(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_consumption_unit(ConsumptionUnit::VolumePerDistance);
        frame.set_distance_unit(DistanceUnit::Mile);
        frame.set_language(Language::English);
        frame.set_units_language_parameters_validity(true);
        frame.set_sound_harmony(SoundHarmony::Harmony4);
        frame.set_parameters_validity(true);
        frame.set_mood_lighting_level(MoodLightingLevel::Level5);
        frame.set_temperature_unit(TemperatureUnit::Fahrenheit);
        frame.set_volume_unit(VolumeUnit::Gallon);
        frame.set_mood_lighting_enable(false);
        frame.set_daytime_running_lamps_enable(false);
        frame.set_adaptive_lamps_enable(true);
        frame.set_welcome_function_enable(false);
        frame.set_boot_selective_unlocking_enable(true);
        frame.set_selective_unlocking_enable(false);
        frame.set_key_selective_unlocking_enable(true);
        frame.set_auto_elec_parking_brake_application_enable(false);
        frame.set_automatic_headlamps_enable(true);
        frame.set_welcome_lighting_duration(LightingDuration2010::SixtySeconds);
        frame.set_welcome_lighting_enable(false);
        frame.set_motorway_lighting_enable(true);
        frame.set_follow_me_home_lighting_duration(LightingDuration2010::SixtySeconds);
        frame.set_follow_me_home_enable(false);
        frame.set_configurable_key_mode(ConfigurableKeyAction2010::CeilingLight);
        frame.set_motorized_tailgate_enable(true);
        frame.set_rear_wiper_in_reverse_gear_enable(false);
        frame.set_blind_spot_monitoring_enable(true);
        frame.set_park_sensors_enable(false);
        frame.set_mirrors_tilting_in_reverse_gear_enable(true);
        frame.set_indirect_under_inflation_reset_status(false);
        frame.set_automatic_emergency_braking_enable(true);
        frame.set_collision_alert_sensibility_level(CollisionAlertSensibilityLevel::Distant);
        frame.set_collision_alert_enable(false);
        frame.set_hands_free_tailgate_enable(true);
        frame.set_speed_limit_recognition_enable(false);
        frame.set_radiator_grill_lamps_enable(true);
        frame.set_automatic_main_beam_enable(false);
        frame.set_driver_alert_assist_enable(true);
        frame.set_hands_free_tailgate_auto_lock_enable(false);
        frame.set_extended_traffic_sign_recognition_enable(true);
        frame.set_electric_child_security_enable(false);
        frame.set_auto_mirrors_folding_inhibit(true);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x01, 0x03, 0xb2, 0x00, 0x00, 0xd0, 0x00, 0x20, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x01, 0x03, 0xb2, 0x00, 0x00, 0xd0, 0x00];
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
