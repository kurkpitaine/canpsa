use core::{cmp::Ordering, fmt};

use crate::{
    vehicle::{
        GearboxDriveModeGear, IndicatorState, LaneCenteringIndicatorState,
        SteeringAssistanceFaultType, SteeringAssistanceIndicatorState,
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
168 CDE_COMBINE_TEMOINS_ABS_DEF_HS7_168                 // OK
168 CDE_COMBINE_TEMOINS_ACQ_DISPO_AFF_HS7_168
168 CDE_COMBINE_TEMOINS_AFF_ACHV_VTH_D_HS7_168
168 CDE_COMBINE_TEMOINS_AFF_ACHV_VTH_G_HS7_168
168 CDE_COMBINE_TEMOINS_ALERTE_MOT_FROID_HS7_168        // OK
168 CDE_COMBINE_TEMOINS_ALERTE_T_EAU_HS7_168            // OK
168 CDE_COMBINE_TEMOINS_ALLUM_REGIME_MAX1_HS7_168
168 CDE_COMBINE_TEMOINS_ALLUM_REGIME_MAX2_HS7_168
168 CDE_COMBINE_TEMOINS_ASR_DEF_HS7_168                 // OK
168 CDE_COMBINE_TEMOINS_ASS_DIR_DEF_HS7_168             // OK
168 CDE_COMBINE_TEMOINS_ASST_FREIN_DEF_HS7_168          // OK
168 CDE_COMBINE_TEMOINS_AUTOR_ACQ_AMBIANCE_HS7_168
168 CDE_COMBINE_TEMOINS_AUTOR_VTH_HS7_168
168 CDE_COMBINE_TEMOINS_BV_DEF_HS7_168                  // OK
168 CDE_COMBINE_TEMOINS_CBAT_DEF_HS7_168                // OK
168 CDE_COMBINE_TEMOINS_CODE_VIR_DEF_HS7_168            // OK
168 CDE_COMBINE_TEMOINS_CREV_AL_HS7_168                 // OK
168 CDE_COMBINE_TEMOINS_DA_DEF_1_HS7_168                // OK
168 CDE_COMBINE_TEMOINS_DA_DEF_2_HS7_168                // OK
168 CDE_COMBINE_TEMOINS_DMD_ALLUM_AFIL_HS7_168          // OK
168 CDE_COMBINE_TEMOINS_DMD_ALLUMAGE_FA_HS7_168         // OK
168 CDE_COMBINE_TEMOINS_DMD_ALLUM_CAAR_HS7_168
168 CDE_COMBINE_TEMOINS_DMD_ALLUM_FAP_HS7_168           // OK
168 CDE_COMBINE_TEMOINS_DMD_ALLUM_STT_HS7_168
168 CDE_COMBINE_TEMOINS_DSG_DEF_HS7_168                 // OK
168 CDE_COMBINE_TEMOINS_DSUSP_DEF_HS7_168
168 CDE_COMBINE_TEMOINS_EAUG_DEF_HS7_168                // OK
168 CDE_COMBINE_TEMOINS_EOBD_DEF_HS7_168                // OK
168 CDE_COMBINE_TEMOINS_ESSUI_AUTO_HS7_168              // OK
168 CDE_COMBINE_TEMOINS_FSE_SER_DEF_HS7_168
168 CDE_COMBINE_TEMOINS_FSE_SYST_DEF_HS7_168
168 CDE_COMBINE_TEMOINS_GENE_DEF_HS7_168                // OK
168 CDE_COMBINE_TEMOINS_NBRE_RAP_BV_HS7_168             // OK
168 CDE_COMBINE_TEMOINS_NIVE_AL_HS7_168                 // OK
168 CDE_COMBINE_TEMOINS_NIVH_AL_HS7_168                 // OK
168 CDE_COMBINE_TEMOINS_NIVL_AL_HS7_168                 // OK
168 CDE_COMBINE_TEMOINS_OUCARD2_CLIG_HS7_168
168 CDE_COMBINE_TEMOINS_OUCARD2_HS7_168
168 CDE_COMBINE_TEMOINS_OUCARG2_CLIG_HS7_168
168 CDE_COMBINE_TEMOINS_OUCARG2_HS7_168
168 CDE_COMBINE_TEMOINS_PHUI_AL_HS7_168                 // OK
168 CDE_COMBINE_TEMOINS_PLAQ_DEF_HS7_168                // OK
168 CDE_COMBINE_TEMOINS_POLL_DEF_HS7_168                // OK
168 CDE_COMBINE_TEMOINS_P_TEM_HADC_FEEDBACK_HS7_168
168 CDE_COMBINE_TEMOINS_RAP_AFF_DRIVE_2_HS7_168         // OK
168 CDE_COMBINE_TEMOINS_REF_DEF_HS7_168                 // OK
168 CDE_COMBINE_TEMOINS_SEC_PASS_DEF_HS7_168            // OK
168 CDE_COMBINE_TEMOINS_SOUG_AL_HS7_168                 // OK
168 CDE_COMBINE_TEMOINS_TEST_TEM_DA_DEF_HS7_168
168 CDE_COMBINE_TEMOINS_UB_DA_DEF_1_HS7_168             // OK
168 CDE_COMBINE_TEMOINS_UB_DA_DEF_2_HS7_168             // OK
168 CDE_COMBINE_TEMOINS_UB_DMD_ALLUMAGE_MIL_HS7_168
168 CDE_COMBINE_TEMOINS_VOY_PLUS_START_HS7_168
*/

mod field {
    /// 1-bit under-inflation detection system failure flag,
    /// 1-bit cold engine alert flag,
    /// 1-bit low brake fluid level alert flag,
    /// 1-bit low oil pressure alert flag,
    /// 1-bit low oil level alert flag,
    /// 1-bit low coolant level alert flag,
    /// 1-bit gearbox has more than 6 speeds flag,
    /// 1-bit coolant temperature alert flag.
    pub const FLAGS_1: usize = 0;
    /// 3-bit unknown,
    /// 1-bit automatic wipers enabled flag,
    /// 1-bit particulate filter indicator display flag,
    /// 1-bit anti-emission system fault flag,
    /// 1-bit tyre puncture alert flag,
    /// 1-bit under-inflation alert flag.
    pub const FLAGS_2: usize = 1;
    /// 1-bit electrical generator fault flag,
    /// 1-bit battery charge fault flag,
    /// 1-bit unknown,
    /// 1-bit EBD fault flag,
    /// 4-bit unknown.
    pub const FLAGS_3: usize = 2;
    /// 1-bit unknown,
    /// 1-bit OBD fault flag (displays the engine fault indicator),
    /// 1-bit worn brake pad fault flag,
    /// 1-bit gearbox fault flag,
    /// 1-bit ESP/ASR fault flag,
    /// 1-bit ABS fault flag,
    /// 2-bit unknown.
    pub const FLAGS_4: usize = 3;
    /// 2-bit unknown,
    /// 1-bit steering assistance fault flag,
    /// 2-bit unknown,
    /// 1-bit passive safety fault flag,
    /// 1-bit turn lights fault flag,
    /// 1-bit water in diesel fault flag.
    pub const FLAGS_5: usize = 4;
    /// 2-bit unknown,
    /// 1-bit steering assistance fault type validity flag.
    /// 2-bit steering assistance fault type field,
    /// 1-bit steering assistance indicator state validity flag.
    /// 2-bit steering assistance indicator state field,
    pub const FLAGS_6: usize = 5;
    /// 1-bit braking assistance fault flag,
    /// 4-bit gearbox drive mode engaged gear field,
    /// 3-bit unknown.
    pub const FLAGS_7: usize = 6;
    /// 2-bit unknown,
    /// 2-bit lane centering indicator state field,
    /// 2-bit unknown,
    /// 2-bit automatic emergency braking indicator state field.
    pub const FLAGS_8: usize = 7;
}

/// Raw x168 CAN frame identifier.
pub const FRAME_ID: u16 = 0x168;
/// Length of a x168 CAN frame.
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

    /// Return the bit in byte B at index I.
    #[inline]
    pub fn read_bit<const B: usize, const I: u8>(&self) -> bool {
        let data = self.buffer.as_ref();
        (data[B] & (1u8 << I)) != 0
    }

    /// Return the steering assistance fault type field.
    #[inline]
    pub fn steering_assistance_fault_type(&self) -> SteeringAssistanceFaultType {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_6] & 0x18) >> 3;
        SteeringAssistanceFaultType::from(raw)
    }

    /// Return the steering assistance indicator state field.
    #[inline]
    pub fn steering_assistance_indicator(&self) -> SteeringAssistanceIndicatorState {
        let data = self.buffer.as_ref();
        let raw = data[field::FLAGS_6] >> 6;
        SteeringAssistanceIndicatorState::from(raw)
    }

    /// Return the gearbox drive mode engaged gear field.
    #[inline]
    pub fn gearbox_drive_mode_gear(&self) -> GearboxDriveModeGear {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_7] & 0x1e) >> 1;
        GearboxDriveModeGear::from(raw)
    }

    /// Return the lane centering indicator state field.
    #[inline]
    pub fn lane_centering_indicator(&self) -> LaneCenteringIndicatorState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_8] & 0x0c) >> 2;
        LaneCenteringIndicatorState::from(raw)
    }

    /// Return the automatic emergency braking indicator state field.
    #[inline]
    pub fn automatic_emergency_braking_indicator(&self) -> IndicatorState {
        let data = self.buffer.as_ref();
        let raw = data[field::FLAGS_8] >> 6;
        IndicatorState::from(raw)
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

    /// Set the steering assistance fault type field.
    #[inline]
    pub fn set_steering_assistance_fault_type(&mut self, value: SteeringAssistanceFaultType) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_6] & !0x18;
        let raw = raw | ((u8::from(value) << 3) & 0x18);
        data[field::FLAGS_6] = raw;
    }

    /// Set the steering assistance indicator state field.
    #[inline]
    pub fn set_steering_assistance_indicator(&mut self, value: SteeringAssistanceIndicatorState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_6] & !0xc0;
        let raw = raw | ((u8::from(value) << 6) & 0xc0);
        data[field::FLAGS_6] = raw;
    }

    /// Set the gearbox drive mode engaged gear field.
    #[inline]
    pub fn set_gearbox_drive_mode_gear(&mut self, value: GearboxDriveModeGear) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_7] & !0x1e;
        let raw = raw | ((u8::from(value) << 1) & 0x1e);
        data[field::FLAGS_7] = raw;
    }

    /// Set the lane centering indicator state field.
    #[inline]
    pub fn set_lane_centering_indicator(&mut self, value: LaneCenteringIndicatorState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_8] & !0x0c;
        let raw = raw | ((u8::from(value) << 2) & 0x0c);
        data[field::FLAGS_8] = raw;
    }

    /// Set the automatic emergency braking indicator state field.
    #[inline]
    pub fn set_automatic_emergency_braking_indicator(&mut self, value: IndicatorState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_8] & !0xc0;
        let raw = raw | (u8::from(value) << 6);
        data[field::FLAGS_8] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x168 ({})", err)?;
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

