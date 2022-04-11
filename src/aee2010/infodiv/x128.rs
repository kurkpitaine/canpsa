use core::fmt;

use crate::{
    vehicle::{
        AdBlueIndicatorState, AutoGearboxMode, IndicatorState,
        GearEfficiencyArrowType, GearboxDriveModeGear, GearboxGear, GearboxType,
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
128 CDE_CMB_SIGNALISATION_ABPI_HS7_128              // OK
128 CDE_CMB_SIGNALISATION_AFF_RAP_CLIGN_HS7_128     // OK
128 CDE_CMB_SIGNALISATION_ALLUM_FLECHE_HS7_128      // OK
128 CDE_CMB_SIGNALISATION_CLIGNO_D_HS7_128          // OK
128 CDE_CMB_SIGNALISATION_CLIGNO_G_HS7_128          // OK
128 CDE_CMB_SIGNALISATION_CMD_WARNING_HS7_128       // OK
128 CDE_CMB_SIGNALISATION_DEFAUT_MOTEUR_HS7_128
128 CDE_CMB_SIGNALISATION_DMD_ALLUM_SCR_HS7_128     // OK
128 CDE_CMB_SIGNALISATION_DMD_COULEUR_CMB_HS7_128
128 CDE_CMB_SIGNALISATION_DMD_PERSO_CMB_HS7_128
128 CDE_CMB_SIGNALISATION_ESPACT_HS7_128            // OK
128 CDE_CMB_SIGNALISATION_ESPI_HS7_128              // OK
128 CDE_CMB_SIGNALISATION_FEUX_ABAR_HS7_128         // OK
128 CDE_CMB_SIGNALISATION_FEUX_ABAV_HS7_128         // OK
128 CDE_CMB_SIGNALISATION_FEUX_CAFR_HS7_128         // OK
128 CDE_CMB_SIGNALISATION_FEUX_CROIS_HS7_128        // OK
128 CDE_CMB_SIGNALISATION_FEUX_DIURNES_HS7_128      // OK
128 CDE_CMB_SIGNALISATION_FEUX_POS_HS7_128          // OK
128 CDE_CMB_SIGNALISATION_FEUX_ROUTE_HS7_128        // OK
128 CDE_CMB_SIGNALISATION_FRPK_HS7_128              // OK
128 CDE_CMB_SIGNALISATION_FSE_INHIB_HS7_128         // OK
128 CDE_CMB_SIGNALISATION_MINC_CLIG_HS7_128         // OK
128 CDE_CMB_SIGNALISATION_MINC_HS7_128              // OK
128 CDE_CMB_SIGNALISATION_MODE_BVA_BVMP_HS7_128     // OK
128 CDE_CMB_SIGNALISATION_MPD_CLIG_HS7_128          // OK
128 CDE_CMB_SIGNALISATION_MPD_HS7_128               // OK
128 CDE_CMB_SIGNALISATION_ON_CMB_HS7_128            // OK
128 CDE_CMB_SIGNALISATION_OUCARD_CLIG_HS7_128       // OK
128 CDE_CMB_SIGNALISATION_OUCARD_HS7_128            // OK
128 CDE_CMB_SIGNALISATION_OUCARG_CLIG_HS7_128       // OK
128 CDE_CMB_SIGNALISATION_OUCARG_HS7_128            // OK
128 CDE_CMB_SIGNALISATION_OUCARM_CLIG_HS7_128       // OK
128 CDE_CMB_SIGNALISATION_OUCARM_HS7_128            // OK
128 CDE_CMB_SIGNALISATION_OUCC_CLIG_HS7_128         // OK
128 CDE_CMB_SIGNALISATION_OUCC_HS7_128              // OK
128 CDE_CMB_SIGNALISATION_OUCP_CLIG_HS7_128         // OK
128 CDE_CMB_SIGNALISATION_OUCP_HS7_128              // OK
128 CDE_CMB_SIGNALISATION_OUVRANT_OUVERT_HS7_128    // OK
128 CDE_CMB_SIGNALISATION_PIED_EMBR_HS7_128
128 CDE_CMB_SIGNALISATION_PIED_FREIN_HS7_128        // OK
128 CDE_CMB_SIGNALISATION_PRE_CHAUFF_HS7_128        // OK
128 CDE_CMB_SIGNALISATION_PROT_PASS_AV_ACT_HS7_128  // OK
128 CDE_CMB_SIGNALISATION_RAP_AFF_CMB_HS7_128       // OK
128 CDE_CMB_SIGNALISATION_RAP_AFF_DRIVE_HS7_128     // OK
128 CDE_CMB_SIGNALISATION_READY_HY_HS7_128
128 CDE_CMB_SIGNALISATION_SECE_ACT_HS7_128          // OK
128 CDE_CMB_SIGNALISATION_SEL_BVA_BVM_BVMP_HS7_128  // OK
128 CDE_CMB_SIGNALISATION_SERVICE_HS7_128           // OK
128 CDE_CMB_SIGNALISATION_STOP_HS7_128              // OK
128 CDE_CMB_SIGNALISATION_SUSP_HS7_128              // OK
128 CDE_CMB_SIGNALISATION_TYPE_ALLUM_FLECHE_HS7_128 // OK
*/

mod field {
    /// 1-bit daytime running lamps indicator display flag,
    /// 1-bit left blinker indicator display flag,
    /// 1-bit right blinker indicator display flag,
    /// 1-bit rear anti-fog light indicator display flag,
    /// 1-bit front anti-fog light indicator display flag,
    /// 1-bit main beam light indicator display flag,
    /// 1-bit headlamps indicator display flag,
    /// 1-bit sidelights indicator display flag.
    pub const FLAGS_1: usize = 0;
    /// 1-bit displayed gear blinking flag,
    /// 3-bit gearbox drive mode engaged gear field,
    /// 4-bit gearbox gear to display field.
    pub const FLAGS_2: usize = 1;
    /// 2-bit gearbox type field,
    /// 2-bit gear efficiency indicator arrow type field,
    /// 3-bit automatic gearbox mode field,
    /// 1-bit gear efficiency indicator arrow blinking flag.
    pub const FLAGS_3: usize = 2;
    /// 1-bit automatic parking brake inhibition flag,
    /// 1-bit parking brake applied flag,
    /// 2-bit foot on brake pedal indicator state field,
    /// 1-bit passenger airbag inhibited flag,
    /// 1-bit child lock security enabled flag,
    /// 1-bit stop indicator display flag,
    /// 1-bit service indicator display flag.
    pub const FLAGS_4: usize = 3;
    /// 1-bit suspension indicator enabled flag,
    /// 1-bit ESP in regulation flag,
    /// 1-bit ESP inhibited flag,
    /// 1-bit automatic main beam enabled flag,
    /// 1-bit available space measurement indicator blinking flag,
    /// 1-bit available space measurement indicator display flag,
    /// 1-bit opened door/boot flag,
    /// 1-bit diesel engine pre-heating flag.
    pub const FLAGS_5: usize = 4;
    /// 1-bit rear left seat belt indicator display flag,
    /// 2-bit AdBlue indicator state field,
    /// 1-bit passenger seat belt indicator blinking flag,
    /// 1-bit passenger seat belt indicator display flag,
    /// 1-bit driver seat belt indicator blinking flag,
    /// 1-bit driver seat belt indicator display flag,
    /// 1-bit low fuel level flag.
    pub const FLAGS_6: usize = 5;
    /// 1-bit passenger protection flag,
    /// 1-bit hazard warning lights enabled flag,
    /// 1-bit instrument cluster ON flag,
    /// 1-bit rear right seat belt indicator blinking flag,
    /// 1-bit rear right seat belt indicator display flag,
    /// 1-bit rear middle seat belt indicator blinking flag,
    /// 1-bit rear middle seat belt indicator display flag,
    /// 1-bit rear left seat belt indicator blinking flag.
    pub const FLAGS_7: usize = 6;
    /// 3-bit unknown,
    /// 1-bit low fuel level indicator blinking,
    /// 4-bit unknown.
    pub const FLAGS_8: usize = 7;
}

/// Length of a x128 CAN frame.
pub const FRAME_LEN: usize = field::FLAGS_8 + 1;

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

    /// Return the bit in byte B at index I.
    #[inline]
    pub fn read_bit<const B: usize, const I: u8>(&self) -> bool {
        let data = self.buffer.as_ref();
        (data[B] & (1u8 << I)) != 0
    }

    /// Return the gearbox drive mode engaged gear field.
    #[inline]
    pub fn gearbox_drive_mode_gear(&self) -> GearboxDriveModeGear {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_2] & 0x0e) >> 1;
        GearboxDriveModeGear::from(raw)
    }

    /// Return the gearbox gear to display field.
    #[inline]
    pub fn gearbox_gear(&self) -> GearboxGear {
        let data = self.buffer.as_ref();
        let raw = data[field::FLAGS_2] >> 4;
        GearboxGear::from(raw)
    }

    /// Return the gearbox type field.
    #[inline]
    pub fn gearbox_type(&self) -> GearboxType {
        let data = self.buffer.as_ref();
        let raw = data[field::FLAGS_3] & 0x03;
        GearboxType::from(raw)
    }

    /// Return the gear efficiency indicator arrow type field.
    #[inline]
    pub fn gear_efficiency_indicator_arrow_type(&self) -> GearEfficiencyArrowType {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_3] & 0x0c) >> 2;
        GearEfficiencyArrowType::from(raw)
    }

    /// Return the automatic gearbox mode field.
    #[inline]
    pub fn automatic_gearbox_mode(&self) -> AutoGearboxMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_3] & 0x70) >> 4;
        AutoGearboxMode::from(raw)
    }

    /// Return the foot on brake pedal indicator state field.
    #[inline]
    pub fn foot_on_brake_pedal_indicator(&self) -> IndicatorState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_4] & 0x0c) >> 2;
        IndicatorState::from(raw)
    }

    /// Return the AdBlue indicator state field.
    #[inline]
    pub fn adblue_indicator(&self) -> AdBlueIndicatorState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_6] & 0x06) >> 1;
        AdBlueIndicatorState::from(raw)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the bit in byte B at index I.
    #[inline]
    pub fn write_bit<const B: usize, const I: u8>(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let mask = 1u8 << I;
        let raw = data[B];
        let raw = if value { raw | mask } else { raw & !mask };
        data[B] = raw;
    }

    /// Set the gearbox drive mode engaged gear field.
    #[inline]
    pub fn set_gearbox_drive_mode_gear(&mut self, value: GearboxDriveModeGear) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_2] & !0x0e;
        let raw = raw | ((u8::from(value) << 1) & 0x0e);
        data[field::FLAGS_2] = raw;
    }

    /// Set the gearbox gear to display field.
    #[inline]
    pub fn set_gearbox_gear(&mut self, value: GearboxGear) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_2] & !0xf0;
        let raw = raw | (u8::from(value) << 4);
        data[field::FLAGS_2] = raw;
    }

    /// Set the gearbox type field.
    #[inline]
    pub fn set_gearbox_type(&mut self, value: GearboxType) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_3] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::FLAGS_3] = raw;
    }

    /// Set the gear efficiency indicator arrow type field.
    #[inline]
    pub fn set_gear_efficiency_indicator_arrow_type(&mut self, value: GearEfficiencyArrowType) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_3] & !0x0c;
        let raw = raw | ((u8::from(value) << 2) & 0x0c);
        data[field::FLAGS_3] = raw;
    }

    /// Set the automatic gearbox mode field.
    #[inline]
    pub fn set_automatic_gearbox_mode(&mut self, value: AutoGearboxMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_3] & !0x70;
        let raw = raw | ((u8::from(value) << 4) & 0x70);
        data[field::FLAGS_3] = raw;
    }

    /// Set the foot on brake pedal indicator state field.
    #[inline]
    pub fn set_foot_on_brake_pedal_indicator(&mut self, value: IndicatorState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_4] & !0x0c;
        let raw = raw | ((u8::from(value) << 2) & 0x0c);
        data[field::FLAGS_4] = raw;
    }

    /// Set the AdBlue indicator state field.
    #[inline]
    pub fn set_adblue_indicator(&mut self, value: AdBlueIndicatorState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_6] & !0x06;
        let raw = raw | ((u8::from(value) << 1) & 0x06);
        data[field::FLAGS_6] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x128 ({})", err)?;
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

