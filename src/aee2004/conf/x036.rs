use core::{cmp::Ordering, fmt, time::Duration};

use crate::{
    config::UserProfile,
    vehicle::{
        ConvertibleRoofPosition, DayNightStatus, HybridPowertrainMode, HybridPowertrainState,
        MainStatusValidity, NetworkState, RheostatMode,
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
    /// 4-bit driver memory setting number to apply field,
    /// 1-bit driver memory setting write to memory request flag,
    /// 1-bit driver memory setting recall request flag,
    /// 2-bit driver profile number field.
    pub const DRIVER_MEM: usize = 0;
    /// 4-bit passenger memory setting number to apply field,
    /// 1-bit passenger memory setting write to memory request flag,
    /// 1-bit passenger memory setting recall request flag,
    /// 2-bit passenger profile number field.
    pub const PASS_MEM: usize = 1;
    /// 5-bit 'délestage' level field,
    /// 2-bit empty,
    /// 1-bit economy mode enabled flag.
    pub const DELESTAGE_ECO: usize = 2;
    /// 4-bit lighting level field,
    /// 1-bit black panel enabled flag,
    /// 1-bit day/night status flag,
    /// 1-bit rheostat mode flag,
    /// 1-bit lighting reset to reference level flag.
    pub const LIGHTING: usize = 3;
    /// 3-bit network state field,
    /// 1-bit fault logging forbidden flag,
    /// 1-bit empty,
    /// 1-bit network supervision authorization flag,
    /// 1-bit fault erase request flag,
    /// 1-bit sport mode enabled flag.
    pub const NET_FLAGS: usize = 4;
    /// 1-bit hybrid powertrain mode updated data flag,
    /// 3-bit hybrid powertrain mode field,
    /// 1-bit hybrid powertrain state updated data flag,
    /// 3-bit hybrid powertrain state field.
    pub const HYBRID: usize = 5;
    /// 1-bit radio on/off synchronization flag,
    /// 1-bit radio on/off button toggle flag,
    /// 1-bit preconditioning menu presence flag,
    /// 1-bit visual parking assistance enable flag,
    /// 3-bit empty,
    /// 1-bit media shutdown request flag.
    pub const RADIO: usize = 6;
    /// 1-bit convertible roof position flag,
    /// 1-bit audio inviolability request flag,
    /// 2-bit empty,
    /// 4-bit vehicle main status value validity field.
    pub const FLAGS_MAIN_STATE: usize = 7;
}

/// Raw x036 CAN frame identifier.
pub const FRAME_ID: u16 = 0x036;
/// Length of a x036 CAN frame.
pub const FRAME_LEN: usize = field::FLAGS_MAIN_STATE + 1;

/// Periodicity of a x036 CAN frame.
pub const PERIODICITY: Duration = Duration::from_millis(100);

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

    /// Return the driver memory setting number to apply field.
    #[inline]
    pub fn driver_memory_setting(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::DRIVER_MEM] & 0x0f
    }

    /// Return the driver memory setting write to memory request flag.
    #[inline]
    pub fn driver_memory_setting_write(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::DRIVER_MEM] & 0x10 != 0
    }

    /// Return the driver memory setting recall request flag.
    #[inline]
    pub fn driver_memory_setting_recall(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::DRIVER_MEM] & 0x20 != 0
    }

    /// Return the driver profile number field.
    #[inline]
    pub fn driver_profile_number(&self) -> UserProfile {
        let data = self.buffer.as_ref();
        let raw = data[field::DRIVER_MEM] >> 6;
        UserProfile::from(raw)
    }

    /// Return the passenger memory setting number to apply field.
    #[inline]
    pub fn passenger_memory_setting(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::PASS_MEM] & 0x0f
    }

    /// Return the passenger memory setting write to memory request flag.
    #[inline]
    pub fn passenger_memory_setting_write(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::PASS_MEM] & 0x10 != 0
    }

    /// Return the passenger memory setting recall request flag.
    #[inline]
    pub fn passenger_memory_setting_recall(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::PASS_MEM] & 0x20 != 0
    }

    /// Return the passenger profile number field.
    #[inline]
    pub fn passenger_profile_number(&self) -> UserProfile {
        let data = self.buffer.as_ref();
        let raw = data[field::PASS_MEM] >> 6;
        UserProfile::from(raw)
    }

    /// Return the 'délestage' level field.
    #[inline]
    pub fn delestage_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::DELESTAGE_ECO] & 0x1f
    }

    /// Return the economy mode enabled flag.
    #[inline]
    pub fn economy_mode_enabled(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::DELESTAGE_ECO] & 0x80 != 0
    }

    /// Return the lighting level field.
    #[inline]
    pub fn lighting_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::LIGHTING] & 0x0f
    }

    /// Return the black panel enabled flag.
    #[inline]
    pub fn black_panel_enabled(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::LIGHTING] & 0x10 != 0
    }

    /// Return the day/night status flag.
    #[inline]
    pub fn day_night(&self) -> DayNightStatus {
        let data = self.buffer.as_ref();
        let raw = (data[field::LIGHTING] & 0x20) >> 5;
        DayNightStatus::from(raw)
    }

    /// Return the rheostat mode flag.
    #[inline]
    pub fn rheostat_mode(&self) -> RheostatMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::LIGHTING] & 0x40) >> 6;
        RheostatMode::from(raw)
    }

    /// Return the lighting reset to reference level request flag.
    #[inline]
    pub fn lighting_reset_to_reference_level_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::LIGHTING] & 0x80 != 0
    }

    /// Return the network state field.
    #[inline]
    pub fn network_state(&self) -> NetworkState {
        let data = self.buffer.as_ref();
        let raw = data[field::NET_FLAGS] & 0x07;
        NetworkState::from(raw)
    }

    /// Return the fault logging forbidden flag.
    #[inline]
    pub fn fault_logging_forbidden(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::NET_FLAGS] & 0x08 != 0
    }

    /// Return the network supervision authorization flag.
    #[inline]
    pub fn network_supervision_authorization(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::NET_FLAGS] & 0x20 != 0
    }

    /// Return the fault erase request flag.
    #[inline]
    pub fn fault_erase_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::NET_FLAGS] & 0x40 != 0
    }

    /// Return the sport mode enabled flag.
    #[inline]
    pub fn sport_mode_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::NET_FLAGS] & 0x80 != 0
    }

    /// Return the hybrid powertrain mode updated data flag.
    #[inline]
    pub fn hybrid_powertrain_mode_updated_data(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::HYBRID] & 0x01 != 0
    }

    /// Return the hybrid powertrain mode field.
    #[inline]
    pub fn hybrid_powertrain_mode(&self) -> HybridPowertrainMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::HYBRID] & 0x0e) >> 1;
        HybridPowertrainMode::from(raw)
    }

    /// Return the hybrid powertrain state updated data flag.
    #[inline]
    pub fn hybrid_powertrain_state_updated_data(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::HYBRID] & 0x10 != 0
    }

    /// Return the hybrid powertrain state field.
    #[inline]
    pub fn hybrid_powertrain_state(&self) -> HybridPowertrainState {
        let data = self.buffer.as_ref();
        let raw = (data[field::HYBRID] & 0xe0) >> 5;
        HybridPowertrainState::from(raw)
    }

    /// Return the radio on/off synchronization flag.
    #[inline]
    pub fn radio_on_off_synchronization(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::RADIO] & 0x01 != 0
    }

    /// Return the radio button toggle flag.
    #[inline]
    pub fn radio_on_off_toggle(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::RADIO] & 0x02 != 0
    }

    /// Return the preconditioning menu presence flag.
    #[inline]
    pub fn preconditioning_menu_presence(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::RADIO] & 0x04 != 0
    }

    /// Return the visual parking assistance enable flag.
    #[inline]
    pub fn visual_parking_assistance_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::RADIO] & 0x08 != 0
    }

    /// Return the media shutdown request flag.
    #[inline]
    pub fn media_shutdown_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::RADIO] & 0x80 != 0
    }

    /// Return the convertible roof position flag.
    #[inline]
    pub fn convertible_roof_position(&self) -> ConvertibleRoofPosition {
        let data = self.buffer.as_ref();
        let raw = data[field::FLAGS_MAIN_STATE] & 0x01;
        ConvertibleRoofPosition::from(raw)
    }

    /// Return the audio inviolability request flag.
    #[inline]
    pub fn audio_inviolability_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_MAIN_STATE] & 0x02 != 0
    }

    /// Return the vehicle main status value validity field.
    #[inline]
    pub fn vehicle_main_status_validity(&self) -> MainStatusValidity {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_MAIN_STATE] & 0xf0) >> 4;
        MainStatusValidity::from(raw)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the driver memory setting number to apply field.
    #[inline]
    pub fn set_driver_memory_setting(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::DRIVER_MEM] & !0x0f;
        let raw = raw | (value & 0x0f);
        data[field::DRIVER_MEM] = raw;
    }

    /// Set the driver memory setting write to memory request flag.
    #[inline]
    pub fn set_driver_memory_setting_write(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::DRIVER_MEM] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::DRIVER_MEM] = raw;
    }

    /// Set the driver memory setting recall to memory request flag.
    #[inline]
    pub fn set_driver_memory_setting_recall(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::DRIVER_MEM] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::DRIVER_MEM] = raw;
    }

    /// Set the driver profile number field.
    #[inline]
    pub fn set_driver_profile_number(&mut self, value: UserProfile) {
        let data = self.buffer.as_mut();
        let raw = data[field::DRIVER_MEM] & !0xc0;
        let raw = raw | (u8::from(value) << 6);
        data[field::DRIVER_MEM] = raw;
    }

    /// Set the passenger memory setting number to apply field.
    #[inline]
    pub fn set_passenger_memory_setting(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::PASS_MEM] & !0x0f;
        let raw = raw | (value & 0x0f);
        data[field::PASS_MEM] = raw;
    }

    /// Set the passenger memory setting write to memory request flag.
    #[inline]
    pub fn set_passenger_memory_setting_write(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::PASS_MEM] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::PASS_MEM] = raw;
    }

    /// Set the passenger memory setting recall to memory request flag.
    #[inline]
    pub fn set_passenger_memory_setting_recall(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::PASS_MEM] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::PASS_MEM] = raw;
    }

    /// Set the passenger profile number field.
    #[inline]
    pub fn set_passenger_profile_number(&mut self, value: UserProfile) {
        let data = self.buffer.as_mut();
        let raw = data[field::PASS_MEM] & !0xc0;
        let raw = raw | (u8::from(value) << 6);
        data[field::PASS_MEM] = raw;
    }

    /// Set the 'délestage' level field.
    #[inline]
    pub fn set_delestage_level(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::DELESTAGE_ECO] & !0x1f;
        let raw = raw | (value & 0x1f);
        data[field::DELESTAGE_ECO] = raw;
    }

    /// Set the economy mode enabled flag.
    #[inline]
    pub fn set_economy_mode_enabled(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::DELESTAGE_ECO] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::DELESTAGE_ECO] = raw;
    }

    /// Set the lighting level field.
    #[inline]
    pub fn set_lighting_level(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::LIGHTING] & !0x0f;
        let raw = raw | (value & 0x0f);
        data[field::LIGHTING] = raw;
    }

    /// Set the black panel enabled flag.
    #[inline]
    pub fn set_black_panel_enabled(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::LIGHTING] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::LIGHTING] = raw;
    }

    /// Set the day/night status flag.
    #[inline]
    pub fn set_day_night(&mut self, value: DayNightStatus) {
        let data = self.buffer.as_mut();
        let raw = data[field::LIGHTING] & !0x20;
        let raw = raw | ((u8::from(value) << 5) & 0x20);
        data[field::LIGHTING] = raw;
    }

    /// Set the rheostat mode flag.
    #[inline]
    pub fn set_rheostat_mode(&mut self, value: RheostatMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::LIGHTING] & !0x40;
        let raw = raw | ((u8::from(value) << 6) & 0x40);
        data[field::LIGHTING] = raw;
    }

    /// Set the lighting reset to reference level request flag.
    #[inline]
    pub fn set_lighting_reset_to_reference_level_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::LIGHTING] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::LIGHTING] = raw;
    }

    /// Set the network state field.
    #[inline]
    pub fn set_network_state(&mut self, value: NetworkState) {
        let data = self.buffer.as_mut();
        let raw = data[field::NET_FLAGS] & !0x07;
        let raw = raw | (u8::from(value) & 0x07);
        data[field::NET_FLAGS] = raw;
    }

    /// Set the fault logging forbidden flag.
    #[inline]
    pub fn set_fault_logging_forbidden(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::NET_FLAGS] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::NET_FLAGS] = raw;
    }

    /// Set the network supervision authorization flag.
    #[inline]
    pub fn set_network_supervision_authorization(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::NET_FLAGS] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::NET_FLAGS] = raw;
    }

    /// Set the fault erase request flag.
    #[inline]
    pub fn set_fault_erase_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::NET_FLAGS] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::NET_FLAGS] = raw;
    }

    /// Set the sport mode enabled flag.
    #[inline]
    pub fn set_sport_mode_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::NET_FLAGS] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::NET_FLAGS] = raw;
    }

    /// Set the hybrid powertrain mode updated data flag.
    #[inline]
    pub fn set_hybrid_powertrain_mode_updated_data(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::HYBRID] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::HYBRID] = raw;
    }

    /// Set the hybrid powertrain mode field.
    #[inline]
    pub fn set_hybrid_powertrain_mode(&mut self, value: HybridPowertrainMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::HYBRID] & !0x0e;
        let raw = raw | ((u8::from(value) << 1) & 0x0e);
        data[field::HYBRID] = raw;
    }

    /// Set the hybrid powertrain state updated data flag.
    #[inline]
    pub fn set_hybrid_powertrain_state_updated_data(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::HYBRID] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::HYBRID] = raw;
    }

    /// Set the hybrid powertrain state field.
    #[inline]
    pub fn set_hybrid_powertrain_state(&mut self, value: HybridPowertrainState) {
        let data = self.buffer.as_mut();
        let raw = data[field::HYBRID] & !0xe0;
        let raw = raw | ((u8::from(value) << 5) & 0xe0);
        data[field::HYBRID] = raw;
    }

    /// Set the radio on/off synchronization flag.
    #[inline]
    pub fn set_radio_on_off_synchronization(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::RADIO] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::RADIO] = raw;
    }

    /// Set the radio button toggle flag.
    #[inline]
    pub fn set_radio_on_off_toggle(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::RADIO] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::RADIO] = raw;
    }

    /// Set the preconditioning menu presence flag.
    #[inline]
    pub fn set_preconditioning_menu_presence(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::RADIO] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::RADIO] = raw;
    }

    /// Set the visual parking assistance enable flag.
    #[inline]
    pub fn set_visual_parking_assistance_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::RADIO] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::RADIO] = raw;
    }

    /// Set the media shutdown request flag.
    #[inline]
    pub fn set_media_shutdown_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::RADIO] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::RADIO] = raw;
    }

    /// Set the convertible roof position flag.
    #[inline]
    pub fn set_convertible_roof_position(&mut self, value: ConvertibleRoofPosition) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_MAIN_STATE] & !0x01;
        let raw = raw | (u8::from(value) & 0x01);
        data[field::FLAGS_MAIN_STATE] = raw;
    }

    /// Set the audio inviolability request flag.
    #[inline]
    pub fn set_audio_inviolability_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_MAIN_STATE] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::FLAGS_MAIN_STATE] = raw;
    }

    /// Set the vehicle main status value validity field.
    #[inline]
    pub fn set_vehicle_main_status_validity(&mut self, value: MainStatusValidity) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_MAIN_STATE] & !0xf0;
        let raw = raw | ((u8::from(value) << 4) & 0xf0);
        data[field::FLAGS_MAIN_STATE] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x036 ({})", err)?;
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