/// A high-level representation of a x168 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub under_inflation_failure: bool,
    pub cold_engine_alert: bool,
    pub low_brake_fluid_level_alert: bool,
    pub low_oil_pressure_alert: bool,
    pub low_oil_level_alert: bool,
    pub low_coolant_level_alert: bool,
    pub gearbox_has_more_than_six_speed: bool,
    pub coolant_temperature_alert: bool,
    pub automatic_wipers_enabled: bool,
    pub particulate_filter_indicator: bool,
    pub anti_emission_fault: bool,
    pub tyre_puncture_alert: bool,
    pub under_inflation_alert_flag: bool,
    pub electrical_generator_fault: bool,
    pub battery_charge_fault: bool,
    pub ebd_fault: bool,
    pub obd_fault: bool,
    pub worn_brake_pad_fault: bool,
    pub gearbox_fault: bool,
    pub esp_asr_fault: bool,
    pub abs_fault: bool,
    pub steering_assistance_fault: bool,
    pub passive_safety_fault: bool,
    pub turn_lights_fault: bool,
    pub water_in_diesel: bool,
    pub steering_assistance_fault_type_validity: bool,
    pub steering_assistance_fault_type: SteeringAssistanceFaultType,
    pub steering_assistance_indicator_validity: bool,
    pub steering_assistance_indicator: SteeringAssistanceIndicatorState,
    pub braking_assistance_fault: bool,
    pub gearbox_drive_mode_gear: GearboxDriveModeGear,
    pub lane_centering_indicator: LaneCenteringIndicatorState,
    pub automatic_emergency_braking_indicator: IndicatorState,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            under_inflation_failure: frame.read_bit::<{ field::FLAGS_1 }, 0>(),
            cold_engine_alert: frame.read_bit::<{ field::FLAGS_1 }, 1>(),
            low_brake_fluid_level_alert: frame.read_bit::<{ field::FLAGS_1 }, 2>(),
            low_oil_pressure_alert: frame.read_bit::<{ field::FLAGS_1 }, 3>(),
            low_oil_level_alert: frame.read_bit::<{ field::FLAGS_1 }, 4>(),
            low_coolant_level_alert: frame.read_bit::<{ field::FLAGS_1 }, 5>(),
            gearbox_has_more_than_six_speed: frame.read_bit::<{ field::FLAGS_1 }, 6>(),
            coolant_temperature_alert: frame.read_bit::<{ field::FLAGS_1 }, 7>(),
            automatic_wipers_enabled: frame.read_bit::<{ field::FLAGS_2 }, 3>(),
            particulate_filter_indicator: frame.read_bit::<{ field::FLAGS_2 }, 4>(),
            anti_emission_fault: frame.read_bit::<{ field::FLAGS_2 }, 5>(),
            tyre_puncture_alert: frame.read_bit::<{ field::FLAGS_2 }, 6>(),
            under_inflation_alert_flag: frame.read_bit::<{ field::FLAGS_2 }, 7>(),
            electrical_generator_fault: frame.read_bit::<{ field::FLAGS_3 }, 0>(),
            battery_charge_fault: frame.read_bit::<{ field::FLAGS_3 }, 1>(),
            ebd_fault: frame.read_bit::<{ field::FLAGS_3 }, 3>(),
            obd_fault: frame.read_bit::<{ field::FLAGS_4 }, 1>(),
            worn_brake_pad_fault: frame.read_bit::<{ field::FLAGS_4 }, 2>(),
            gearbox_fault: frame.read_bit::<{ field::FLAGS_4 }, 3>(),
            esp_asr_fault: frame.read_bit::<{ field::FLAGS_4 }, 4>(),
            abs_fault: frame.read_bit::<{ field::FLAGS_4 }, 5>(),
            steering_assistance_fault: frame.read_bit::<{ field::FLAGS_5 }, 2>(),
            passive_safety_fault: frame.read_bit::<{ field::FLAGS_5 }, 5>(),
            turn_lights_fault: frame.read_bit::<{ field::FLAGS_5 }, 6>(),
            water_in_diesel: frame.read_bit::<{ field::FLAGS_5 }, 7>(),
            steering_assistance_fault_type_validity: frame.read_bit::<{ field::FLAGS_6 }, 2>(),
            steering_assistance_fault_type: frame.steering_assistance_fault_type(),
            steering_assistance_indicator_validity: frame.read_bit::<{ field::FLAGS_6 }, 5>(),
            steering_assistance_indicator: frame.steering_assistance_indicator(),
            braking_assistance_fault: frame.read_bit::<{ field::FLAGS_7 }, 0>(),
            gearbox_drive_mode_gear: frame.gearbox_drive_mode_gear(),
            lane_centering_indicator: frame.lane_centering_indicator(),
            automatic_emergency_braking_indicator: frame.automatic_emergency_braking_indicator(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x168 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.write_bit::<{ field::FLAGS_1 }, 0>(self.under_inflation_failure);
        frame.write_bit::<{ field::FLAGS_1 }, 1>(self.cold_engine_alert);
        frame.write_bit::<{ field::FLAGS_1 }, 2>(self.low_brake_fluid_level_alert);
        frame.write_bit::<{ field::FLAGS_1 }, 3>(self.low_oil_pressure_alert);
        frame.write_bit::<{ field::FLAGS_1 }, 4>(self.low_oil_level_alert);
        frame.write_bit::<{ field::FLAGS_1 }, 5>(self.low_coolant_level_alert);
        frame.write_bit::<{ field::FLAGS_1 }, 6>(self.gearbox_has_more_than_six_speed);
        frame.write_bit::<{ field::FLAGS_1 }, 7>(self.coolant_temperature_alert);
        frame.write_bit::<{ field::FLAGS_2 }, 3>(self.automatic_wipers_enabled);
        frame.write_bit::<{ field::FLAGS_2 }, 4>(self.particulate_filter_indicator);
        frame.write_bit::<{ field::FLAGS_2 }, 5>(self.anti_emission_fault);
        frame.write_bit::<{ field::FLAGS_2 }, 6>(self.tyre_puncture_alert);
        frame.write_bit::<{ field::FLAGS_2 }, 7>(self.under_inflation_alert_flag);
        frame.write_bit::<{ field::FLAGS_3 }, 0>(self.electrical_generator_fault);
        frame.write_bit::<{ field::FLAGS_3 }, 1>(self.battery_charge_fault);
        frame.write_bit::<{ field::FLAGS_3 }, 3>(self.ebd_fault);
        frame.write_bit::<{ field::FLAGS_4 }, 1>(self.obd_fault);
        frame.write_bit::<{ field::FLAGS_4 }, 2>(self.worn_brake_pad_fault);
        frame.write_bit::<{ field::FLAGS_4 }, 3>(self.gearbox_fault);
        frame.write_bit::<{ field::FLAGS_4 }, 4>(self.esp_asr_fault);
        frame.write_bit::<{ field::FLAGS_4 }, 5>(self.abs_fault);
        frame.write_bit::<{ field::FLAGS_5 }, 2>(self.steering_assistance_fault);
        frame.write_bit::<{ field::FLAGS_5 }, 5>(self.passive_safety_fault);
        frame.write_bit::<{ field::FLAGS_5 }, 6>(self.turn_lights_fault);
        frame.write_bit::<{ field::FLAGS_5 }, 7>(self.water_in_diesel);
        frame.write_bit::<{ field::FLAGS_6 }, 2>(self.steering_assistance_fault_type_validity);
        frame.set_steering_assistance_fault_type(self.steering_assistance_fault_type);
        frame.write_bit::<{ field::FLAGS_6 }, 5>(self.steering_assistance_indicator_validity);
        frame.set_steering_assistance_indicator(self.steering_assistance_indicator);
        frame.write_bit::<{ field::FLAGS_7 }, 0>(self.braking_assistance_fault);
        frame.set_gearbox_drive_mode_gear(self.gearbox_drive_mode_gear);
        frame.set_lane_centering_indicator(self.lane_centering_indicator);
        frame.set_automatic_emergency_braking_indicator(self.automatic_emergency_braking_indicator);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x168")?;
        writeln!(
            f,
            " under_inflation_failure={}",
            self.under_inflation_failure
        )?;
        writeln!(f, " cold_engine_alert={}", self.cold_engine_alert)?;
        writeln!(
            f,
            " low_brake_fluid_level_alert={}",
            self.low_brake_fluid_level_alert
        )?;
        writeln!(f, " low_oil_pressure_alert={}", self.low_oil_pressure_alert)?;
        writeln!(f, " low_oil_level_alert={}", self.low_oil_level_alert)?;
        writeln!(
            f,
            " low_coolant_level_alert={}",
            self.low_coolant_level_alert
        )?;
        writeln!(
            f,
            " gearbox_has_more_than_six_speed={}",
            self.gearbox_has_more_than_six_speed
        )?;
        writeln!(
            f,
            " coolant_temperature_alert={}",
            self.coolant_temperature_alert
        )?;
        writeln!(
            f,
            " automatic_wipers_enabled={}",
            self.automatic_wipers_enabled
        )?;
        writeln!(
            f,
            " particulate_filter_indicator={}",
            self.particulate_filter_indicator
        )?;
        writeln!(f, " anti_emission_fault={}", self.anti_emission_fault)?;
        writeln!(f, " tyre_puncture_alert={}", self.tyre_puncture_alert)?;
        writeln!(
            f,
            " under_inflation_alert_flag={}",
            self.under_inflation_alert_flag
        )?;
        writeln!(
            f,
            " electrical_generator_fault={}",
            self.electrical_generator_fault
        )?;
        writeln!(f, " battery_charge_fault={}", self.battery_charge_fault)?;
        writeln!(f, " ebd_fault={}", self.ebd_fault)?;
        writeln!(f, " obd_fault={}", self.obd_fault)?;
        writeln!(f, " worn_brake_pad_fault={}", self.worn_brake_pad_fault)?;
        writeln!(f, " gearbox_fault={}", self.gearbox_fault)?;
        writeln!(f, " esp_asr_fault={}", self.esp_asr_fault)?;
        writeln!(f, " abs_fault={}", self.abs_fault)?;
        writeln!(
            f,
            " steering_assistance_fault={}",
            self.steering_assistance_fault
        )?;
        writeln!(f, " passive_safety_fault={}", self.passive_safety_fault)?;
        writeln!(f, " turn_lights_fault={}", self.turn_lights_fault)?;
        writeln!(f, " water_in_diesel={}", self.water_in_diesel)?;
        writeln!(
            f,
            " steering_assistance_fault_type_validity={}",
            self.steering_assistance_fault_type_validity
        )?;
        writeln!(
            f,
            " steering_assistance_fault_type={}",
            self.steering_assistance_fault_type
        )?;
        writeln!(
            f,
            " steering_assistance_indicator_validity={}",
            self.steering_assistance_indicator_validity
        )?;
        writeln!(
            f,
            " steering_assistance_indicator={}",
            self.steering_assistance_indicator
        )?;
        writeln!(
            f,
            " braking_assistance_fault={}",
            self.braking_assistance_fault
        )?;
        writeln!(
            f,
            " gearbox_drive_mode_gear={}",
            self.gearbox_drive_mode_gear
        )?;
        writeln!(
            f,
            " lane_centering_indicator={}",
            self.lane_centering_indicator
        )?;
        writeln!(
            f,
            " automatic_emergency_braking_indicator={}",
            self.automatic_emergency_braking_indicator
        )
    }
}

