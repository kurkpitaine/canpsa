use core::fmt;

use crate::{
    config::{
        CollisionAlertSensibilityLevel, ConfigurableKeyAction2010, ConsumptionUnit, DistanceUnit,
        Language, LightingDuration, MoodLightingLevel, SoundHarmony, TemperatureUnit, VolumeUnit,
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
    /// 1-bit extended traffic sign recognition enable flag,
    /// 1-bit 'ECS/SEE' temporary disable flag,
    /// 1-bit mirrors tilting in reverse gear enable flag,
    /// 1-bit indirect under inflation detection enable flag,
    /// 1-bit automatic emergency braking enable flag,
    /// 2-bit collision alert sensibility level field,
    /// 1-bit collision alert enable flag.
    pub const OPT_5: usize = 5;
    /// 1-bit driver alert assist enable flag,
    /// 1-bit hands-free tailgate automatic locking enable flag,
    /// 1-bit hands-free tailgate enable flag,
    /// 1-bit speed limit recognition enable flag,
    /// 1-bit radiator grill lamps option presence flag (maybe anti-fog lights?),
    /// 1-bit automatic main beam enable flag,
    /// 2-bit empty.
    pub const OPT_6: usize = 6;
    /// 2-bit empty,
    /// 1-bit 'IRV' disable flag (maybe InfraRed Vision?),
    /// 5-bit empty.
    pub const OPT_7: usize = 7;
}

/// Length of a x260 CAN frame.
pub const FRAME_LEN: usize = field::OPT_7 + 1;

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
    pub fn welcome_lighting_duration(&self) -> LightingDuration {
        let data = self.buffer.as_ref();
        let raw = (data[field::OPT_3] & 0x06) >> 1;
        LightingDuration::from(raw)
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
    pub fn follow_me_home_lighting_duration(&self) -> LightingDuration {
        let data = self.buffer.as_ref();
        let raw = (data[field::OPT_3] & 0x60) >> 5;
        LightingDuration::from(raw)
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

    /// Return the extended traffic sign recognition enable flag.
    #[inline]
    pub fn extended_traffic_sign_recognition_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x01 != 0
    }

    /// Return the 'ECS/SEE' temporary disable flag.
    #[inline]
    pub fn ecs_tempo_disable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x02 != 0
    }

    /// Return the mirrors tilting in reverse gear enable flag.
    #[inline]
    pub fn mirrors_tilting_in_reverse_gear_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x04 != 0
    }

    /// Return the indirect under-inflation detection enable flag.
    #[inline]
    pub fn indirect_under_inflation_enable(&self) -> bool {
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

    /// Return the driver alert assist enable flag.
    #[inline]
    pub fn driver_alert_assist_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x01 != 0
    }

    /// Return the hands-free tailgate automatic locking enable flag.
    #[inline]
    pub fn hands_free_tailgate_auto_lock_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x02 != 0
    }

    /// Return the hands-free tailgate enable flag.
    #[inline]
    pub fn hands_free_tailgate_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x04 != 0
    }

    /// Return the speed limit recognition enable flag.
    #[inline]
    pub fn speed_limit_recognition_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x08 != 0
    }

    /// Return the radiator grill lamps enable flag (maybe anti-fog lights?).
    #[inline]
    pub fn radiator_grill_lamps_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x10 != 0
    }

    /// Return the automatic main beam enable flag.
    #[inline]
    pub fn automatic_main_beam_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x20 != 0
    }

    /// Return the 'IRV' enable flag (maybe InfraRed Vision).
    #[inline]
    pub fn irv_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_7] & 0x04 != 0
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
    pub fn set_welcome_lighting_duration(&mut self, value: LightingDuration) {
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
    pub fn set_follow_me_home_lighting_duration(&mut self, value: LightingDuration) {
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

    /// Set the extended traffic sign recognition enable flag.
    #[inline]
    pub fn set_extended_traffic_sign_recognition_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_5] = raw;
    }

    /// Set the 'ECS/SEE' temporary disable flag.
    #[inline]
    pub fn set_ecs_tempo_disable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_5] = raw;
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
    pub fn set_indirect_under_inflation_enable(&mut self, value: bool) {
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

    /// Set the driver alert assist enable flag.
    #[inline]
    pub fn set_driver_alert_assist_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_6] = raw;
    }

    /// Set the hands-free tailgate automatic locking enable flag.
    #[inline]
    pub fn set_hands_free_tailgate_auto_lock_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_6] = raw;
    }

    /// Set the hands-free tailgate enable flag.
    #[inline]
    pub fn set_hands_free_tailgate_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_6] = raw;
    }

    /// Set the speed limit recognition enable flag.
    #[inline]
    pub fn set_speed_limit_recognition_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_6] = raw;
    }

    /// Set the radiator grill lamps enable flag (maybe anti-fog lights).
    #[inline]
    pub fn set_radiator_grill_lamps_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_6] = raw;
    }

    /// Set the automatic main beam enable flag.
    #[inline]
    pub fn set_automatic_main_beam_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_6] = raw;
    }

    /// Set the 'IRV' enable flag (maybe InfraRed Vision).
    #[inline]
    pub fn set_irv_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_7] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
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
    consumption_unit: ConsumptionUnit,
    distance_unit: DistanceUnit,
    language: Language,
    units_language_parameters_validity: bool,
    sound_harmony: SoundHarmony,
    parameters_validity: bool,
    mood_lighting_level: MoodLightingLevel,
    temperature_unit: TemperatureUnit,
    volume_unit: VolumeUnit,
    mood_lighting_enabled: bool,
    daytime_running_lamps_enabled: bool,
    adaptive_lamps_enabled: bool,
    welcome_function_enabled: bool,
    boot_selective_unlocking_enabled: bool,
    selective_unlocking_enabled: bool,
    key_selective_unlocking_enabled: bool,
    automatic_elec_parking_brake_application_enabled: bool,
    automatic_headlamps_enabled: bool,
    welcome_lighting_duration: LightingDuration,
    welcome_lighting_enabled: bool,
    motorway_lighting_enabled: bool,
    follow_me_home_lighting_duration: LightingDuration,
    follow_me_home_enabled: bool,
    configurable_key_mode: ConfigurableKeyAction2010,
    motorized_tailgate_enabled: bool,
    rear_wiper_in_reverse_gear_enabled: bool,
    blind_spot_monitoring_enabled: bool,
    park_sensors_enabled: bool,
    mirrors_tilting_in_reverse_gear_enabled: bool,
    indirect_under_inflation_enabled: bool,
    automatic_emergency_braking_enabled: bool,
    collision_alert_sensibility_level: CollisionAlertSensibilityLevel,
    collision_alert_enabled: bool,
    hands_free_tailgate_enabled: bool,
    speed_limit_recognition_enabled: bool,
    radiator_grill_lamps_enabled: bool,
    automatic_main_beam_enabled: bool,
    driver_alert_assist_enabled: bool,
    hands_free_tailgate_auto_lock_enabled: bool,
    extended_traffic_sign_recognition_enabled: bool,
    ecs_temp_disabled: bool,
    irv_enabled: bool,
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
            indirect_under_inflation_enabled: frame.indirect_under_inflation_enable(),
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
            ecs_temp_disabled: frame.ecs_tempo_disable(),
            irv_enabled: frame.irv_enable(),
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
        frame.set_indirect_under_inflation_enable(self.indirect_under_inflation_enabled);
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
        frame.set_ecs_tempo_disable(self.ecs_temp_disabled);
        frame.set_irv_enable(self.irv_enabled);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x260 consumption_unit={}", self.consumption_unit)?;
        write!(f, " distance_unit={}", self.distance_unit)?;
        write!(f, " language={}", self.language)?;
        write!(
            f,
            " units_language_parameters_validity={}",
            self.units_language_parameters_validity
        )?;
        write!(f, " sound_harmony={}", self.sound_harmony)?;
        write!(f, " parameters_validity={}", self.parameters_validity)?;
        write!(f, " mood_lighting_level={}", self.mood_lighting_level)?;
        write!(f, " temperature_unit={}", self.temperature_unit)?;
        write!(f, " volume_unit={}", self.volume_unit)?;
        write!(f, " mood_lighting_enabled={}", self.mood_lighting_enabled)?;
        write!(
            f,
            " daytime_running_lamps_enabled={}",
            self.daytime_running_lamps_enabled
        )?;
        write!(f, " adaptive_lamps_enabled={}", self.adaptive_lamps_enabled)?;
        write!(
            f,
            " welcome_function_enabled={}",
            self.welcome_function_enabled
        )?;
        write!(
            f,
            " boot_selective_unlocking_enabled={}",
            self.boot_selective_unlocking_enabled
        )?;
        write!(
            f,
            " selective_unlocking_enabled={}",
            self.selective_unlocking_enabled
        )?;
        write!(
            f,
            " key_selective_unlocking_enabled={}",
            self.key_selective_unlocking_enabled
        )?;
        write!(
            f,
            " automatic_elec_parking_brake_application_enabled={}",
            self.automatic_elec_parking_brake_application_enabled
        )?;
        write!(
            f,
            " automatic_headlamps_enabled={}",
            self.automatic_headlamps_enabled
        )?;
        write!(
            f,
            " welcome_lighting_duration={}",
            self.welcome_lighting_duration
        )?;
        write!(
            f,
            " welcome_lighting_enabled={}",
            self.welcome_lighting_enabled
        )?;
        write!(
            f,
            " motorway_lighting_enabled={}",
            self.motorway_lighting_enabled
        )?;
        write!(
            f,
            " follow_me_home_lighting_duration={}",
            self.follow_me_home_lighting_duration
        )?;
        write!(f, " follow_me_home_enabled={}", self.follow_me_home_enabled)?;
        write!(f, " configurable_key_mode={}", self.configurable_key_mode)?;
        write!(
            f,
            " motorized_tailgate_enabled={}",
            self.motorized_tailgate_enabled
        )?;
        write!(
            f,
            " rear_wiper_in_reverse_gear_enabled={}",
            self.rear_wiper_in_reverse_gear_enabled
        )?;
        write!(
            f,
            " blind_spot_monitoring_enabled={}",
            self.blind_spot_monitoring_enabled
        )?;
        write!(f, " park_sensors_enabled={}", self.park_sensors_enabled)?;
        write!(
            f,
            " mirrors_tilting_in_reverse_gear_enabled={}",
            self.mirrors_tilting_in_reverse_gear_enabled
        )?;
        write!(
            f,
            " indirect_under_inflation_reset_status={}",
            self.indirect_under_inflation_enabled
        )?;
        write!(
            f,
            " automatic_emergency_braking_enabled={}",
            self.automatic_emergency_braking_enabled
        )?;
        write!(
            f,
            " collision_alert_sensibility_level={}",
            self.collision_alert_sensibility_level
        )?;
        write!(
            f,
            " collision_alert_enabled={}",
            self.collision_alert_enabled
        )?;
        write!(
            f,
            " hands_free_tailgate_enabled={}",
            self.hands_free_tailgate_enabled
        )?;
        write!(
            f,
            " speed_limit_recognition_enabled={}",
            self.speed_limit_recognition_enabled
        )?;
        write!(
            f,
            " radiator_grill_lamps_enabled={}",
            self.radiator_grill_lamps_enabled
        )?;
        write!(
            f,
            " automatic_main_beam_enabled={}",
            self.automatic_main_beam_enabled
        )?;
        write!(
            f,
            " driver_alert_assist_enabled={}",
            self.driver_alert_assist_enabled
        )?;
        write!(
            f,
            " hands_free_tailgate_auto_lock_enabled={}",
            self.hands_free_tailgate_auto_lock_enabled
        )?;
        write!(
            f,
            " extended_traffic_sign_recognition_enabled={}",
            self.extended_traffic_sign_recognition_enabled
        )?;
        write!(f, " ecs_temp_disabled={}", self.ecs_temp_disabled)?;
        write!(f, " irv_enabled={}", self.irv_enabled)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        config::{
            CollisionAlertSensibilityLevel, ConfigurableKeyAction2010, ConsumptionUnit,
            DistanceUnit, Language, LightingDuration, MoodLightingLevel, SoundHarmony,
            TemperatureUnit, VolumeUnit,
        },
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x01, 0x00, 0xab, 0xaa, 0xa3, 0xaa, 0x2a, 0x00];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0x86, 0xef, 0x54, 0x55, 0x50, 0x75, 0x15, 0x04];

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
            welcome_lighting_duration: LightingDuration::ThirtySeconds,
            welcome_lighting_enabled: true,
            motorway_lighting_enabled: false,
            follow_me_home_lighting_duration: LightingDuration::ThirtySeconds,
            follow_me_home_enabled: true,
            configurable_key_mode: ConfigurableKeyAction2010::ClusterCustomization,
            motorized_tailgate_enabled: false,
            rear_wiper_in_reverse_gear_enabled: true,
            blind_spot_monitoring_enabled: false,
            park_sensors_enabled: true,
            mirrors_tilting_in_reverse_gear_enabled: false,
            indirect_under_inflation_enabled: true,
            automatic_emergency_braking_enabled: false,
            collision_alert_sensibility_level: CollisionAlertSensibilityLevel::Level1,
            collision_alert_enabled: true,
            hands_free_tailgate_enabled: false,
            speed_limit_recognition_enabled: true,
            radiator_grill_lamps_enabled: false,
            automatic_main_beam_enabled: true,
            driver_alert_assist_enabled: false,
            hands_free_tailgate_auto_lock_enabled: true,
            extended_traffic_sign_recognition_enabled: false,
            ecs_temp_disabled: true,
            irv_enabled: false,
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
            welcome_lighting_duration: LightingDuration::SixtySeconds,
            welcome_lighting_enabled: false,
            motorway_lighting_enabled: true,
            follow_me_home_lighting_duration: LightingDuration::SixtySeconds,
            follow_me_home_enabled: false,
            configurable_key_mode: ConfigurableKeyAction2010::CeilingLight,
            motorized_tailgate_enabled: true,
            rear_wiper_in_reverse_gear_enabled: false,
            blind_spot_monitoring_enabled: true,
            park_sensors_enabled: false,
            mirrors_tilting_in_reverse_gear_enabled: true,
            indirect_under_inflation_enabled: false,
            automatic_emergency_braking_enabled: true,
            collision_alert_sensibility_level: CollisionAlertSensibilityLevel::Level3,
            collision_alert_enabled: false,
            hands_free_tailgate_enabled: true,
            speed_limit_recognition_enabled: false,
            radiator_grill_lamps_enabled: true,
            automatic_main_beam_enabled: false,
            driver_alert_assist_enabled: true,
            hands_free_tailgate_auto_lock_enabled: false,
            extended_traffic_sign_recognition_enabled: true,
            ecs_temp_disabled: false,
            irv_enabled: true,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
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
            LightingDuration::ThirtySeconds
        );
        assert_eq!(frame.welcome_lighting_enable(), true);
        assert_eq!(frame.motorway_lighting_enable(), false);
        assert_eq!(
            frame.follow_me_home_lighting_duration(),
            LightingDuration::ThirtySeconds
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
        assert_eq!(frame.indirect_under_inflation_enable(), true);
        assert_eq!(frame.automatic_emergency_braking_enable(), false);
        assert_eq!(
            frame.collision_alert_sensibility_level(),
            CollisionAlertSensibilityLevel::Level1
        );
        assert_eq!(frame.collision_alert_enable(), true);
        assert_eq!(frame.hands_free_tailgate_enable(), false);
        assert_eq!(frame.speed_limit_recognition_enable(), true);
        assert_eq!(frame.radiator_grill_lamps_enable(), false);
        assert_eq!(frame.automatic_main_beam_enable(), true);
        assert_eq!(frame.driver_alert_assist_enable(), false);
        assert_eq!(frame.hands_free_tailgate_auto_lock_enable(), true);
        assert_eq!(frame.extended_traffic_sign_recognition_enable(), false);
        assert_eq!(frame.ecs_tempo_disable(), true);
        assert_eq!(frame.irv_enable(), false);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
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
            LightingDuration::SixtySeconds
        );
        assert_eq!(frame.welcome_lighting_enable(), false);
        assert_eq!(frame.motorway_lighting_enable(), true);
        assert_eq!(
            frame.follow_me_home_lighting_duration(),
            LightingDuration::SixtySeconds
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
        assert_eq!(frame.indirect_under_inflation_enable(), false);
        assert_eq!(frame.automatic_emergency_braking_enable(), true);
        assert_eq!(
            frame.collision_alert_sensibility_level(),
            CollisionAlertSensibilityLevel::Level3
        );
        assert_eq!(frame.collision_alert_enable(), false);
        assert_eq!(frame.hands_free_tailgate_enable(), true);
        assert_eq!(frame.speed_limit_recognition_enable(), false);
        assert_eq!(frame.radiator_grill_lamps_enable(), true);
        assert_eq!(frame.automatic_main_beam_enable(), false);
        assert_eq!(frame.driver_alert_assist_enable(), true);
        assert_eq!(frame.hands_free_tailgate_auto_lock_enable(), false);
        assert_eq!(frame.extended_traffic_sign_recognition_enable(), true);
        assert_eq!(frame.ecs_tempo_disable(), false);
        assert_eq!(frame.irv_enable(), true);
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
        frame.set_welcome_lighting_duration(LightingDuration::ThirtySeconds);
        frame.set_welcome_lighting_enable(true);
        frame.set_motorway_lighting_enable(false);
        frame.set_follow_me_home_lighting_duration(LightingDuration::ThirtySeconds);
        frame.set_follow_me_home_enable(true);
        frame.set_configurable_key_mode(ConfigurableKeyAction2010::ClusterCustomization);
        frame.set_motorized_tailgate_enable(false);
        frame.set_rear_wiper_in_reverse_gear_enable(true);
        frame.set_blind_spot_monitoring_enable(false);
        frame.set_park_sensors_enable(true);
        frame.set_mirrors_tilting_in_reverse_gear_enable(false);
        frame.set_indirect_under_inflation_enable(true);
        frame.set_automatic_emergency_braking_enable(false);
        frame.set_collision_alert_sensibility_level(CollisionAlertSensibilityLevel::Level1);
        frame.set_collision_alert_enable(true);
        frame.set_hands_free_tailgate_enable(false);
        frame.set_speed_limit_recognition_enable(true);
        frame.set_radiator_grill_lamps_enable(false);
        frame.set_automatic_main_beam_enable(true);
        frame.set_driver_alert_assist_enable(false);
        frame.set_hands_free_tailgate_auto_lock_enable(true);
        frame.set_extended_traffic_sign_recognition_enable(false);
        frame.set_ecs_tempo_disable(true);
        frame.set_irv_enable(false);

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
        frame.set_welcome_lighting_duration(LightingDuration::SixtySeconds);
        frame.set_welcome_lighting_enable(false);
        frame.set_motorway_lighting_enable(true);
        frame.set_follow_me_home_lighting_duration(LightingDuration::SixtySeconds);
        frame.set_follow_me_home_enable(false);
        frame.set_configurable_key_mode(ConfigurableKeyAction2010::CeilingLight);
        frame.set_motorized_tailgate_enable(true);
        frame.set_rear_wiper_in_reverse_gear_enable(false);
        frame.set_blind_spot_monitoring_enable(true);
        frame.set_park_sensors_enable(false);
        frame.set_mirrors_tilting_in_reverse_gear_enable(true);
        frame.set_indirect_under_inflation_enable(false);
        frame.set_automatic_emergency_braking_enable(true);
        frame.set_collision_alert_sensibility_level(CollisionAlertSensibilityLevel::Level3);
        frame.set_collision_alert_enable(false);
        frame.set_hands_free_tailgate_enable(true);
        frame.set_speed_limit_recognition_enable(false);
        frame.set_radiator_grill_lamps_enable(true);
        frame.set_automatic_main_beam_enable(false);
        frame.set_driver_alert_assist_enable(true);
        frame.set_hands_free_tailgate_auto_lock_enable(false);
        frame.set_extended_traffic_sign_recognition_enable(true);
        frame.set_ecs_tempo_disable(false);
        frame.set_irv_enable(true);

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