use core::fmt;

use crate::{
    config::{ConfigurableKeyAction2004, UserProfile},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 3-bit profile number,
    /// 1-bit parameters validity flag,
    /// 4-bit empty.
    pub const PROFILE: usize = 0;
    /// 1-bit automatic electrical parking brake application enable flag,
    /// 1-bit welcome function enable flag,
    /// 1-bit partial window opening enable flag,
    /// 1-bit 'COE' locking enable flag,
    /// 1-bit automatic door locking when leaving enable flag,
    /// 1-bit boot permanent locking enable flag,
    /// 1-bit automatic door locking when driving enable flag,
    /// 1-bit selective unlocking enable flag.
    pub const OPT_1: usize = 1;
    /// 4-bit follow-me-home lighting duration field,
    /// 1-bit automatic headlamps enable flag,
    /// 1-bit follow-me-home enable field,
    /// 1-bit motorway lighting enable flag,
    /// 1-bit adaptive lamps enable flag.
    pub const OPT_2: usize = 2;
    /// 4-bit ceiling light out delay field,
    /// 2-bit empty,
    /// 1-bit daytime running lamps enable flag,
    /// 1-bit mood lighting enable flag,
    pub const OPT_3: usize = 3;
    /// 1-bit low fuel level alert enable flag,
    /// 1-bit key left in car alert enable flag,
    /// 1-bit lighting left on alert enable flag,
    /// 1-bit 'ALT_GEN' (maybe ALerT GENerator?) flag,
    /// 1-bit ESP in regulation sound alert enable flag,
    /// 3-bit empty.
    pub const OPT_4: usize = 4;
    /// 3-bit empty,
    /// 1-bit automatic mirrors folding enable flag,
    /// 1-bit rear wiper in reverse gear enable flag,
    /// 1-bit mirrors tilting in reverse gear enable flag,
    /// 2-bit parking sensors status field.
    pub const OPT_5: usize = 5;
    /// 5-bit empty,
    /// 2-bit blind spot monitoring status field,
    /// 1-bit 'SECU' (maybe child lock feature?) flag.
    pub const OPT_6: usize = 6;
    /// 4-bit empty,
    /// 4-bit configurable button/key mode field.
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

    /// Return the profile number field.
    #[inline]
    pub fn profile_number(&self) -> UserProfile {
        let data = self.buffer.as_ref();
        let raw = data[field::PROFILE] & 0x07;
        UserProfile::from(raw)
    }

    /// Return the parameters validity flag.
    #[inline]
    pub fn parameters_validity(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::PROFILE] & 0x08 != 0
    }

    /// Return the automatic electrical parking brake application enable flag.
    #[inline]
    pub fn auto_elec_parking_brake_application_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x01 != 0
    }

    /// Return the welcome function enable flag.
    #[inline]
    pub fn welcome_function_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x02 != 0
    }

    /// Return the partial window opening enable flag.
    #[inline]
    pub fn partial_window_opening_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x04 != 0
    }

    /// Return the locking mode on 'COE' enable flag.
    #[inline]
    pub fn locking_mode_on_coe_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x08 != 0
    }

    /// Return the automatic door locking when leaving enable flag.
    #[inline]
    pub fn auto_door_locking_when_leaving_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x10 != 0
    }

    /// Return the boot permanent locking enable flag.
    #[inline]
    pub fn boot_permanent_locking_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x20 != 0
    }

    /// Return the automatic door locking when driving enable flag.
    #[inline]
    pub fn auto_door_locking_when_driving_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x40 != 0
    }

    /// Return the selective unlocking enable flag.
    #[inline]
    pub fn selective_unlocking_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x80 != 0
    }

    /// Return the follow-me-home lighting duration field.
    #[inline]
    pub fn follow_me_home_lighting_duration(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x0f
    }

    /// Return the automatic headlamps enable flag.
    #[inline]
    pub fn automatic_headlamps_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x10 != 0
    }

    /// Return the follow-me-home enable flag.
    #[inline]
    pub fn follow_me_home_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x20 != 0
    }

    /// Return the motorway lighting enable flag.
    #[inline]
    pub fn motorway_lighting_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x40 != 0
    }

    /// Return the adaptive lamps enable flag.
    #[inline]
    pub fn adaptive_lamps_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x80 != 0
    }

    /// Return the ceiling light out delay field.
    #[inline]
    pub fn ceiling_light_out_delay(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x0f
    }

    /// Return the daytime running lamps enable flag.
    #[inline]
    pub fn daytime_running_lamps_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x40 != 0
    }

    /// Return the mood lighting enable flag.
    #[inline]
    pub fn mood_lighting_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x80 != 0
    }

    /// Return the low fuel level alert enable flag.
    #[inline]
    pub fn low_fuel_level_alert_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x01 != 0
    }

    /// Return the key left in car alert enable flag.
    #[inline]
    pub fn key_left_in_car_alert_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x02 != 0
    }

    /// Return the lighting left on alert enable flag.
    #[inline]
    pub fn lighting_left_on_alert_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x04 != 0
    }

    /// Return the 'ALT_GEN' (maybe ALerT GENerator?) enable flag.
    #[inline]
    pub fn alt_gen_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x08 != 0
    }

    /// Return the ESP in regulation sound alert enable flag.
    #[inline]
    pub fn esp_in_regulation_alert_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x10 != 0
    }

    /// Return the automatic mirrors folding enable flag.
    #[inline]
    pub fn auto_mirrors_folding_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x08 != 0
    }

    /// Return the rear wiper in reverse gear enable flag.
    #[inline]
    pub fn rear_wiper_in_reverse_gear_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x10 != 0
    }

    /// Return the mirrors tilting in reverse gear enable flag.
    #[inline]
    pub fn mirrors_tilting_in_reverse_gear_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x20 != 0
    }

    /// Return the parking sensors status field.
    #[inline]
    pub fn park_sensors_status(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::OPT_5] & 0xc0) >> 6
    }

    /// Return the blind spot monitoring status field.
    #[inline]
    pub fn blind_spot_monitoring_status(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::OPT_6] & 0x60) >> 5
    }

    /// Return the 'SECU' (maybe child lock feature?) enable flag.
    #[inline]
    pub fn secu_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_6] & 0x80 != 0
    }

    /// Return the configurable button/key mode field.
    #[inline]
    pub fn configurable_key_mode(&self) -> ConfigurableKeyAction2004 {
        let data = self.buffer.as_ref();
        let raw = (data[field::OPT_7] & 0xf0) >> 4;
        ConfigurableKeyAction2004::from(raw)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the profile number field.
    #[inline]
    pub fn set_profile_number(&mut self, value: UserProfile) {
        let data = self.buffer.as_mut();
        let raw = data[field::PROFILE] & !0x07;
        let raw = raw | (u8::from(value) & 0x07);
        data[field::PROFILE] = raw;
    }

    /// Set the parameters validity flag.
    #[inline]
    pub fn set_parameters_validity(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::PROFILE];
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::PROFILE] = raw;
    }

    /// Set the automatic electrical parking brake application enable flag.
    #[inline]
    pub fn set_auto_elec_parking_brake_application_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_1] = raw;
    }

    /// Set the welcome function enable flag.
    #[inline]
    pub fn set_welcome_function_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_1] = raw;
    }

    /// Set the partial window opening enable flag.
    #[inline]
    pub fn set_partial_window_opening_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_1] = raw;
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

    /// Set the selective unlocking enable flag.
    #[inline]
    pub fn set_selective_unlocking_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_1] = raw;
    }

    /// Set the follow-me-home lighting duration field.
    #[inline]
    pub fn set_follow_me_home_lighting_duration(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x0f;
        let raw = raw | (value & 0x0f);
        data[field::OPT_2] = raw;
    }

    /// Set the automatic headlamps enable flag.
    #[inline]
    pub fn set_automatic_headlamps_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_2] = raw;
    }

    /// Set the follow-me-home enable flag.
    #[inline]
    pub fn set_follow_me_home_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_2] = raw;
    }

    /// Set the motorway lighting enable flag.
    #[inline]
    pub fn set_motorway_lighting_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_2] = raw;
    }

    /// Set the adaptive lamps enable flag.
    #[inline]
    pub fn set_adaptive_lamps_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_2] = raw;
    }

    /// Set the ceiling light out delay field.
    #[inline]
    pub fn set_ceiling_light_out_delay(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x0f;
        let raw = raw | (value & 0x0f);
        data[field::OPT_3] = raw;
    }

    /// Set the daytime running lamps enable flag.
    #[inline]
    pub fn set_daytime_running_lamps_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_3] = raw;
    }

    /// Set the mood lighting enable flag.
    #[inline]
    pub fn set_mood_lighting_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_3] = raw;
    }

    /// Set the low fuel level alert enable flag.
    #[inline]
    pub fn set_low_fuel_level_alert_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_4] = raw;
    }

    /// Set the key left in car alert enable flag.
    #[inline]
    pub fn set_key_left_in_car_alert_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_4] = raw;
    }

    /// Set the lighting left on alert enable flag.
    #[inline]
    pub fn set_lighting_left_on_alert_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_4] = raw;
    }

    /// Set the 'ALT_GEN' (maybe ALerT GENerator?) enable flag.
    #[inline]
    pub fn set_alt_gen_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_4] = raw;
    }

    /// Set the ESP in regulation sound alert enable flag.
    #[inline]
    pub fn set_esp_in_regulation_alert_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_4] = raw;
    }

    /// Set the automatic mirrors folding enable flag.
    #[inline]
    pub fn set_auto_mirrors_folding_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_5] = raw;
    }

    /// Set the rear wiper in reverse gear enable flag.
    #[inline]
    pub fn set_rear_wiper_in_reverse_gear_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_5] = raw;
    }

    /// Set the mirrors tilting in reverse gear enable flag.
    #[inline]
    pub fn set_mirrors_tilting_in_reverse_gear_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_5] = raw;
    }

    /// Set the parking sensors status field.
    #[inline]
    pub fn set_park_sensors_status(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0xc0;
        let raw = raw | ((value << 6) & 0xc0);
        data[field::OPT_5] = raw;
    }

    /// Set the blind spot monitoring status field.
    #[inline]
    pub fn set_blind_spot_monitoring_status(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x60;
        let raw = raw | ((value << 5) & 0x60);
        data[field::OPT_6] = raw;
    }

    /// Set the 'SECU' (maybe child lock feature?) enable flag.
    #[inline]
    pub fn set_secu_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_6] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_6] = raw;
    }

    /// Set the configurable button/key enable flag.
    #[inline]
    pub fn set_configurable_key_mode(&mut self, value: ConfigurableKeyAction2004) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_7] & !0xf0;
        let raw = raw | ((u8::from(value) << 4) & 0xf0);
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
    pub profile_number: UserProfile,
    pub parameters_validity: bool,
    pub auto_elec_parking_brake_application_enabled: bool,
    pub welcome_function_enabled: bool,
    pub partial_window_opening_enabled: bool,
    pub locking_mode_on_coe_enabled: bool,
    pub auto_door_locking_when_leaving_enabled: bool,
    pub boot_permanent_locking_enabled: bool,
    pub auto_door_locking_when_driving_enabled: bool,
    pub selective_unlocking_enabled: bool,
    pub follow_me_home_lighting_duration: u8,
    pub automatic_headlamps_enabled: bool,
    pub follow_me_home_enabled: bool,
    pub motorway_lighting_enabled: bool,
    pub adaptive_lamps_enabled: bool,
    pub ceiling_light_out_delay: u8,
    pub daytime_running_lamps_enabled: bool,
    pub mood_lighting_enabled: bool,
    pub low_fuel_level_alert_enabled: bool,
    pub key_left_in_car_alert_enabled: bool,
    pub lighting_left_on_alert_enabled: bool,
    pub alt_gen_enabled: bool,
    pub esp_in_regulation_alert_enabled: bool,
    pub auto_mirrors_folding_enabled: bool,
    pub rear_wiper_in_reverse_gear_enabled: bool,
    pub mirrors_tilting_in_reverse_gear_enabled: bool,
    pub park_sensors_status: u8,
    pub blind_spot_monitoring_status: u8,
    pub secu_enabled: bool,
    pub configurable_key_mode: ConfigurableKeyAction2004,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            profile_number: frame.profile_number(),
            parameters_validity: frame.parameters_validity(),
            auto_elec_parking_brake_application_enabled: frame
                .auto_elec_parking_brake_application_enable(),
            welcome_function_enabled: frame.welcome_function_enable(),
            partial_window_opening_enabled: frame.partial_window_opening_enable(),
            locking_mode_on_coe_enabled: frame.locking_mode_on_coe_enable(),
            auto_door_locking_when_leaving_enabled: frame.auto_door_locking_when_leaving_enable(),
            boot_permanent_locking_enabled: frame.boot_permanent_locking_enable(),
            auto_door_locking_when_driving_enabled: frame.auto_door_locking_when_driving_enable(),
            selective_unlocking_enabled: frame.selective_unlocking_enable(),
            follow_me_home_lighting_duration: frame.follow_me_home_lighting_duration(),
            automatic_headlamps_enabled: frame.automatic_headlamps_enable(),
            follow_me_home_enabled: frame.follow_me_home_enable(),
            motorway_lighting_enabled: frame.motorway_lighting_enable(),
            adaptive_lamps_enabled: frame.adaptive_lamps_enable(),
            ceiling_light_out_delay: frame.ceiling_light_out_delay(),
            daytime_running_lamps_enabled: frame.daytime_running_lamps_enable(),
            mood_lighting_enabled: frame.mood_lighting_enable(),
            low_fuel_level_alert_enabled: frame.low_fuel_level_alert_enable(),
            key_left_in_car_alert_enabled: frame.key_left_in_car_alert_enable(),
            lighting_left_on_alert_enabled: frame.lighting_left_on_alert_enable(),
            alt_gen_enabled: frame.alt_gen_enable(),
            esp_in_regulation_alert_enabled: frame.esp_in_regulation_alert_enable(),
            auto_mirrors_folding_enabled: frame.auto_mirrors_folding_enable(),
            rear_wiper_in_reverse_gear_enabled: frame.rear_wiper_in_reverse_gear_enable(),
            mirrors_tilting_in_reverse_gear_enabled: frame.mirrors_tilting_in_reverse_gear_enable(),
            park_sensors_status: frame.park_sensors_status(),
            blind_spot_monitoring_status: frame.blind_spot_monitoring_status(),
            secu_enabled: frame.secu_enable(),
            configurable_key_mode: frame.configurable_key_mode(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x260 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_profile_number(self.profile_number);
        frame.set_parameters_validity(self.parameters_validity);
        frame.set_auto_elec_parking_brake_application_enable(
            self.auto_elec_parking_brake_application_enabled,
        );
        frame.set_welcome_function_enable(self.welcome_function_enabled);
        frame.set_partial_window_opening_enable(self.partial_window_opening_enabled);
        frame.set_locking_mode_on_coe_enable(self.locking_mode_on_coe_enabled);
        frame
            .set_auto_door_locking_when_leaving_enable(self.auto_door_locking_when_leaving_enabled);
        frame.set_boot_permanent_locking_enable(self.boot_permanent_locking_enabled);
        frame
            .set_auto_door_locking_when_driving_enable(self.auto_door_locking_when_driving_enabled);
        frame.set_selective_unlocking_enable(self.selective_unlocking_enabled);
        frame.set_follow_me_home_lighting_duration(self.follow_me_home_lighting_duration);
        frame.set_automatic_headlamps_enable(self.automatic_headlamps_enabled);
        frame.set_follow_me_home_enable(self.follow_me_home_enabled);
        frame.set_motorway_lighting_enable(self.motorway_lighting_enabled);
        frame.set_adaptive_lamps_enable(self.adaptive_lamps_enabled);
        frame.set_ceiling_light_out_delay(self.ceiling_light_out_delay);
        frame.set_daytime_running_lamps_enable(self.daytime_running_lamps_enabled);
        frame.set_low_fuel_level_alert_enable(self.low_fuel_level_alert_enabled);
        frame.set_key_left_in_car_alert_enable(self.key_left_in_car_alert_enabled);
        frame.set_lighting_left_on_alert_enable(self.lighting_left_on_alert_enabled);
        frame.set_alt_gen_enable(self.alt_gen_enabled);
        frame.set_esp_in_regulation_alert_enable(self.esp_in_regulation_alert_enabled);
        frame.set_auto_mirrors_folding_enable(self.auto_mirrors_folding_enabled);
        frame.set_rear_wiper_in_reverse_gear_enable(self.rear_wiper_in_reverse_gear_enabled);
        frame.set_mirrors_tilting_in_reverse_gear_enable(
            self.mirrors_tilting_in_reverse_gear_enabled,
        );
        frame.set_park_sensors_status(self.park_sensors_status);
        frame.set_blind_spot_monitoring_status(self.blind_spot_monitoring_status);
        frame.set_secu_enable(self.secu_enabled);
        frame.set_configurable_key_mode(self.configurable_key_mode);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x260 profile_number={}", self.profile_number)?;
        write!(f, " parameters_validity={}", self.parameters_validity)?;
        write!(
            f,
            " auto_elec_parking_brake_application_enabled={}",
            self.auto_elec_parking_brake_application_enabled
        )?;
        write!(
            f,
            " welcome_function_enabled={}",
            self.welcome_function_enabled
        )?;
        write!(
            f,
            " partial_window_opening_enabled={}",
            self.partial_window_opening_enabled
        )?;
        write!(
            f,
            " locking_mode_on_coe_enabled={}",
            self.locking_mode_on_coe_enabled
        )?;
        write!(
            f,
            " auto_door_locking_when_leaving_enabled={}",
            self.auto_door_locking_when_leaving_enabled
        )?;
        write!(
            f,
            " boot_permanent_locking_enabled={}",
            self.boot_permanent_locking_enabled
        )?;
        write!(
            f,
            " auto_door_locking_when_driving_enabled={}",
            self.auto_door_locking_when_driving_enabled
        )?;
        write!(
            f,
            " selective_unlocking_enabled={}",
            self.selective_unlocking_enabled
        )?;
        write!(
            f,
            " follow_me_home_lighting_duration={}",
            self.follow_me_home_lighting_duration
        )?;
        write!(
            f,
            " automatic_headlamps_enabled={}",
            self.automatic_headlamps_enabled
        )?;
        write!(f, " follow_me_home_enabled={}", self.follow_me_home_enabled)?;
        write!(
            f,
            " motorway_lighting_enabled={}",
            self.motorway_lighting_enabled
        )?;
        write!(f, " adaptive_lamps_enabled={}", self.adaptive_lamps_enabled)?;
        write!(
            f,
            " ceiling_light_out_delay={}",
            self.ceiling_light_out_delay
        )?;
        write!(
            f,
            " daytime_running_lamps_enabled={}",
            self.daytime_running_lamps_enabled
        )?;
        write!(
            f,
            " low_fuel_level_alert_enabled={}",
            self.low_fuel_level_alert_enabled
        )?;
        write!(
            f,
            " key_left_in_car_alert_enabled={}",
            self.key_left_in_car_alert_enabled
        )?;
        write!(
            f,
            " lighting_left_on_alert_enabled={}",
            self.lighting_left_on_alert_enabled
        )?;
        write!(f, " alt_gen_enabled={}", self.alt_gen_enabled)?;
        write!(
            f,
            " esp_in_regulation_alert_enabled={}",
            self.esp_in_regulation_alert_enabled
        )?;
        write!(
            f,
            " auto_mirrors_folding_enabled={}",
            self.auto_mirrors_folding_enabled
        )?;
        write!(
            f,
            " rear_wiper_in_reverse_gear_enabled={}",
            self.rear_wiper_in_reverse_gear_enabled
        )?;
        write!(
            f,
            " mirrors_tilting_in_reverse_gear_enabled={}",
            self.mirrors_tilting_in_reverse_gear_enabled
        )?;
        write!(f, " park_sensors_status={}", self.park_sensors_status)?;
        write!(
            f,
            " blind_spot_monitoring_status={}",
            self.blind_spot_monitoring_status
        )?;
        write!(f, " secu_enabled={}", self.secu_enabled)?;
        write!(f, " configurable_key_mode={}", self.configurable_key_mode)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        config::{ConfigurableKeyAction2004, UserProfile},
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x01, 0x03, 0xb2, 0x00, 0x00, 0xd0, 0x00, 0x20];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0x02, 0x03, 0x92, 0x40, 0x00, 0xd0, 0x00, 0x10];

    fn frame_1_repr() -> Repr {
        Repr {
            profile_number: UserProfile::Profile1,
            parameters_validity: false,
            auto_elec_parking_brake_application_enabled: true,
            welcome_function_enabled: true,
            partial_window_opening_enabled: false,
            locking_mode_on_coe_enabled: false,
            auto_door_locking_when_leaving_enabled: false,
            boot_permanent_locking_enabled: false,
            auto_door_locking_when_driving_enabled: false,
            selective_unlocking_enabled: false,
            follow_me_home_lighting_duration: 2,
            automatic_headlamps_enabled: true,
            follow_me_home_enabled: true,
            motorway_lighting_enabled: false,
            adaptive_lamps_enabled: true,
            ceiling_light_out_delay: 0,
            daytime_running_lamps_enabled: false,
            mood_lighting_enabled: false,
            low_fuel_level_alert_enabled: false,
            key_left_in_car_alert_enabled: false,
            lighting_left_on_alert_enabled: false,
            alt_gen_enabled: false,
            esp_in_regulation_alert_enabled: false,
            auto_mirrors_folding_enabled: false,
            rear_wiper_in_reverse_gear_enabled: true,
            mirrors_tilting_in_reverse_gear_enabled: false,
            park_sensors_status: 3,
            blind_spot_monitoring_status: 0,
            secu_enabled: false,
            configurable_key_mode: ConfigurableKeyAction2004::CeilingLight,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            profile_number: UserProfile::Profile2,
            parameters_validity: false,
            auto_elec_parking_brake_application_enabled: true,
            welcome_function_enabled: true,
            partial_window_opening_enabled: false,
            locking_mode_on_coe_enabled: false,
            auto_door_locking_when_leaving_enabled: false,
            boot_permanent_locking_enabled: false,
            auto_door_locking_when_driving_enabled: false,
            selective_unlocking_enabled: false,
            follow_me_home_lighting_duration: 2,
            automatic_headlamps_enabled: true,
            follow_me_home_enabled: false,
            motorway_lighting_enabled: false,
            adaptive_lamps_enabled: true,
            ceiling_light_out_delay: 0,
            daytime_running_lamps_enabled: true,
            mood_lighting_enabled: false,
            low_fuel_level_alert_enabled: false,
            key_left_in_car_alert_enabled: false,
            lighting_left_on_alert_enabled: false,
            alt_gen_enabled: false,
            esp_in_regulation_alert_enabled: false,
            auto_mirrors_folding_enabled: false,
            rear_wiper_in_reverse_gear_enabled: true,
            mirrors_tilting_in_reverse_gear_enabled: false,
            park_sensors_status: 3,
            blind_spot_monitoring_status: 0,
            secu_enabled: false,
            configurable_key_mode: ConfigurableKeyAction2004::BlackPanel,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.profile_number(), UserProfile::Profile1);
        assert_eq!(frame.parameters_validity(), false);
        assert_eq!(frame.auto_elec_parking_brake_application_enable(), true);
        assert_eq!(frame.welcome_function_enable(), true);
        assert_eq!(frame.partial_window_opening_enable(), false);
        assert_eq!(frame.locking_mode_on_coe_enable(), false);
        assert_eq!(frame.auto_door_locking_when_leaving_enable(), false);
        assert_eq!(frame.boot_permanent_locking_enable(), false);
        assert_eq!(frame.auto_door_locking_when_driving_enable(), false);
        assert_eq!(frame.selective_unlocking_enable(), false);
        assert_eq!(frame.follow_me_home_lighting_duration(), 2);
        assert_eq!(frame.automatic_headlamps_enable(), true);
        assert_eq!(frame.follow_me_home_enable(), true);
        assert_eq!(frame.motorway_lighting_enable(), false);
        assert_eq!(frame.adaptive_lamps_enable(), true);
        assert_eq!(frame.ceiling_light_out_delay(), 0);
        assert_eq!(frame.daytime_running_lamps_enable(), false);
        assert_eq!(frame.low_fuel_level_alert_enable(), false);
        assert_eq!(frame.key_left_in_car_alert_enable(), false);
        assert_eq!(frame.lighting_left_on_alert_enable(), false);
        assert_eq!(frame.alt_gen_enable(), false);
        assert_eq!(frame.esp_in_regulation_alert_enable(), false);
        assert_eq!(frame.auto_mirrors_folding_enable(), false);
        assert_eq!(frame.rear_wiper_in_reverse_gear_enable(), true);
        assert_eq!(frame.mirrors_tilting_in_reverse_gear_enable(), false);
        assert_eq!(frame.park_sensors_status(), 3);
        assert_eq!(frame.blind_spot_monitoring_status(), 0);
        assert_eq!(frame.secu_enable(), false);
        assert_eq!(
            frame.configurable_key_mode(),
            ConfigurableKeyAction2004::CeilingLight
        );
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.profile_number(), UserProfile::Profile2);
        assert_eq!(frame.parameters_validity(), false);
        assert_eq!(frame.auto_elec_parking_brake_application_enable(), true);
        assert_eq!(frame.welcome_function_enable(), true);
        assert_eq!(frame.partial_window_opening_enable(), false);
        assert_eq!(frame.locking_mode_on_coe_enable(), false);
        assert_eq!(frame.auto_door_locking_when_leaving_enable(), false);
        assert_eq!(frame.boot_permanent_locking_enable(), false);
        assert_eq!(frame.auto_door_locking_when_driving_enable(), false);
        assert_eq!(frame.selective_unlocking_enable(), false);
        assert_eq!(frame.follow_me_home_lighting_duration(), 2);
        assert_eq!(frame.automatic_headlamps_enable(), true);
        assert_eq!(frame.follow_me_home_enable(), false);
        assert_eq!(frame.motorway_lighting_enable(), false);
        assert_eq!(frame.adaptive_lamps_enable(), true);
        assert_eq!(frame.ceiling_light_out_delay(), 0);
        assert_eq!(frame.daytime_running_lamps_enable(), true);
        assert_eq!(frame.low_fuel_level_alert_enable(), false);
        assert_eq!(frame.key_left_in_car_alert_enable(), false);
        assert_eq!(frame.lighting_left_on_alert_enable(), false);
        assert_eq!(frame.alt_gen_enable(), false);
        assert_eq!(frame.esp_in_regulation_alert_enable(), false);
        assert_eq!(frame.auto_mirrors_folding_enable(), false);
        assert_eq!(frame.rear_wiper_in_reverse_gear_enable(), true);
        assert_eq!(frame.mirrors_tilting_in_reverse_gear_enable(), false);
        assert_eq!(frame.park_sensors_status(), 3);
        assert_eq!(frame.blind_spot_monitoring_status(), 0);
        assert_eq!(frame.secu_enable(), false);
        assert_eq!(
            frame.configurable_key_mode(),
            ConfigurableKeyAction2004::BlackPanel
        );
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_profile_number(UserProfile::Profile1);
        frame.set_parameters_validity(false);
        frame.set_auto_elec_parking_brake_application_enable(true);
        frame.set_welcome_function_enable(true);
        frame.set_partial_window_opening_enable(false);
        frame.set_locking_mode_on_coe_enable(false);
        frame.set_auto_door_locking_when_leaving_enable(false);
        frame.set_boot_permanent_locking_enable(false);
        frame.set_auto_door_locking_when_driving_enable(false);
        frame.set_selective_unlocking_enable(false);
        frame.set_follow_me_home_lighting_duration(2);
        frame.set_automatic_headlamps_enable(true);
        frame.set_follow_me_home_enable(true);
        frame.set_motorway_lighting_enable(false);
        frame.set_adaptive_lamps_enable(true);
        frame.set_ceiling_light_out_delay(0);
        frame.set_daytime_running_lamps_enable(false);
        frame.set_low_fuel_level_alert_enable(false);
        frame.set_key_left_in_car_alert_enable(false);
        frame.set_lighting_left_on_alert_enable(false);
        frame.set_alt_gen_enable(false);
        frame.set_esp_in_regulation_alert_enable(false);
        frame.set_auto_mirrors_folding_enable(false);
        frame.set_rear_wiper_in_reverse_gear_enable(true);
        frame.set_mirrors_tilting_in_reverse_gear_enable(false);
        frame.set_park_sensors_status(3);
        frame.set_blind_spot_monitoring_status(0);
        frame.set_secu_enable(false);
        frame.set_configurable_key_mode(ConfigurableKeyAction2004::CeilingLight);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_profile_number(UserProfile::Profile2);
        frame.set_parameters_validity(false);
        frame.set_auto_elec_parking_brake_application_enable(true);
        frame.set_welcome_function_enable(true);
        frame.set_partial_window_opening_enable(false);
        frame.set_locking_mode_on_coe_enable(false);
        frame.set_auto_door_locking_when_leaving_enable(false);
        frame.set_boot_permanent_locking_enable(false);
        frame.set_auto_door_locking_when_driving_enable(false);
        frame.set_selective_unlocking_enable(false);
        frame.set_follow_me_home_lighting_duration(2);
        frame.set_automatic_headlamps_enable(true);
        frame.set_follow_me_home_enable(false);
        frame.set_motorway_lighting_enable(false);
        frame.set_adaptive_lamps_enable(true);
        frame.set_ceiling_light_out_delay(0);
        frame.set_daytime_running_lamps_enable(true);
        frame.set_low_fuel_level_alert_enable(false);
        frame.set_key_left_in_car_alert_enable(false);
        frame.set_lighting_left_on_alert_enable(false);
        frame.set_alt_gen_enable(false);
        frame.set_esp_in_regulation_alert_enable(false);
        frame.set_auto_mirrors_folding_enable(false);
        frame.set_rear_wiper_in_reverse_gear_enable(true);
        frame.set_mirrors_tilting_in_reverse_gear_enable(false);
        frame.set_park_sensors_status(3);
        frame.set_blind_spot_monitoring_status(0);
        frame.set_secu_enable(false);
        frame.set_configurable_key_mode(ConfigurableKeyAction2004::BlackPanel);

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