impl From<&crate::aee2004::conf::x168::Repr> for Repr {
    fn from(repr_2004: &crate::aee2004::conf::x168::Repr) -> Self {
        Repr {
            under_inflation_failure: repr_2004.under_inflation_failure,
            cold_engine_alert: repr_2004.cold_engine_alert,
            low_brake_fluid_level_alert: repr_2004.low_brake_fluid_level_alert,
            low_oil_pressure_alert: repr_2004.low_oil_pressure_alert,
            low_oil_level_alert: repr_2004.low_oil_level_alert,
            low_coolant_level_alert: repr_2004.low_coolant_level_alert,
            gearbox_has_more_than_six_speed: false, // No corresponding data.
            coolant_temperature_alert: repr_2004.coolant_temperature_alert,
            automatic_wipers_enabled: repr_2004.automatic_wipers_enabled,
            particulate_filter_indicator: repr_2004.particulate_filter_indicator,
            anti_emission_fault: repr_2004.anti_emission_fault,
            tyre_puncture_alert: repr_2004.tyre_puncture_alert,
            under_inflation_alert_flag: repr_2004.under_inflation_alert_flag,
            electrical_generator_fault: repr_2004.electrical_generator_fault,
            battery_charge_fault: repr_2004.battery_charge_fault,
            ebd_fault: repr_2004.ebd_fault,
            obd_fault: repr_2004.obd_fault,
            worn_brake_pad_fault: repr_2004.worn_brake_pad_fault,
            gearbox_fault: repr_2004.gearbox_fault,
            esp_asr_fault: repr_2004.esp_asr_fault,
            abs_fault: repr_2004.abs_fault,
            steering_assistance_fault: repr_2004.steering_assistance_fault,
            passive_safety_fault: repr_2004.passive_safety_fault,
            turn_lights_fault: repr_2004.turn_lights_fault,
            water_in_diesel: repr_2004.water_in_diesel,
            steering_assistance_fault_type_validity: false,
            steering_assistance_fault_type: SteeringAssistanceFaultType::None,
            steering_assistance_indicator_validity: false,
            steering_assistance_indicator: SteeringAssistanceIndicatorState::Off,
            braking_assistance_fault: false,
            gearbox_drive_mode_gear: repr_2004.gearbox_drive_mode_gear,
            lane_centering_indicator: LaneCenteringIndicatorState::Off, // No lane centering on AEE2004.
            automatic_emergency_braking_indicator: IndicatorState::Off, // No corresponding data.
        }
    }
}

