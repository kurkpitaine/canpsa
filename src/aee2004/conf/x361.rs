use core::fmt;

use crate::{
    config::{UnderInflationDetectionSystem, UserProfile},
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
    /// 1-bit profile change allowed flag,
    /// 4-bit empty.
    pub const PROFILE: usize = 0;
    /// 1-bit boot permanent locking option presence flag,
    /// 1-bit partial window opening option presence flag,
    /// 1-bit welcome function option presence flag,
    /// 1-bit 'securoscope' option presence flag,
    /// 1-bit configurable button/key option presence flag,
    /// 3-bit empty.
    pub const OPT_1: usize = 1;
    /// 1-bit automatic headlamps option presence flag,
    /// 1-bit gear efficiency indicator presence flag,
    /// 1-bit automatic electrical parking brake application presence option flag,
    /// 1-bit welcome lighting option presence flag,
    /// 1-bit follow-me-home option presence flag,
    /// 1-bit locking mode on 'COE' option presence flag,
    /// 1-bit automatic door locking when leaving option presence flag,
    /// 1-bit selective unlocking option presence flag.
    pub const OPT_2: usize = 2;
    /// 5-bit empty,
    /// 1-bit rear wiper in reverse gear option presence flag.
    /// 1-bit daytime running lamps option presence flag,
    /// 1-bit adaptive lamps option presence flag.
    pub const OPT_3: usize = 3;
    /// 1-bit blind spot monitoring inhibition option presence flag,
    /// 1-bit blind spot monitoring option presence flag,
    /// 1-bit mood lighting option presence flag,
    /// 1-bit motorway lighting option presence flag,
    /// 1-bit multi-function display presence flag,
    /// 1-bit parking sensors inhibition option presence flag,
    /// 1-bit parking sensors audible assistance option presence flag,
    /// 1-bit parking sensors visual assistance option presence flag.
    pub const OPT_4: usize = 4;
    /// 1-bit empty,
    /// 1-bit automatic emergency braking option presence flag,
    /// 1-bit under-inflation detection reset menu option presence activation,
    /// 1-bit seat belt not fastened / unfastened warning lamps presence flag,
    /// 3-bit under-inflation detection option system type,
    /// 1-bit blind spot audible assistance inhibition option presence flag.
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

    /// Return the profile number field.
    #[inline]
    pub fn profile_number(&self) -> UserProfile {
        let data = self.buffer.as_ref();
        let raw = data[field::PROFILE] & 0x07;
        UserProfile::from(raw)
    }

    /// Return the profile change allowed flag.
    #[inline]
    pub fn profile_change_allowed(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::PROFILE] & 0x08 != 0
    }

    /// Return the boot permanent locking option presence flag.
    #[inline]
    pub fn boot_permanent_locking_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x01 != 0
    }

    /// Return the partial window opening option presence flag.
    #[inline]
    pub fn partial_window_opening_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x02 != 0
    }

    /// Return the welcome function option presence flag.
    #[inline]
    pub fn welcome_function_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x04 != 0
    }

    /// Return the 'securoscope' option presence flag.
    #[inline]
    pub fn securoscope_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x08 != 0
    }

    /// Return the configurable button/key option presence flag.
    #[inline]
    pub fn configurable_key_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_1] & 0x10 != 0
    }

    /// Return the automatic headlamps option presence flag.
    #[inline]
    pub fn automatic_headlamps_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x01 != 0
    }

    /// Return the gear efficiency indicator option presence flag.
    #[inline]
    pub fn gear_efficiency_indicator_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x02 != 0
    }

    /// Return the automatic electrical parking brake application option presence flag.
    #[inline]
    pub fn auto_elec_parking_brake_application_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x04 != 0
    }

    /// Return the welcome lighting option presence flag.
    #[inline]
    pub fn welcome_lighting_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x08 != 0
    }

    /// Return the follow-me-home option presence flag.
    #[inline]
    pub fn follow_me_home_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x10 != 0
    }

    /// Return the locking mode on 'COE' option presence flag.
    #[inline]
    pub fn locking_mode_on_coe_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x20 != 0
    }

    /// Return the automatic door locking when leaving option presence flag.
    #[inline]
    pub fn auto_door_locking_when_leaving_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x40 != 0
    }

    /// Return the selective unlocking option presence flag.
    #[inline]
    pub fn selective_unlocking_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_2] & 0x80 != 0
    }

    /// Return the rear wiper in reverse gear option presence flag.
    #[inline]
    pub fn rear_wiper_in_reverse_gear_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x20 != 0
    }

    /// Return the daytime running lamps option presence flag.
    #[inline]
    pub fn daytime_running_lamps_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x40 != 0
    }

    /// Return the adaptive lamps option presence flag.
    #[inline]
    pub fn adaptive_lamps_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_3] & 0x80 != 0
    }

    /// Return the blind spot monitoring inhibition option presence flag.
    #[inline]
    pub fn blind_spot_monitoring_inhibition_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x01 != 0
    }

    /// Return the blind spot monitoring option presence flag.
    #[inline]
    pub fn blind_spot_monitoring_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x02 != 0
    }

    /// Return the mood lighting option presence flag.
    #[inline]
    pub fn mood_lighting_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x04 != 0
    }

    /// Return the motorway lighting option presence flag.
    #[inline]
    pub fn motorway_lighting_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x08 != 0
    }

    /// Return the multi-function display presence flag.
    #[inline]
    pub fn multi_function_display_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x10 != 0
    }

    /// Return the parking sensors inhibition option presence flag.
    #[inline]
    pub fn park_sensors_inhibition_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x20 != 0
    }

    /// Return the parking audible assistance option presence flag.
    #[inline]
    pub fn park_sensors_audible_assistance_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x40 != 0
    }

    /// Return the parking visual assistance option presence flag.
    #[inline]
    pub fn park_sensors_visual_assistance_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_4] & 0x80 != 0
    }

    /// Return the automatic emergency braking option presence flag.
    #[inline]
    pub fn automatic_emergency_braking_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x02 != 0
    }

    /// Return the under-inflation detection reset menu option presence flag.
    #[inline]
    pub fn under_inflation_detection_reset_menu_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x04 != 0
    }

    /// Return the seat belt not fastened / unfastened warning lamps presence flag.
    #[inline]
    pub fn seat_belt_status_lamps_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x08 != 0
    }

    /// Return the under-inflation detection option system type field.
    #[inline]
    pub fn under_inflation_detection(&self) -> UnderInflationDetectionSystem {
        let data = self.buffer.as_ref();
        let raw = (data[field::OPT_5] & 0x70) >> 4;
        UnderInflationDetectionSystem::from(raw)
    }

    /// Return the blind spot audible assistance inhibition option presence flag.
    #[inline]
    pub fn blind_spot_audible_assistance_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::OPT_5] & 0x80 != 0
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

    /// Set the profile change allowed flag.
    #[inline]
    pub fn set_profile_change_allowed(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::PROFILE];
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::PROFILE] = raw;
    }

    /// Set the boot permanent locking option presence flag.
    #[inline]
    pub fn set_boot_permanent_locking_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_1] = raw;
    }

    /// Set the partial window opening option presence flag.
    #[inline]
    pub fn set_partial_window_opening_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_1] = raw;
    }

    /// Set the welcome function option presence flag.
    #[inline]
    pub fn set_welcome_function_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_1] = raw;
    }

    /// Set the 'securoscope' option presence flag.
    #[inline]
    pub fn set_securoscope_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_1] = raw;
    }

    /// Set the configurable button/key option presence flag.
    #[inline]
    pub fn set_configurable_key_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_1] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_1] = raw;
    }

    /// Set the automatic headlamps option presence flag.
    #[inline]
    pub fn set_automatic_headlamps_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_2] = raw;
    }

    /// Set the gear efficiency indicator option presence flag.
    #[inline]
    pub fn set_gear_efficiency_indicator_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_2] = raw;
    }

    /// Set the automatic electrical parking brake application option presence flag.
    #[inline]
    pub fn set_auto_elec_parking_brake_application_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_2] = raw;
    }

    /// Set the welcome lighting option presence flag.
    #[inline]
    pub fn set_welcome_lighting_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_2] = raw;
    }

    /// Set the follow-me-home option presence flag.
    #[inline]
    pub fn set_follow_me_home_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_2] = raw;
    }

    /// Set the locking mode on 'COE' option presence flag.
    #[inline]
    pub fn set_locking_mode_on_coe_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_2] = raw;
    }

    /// Set the automatic door locking when leaving option presence flag.
    #[inline]
    pub fn set_auto_door_locking_when_leaving_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_2] = raw;
    }

    /// Set the selective unlocking option presence flag.
    #[inline]
    pub fn set_selective_unlocking_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_2] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_2] = raw;
    }

    /// Set the rear wiper in reverse gear option presence flag.
    #[inline]
    pub fn set_rear_wiper_in_reverse_gear_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_3] = raw;
    }

    /// Set the daytime running lamps option presence flag.
    #[inline]
    pub fn set_daytime_running_lamps_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_3] = raw;
    }

    /// Set the adaptive lamps option presence flag.
    #[inline]
    pub fn set_adaptive_lamps_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_3] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_3] = raw;
    }

    /// Set the blind spot monitoring inhibition option presence flag.
    #[inline]
    pub fn set_blind_spot_monitoring_inhibition_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::OPT_4] = raw;
    }

    /// Set the blind spot monitoring option presence flag.
    #[inline]
    pub fn set_blind_spot_monitoring_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_4] = raw;
    }

    /// Set the mood lighting option presence flag.
    #[inline]
    pub fn set_mood_lighting_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_4] = raw;
    }

    /// Set the motorway lighting option presence flag.
    #[inline]
    pub fn set_motorway_lighting_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_4] = raw;
    }

    /// Set the multi-function display presence flag.
    #[inline]
    pub fn set_multi_function_display_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::OPT_4] = raw;
    }

    /// Set the parking sensors inhibition option presence flag.
    #[inline]
    pub fn set_park_sensors_inhibition_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::OPT_4] = raw;
    }

    /// Set the parking audible assistance option presence flag.
    #[inline]
    pub fn set_park_sensors_audible_assistance_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::OPT_4] = raw;
    }

    /// Set the parking visual assistance option presence flag.
    #[inline]
    pub fn set_park_sensors_visual_assistance_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_4] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::OPT_4] = raw;
    }

    /// Set the automatic emergency braking option presence flag.
    #[inline]
    pub fn set_automatic_emergency_braking_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::OPT_5] = raw;
    }

    /// Set the under-inflation detection reset menu option presence flag.
    #[inline]
    pub fn set_under_inflation_detection_reset_menu_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::OPT_5] = raw;
    }

    /// Set the seat belt not fastened / unfastened warning lamps presence flag.
    #[inline]
    pub fn set_seat_belt_status_lamps_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::OPT_5] = raw;
    }

    /// Set the under-inflation detection option system type field.
    #[inline]
    pub fn set_under_inflation_detection(&mut self, value: UnderInflationDetectionSystem) {
        let data = self.buffer.as_mut();
        let raw = data[field::OPT_5] & !0x70;
        let raw = raw | ((u8::from(value) << 4) & 0x70);
        data[field::OPT_5] = raw;
    }

    /// Set the blind spot audible assistance inhibition option presence flag.
    #[inline]
    pub fn set_blind_spot_audible_assistance_presence(&mut self, value: bool) {
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
    profile_number: UserProfile,
    profile_change_allowed: bool,
    boot_permanent_locking_present: bool,
    partial_window_opening_present: bool,
    welcome_function_present: bool,
    securoscope_present: bool,
    configurable_key_present: bool,
    automatic_headlamps_present: bool,
    gear_efficiency_indicator_present: bool,
    automatic_electric_parking_brake_application_present: bool,
    welcome_lighting_present: bool,
    follow_me_home_present: bool,
    locking_mode_on_coe_present: bool,
    automatic_door_locking_when_leaving_present: bool,
    selective_unlocking_present: bool,
    rear_wiper_in_reverse_gear_present: bool,
    daytime_running_lamps_present: bool,
    adaptive_lamps_present: bool,
    blind_spot_monitoring_inhibition_present: bool,
    blind_spot_monitoring_present: bool,
    mood_lighting_present: bool,
    motorway_lighting_present: bool,
    multi_function_display_present: bool,
    parking_sensors_inhibition_present: bool,
    parking_sensors_audible_assistance_present: bool,
    parking_sensors_visual_assistance_presence: bool,
    automatic_emergency_braking_presence: bool,
    under_inflation_detection_reset_menu_present: bool,
    seat_belt_status_lamps_present: bool,
    under_inflation_detection: UnderInflationDetectionSystem,
    blind_spot_audible_assistance_present: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            profile_number: frame.profile_number(),
            profile_change_allowed: frame.profile_change_allowed(),
            boot_permanent_locking_present: frame.boot_permanent_locking_presence(),
            partial_window_opening_present: frame.partial_window_opening_presence(),
            welcome_function_present: frame.welcome_function_presence(),
            securoscope_present: frame.securoscope_presence(),
            configurable_key_present: frame.configurable_key_presence(),
            automatic_headlamps_present: frame.automatic_headlamps_presence(),
            gear_efficiency_indicator_present: frame.gear_efficiency_indicator_presence(),
            automatic_electric_parking_brake_application_present: frame
                .auto_elec_parking_brake_application_presence(),
            welcome_lighting_present: frame.welcome_lighting_presence(),
            follow_me_home_present: frame.follow_me_home_presence(),
            locking_mode_on_coe_present: frame.locking_mode_on_coe_presence(),
            automatic_door_locking_when_leaving_present: frame
                .auto_door_locking_when_leaving_presence(),
            selective_unlocking_present: frame.selective_unlocking_presence(),
            rear_wiper_in_reverse_gear_present: frame.rear_wiper_in_reverse_gear_presence(),
            daytime_running_lamps_present: frame.daytime_running_lamps_presence(),
            adaptive_lamps_present: frame.adaptive_lamps_presence(),
            blind_spot_monitoring_inhibition_present: frame
                .blind_spot_monitoring_inhibition_presence(),
            blind_spot_monitoring_present: frame.blind_spot_monitoring_presence(),
            mood_lighting_present: frame.mood_lighting_presence(),
            motorway_lighting_present: frame.motorway_lighting_presence(),
            multi_function_display_present: frame.multi_function_display_presence(),
            parking_sensors_inhibition_present: frame.park_sensors_inhibition_presence(),
            parking_sensors_audible_assistance_present: frame
                .park_sensors_audible_assistance_presence(),
            parking_sensors_visual_assistance_presence: frame
                .park_sensors_visual_assistance_presence(),
            automatic_emergency_braking_presence: frame.automatic_emergency_braking_presence(),
            under_inflation_detection_reset_menu_present: frame
                .under_inflation_detection_reset_menu_presence(),
            seat_belt_status_lamps_present: frame.seat_belt_status_lamps_presence(),
            under_inflation_detection: frame.under_inflation_detection(),
            blind_spot_audible_assistance_present: frame.blind_spot_audible_assistance_presence(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x361 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_profile_number(self.profile_number);
        frame.set_profile_change_allowed(self.profile_change_allowed);
        frame.set_boot_permanent_locking_presence(self.boot_permanent_locking_present);
        frame.set_partial_window_opening_presence(self.partial_window_opening_present);
        frame.set_welcome_function_presence(self.welcome_function_present);
        frame.set_securoscope_presence(self.securoscope_present);
        frame.set_configurable_key_presence(self.configurable_key_present);
        frame.set_automatic_headlamps_presence(self.automatic_headlamps_present);
        frame.set_gear_efficiency_indicator_presence(self.gear_efficiency_indicator_present);
        frame.set_auto_elec_parking_brake_application_presence(
            self.automatic_electric_parking_brake_application_present,
        );
        frame.set_welcome_lighting_presence(self.welcome_lighting_present);
        frame.set_follow_me_home_presence(self.follow_me_home_present);
        frame.set_locking_mode_on_coe_presence(self.locking_mode_on_coe_present);
        frame.set_auto_door_locking_when_leaving_presence(
            self.automatic_door_locking_when_leaving_present,
        );
        frame.set_selective_unlocking_presence(self.selective_unlocking_present);
        frame.set_rear_wiper_in_reverse_gear_presence(self.rear_wiper_in_reverse_gear_present);
        frame.set_daytime_running_lamps_presence(self.daytime_running_lamps_present);
        frame.set_adaptive_lamps_presence(self.adaptive_lamps_present);
        frame.set_blind_spot_monitoring_inhibition_presence(
            self.blind_spot_monitoring_inhibition_present,
        );
        frame.set_blind_spot_monitoring_presence(self.blind_spot_monitoring_present);
        frame.set_mood_lighting_presence(self.mood_lighting_present);
        frame.set_motorway_lighting_presence(self.motorway_lighting_present);
        frame.set_multi_function_display_presence(self.multi_function_display_present);
        frame.set_park_sensors_inhibition_presence(self.parking_sensors_inhibition_present);
        frame.set_park_sensors_audible_assistance_presence(
            self.parking_sensors_audible_assistance_present,
        );
        frame.set_park_sensors_visual_assistance_presence(
            self.parking_sensors_visual_assistance_presence,
        );
        frame.set_automatic_emergency_braking_presence(self.automatic_emergency_braking_presence);
        frame.set_under_inflation_detection_reset_menu_presence(
            self.under_inflation_detection_reset_menu_present,
        );
        frame.set_seat_belt_status_lamps_presence(self.seat_belt_status_lamps_present);
        frame.set_under_inflation_detection(self.under_inflation_detection);
        frame
            .set_blind_spot_audible_assistance_presence(self.blind_spot_audible_assistance_present);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x361 profile number={}", self.profile_number)?;
        write!(f, " profile change allowed={}", self.profile_change_allowed)?;
        write!(
            f,
            " boot permanent locking present={}",
            self.boot_permanent_locking_present
        )?;
        write!(
            f,
            " partial window opening present={}",
            self.partial_window_opening_present
        )?;
        write!(
            f,
            " welcome function present={}",
            self.welcome_function_present
        )?;
        write!(f, " securoscope present={}", self.securoscope_present)?;
        write!(
            f,
            " configurable key present={}",
            self.configurable_key_present
        )?;
        write!(
            f,
            " automatic headlamps present={}",
            self.automatic_headlamps_present
        )?;
        write!(
            f,
            " gear efficiency indicator present={}",
            self.gear_efficiency_indicator_present
        )?;
        write!(
            f,
            " automatic electric parking brake application present={}",
            self.automatic_electric_parking_brake_application_present
        )?;
        write!(
            f,
            " welcome lighting present={}",
            self.welcome_lighting_present
        )?;
        write!(f, " follow-me-home present={}", self.follow_me_home_present)?;
        write!(
            f,
            " locking mode on coe present={}",
            self.locking_mode_on_coe_present
        )?;
        write!(
            f,
            " automatic door locking when leaving present={}",
            self.automatic_door_locking_when_leaving_present
        )?;
        write!(
            f,
            " selective unlocking present={}",
            self.selective_unlocking_present
        )?;
        write!(
            f,
            " rear wiper in reverse gear present={}",
            self.rear_wiper_in_reverse_gear_present
        )?;
        write!(
            f,
            " daytime running lamps present={}",
            self.daytime_running_lamps_present
        )?;
        write!(f, " adaptive lamps present={}", self.adaptive_lamps_present)?;
        write!(
            f,
            " blind spot monitoring inhibition present={}",
            self.blind_spot_monitoring_inhibition_present
        )?;
        write!(
            f,
            " blind spot monitoring present={}",
            self.blind_spot_monitoring_present
        )?;
        write!(f, " mood lighting present={}", self.mood_lighting_present)?;
        write!(
            f,
            " motorway lighting present={}",
            self.motorway_lighting_present
        )?;
        write!(
            f,
            " multi function display present={}",
            self.multi_function_display_present
        )?;
        write!(
            f,
            " parking sensors inhibition present={}",
            self.parking_sensors_inhibition_present
        )?;
        write!(
            f,
            " parking sensors audible assistance present={}",
            self.parking_sensors_audible_assistance_present
        )?;
        write!(
            f,
            " parking sensors visual assistance presence={}",
            self.parking_sensors_visual_assistance_presence
        )?;
        write!(
            f,
            " automatic emergency braking presence={}",
            self.automatic_emergency_braking_presence
        )?;
        write!(
            f,
            " under inflation detection reset menu present={}",
            self.under_inflation_detection_reset_menu_present
        )?;
        write!(
            f,
            " seat belt status lamps present={}",
            self.seat_belt_status_lamps_present
        )?;
        write!(
            f,
            " under inflation detection={}",
            self.under_inflation_detection
        )?;
        write!(
            f,
            " blind spot audible assistance present={}",
            self.blind_spot_audible_assistance_present
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        config::{UnderInflationDetectionSystem, UserProfile},
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 6] = [0x01, 0x00, 0x12, 0xe0, 0x30, 0x34];
    static REPR_FRAME_BYTES_2: [u8; 6] = [0x01, 0x10, 0x10, 0xa0, 0x10, 0x20];

    fn frame_1_repr() -> Repr {
        Repr {
            profile_number: UserProfile::Profile1,
            profile_change_allowed: false,
            boot_permanent_locking_present: false,
            partial_window_opening_present: false,
            welcome_function_present: false,
            securoscope_present: false,
            configurable_key_present: false,
            automatic_headlamps_present: false,
            gear_efficiency_indicator_present: true,
            automatic_electric_parking_brake_application_present: false,
            welcome_lighting_present: false,
            follow_me_home_present: true,
            locking_mode_on_coe_present: false,
            automatic_door_locking_when_leaving_present: false,
            selective_unlocking_present: false,
            rear_wiper_in_reverse_gear_present: true,
            daytime_running_lamps_present: true,
            adaptive_lamps_present: true,
            blind_spot_monitoring_inhibition_present: false,
            blind_spot_monitoring_present: false,
            mood_lighting_present: false,
            motorway_lighting_present: false,
            multi_function_display_present: true,
            parking_sensors_inhibition_present: true,
            parking_sensors_audible_assistance_present: false,
            parking_sensors_visual_assistance_presence: false,
            automatic_emergency_braking_presence: false,
            under_inflation_detection_reset_menu_present: true,
            seat_belt_status_lamps_present: false,
            under_inflation_detection: UnderInflationDetectionSystem::Indirect,
            blind_spot_audible_assistance_present: false,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            profile_number: UserProfile::Profile1,
            profile_change_allowed: false,
            boot_permanent_locking_present: false,
            partial_window_opening_present: false,
            welcome_function_present: false,
            securoscope_present: false,
            configurable_key_present: true,
            automatic_headlamps_present: false,
            gear_efficiency_indicator_present: false,
            automatic_electric_parking_brake_application_present: false,
            welcome_lighting_present: false,
            follow_me_home_present: true,
            locking_mode_on_coe_present: false,
            automatic_door_locking_when_leaving_present: false,
            selective_unlocking_present: false,
            rear_wiper_in_reverse_gear_present: true,
            daytime_running_lamps_present: false,
            adaptive_lamps_present: true,
            blind_spot_monitoring_inhibition_present: false,
            blind_spot_monitoring_present: false,
            mood_lighting_present: false,
            motorway_lighting_present: false,
            multi_function_display_present: true,
            parking_sensors_inhibition_present: false,
            parking_sensors_audible_assistance_present: false,
            parking_sensors_visual_assistance_presence: false,
            automatic_emergency_braking_presence: false,
            under_inflation_detection_reset_menu_present: false,
            seat_belt_status_lamps_present: false,
            under_inflation_detection: UnderInflationDetectionSystem::DirectWithoutAbsolutePressure,
            blind_spot_audible_assistance_present: false,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.profile_number(), UserProfile::Profile1);
        assert_eq!(frame.profile_change_allowed(), false);
        assert_eq!(frame.boot_permanent_locking_presence(), false);
        assert_eq!(frame.partial_window_opening_presence(), false);
        assert_eq!(frame.welcome_function_presence(), false);
        assert_eq!(frame.securoscope_presence(), false);
        assert_eq!(frame.configurable_key_presence(), false);
        assert_eq!(frame.automatic_headlamps_presence(), false);
        assert_eq!(frame.gear_efficiency_indicator_presence(), true);
        assert_eq!(frame.auto_elec_parking_brake_application_presence(), false);
        assert_eq!(frame.welcome_lighting_presence(), false);
        assert_eq!(frame.follow_me_home_presence(), true);
        assert_eq!(frame.locking_mode_on_coe_presence(), false);
        assert_eq!(frame.auto_door_locking_when_leaving_presence(), false);
        assert_eq!(frame.selective_unlocking_presence(), false);
        assert_eq!(frame.rear_wiper_in_reverse_gear_presence(), true);
        assert_eq!(frame.daytime_running_lamps_presence(), true);
        assert_eq!(frame.adaptive_lamps_presence(), true);
        assert_eq!(frame.blind_spot_monitoring_inhibition_presence(), false);
        assert_eq!(frame.blind_spot_monitoring_presence(), false);
        assert_eq!(frame.mood_lighting_presence(), false);
        assert_eq!(frame.motorway_lighting_presence(), false);
        assert_eq!(frame.multi_function_display_presence(), true);
        assert_eq!(frame.park_sensors_inhibition_presence(), true);
        assert_eq!(frame.park_sensors_audible_assistance_presence(), false);
        assert_eq!(frame.park_sensors_visual_assistance_presence(), false);
        assert_eq!(frame.automatic_emergency_braking_presence(), false);
        assert_eq!(frame.under_inflation_detection_reset_menu_presence(), true);
        assert_eq!(frame.seat_belt_status_lamps_presence(), false);
        assert_eq!(
            frame.under_inflation_detection(),
            UnderInflationDetectionSystem::Indirect
        );
        assert_eq!(frame.blind_spot_audible_assistance_presence(), false);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.profile_number(), UserProfile::Profile1);
        assert_eq!(frame.profile_change_allowed(), false);
        assert_eq!(frame.boot_permanent_locking_presence(), false);
        assert_eq!(frame.partial_window_opening_presence(), false);
        assert_eq!(frame.welcome_function_presence(), false);
        assert_eq!(frame.securoscope_presence(), false);
        assert_eq!(frame.configurable_key_presence(), true);
        assert_eq!(frame.automatic_headlamps_presence(), false);
        assert_eq!(frame.gear_efficiency_indicator_presence(), false);
        assert_eq!(frame.auto_elec_parking_brake_application_presence(), false);
        assert_eq!(frame.welcome_lighting_presence(), false);
        assert_eq!(frame.follow_me_home_presence(), true);
        assert_eq!(frame.locking_mode_on_coe_presence(), false);
        assert_eq!(frame.auto_door_locking_when_leaving_presence(), false);
        assert_eq!(frame.selective_unlocking_presence(), false);
        assert_eq!(frame.rear_wiper_in_reverse_gear_presence(), true);
        assert_eq!(frame.daytime_running_lamps_presence(), false);
        assert_eq!(frame.adaptive_lamps_presence(), true);
        assert_eq!(frame.blind_spot_monitoring_inhibition_presence(), false);
        assert_eq!(frame.blind_spot_monitoring_presence(), false);
        assert_eq!(frame.mood_lighting_presence(), false);
        assert_eq!(frame.motorway_lighting_presence(), false);
        assert_eq!(frame.multi_function_display_presence(), true);
        assert_eq!(frame.park_sensors_inhibition_presence(), false);
        assert_eq!(frame.park_sensors_audible_assistance_presence(), false);
        assert_eq!(frame.park_sensors_visual_assistance_presence(), false);
        assert_eq!(frame.automatic_emergency_braking_presence(), false);
        assert_eq!(frame.under_inflation_detection_reset_menu_presence(), false);
        assert_eq!(frame.seat_belt_status_lamps_presence(), false);
        assert_eq!(
            frame.under_inflation_detection(),
            UnderInflationDetectionSystem::DirectWithoutAbsolutePressure
        );
        assert_eq!(frame.blind_spot_audible_assistance_presence(), false);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 6];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_profile_number(UserProfile::Profile1);
        frame.set_profile_change_allowed(false);
        frame.set_boot_permanent_locking_presence(false);
        frame.set_partial_window_opening_presence(false);
        frame.set_welcome_function_presence(false);
        frame.set_securoscope_presence(false);
        frame.set_configurable_key_presence(false);
        frame.set_automatic_headlamps_presence(false);
        frame.set_gear_efficiency_indicator_presence(true);
        frame.set_auto_elec_parking_brake_application_presence(false);
        frame.set_welcome_lighting_presence(false);
        frame.set_follow_me_home_presence(true);
        frame.set_locking_mode_on_coe_presence(false);
        frame.set_auto_door_locking_when_leaving_presence(false);
        frame.set_selective_unlocking_presence(false);
        frame.set_rear_wiper_in_reverse_gear_presence(true);
        frame.set_daytime_running_lamps_presence(true);
        frame.set_adaptive_lamps_presence(true);
        frame.set_blind_spot_monitoring_inhibition_presence(false);
        frame.set_blind_spot_monitoring_presence(false);
        frame.set_mood_lighting_presence(false);
        frame.set_motorway_lighting_presence(false);
        frame.set_multi_function_display_presence(true);
        frame.set_park_sensors_inhibition_presence(true);
        frame.set_park_sensors_audible_assistance_presence(false);
        frame.set_park_sensors_visual_assistance_presence(false);
        frame.set_automatic_emergency_braking_presence(false);
        frame.set_under_inflation_detection_reset_menu_presence(true);
        frame.set_seat_belt_status_lamps_presence(false);
        frame.set_under_inflation_detection(UnderInflationDetectionSystem::Indirect);
        frame.set_blind_spot_audible_assistance_presence(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 6];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_profile_number(UserProfile::Profile1);
        frame.set_profile_change_allowed(false);
        frame.set_boot_permanent_locking_presence(false);
        frame.set_partial_window_opening_presence(false);
        frame.set_welcome_function_presence(false);
        frame.set_securoscope_presence(false);
        frame.set_configurable_key_presence(true);
        frame.set_automatic_headlamps_presence(false);
        frame.set_gear_efficiency_indicator_presence(false);
        frame.set_auto_elec_parking_brake_application_presence(false);
        frame.set_welcome_lighting_presence(false);
        frame.set_follow_me_home_presence(true);
        frame.set_locking_mode_on_coe_presence(false);
        frame.set_auto_door_locking_when_leaving_presence(false);
        frame.set_selective_unlocking_presence(false);
        frame.set_rear_wiper_in_reverse_gear_presence(true);
        frame.set_daytime_running_lamps_presence(false);
        frame.set_adaptive_lamps_presence(true);
        frame.set_blind_spot_monitoring_inhibition_presence(false);
        frame.set_blind_spot_monitoring_presence(false);
        frame.set_mood_lighting_presence(false);
        frame.set_motorway_lighting_presence(false);
        frame.set_multi_function_display_presence(true);
        frame.set_park_sensors_inhibition_presence(false);
        frame.set_park_sensors_audible_assistance_presence(false);
        frame.set_park_sensors_visual_assistance_presence(false);
        frame.set_automatic_emergency_braking_presence(false);
        frame.set_under_inflation_detection_reset_menu_presence(false);
        frame.set_seat_belt_status_lamps_presence(false);
        frame.set_under_inflation_detection(UnderInflationDetectionSystem::DirectWithoutAbsolutePressure);
        frame.set_blind_spot_audible_assistance_presence(false);

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