/// A high-level representation of a x128 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub daytime_running_lamps_indicator: bool,
    pub left_blinker_indicator: bool,
    pub right_blinker_indicator: bool,
    pub rear_anti_fog_light_indicator: bool,
    pub front_anti_fog_light_indicator: bool,
    pub main_beam_indicator: bool,
    pub headlamps_indicator: bool,
    pub sidelights_indicator: bool,
    pub displayed_gear_blinking: bool,
    pub gearbox_drive_mode_gear: GearboxDriveModeGear,
    pub gearbox_gear: GearboxGear,
    pub gearbox_type: GearboxType,
    pub gear_efficiency_indicator_arrow_type: GearEfficiencyArrowType,
    pub automatic_gearbox_mode: AutoGearboxMode,
    pub gear_efficiency_indicator_blinking: bool,
    pub automatic_parking_brake_inhibited: bool,
    pub parking_brake_applied: bool,
    pub foot_on_brake_pedal_indicator: IndicatorState,
    pub passenger_airbag_inhibited: bool,
    pub child_lock_security: bool,
    pub stop_indicator: bool,
    pub service_indicator: bool,
    pub suspension_indicator: bool,
    pub esp_indicator: bool,
    pub esp_inhibited: bool,
    pub automatic_main_beam_indicator: bool,
    pub available_space_measurement_indicator_blinking: bool,
    pub available_space_measurement_indicator: bool,
    pub opened_door: bool,
    pub diesel_pre_heating: bool,
    pub rear_left_seat_belt_indicator: bool,
    pub adblue_indicator: AdBlueIndicatorState,
    pub passenger_seat_belt_indicator_blinking: bool,
    pub passenger_seat_belt_indicator: bool,
    pub driver_seat_belt_indicator_blinking: bool,
    pub driver_seat_belt_indicator: bool,
    pub low_fuel: bool,
    pub passenger_protection: bool,
    pub hazard_warning_lights: bool,
    pub instrument_cluster_on: bool,
    pub rear_right_seat_belt_indicator_blinking: bool,
    pub rear_right_seat_belt_indicator: bool,
    pub rear_middle_seat_belt_indicator_blinking: bool,
    pub rear_middle_seat_belt_indicator: bool,
    pub rear_left_seat_belt_indicator_blinking: bool,
    pub low_fuel_indicator_blinking: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            daytime_running_lamps_indicator: frame.read_bit::<{ field::FLAGS_1 }, 0>(),
            left_blinker_indicator: frame.read_bit::<{ field::FLAGS_1 }, 1>(),
            right_blinker_indicator: frame.read_bit::<{ field::FLAGS_1 }, 2>(),
            rear_anti_fog_light_indicator: frame.read_bit::<{ field::FLAGS_1 }, 3>(),
            front_anti_fog_light_indicator: frame.read_bit::<{ field::FLAGS_1 }, 4>(),
            main_beam_indicator: frame.read_bit::<{ field::FLAGS_1 }, 5>(),
            headlamps_indicator: frame.read_bit::<{ field::FLAGS_1 }, 6>(),
            sidelights_indicator: frame.read_bit::<{ field::FLAGS_1 }, 7>(),
            displayed_gear_blinking: frame.read_bit::<{ field::FLAGS_2 }, 0>(),
            gearbox_drive_mode_gear: frame.gearbox_drive_mode_gear(),
            gearbox_gear: frame.gearbox_gear(),
            gearbox_type: frame.gearbox_type(),
            gear_efficiency_indicator_arrow_type: frame.gear_efficiency_indicator_arrow_type(),
            automatic_gearbox_mode: frame.automatic_gearbox_mode(),
            gear_efficiency_indicator_blinking: frame.read_bit::<{ field::FLAGS_3 }, 7>(),
            automatic_parking_brake_inhibited: frame.read_bit::<{ field::FLAGS_4 }, 0>(),
            parking_brake_applied: frame.read_bit::<{ field::FLAGS_4 }, 1>(),
            foot_on_brake_pedal_indicator: frame.foot_on_brake_pedal_indicator(),
            passenger_airbag_inhibited: frame.read_bit::<{ field::FLAGS_4 }, 4>(),
            child_lock_security: frame.read_bit::<{ field::FLAGS_4 }, 5>(),
            stop_indicator: frame.read_bit::<{ field::FLAGS_4 }, 6>(),
            service_indicator: frame.read_bit::<{ field::FLAGS_4 }, 7>(),
            suspension_indicator: frame.read_bit::<{ field::FLAGS_5 }, 0>(),
            esp_indicator: frame.read_bit::<{ field::FLAGS_5 }, 1>(),
            esp_inhibited: frame.read_bit::<{ field::FLAGS_5 }, 2>(),
            automatic_main_beam_indicator: frame.read_bit::<{ field::FLAGS_5 }, 3>(),
            available_space_measurement_indicator_blinking: frame
                .read_bit::<{ field::FLAGS_5 }, 4>(),
            available_space_measurement_indicator: frame.read_bit::<{ field::FLAGS_5 }, 5>(),
            opened_door: frame.read_bit::<{ field::FLAGS_5 }, 6>(),
            diesel_pre_heating: frame.read_bit::<{ field::FLAGS_5 }, 7>(),
            rear_left_seat_belt_indicator: frame.read_bit::<{ field::FLAGS_6 }, 0>(),
            adblue_indicator: frame.adblue_indicator(),
            passenger_seat_belt_indicator_blinking: frame.read_bit::<{ field::FLAGS_6 }, 3>(),
            passenger_seat_belt_indicator: frame.read_bit::<{ field::FLAGS_6 }, 4>(),
            driver_seat_belt_indicator_blinking: frame.read_bit::<{ field::FLAGS_6 }, 5>(),
            driver_seat_belt_indicator: frame.read_bit::<{ field::FLAGS_6 }, 6>(),
            low_fuel: frame.read_bit::<{ field::FLAGS_6 }, 7>(),
            passenger_protection: frame.read_bit::<{ field::FLAGS_7 }, 0>(),
            hazard_warning_lights: frame.read_bit::<{ field::FLAGS_7 }, 1>(),
            instrument_cluster_on: frame.read_bit::<{ field::FLAGS_7 }, 2>(),
            rear_right_seat_belt_indicator_blinking: frame.read_bit::<{ field::FLAGS_7 }, 3>(),
            rear_right_seat_belt_indicator: frame.read_bit::<{ field::FLAGS_7 }, 4>(),
            rear_middle_seat_belt_indicator_blinking: frame.read_bit::<{ field::FLAGS_7 }, 5>(),
            rear_middle_seat_belt_indicator: frame.read_bit::<{ field::FLAGS_7 }, 6>(),
            rear_left_seat_belt_indicator_blinking: frame.read_bit::<{ field::FLAGS_7 }, 7>(),
            low_fuel_indicator_blinking: frame.read_bit::<{ field::FLAGS_8 }, 3>(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x128 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.write_bit::<{ field::FLAGS_1 }, 0>(self.daytime_running_lamps_indicator);
        frame.write_bit::<{ field::FLAGS_1 }, 1>(self.left_blinker_indicator);
        frame.write_bit::<{ field::FLAGS_1 }, 2>(self.right_blinker_indicator);
        frame.write_bit::<{ field::FLAGS_1 }, 3>(self.rear_anti_fog_light_indicator);
        frame.write_bit::<{ field::FLAGS_1 }, 4>(self.front_anti_fog_light_indicator);
        frame.write_bit::<{ field::FLAGS_1 }, 5>(self.main_beam_indicator);
        frame.write_bit::<{ field::FLAGS_1 }, 6>(self.headlamps_indicator);
        frame.write_bit::<{ field::FLAGS_1 }, 7>(self.sidelights_indicator);
        frame.write_bit::<{ field::FLAGS_2 }, 0>(self.displayed_gear_blinking);
        frame.set_gearbox_drive_mode_gear(self.gearbox_drive_mode_gear);
        frame.set_gearbox_gear(self.gearbox_gear);
        frame.set_gearbox_type(self.gearbox_type);
        frame.set_gear_efficiency_indicator_arrow_type(self.gear_efficiency_indicator_arrow_type);
        frame.set_automatic_gearbox_mode(self.automatic_gearbox_mode);
        frame.write_bit::<{ field::FLAGS_3 }, 7>(self.gear_efficiency_indicator_blinking);
        frame.write_bit::<{ field::FLAGS_4 }, 0>(self.automatic_parking_brake_inhibited);
        frame.write_bit::<{ field::FLAGS_4 }, 1>(self.parking_brake_applied);
        frame.set_foot_on_brake_pedal_indicator(self.foot_on_brake_pedal_indicator);
        frame.write_bit::<{ field::FLAGS_4 }, 4>(self.passenger_airbag_inhibited);
        frame.write_bit::<{ field::FLAGS_4 }, 5>(self.child_lock_security);
        frame.write_bit::<{ field::FLAGS_4 }, 6>(self.stop_indicator);
        frame.write_bit::<{ field::FLAGS_4 }, 7>(self.service_indicator);
        frame.write_bit::<{ field::FLAGS_5 }, 0>(self.suspension_indicator);
        frame.write_bit::<{ field::FLAGS_5 }, 1>(self.esp_indicator);
        frame.write_bit::<{ field::FLAGS_5 }, 2>(self.esp_inhibited);
        frame.write_bit::<{ field::FLAGS_5 }, 3>(self.automatic_main_beam_indicator);
        frame.write_bit::<{ field::FLAGS_5 }, 4>(
            self.available_space_measurement_indicator_blinking,
        );
        frame.write_bit::<{ field::FLAGS_5 }, 5>(self.available_space_measurement_indicator);
        frame.write_bit::<{ field::FLAGS_5 }, 6>(self.opened_door);
        frame.write_bit::<{ field::FLAGS_5 }, 7>(self.diesel_pre_heating);
        frame.write_bit::<{ field::FLAGS_6 }, 0>(self.rear_left_seat_belt_indicator);
        frame.set_adblue_indicator(self.adblue_indicator);
        frame.write_bit::<{ field::FLAGS_6 }, 3>(self.passenger_seat_belt_indicator_blinking);
        frame.write_bit::<{ field::FLAGS_6 }, 4>(self.passenger_seat_belt_indicator);
        frame.write_bit::<{ field::FLAGS_6 }, 5>(self.driver_seat_belt_indicator_blinking);
        frame.write_bit::<{ field::FLAGS_6 }, 6>(self.driver_seat_belt_indicator);
        frame.write_bit::<{ field::FLAGS_6 }, 7>(self.low_fuel);
        frame.write_bit::<{ field::FLAGS_7 }, 0>(self.passenger_protection);
        frame.write_bit::<{ field::FLAGS_7 }, 1>(self.hazard_warning_lights);
        frame.write_bit::<{ field::FLAGS_7 }, 2>(self.instrument_cluster_on);
        frame.write_bit::<{ field::FLAGS_7 }, 3>(self.rear_right_seat_belt_indicator_blinking);
        frame.write_bit::<{ field::FLAGS_7 }, 4>(self.rear_right_seat_belt_indicator);
        frame.write_bit::<{ field::FLAGS_7 }, 5>(self.rear_middle_seat_belt_indicator_blinking);
        frame.write_bit::<{ field::FLAGS_7 }, 6>(self.rear_middle_seat_belt_indicator);
        frame.write_bit::<{ field::FLAGS_7 }, 7>(self.rear_left_seat_belt_indicator_blinking);
        frame.write_bit::<{ field::FLAGS_8 }, 3>(self.low_fuel_indicator_blinking);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x128")?;
        writeln!(
            f,
            " daytime_running_lamps_indicator={}",
            self.daytime_running_lamps_indicator
        )?;
        writeln!(f, " left_blinker_indicator={}", self.left_blinker_indicator)?;
        writeln!(
            f,
            " right_blinker_indicator={}",
            self.right_blinker_indicator
        )?;
        writeln!(
            f,
            " rear_anti_fog_light_indicator={}",
            self.rear_anti_fog_light_indicator
        )?;
        writeln!(
            f,
            " front_anti_fog_light_indicator={}",
            self.front_anti_fog_light_indicator
        )?;
        writeln!(f, " main_beam_indicator={}", self.main_beam_indicator)?;
        writeln!(f, " headlamps_indicator={}", self.headlamps_indicator)?;
        writeln!(f, " sidelights_indicator={}", self.sidelights_indicator)?;
        writeln!(
            f,
            " displayed_gear_blinking={}",
            self.displayed_gear_blinking
        )?;
        writeln!(
            f,
            " gearbox_drive_mode_gear={}",
            self.gearbox_drive_mode_gear
        )?;
        writeln!(f, " gearbox_gear={}", self.gearbox_gear)?;
        writeln!(f, " gearbox_type={}", self.gearbox_type)?;
        writeln!(
            f,
            " gear_efficiency_indicator_arrow_type={}",
            self.gear_efficiency_indicator_arrow_type
        )?;
        writeln!(f, " automatic_gearbox_mode={}", self.automatic_gearbox_mode)?;
        writeln!(
            f,
            " gear_efficiency_indicator_blinking={}",
            self.gear_efficiency_indicator_blinking
        )?;
        writeln!(
            f,
            " automatic_parking_brake_inhibited={}",
            self.automatic_parking_brake_inhibited
        )?;
        writeln!(f, " parking_brake_applied={}", self.parking_brake_applied)?;
        writeln!(
            f,
            " foot_on_brake_pedal_indicator={}",
            self.foot_on_brake_pedal_indicator
        )?;
        writeln!(
            f,
            " passenger_airbag_inhibited={}",
            self.passenger_airbag_inhibited
        )?;
        writeln!(f, " child_lock_security={}", self.child_lock_security)?;
        writeln!(f, " stop_indicator={}", self.stop_indicator)?;
        writeln!(f, " service_indicator={}", self.service_indicator)?;
        writeln!(f, " suspension_indicator={}", self.suspension_indicator)?;
        writeln!(f, " esp_indicator={}", self.esp_indicator)?;
        writeln!(f, " esp_inhibited={}", self.esp_inhibited)?;
        writeln!(
            f,
            " automatic_main_beam_indicator={}",
            self.automatic_main_beam_indicator
        )?;
        writeln!(
            f,
            " available_space_measurement_indicator_blinking={}",
            self.available_space_measurement_indicator_blinking
        )?;
        writeln!(
            f,
            " available_space_measurement_indicator={}",
            self.available_space_measurement_indicator
        )?;
        writeln!(f, " opened_door={}", self.opened_door)?;
        writeln!(f, " diesel_pre_heating={}", self.diesel_pre_heating)?;
        writeln!(
            f,
            " rear_left_seat_belt_indicator={}",
            self.rear_left_seat_belt_indicator
        )?;
        writeln!(f, " adblue_indicator={}", self.adblue_indicator)?;
        writeln!(
            f,
            " passenger_seat_belt_indicator_blinking={}",
            self.passenger_seat_belt_indicator_blinking
        )?;
        writeln!(
            f,
            " passenger_seat_belt_indicator={}",
            self.passenger_seat_belt_indicator
        )?;
        writeln!(
            f,
            " driver_seat_belt_indicator_blinking={}",
            self.driver_seat_belt_indicator_blinking
        )?;
        writeln!(
            f,
            " driver_seat_belt_indicator={}",
            self.driver_seat_belt_indicator
        )?;
        writeln!(f, " low_fuel={}", self.low_fuel)?;
        writeln!(f, " passenger_protection={}", self.passenger_protection)?;
        writeln!(f, " hazard_warning_lights={}", self.hazard_warning_lights)?;
        writeln!(f, " instrument_cluster_on={}", self.instrument_cluster_on)?;
        writeln!(
            f,
            " rear_right_seat_belt_indicator_blinking={}",
            self.rear_right_seat_belt_indicator_blinking
        )?;
        writeln!(
            f,
            " rear_right_seat_belt_indicator={}",
            self.rear_right_seat_belt_indicator
        )?;
        writeln!(
            f,
            " rear_middle_seat_belt_indicator_blinking={}",
            self.rear_middle_seat_belt_indicator_blinking
        )?;
        writeln!(
            f,
            " rear_middle_seat_belt_indicator={}",
            self.rear_middle_seat_belt_indicator
        )?;
        writeln!(
            f,
            " rear_left_seat_belt_indicator_blinking={}",
            self.rear_left_seat_belt_indicator_blinking
        )?;
        writeln!(
            f,
            " low_fuel_indicator_blinking={}",
            self.low_fuel_indicator_blinking
        )
    }
}