/// A high-level representation of a x036 CAN frame.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub driver_memory_setting: u8,
    pub driver_memory_setting_write: bool,
    pub driver_memory_setting_recall: bool,
    pub driver_profile_number: UserProfile,
    pub passenger_memory_setting: u8,
    pub passenger_memory_setting_write: bool,
    pub passenger_memory_setting_recall: bool,
    pub passenger_profile_number: UserProfile,
    pub delestage_level: u8,
    pub economy_mode_enabled: bool,
    pub lighting_level: u8,
    pub black_panel_enabled: bool,
    pub day_night: DayNightStatus,
    pub rheostat_mode: RheostatMode,
    pub lighting_reset_to_reference_level_request: bool,
    pub network_state: NetworkState,
    pub fault_logging_forbidden: bool,
    pub network_supervision_authorization: bool,
    pub fault_erase_request: bool,
    pub sport_mode_enable: bool,
    pub hybrid_powertrain_mode_updated_data: bool,
    pub hybrid_powertrain_mode: HybridPowertrainMode,
    pub hybrid_powertrain_state_updated_data: bool,
    pub hybrid_powertrain_state: HybridPowertrainState,
    pub radio_on_off_synchronization: bool,
    pub radio_on_off_toggle: bool,
    pub preconditioning_menu_presence: bool,
    pub visual_parking_assistance_enable: bool,
    pub media_shutdown_request: bool,
    pub convertible_roof_position: ConvertibleRoofPosition,
    pub audio_inviolability_request: bool,
    pub vehicle_main_status_validity: MainStatusValidity,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            driver_memory_setting: frame.driver_memory_setting(),
            driver_memory_setting_write: frame.driver_memory_setting_write(),
            driver_memory_setting_recall: frame.driver_memory_setting_recall(),
            driver_profile_number: frame.driver_profile_number(),
            passenger_memory_setting: frame.passenger_memory_setting(),
            passenger_memory_setting_write: frame.passenger_memory_setting_write(),
            passenger_memory_setting_recall: frame.passenger_memory_setting_recall(),
            passenger_profile_number: frame.passenger_profile_number(),
            delestage_level: frame.delestage_level(),
            economy_mode_enabled: frame.economy_mode_enabled(),
            lighting_level: frame.lighting_level(),
            black_panel_enabled: frame.black_panel_enabled(),
            day_night: frame.day_night(),
            rheostat_mode: frame.rheostat_mode(),
            lighting_reset_to_reference_level_request: frame
                .lighting_reset_to_reference_level_request(),
            network_state: frame.network_state(),
            fault_logging_forbidden: frame.fault_logging_forbidden(),
            network_supervision_authorization: frame.network_supervision_authorization(),
            fault_erase_request: frame.fault_erase_request(),
            sport_mode_enable: frame.sport_mode_enable(),
            hybrid_powertrain_mode_updated_data: frame.hybrid_powertrain_mode_updated_data(),
            hybrid_powertrain_mode: frame.hybrid_powertrain_mode(),
            hybrid_powertrain_state_updated_data: frame.hybrid_powertrain_state_updated_data(),
            hybrid_powertrain_state: frame.hybrid_powertrain_state(),
            radio_on_off_synchronization: frame.radio_on_off_synchronization(),
            radio_on_off_toggle: frame.radio_on_off_toggle(),
            preconditioning_menu_presence: frame.preconditioning_menu_presence(),
            visual_parking_assistance_enable: frame.visual_parking_assistance_enable(),
            media_shutdown_request: frame.media_shutdown_request(),
            convertible_roof_position: frame.convertible_roof_position(),
            audio_inviolability_request: frame.audio_inviolability_request(),
            vehicle_main_status_validity: frame.vehicle_main_status_validity(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x036 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_driver_memory_setting(self.driver_memory_setting);
        frame.set_driver_memory_setting_write(self.driver_memory_setting_write);
        frame.set_driver_memory_setting_recall(self.driver_memory_setting_recall);
        frame.set_driver_profile_number(self.driver_profile_number);
        frame.set_passenger_memory_setting(self.passenger_memory_setting);
        frame.set_passenger_memory_setting_write(self.passenger_memory_setting_write);
        frame.set_passenger_memory_setting_recall(self.passenger_memory_setting_recall);
        frame.set_passenger_profile_number(self.passenger_profile_number);
        frame.set_delestage_level(self.delestage_level);
        frame.set_economy_mode_enabled(self.economy_mode_enabled);
        frame.set_lighting_level(self.lighting_level);
        frame.set_black_panel_enabled(self.black_panel_enabled);
        frame.set_day_night(self.day_night);
        frame.set_rheostat_mode(self.rheostat_mode);
        frame.set_lighting_reset_to_reference_level_request(
            self.lighting_reset_to_reference_level_request,
        );
        frame.set_network_state(self.network_state);
        frame.set_fault_logging_forbidden(self.fault_logging_forbidden);
        frame.set_network_supervision_authorization(self.network_supervision_authorization);
        frame.set_fault_erase_request(self.fault_erase_request);
        frame.set_sport_mode_enable(self.sport_mode_enable);
        frame.set_hybrid_powertrain_mode_updated_data(self.hybrid_powertrain_mode_updated_data);
        frame.set_hybrid_powertrain_mode(self.hybrid_powertrain_mode);
        frame.set_hybrid_powertrain_state_updated_data(self.hybrid_powertrain_state_updated_data);
        frame.set_hybrid_powertrain_state(self.hybrid_powertrain_state);
        frame.set_radio_on_off_synchronization(self.radio_on_off_synchronization);
        frame.set_radio_on_off_toggle(self.radio_on_off_toggle);
        frame.set_preconditioning_menu_presence(self.preconditioning_menu_presence);
        frame.set_visual_parking_assistance_enable(self.visual_parking_assistance_enable);
        frame.set_media_shutdown_request(self.media_shutdown_request);
        frame.set_convertible_roof_position(self.convertible_roof_position);
        frame.set_audio_inviolability_request(self.audio_inviolability_request);
        frame.set_vehicle_main_status_validity(self.vehicle_main_status_validity);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x036")?;
        writeln!(f, " driver_memory_setting={}", self.driver_memory_setting)?;
        writeln!(
            f,
            " driver_memory_setting_write={}",
            self.driver_memory_setting_write
        )?;
        writeln!(
            f,
            " driver_memory_setting_recall={}",
            self.driver_memory_setting_recall
        )?;
        writeln!(f, " driver_profile_number={}", self.driver_profile_number)?;
        writeln!(
            f,
            " passenger_memory_setting={}",
            self.passenger_memory_setting
        )?;
        writeln!(
            f,
            " passenger_memory_setting_write={}",
            self.driver_memory_setting_write
        )?;
        writeln!(
            f,
            " passenger_memory_setting_recall={}",
            self.passenger_memory_setting_recall
        )?;
        writeln!(
            f,
            " passenger_profile_number={}",
            self.passenger_profile_number
        )?;
        writeln!(f, " delestage_level={}", self.delestage_level)?;
        writeln!(f, " economy_mode_enabled={}", self.economy_mode_enabled)?;
        writeln!(f, " lighting_level={}", self.lighting_level)?;
        writeln!(f, " black_panel_enabled={}", self.black_panel_enabled)?;
        writeln!(f, " day_night={}", self.day_night)?;
        writeln!(f, " rheostat_mode={}", self.rheostat_mode)?;
        writeln!(
            f,
            " lighting_reset_to_reference_level_request={}",
            self.lighting_reset_to_reference_level_request
        )?;
        writeln!(f, " network_state={}", self.network_state)?;
        writeln!(
            f,
            " fault_logging_forbidden={}",
            self.fault_logging_forbidden
        )?;
        writeln!(
            f,
            " network_supervision_authorization={}",
            self.network_supervision_authorization
        )?;
        writeln!(f, " fault_erase_request={}", self.fault_erase_request)?;
        writeln!(f, " sport_mode_enable={}", self.sport_mode_enable)?;
        writeln!(
            f,
            " hybrid_powertrain_mode_updated_data={}",
            self.hybrid_powertrain_mode_updated_data
        )?;
        writeln!(f, " hybrid_powertrain_mode={}", self.hybrid_powertrain_mode)?;
        writeln!(
            f,
            " hybrid_powertrain_state_updated_data={}",
            self.hybrid_powertrain_state_updated_data
        )?;
        writeln!(
            f,
            " hybrid_powertrain_state={}",
            self.hybrid_powertrain_state
        )?;
        writeln!(
            f,
            " radio_on_off_synchronization={}",
            self.radio_on_off_synchronization
        )?;
        writeln!(f, " radio_on_off_toggle={}", self.radio_on_off_toggle)?;
        writeln!(
            f,
            " preconditioning_menu_presence={}",
            self.preconditioning_menu_presence
        )?;
        writeln!(
            f,
            " visual_parking_assistance_enable={}",
            self.visual_parking_assistance_enable
        )?;
        writeln!(f, " media_shutdown_request={}", self.media_shutdown_request)?;
        writeln!(
            f,
            " convertible_roof_position={}",
            self.convertible_roof_position
        )?;
        writeln!(
            f,
            " audio_inviolability_request={}",
            self.audio_inviolability_request
        )?;
        writeln!(
            f,
            " vehicle_main_status_validity={}",
            self.vehicle_main_status_validity
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        config::UserProfile,
        vehicle::{
            ConvertibleRoofPosition, DayNightStatus, HybridPowertrainMode, HybridPowertrainState,
            MainStatusValidity, NetworkState, RheostatMode,
        },
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x51, 0x51, 0x88, 0xc8, 0xa1, 0xb0, 0x0a, 0xa2];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0xa3, 0xa3, 0x08, 0x38, 0x4c, 0x83, 0x85, 0xa1];

    fn frame_1_repr() -> Repr {
        Repr {
            driver_memory_setting: 1,
            driver_memory_setting_write: true,
            driver_memory_setting_recall: false,
            driver_profile_number: UserProfile::Profile1,
            passenger_memory_setting: 1,
            passenger_memory_setting_write: true,
            passenger_memory_setting_recall: false,
            passenger_profile_number: UserProfile::Profile1,
            delestage_level: 8,
            economy_mode_enabled: true,
            lighting_level: 8,
            black_panel_enabled: false,
            day_night: DayNightStatus::Day,
            rheostat_mode: RheostatMode::Automatic,
            lighting_reset_to_reference_level_request: true,
            network_state: NetworkState::Normal,
            fault_logging_forbidden: false,
            network_supervision_authorization: true,
            fault_erase_request: false,
            sport_mode_enable: true,
            hybrid_powertrain_mode_updated_data: false,
            hybrid_powertrain_mode: HybridPowertrainMode::FourWheelDrive,
            hybrid_powertrain_state_updated_data: true,
            hybrid_powertrain_state: HybridPowertrainState::Hybrid,
            radio_on_off_synchronization: false,
            radio_on_off_toggle: true,
            preconditioning_menu_presence: false,
            visual_parking_assistance_enable: true,
            media_shutdown_request: false,
            convertible_roof_position: ConvertibleRoofPosition::Coupe,
            audio_inviolability_request: true,
            vehicle_main_status_validity: MainStatusValidity::Valid,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            driver_memory_setting: 3,
            driver_memory_setting_write: false,
            driver_memory_setting_recall: true,
            driver_profile_number: UserProfile::Profile2,
            passenger_memory_setting: 3,
            passenger_memory_setting_write: false,
            passenger_memory_setting_recall: true,
            passenger_profile_number: UserProfile::Profile2,
            delestage_level: 8,
            economy_mode_enabled: false,
            lighting_level: 8,
            black_panel_enabled: true,
            day_night: DayNightStatus::Night,
            rheostat_mode: RheostatMode::Manual,
            lighting_reset_to_reference_level_request: false,
            network_state: NetworkState::Off,
            fault_logging_forbidden: true,
            network_supervision_authorization: false,
            fault_erase_request: true,
            sport_mode_enable: false,
            hybrid_powertrain_mode_updated_data: true,
            hybrid_powertrain_mode: HybridPowertrainMode::Sport,
            hybrid_powertrain_state_updated_data: false,
            hybrid_powertrain_state: HybridPowertrainState::RearWheelDrive,
            radio_on_off_synchronization: true,
            radio_on_off_toggle: false,
            preconditioning_menu_presence: true,
            visual_parking_assistance_enable: false,
            media_shutdown_request: true,
            convertible_roof_position: ConvertibleRoofPosition::Convertible,
            audio_inviolability_request: false,
            vehicle_main_status_validity: MainStatusValidity::Valid,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.driver_memory_setting(), 1);
        assert_eq!(frame.driver_memory_setting_write(), true);
        assert_eq!(frame.driver_memory_setting_recall(), false);
        assert_eq!(frame.driver_profile_number(), UserProfile::Profile1);
        assert_eq!(frame.passenger_memory_setting(), 1);
        assert_eq!(frame.passenger_memory_setting_write(), true);
        assert_eq!(frame.passenger_memory_setting_recall(), false);
        assert_eq!(frame.passenger_profile_number(), UserProfile::Profile1);
        assert_eq!(frame.delestage_level(), 8);
        assert_eq!(frame.economy_mode_enabled(), true);
        assert_eq!(frame.lighting_level(), 8);
        assert_eq!(frame.black_panel_enabled(), false);
        assert_eq!(frame.day_night(), DayNightStatus::Day);
        assert_eq!(frame.rheostat_mode(), RheostatMode::Automatic);
        assert_eq!(frame.lighting_reset_to_reference_level_request(), true);
        assert_eq!(frame.network_state(), NetworkState::Normal);
        assert_eq!(frame.fault_logging_forbidden(), false);
        assert_eq!(frame.network_supervision_authorization(), true);
        assert_eq!(frame.fault_erase_request(), false);
        assert_eq!(frame.sport_mode_enable(), true);
        assert_eq!(frame.hybrid_powertrain_mode_updated_data(), false);
        assert_eq!(
            frame.hybrid_powertrain_mode(),
            HybridPowertrainMode::FourWheelDrive
        );
        assert_eq!(frame.hybrid_powertrain_state_updated_data(), true);
        assert_eq!(
            frame.hybrid_powertrain_state(),
            HybridPowertrainState::Hybrid
        );
        assert_eq!(frame.radio_on_off_synchronization(), false);
        assert_eq!(frame.radio_on_off_toggle(), true);
        assert_eq!(frame.preconditioning_menu_presence(), false);
        assert_eq!(frame.visual_parking_assistance_enable(), true);
        assert_eq!(frame.media_shutdown_request(), false);
        assert_eq!(
            frame.convertible_roof_position(),
            ConvertibleRoofPosition::Coupe
        );
        assert_eq!(frame.audio_inviolability_request(), true);
        assert_eq!(
            frame.vehicle_main_status_validity(),
            MainStatusValidity::Valid
        );
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.driver_memory_setting(), 3);
        assert_eq!(frame.driver_memory_setting_write(), false);
        assert_eq!(frame.driver_memory_setting_recall(), true);
        assert_eq!(frame.driver_profile_number(), UserProfile::Profile2);
        assert_eq!(frame.passenger_memory_setting(), 3);
        assert_eq!(frame.passenger_memory_setting_write(), false);
        assert_eq!(frame.passenger_memory_setting_recall(), true);
        assert_eq!(frame.passenger_profile_number(), UserProfile::Profile2);
        assert_eq!(frame.delestage_level(), 8);
        assert_eq!(frame.economy_mode_enabled(), false);
        assert_eq!(frame.lighting_level(), 8);
        assert_eq!(frame.black_panel_enabled(), true);
        assert_eq!(frame.day_night(), DayNightStatus::Night);
        assert_eq!(frame.rheostat_mode(), RheostatMode::Manual);
        assert_eq!(frame.lighting_reset_to_reference_level_request(), false);
        assert_eq!(frame.network_state(), NetworkState::Off);
        assert_eq!(frame.fault_logging_forbidden(), true);
        assert_eq!(frame.network_supervision_authorization(), false);
        assert_eq!(frame.fault_erase_request(), true);
        assert_eq!(frame.sport_mode_enable(), false);
        assert_eq!(frame.hybrid_powertrain_mode_updated_data(), true);
        assert_eq!(frame.hybrid_powertrain_mode(), HybridPowertrainMode::Sport);
        assert_eq!(frame.hybrid_powertrain_state_updated_data(), false);
        assert_eq!(
            frame.hybrid_powertrain_state(),
            HybridPowertrainState::RearWheelDrive
        );
        assert_eq!(frame.radio_on_off_synchronization(), true);
        assert_eq!(frame.radio_on_off_toggle(), false);
        assert_eq!(frame.preconditioning_menu_presence(), true);
        assert_eq!(frame.visual_parking_assistance_enable(), false);
        assert_eq!(frame.media_shutdown_request(), true);
        assert_eq!(
            frame.convertible_roof_position(),
            ConvertibleRoofPosition::Convertible
        );
        assert_eq!(frame.audio_inviolability_request(), false);
        assert_eq!(
            frame.vehicle_main_status_validity(),
            MainStatusValidity::Valid
        );
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_driver_memory_setting(1);
        frame.set_driver_memory_setting_write(true);
        frame.set_driver_memory_setting_recall(false);
        frame.set_driver_profile_number(UserProfile::Profile1);
        frame.set_passenger_memory_setting(1);
        frame.set_passenger_memory_setting_write(true);
        frame.set_passenger_memory_setting_recall(false);
        frame.set_passenger_profile_number(UserProfile::Profile1);
        frame.set_delestage_level(8);
        frame.set_economy_mode_enabled(true);
        frame.set_lighting_level(8);
        frame.set_black_panel_enabled(false);
        frame.set_day_night(DayNightStatus::Day);
        frame.set_rheostat_mode(RheostatMode::Automatic);
        frame.set_lighting_reset_to_reference_level_request(true);
        frame.set_network_state(NetworkState::Normal);
        frame.set_fault_logging_forbidden(false);
        frame.set_network_supervision_authorization(true);
        frame.set_fault_erase_request(false);
        frame.set_sport_mode_enable(true);
        frame.set_hybrid_powertrain_mode_updated_data(false);
        frame.set_hybrid_powertrain_mode(HybridPowertrainMode::FourWheelDrive);
        frame.set_hybrid_powertrain_state_updated_data(true);
        frame.set_hybrid_powertrain_state(HybridPowertrainState::Hybrid);
        frame.set_radio_on_off_synchronization(false);
        frame.set_radio_on_off_toggle(true);
        frame.set_preconditioning_menu_presence(false);
        frame.set_visual_parking_assistance_enable(true);
        frame.set_media_shutdown_request(false);
        frame.set_convertible_roof_position(ConvertibleRoofPosition::Coupe);
        frame.set_audio_inviolability_request(true);
        frame.set_vehicle_main_status_validity(MainStatusValidity::Valid);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_driver_memory_setting(3);
        frame.set_driver_memory_setting_write(false);
        frame.set_driver_memory_setting_recall(true);
        frame.set_driver_profile_number(UserProfile::Profile2);
        frame.set_passenger_memory_setting(3);
        frame.set_passenger_memory_setting_write(false);
        frame.set_passenger_memory_setting_recall(true);
        frame.set_passenger_profile_number(UserProfile::Profile2);
        frame.set_delestage_level(8);
        frame.set_economy_mode_enabled(false);
        frame.set_lighting_level(8);
        frame.set_black_panel_enabled(true);
        frame.set_day_night(DayNightStatus::Night);
        frame.set_rheostat_mode(RheostatMode::Manual);
        frame.set_lighting_reset_to_reference_level_request(false);
        frame.set_network_state(NetworkState::Off);
        frame.set_fault_logging_forbidden(true);
        frame.set_network_supervision_authorization(false);
        frame.set_fault_erase_request(true);
        frame.set_sport_mode_enable(false);
        frame.set_hybrid_powertrain_mode_updated_data(true);
        frame.set_hybrid_powertrain_mode(HybridPowertrainMode::Sport);
        frame.set_hybrid_powertrain_state_updated_data(false);
        frame.set_hybrid_powertrain_state(HybridPowertrainState::RearWheelDrive);
        frame.set_radio_on_off_synchronization(true);
        frame.set_radio_on_off_toggle(false);
        frame.set_preconditioning_menu_presence(true);
        frame.set_visual_parking_assistance_enable(false);
        frame.set_media_shutdown_request(true);
        frame.set_convertible_roof_position(ConvertibleRoofPosition::Convertible);
        frame.set_audio_inviolability_request(false);
        frame.set_vehicle_main_status_validity(MainStatusValidity::Valid);

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