#[cfg(test)]
mod test {
    use super::{field, Frame, Repr};
    use crate::{
        vehicle::{
            GearboxDriveModeGear, IndicatorState, LaneCenteringIndicatorState,
            SteeringAssistanceFaultType, SteeringAssistanceIndicatorState,
        },
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x55, 0x50, 0x01, 0x14, 0x44, 0x94, 0x11, 0x84];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0xaa, 0xa8, 0x0a, 0x2a, 0xa0, 0x78, 0x06, 0x08];

    fn frame_1_repr() -> Repr {
        Repr {
            under_inflation_failure: true,
            cold_engine_alert: false,
            low_brake_fluid_level_alert: true,
            low_oil_pressure_alert: false,
            low_oil_level_alert: true,
            low_coolant_level_alert: false,
            gearbox_has_more_than_six_speed: true,
            coolant_temperature_alert: false,
            automatic_wipers_enabled: false,
            particulate_filter_indicator: true,
            anti_emission_fault: false,
            tyre_puncture_alert: true,
            under_inflation_alert_flag: false,
            electrical_generator_fault: true,
            battery_charge_fault: false,
            ebd_fault: false,
            obd_fault: false,
            worn_brake_pad_fault: true,
            gearbox_fault: false,
            esp_asr_fault: true,
            abs_fault: false,
            steering_assistance_fault: true,
            passive_safety_fault: false,
            turn_lights_fault: true,
            water_in_diesel: false,
            steering_assistance_fault_type_validity: true,
            steering_assistance_fault_type: SteeringAssistanceFaultType::G3,
            steering_assistance_indicator_validity: false,
            steering_assistance_indicator: SteeringAssistanceIndicatorState::Orange,
            braking_assistance_fault: true,
            gearbox_drive_mode_gear: GearboxDriveModeGear::Gear8,
            lane_centering_indicator: LaneCenteringIndicatorState::Steady,
            automatic_emergency_braking_indicator: IndicatorState::Blinking,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            under_inflation_failure: false,
            cold_engine_alert: true,
            low_brake_fluid_level_alert: false,
            low_oil_pressure_alert: true,
            low_oil_level_alert: false,
            low_coolant_level_alert: true,
            gearbox_has_more_than_six_speed: false,
            coolant_temperature_alert: true,
            automatic_wipers_enabled: true,
            particulate_filter_indicator: false,
            anti_emission_fault: true,
            tyre_puncture_alert: false,
            under_inflation_alert_flag: true,
            electrical_generator_fault: false,
            battery_charge_fault: true,
            ebd_fault: true,
            obd_fault: true,
            worn_brake_pad_fault: false,
            gearbox_fault: true,
            esp_asr_fault: false,
            abs_fault: true,
            steering_assistance_fault: false,
            passive_safety_fault: true,
            turn_lights_fault: false,
            water_in_diesel: true,
            steering_assistance_fault_type_validity: false,
            steering_assistance_fault_type: SteeringAssistanceFaultType::G3AndG4,
            steering_assistance_indicator_validity: true,
            steering_assistance_indicator: SteeringAssistanceIndicatorState::Red,
            braking_assistance_fault: false,
            gearbox_drive_mode_gear: GearboxDriveModeGear::Gear3,
            lane_centering_indicator: LaneCenteringIndicatorState::BlinkingFault,
            automatic_emergency_braking_indicator: IndicatorState::Off,
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
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 3>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 4>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 5>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 6>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 7>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 0>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 1>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 3>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 1>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 2>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 3>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 4>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 5>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 2>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 5>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 6>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 7>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 2>(), true);
        assert_eq!(
            frame.steering_assistance_fault_type(),
            SteeringAssistanceFaultType::G3
        );
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 5>(), false);
        assert_eq!(
            frame.steering_assistance_indicator(),
            SteeringAssistanceIndicatorState::Orange
        );
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 0>(), true);
        assert_eq!(frame.gearbox_drive_mode_gear(), GearboxDriveModeGear::Gear8);
        assert_eq!(
            frame.lane_centering_indicator(),
            LaneCenteringIndicatorState::Steady
        );
        assert_eq!(
            frame.automatic_emergency_braking_indicator(),
            IndicatorState::Blinking
        );
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
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 3>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 4>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 5>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 6>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 7>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 0>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 1>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 3>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 1>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 2>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 3>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 4>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 5>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 2>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 5>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 6>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_5 }, 7>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 2>(), false);
        assert_eq!(
            frame.steering_assistance_fault_type(),
            SteeringAssistanceFaultType::G3AndG4
        );
        assert_eq!(frame.read_bit::<{ field::FLAGS_6 }, 5>(), true);
        assert_eq!(
            frame.steering_assistance_indicator(),
            SteeringAssistanceIndicatorState::Red
        );
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 0>(), false);
        assert_eq!(frame.gearbox_drive_mode_gear(), GearboxDriveModeGear::Gear3);
        assert_eq!(
            frame.lane_centering_indicator(),
            LaneCenteringIndicatorState::BlinkingFault
        );
        assert_eq!(
            frame.automatic_emergency_braking_indicator(),
            IndicatorState::Off
        );
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
        frame.write_bit::<{ field::FLAGS_2 }, 3>(false);
        frame.write_bit::<{ field::FLAGS_2 }, 4>(true);
        frame.write_bit::<{ field::FLAGS_2 }, 5>(false);
        frame.write_bit::<{ field::FLAGS_2 }, 6>(true);
        frame.write_bit::<{ field::FLAGS_2 }, 7>(false);
        frame.write_bit::<{ field::FLAGS_3 }, 0>(true);
        frame.write_bit::<{ field::FLAGS_3 }, 1>(false);
        frame.write_bit::<{ field::FLAGS_3 }, 3>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 1>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 2>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 3>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 4>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 5>(false);
        frame.write_bit::<{ field::FLAGS_5 }, 2>(true);
        frame.write_bit::<{ field::FLAGS_5 }, 5>(false);
        frame.write_bit::<{ field::FLAGS_5 }, 6>(true);
        frame.write_bit::<{ field::FLAGS_5 }, 7>(false);
        frame.write_bit::<{ field::FLAGS_6 }, 2>(true);
        frame.set_steering_assistance_fault_type(SteeringAssistanceFaultType::G3);
        frame.write_bit::<{ field::FLAGS_6 }, 5>(false);
        frame.set_steering_assistance_indicator(SteeringAssistanceIndicatorState::Orange);
        frame.write_bit::<{ field::FLAGS_7 }, 0>(true);
        frame.set_gearbox_drive_mode_gear(GearboxDriveModeGear::Gear8);
        frame.set_lane_centering_indicator(LaneCenteringIndicatorState::Steady);
        frame.set_automatic_emergency_braking_indicator(IndicatorState::Blinking);

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
        frame.write_bit::<{ field::FLAGS_2 }, 3>(true);
        frame.write_bit::<{ field::FLAGS_2 }, 4>(false);
        frame.write_bit::<{ field::FLAGS_2 }, 5>(true);
        frame.write_bit::<{ field::FLAGS_2 }, 6>(false);
        frame.write_bit::<{ field::FLAGS_2 }, 7>(true);
        frame.write_bit::<{ field::FLAGS_3 }, 0>(false);
        frame.write_bit::<{ field::FLAGS_3 }, 1>(true);
        frame.write_bit::<{ field::FLAGS_3 }, 3>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 1>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 2>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 3>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 4>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 5>(true);
        frame.write_bit::<{ field::FLAGS_5 }, 2>(false);
        frame.write_bit::<{ field::FLAGS_5 }, 5>(true);
        frame.write_bit::<{ field::FLAGS_5 }, 6>(false);
        frame.write_bit::<{ field::FLAGS_5 }, 7>(true);
        frame.write_bit::<{ field::FLAGS_6 }, 2>(false);
        frame.set_steering_assistance_fault_type(SteeringAssistanceFaultType::G3AndG4);
        frame.write_bit::<{ field::FLAGS_6 }, 5>(true);
        frame.set_steering_assistance_indicator(SteeringAssistanceIndicatorState::Red);
        frame.write_bit::<{ field::FLAGS_7 }, 0>(false);
        frame.set_gearbox_drive_mode_gear(GearboxDriveModeGear::Gear3);
        frame.set_lane_centering_indicator(LaneCenteringIndicatorState::BlinkingFault);
        frame.set_automatic_emergency_braking_indicator(IndicatorState::Off);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x55, 0x50, 0x01, 0x14, 0x44, 0x94, 0x11, 0x84, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x55, 0x50, 0x01, 0x14, 0x44, 0x94, 0x11];
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