#[cfg(test)]
mod test {
    use super::{field, Frame, Repr};
    use crate::{
        vehicle::{
            AdBlueIndicatorState, AutoGearboxMode, IndicatorState,
            GearEfficiencyArrowType, GearboxDriveModeGear, GearboxGear, GearboxType,
        },
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x55, 0x35, 0x00, 0x55, 0x55, 0x53, 0x55, 0x08];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0xaa, 0xb0, 0xc5, 0xaa, 0xaa, 0xa8, 0xaa, 0x00];

    fn frame_1_repr() -> Repr {
        Repr {
            daytime_running_lamps_indicator: true,
            left_blinker_indicator: false,
            right_blinker_indicator: true,
            rear_anti_fog_light_indicator: false,
            front_anti_fog_light_indicator: true,
            main_beam_indicator: false,
            headlamps_indicator: true,
            sidelights_indicator: false,
            displayed_gear_blinking: true,
            gearbox_drive_mode_gear: GearboxDriveModeGear::Gear2,
            gearbox_gear: GearboxGear::D,
            gearbox_type: GearboxType::Automatic,
            gear_efficiency_indicator_arrow_type: GearEfficiencyArrowType::Nothing,
            automatic_gearbox_mode: AutoGearboxMode::Automatic,
            gear_efficiency_indicator_blinking: false,
            automatic_parking_brake_inhibited: true,
            parking_brake_applied: false,
            foot_on_brake_pedal_indicator: IndicatorState::On,
            passenger_airbag_inhibited: true,
            child_lock_security: false,
            stop_indicator: true,
            service_indicator: false,
            suspension_indicator: true,
            esp_indicator: false,
            esp_inhibited: true,
            automatic_main_beam_indicator: false,
            available_space_measurement_indicator_blinking: true,
            available_space_measurement_indicator: false,
            opened_door: true,
            diesel_pre_heating: false,
            rear_left_seat_belt_indicator: true,
            adblue_indicator: AdBlueIndicatorState::Blinking,
            passenger_seat_belt_indicator_blinking: false,
            passenger_seat_belt_indicator: true,
            driver_seat_belt_indicator_blinking: false,
            driver_seat_belt_indicator: true,
            low_fuel: false,
            passenger_protection: true,
            hazard_warning_lights: false,
            instrument_cluster_on: true,
            rear_right_seat_belt_indicator_blinking: false,
            rear_right_seat_belt_indicator: true,
            rear_middle_seat_belt_indicator_blinking: false,
            rear_middle_seat_belt_indicator: true,
            rear_left_seat_belt_indicator_blinking: false,
            low_fuel_indicator_blinking: true,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            daytime_running_lamps_indicator: false,
            left_blinker_indicator: true,
            right_blinker_indicator: false,
            rear_anti_fog_light_indicator: true,
            front_anti_fog_light_indicator: false,
            main_beam_indicator: true,
            headlamps_indicator: false,
            sidelights_indicator: true,
            displayed_gear_blinking: false,
            gearbox_drive_mode_gear: GearboxDriveModeGear::Disengaged,
            gearbox_gear: GearboxGear::Nothing,
            gearbox_type: GearboxType::Manual,
            gear_efficiency_indicator_arrow_type: GearEfficiencyArrowType::Up,
            automatic_gearbox_mode: AutoGearboxMode::Sequential,
            gear_efficiency_indicator_blinking: true,
            automatic_parking_brake_inhibited: false,
            parking_brake_applied: true,
            foot_on_brake_pedal_indicator: IndicatorState::Blinking,
            passenger_airbag_inhibited: false,
            child_lock_security: true,
            stop_indicator: false,
            service_indicator: true,
            suspension_indicator: false,
            esp_indicator: true,
            esp_inhibited: false,
            automatic_main_beam_indicator: true,
            available_space_measurement_indicator_blinking: false,
            available_space_measurement_indicator: true,
            opened_door: false,
            diesel_pre_heating: true,
            rear_left_seat_belt_indicator: false,
            adblue_indicator: AdBlueIndicatorState::Off,
            passenger_seat_belt_indicator_blinking: true,
            passenger_seat_belt_indicator: false,
            driver_seat_belt_indicator_blinking: true,
            driver_seat_belt_indicator: false,
            low_fuel: true,
            passenger_protection: false,
            hazard_warning_lights: true,
            instrument_cluster_on: false,
            rear_right_seat_belt_indicator_blinking: true,
            rear_right_seat_belt_indicator: false,
            rear_middle_seat_belt_indicator_blinking: true,
            rear_middle_seat_belt_indicator: false,
            rear_left_seat_belt_indicator_blinking: true,
            low_fuel_indicator_blinking: false,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 0>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 1>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 2>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 3>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 4>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 5>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 6>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 7>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 0>(), true);
        assert_eq!(frame.gearbox_drive_mode_gear(), GearboxDriveModeGear::Gear2);
        assert_eq!(frame.gearbox_gear(), GearboxGear::D);
        assert_eq!(frame.gearbox_type(), GearboxType::Automatic);
        assert_eq!(
            frame.gear_efficiency_indicator_arrow_type(),
            GearEfficiencyArrowType::Nothing
        );
        assert_eq!(frame.automatic_gearbox_mode(), AutoGearboxMode::Automatic);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 7>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 0>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 1>(), false);
        assert_eq!(
            frame.foot_on_brake_pedal_indicator(),
            IndicatorState::On
        );
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 4>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 5>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 6>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 7>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 0>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 1>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 2>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 3>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 4>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 5>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 6>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 7>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 0>(), true);
        assert_eq!(frame.adblue_indicator(), AdBlueIndicatorState::Blinking);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 3>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 4>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 5>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 6>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 7>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 0>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 1>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 2>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 3>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 4>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 5>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 6>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 7>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_8 }, 3>(), true);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 0>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 1>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 2>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 3>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 4>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 5>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 6>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_1 }, 7>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 0>(), false);
        assert_eq!(
            frame.gearbox_drive_mode_gear(),
            GearboxDriveModeGear::Disengaged
        );
        assert_eq!(frame.gearbox_gear(), GearboxGear::Nothing);
        assert_eq!(frame.gearbox_type(), GearboxType::Manual);
        assert_eq!(
            frame.gear_efficiency_indicator_arrow_type(),
            GearEfficiencyArrowType::Up
        );
        assert_eq!(frame.automatic_gearbox_mode(), AutoGearboxMode::Sequential);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 7>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 0>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 1>(), true);
        assert_eq!(
            frame.foot_on_brake_pedal_indicator(),
            IndicatorState::Blinking
        );
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 4>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 5>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 6>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 7>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 0>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 1>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 2>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 3>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 4>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 5>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 6>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 7>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 0>(), false);
        assert_eq!(frame.adblue_indicator(), AdBlueIndicatorState::Off);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 3>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 4>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 5>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 6>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 7>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 0>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 1>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 2>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 3>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 4>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 5>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 6>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 7>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_8 }, 3>(), false);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.write_bit::<{ field::FLAGS_1 }, 0>(true);
        frame.write_bit::<{ field::FLAGS_1 }, 1>(false);
        frame.write_bit::<{ field::FLAGS_1 }, 2>(true);
        frame.write_bit::<{ field::FLAGS_1 }, 3>(false);
        frame.write_bit::<{ field::FLAGS_1 }, 4>(true);
        frame.write_bit::<{ field::FLAGS_1 }, 5>(false);
        frame.write_bit::<{ field::FLAGS_1 }, 6>(true);
        frame.write_bit::<{ field::FLAGS_1 }, 7>(false);
        frame.write_bit::<{ field::FLAGS_2 }, 0>(true);
        frame.set_gearbox_drive_mode_gear(GearboxDriveModeGear::Gear2);
        frame.set_gearbox_gear(GearboxGear::D);
        frame.set_gearbox_type(GearboxType::Automatic);
        frame.set_gear_efficiency_indicator_arrow_type(GearEfficiencyArrowType::Nothing);
        frame.set_automatic_gearbox_mode(AutoGearboxMode::Automatic);
        frame.write_bit::<{ field::FLAGS_3 }, 7>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 0>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 1>(false);
        frame.set_foot_on_brake_pedal_indicator(IndicatorState::On);
        frame.write_bit::<{ field::FLAGS_4 }, 4>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 5>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 6>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 7>(false);
        frame.write_bit::<{ field::FLAGS_5 }, 0>(true);
        frame.write_bit::<{ field::FLAGS_5 }, 1>(false);
        frame.write_bit::<{ field::FLAGS_5 }, 2>(true);
        frame.write_bit::<{ field::FLAGS_5 }, 3>(false);
        frame.write_bit::<{ field::FLAGS_5 }, 4>(true);
        frame.write_bit::<{ field::FLAGS_5 }, 5>(false);
        frame.write_bit::<{ field::FLAGS_5 }, 6>(true);
        frame.write_bit::<{ field::FLAGS_5 }, 7>(false);
        frame.write_bit::<{ field::FLAGS_6 }, 0>(true);
        frame.set_adblue_indicator(AdBlueIndicatorState::Blinking);
        frame.write_bit::<{ field::FLAGS_6 }, 3>(false);
        frame.write_bit::<{ field::FLAGS_6 }, 4>(true);
        frame.write_bit::<{ field::FLAGS_6 }, 5>(false);
        frame.write_bit::<{ field::FLAGS_6 }, 6>(true);
        frame.write_bit::<{ field::FLAGS_6 }, 7>(false);
        frame.write_bit::<{ field::FLAGS_7 }, 0>(true);
        frame.write_bit::<{ field::FLAGS_7 }, 1>(false);
        frame.write_bit::<{ field::FLAGS_7 }, 2>(true);
        frame.write_bit::<{ field::FLAGS_7 }, 3>(false);
        frame.write_bit::<{ field::FLAGS_7 }, 4>(true);
        frame.write_bit::<{ field::FLAGS_7 }, 5>(false);
        frame.write_bit::<{ field::FLAGS_7 }, 6>(true);
        frame.write_bit::<{ field::FLAGS_7 }, 7>(false);
        frame.write_bit::<{ field::FLAGS_8 }, 3>(true);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.write_bit::<{ field::FLAGS_1 }, 0>(false);
        frame.write_bit::<{ field::FLAGS_1 }, 1>(true);
        frame.write_bit::<{ field::FLAGS_1 }, 2>(false);
        frame.write_bit::<{ field::FLAGS_1 }, 3>(true);
        frame.write_bit::<{ field::FLAGS_1 }, 4>(false);
        frame.write_bit::<{ field::FLAGS_1 }, 5>(true);
        frame.write_bit::<{ field::FLAGS_1 }, 6>(false);
        frame.write_bit::<{ field::FLAGS_1 }, 7>(true);
        frame.write_bit::<{ field::FLAGS_2 }, 0>(false);
        frame.set_gearbox_drive_mode_gear(GearboxDriveModeGear::Disengaged);
        frame.set_gearbox_gear(GearboxGear::Nothing);
        frame.set_gearbox_type(GearboxType::Manual);
        frame.set_gear_efficiency_indicator_arrow_type(GearEfficiencyArrowType::Up);
        frame.set_automatic_gearbox_mode(AutoGearboxMode::Sequential);
        frame.write_bit::<{ field::FLAGS_3 }, 7>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 0>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 1>(true);
        frame.set_foot_on_brake_pedal_indicator(IndicatorState::Blinking);
        frame.write_bit::<{ field::FLAGS_4 }, 4>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 5>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 6>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 7>(true);
        frame.write_bit::<{ field::FLAGS_5 }, 0>(false);
        frame.write_bit::<{ field::FLAGS_5 }, 1>(true);
        frame.write_bit::<{ field::FLAGS_5 }, 2>(false);
        frame.write_bit::<{ field::FLAGS_5 }, 3>(true);
        frame.write_bit::<{ field::FLAGS_5 }, 4>(false);
        frame.write_bit::<{ field::FLAGS_5 }, 5>(true);
        frame.write_bit::<{ field::FLAGS_5 }, 6>(false);
        frame.write_bit::<{ field::FLAGS_5 }, 7>(true);
        frame.write_bit::<{ field::FLAGS_6 }, 0>(false);
        frame.set_adblue_indicator(AdBlueIndicatorState::Off);
        frame.write_bit::<{ field::FLAGS_6 }, 3>(true);
        frame.write_bit::<{ field::FLAGS_6 }, 4>(false);
        frame.write_bit::<{ field::FLAGS_6 }, 5>(true);
        frame.write_bit::<{ field::FLAGS_6 }, 6>(false);
        frame.write_bit::<{ field::FLAGS_6 }, 7>(true);
        frame.write_bit::<{ field::FLAGS_7 }, 0>(false);
        frame.write_bit::<{ field::FLAGS_7 }, 1>(true);
        frame.write_bit::<{ field::FLAGS_7 }, 2>(false);
        frame.write_bit::<{ field::FLAGS_7 }, 3>(true);
        frame.write_bit::<{ field::FLAGS_7 }, 4>(false);
        frame.write_bit::<{ field::FLAGS_7 }, 5>(true);
        frame.write_bit::<{ field::FLAGS_7 }, 6>(false);
        frame.write_bit::<{ field::FLAGS_7 }, 7>(true);
        frame.write_bit::<{ field::FLAGS_8 }, 3>(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x55, 0x35, 0x00, 0x55, 0x55, 0x53, 0x55, 0x08, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x55, 0x35, 0x00, 0x55, 0x55, 0x53, 0x55];
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
