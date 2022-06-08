use core::{cmp::Ordering, fmt};

use byteorder::{ByteOrder, NetworkEndian};

use crate::{
    vehicle::{AdBlueIndicatorState, GearboxDriveModeGear, IndicatorState},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    use crate::field::Field;
    /// 1-bit under-inflation detection system failure flag,
    /// 1-bit cold engine alert flag,
    /// 1-bit low brake fluid level alert flag,
    /// 1-bit low oil pressure alert flag,
    /// 1-bit low oil level alert flag,
    /// 1-bit low coolant level alert flag,
    /// 1-bit oil temperature alert flag,
    /// 1-bit coolant temperature alert flag.
    pub const FLAGS_1: usize = 0;
    /// 1-bit max engine rpm level 2 indicator display flag,
    /// 1-bit low fuel level indicator blinking flag,
    /// 1-bit max engine rpm level 1 indicator display flag,
    /// 1-bit automatic wipers enabled flag,
    /// 1-bit particulate filter indicator display flag,
    /// 1-bit automatic stop indicator display flag,
    /// 1-bit tyre puncture alert flag,
    /// 1-bit under-inflation alert flag.
    pub const FLAGS_2: usize = 1;
    /// 2-bit foot on clutch pedal indicator state field,
    /// 1-bit rear right seat belt indicator blinking flag,
    /// 1-bit rear right seat belt indicator display flag,
    /// 1-bit rear middle seat belt indicator blinking flag,
    /// 1-bit rear middle seat belt indicator display flag,
    /// 1-bit rear left seat belt indicator blinking flag,
    /// 1-bit rear left seat belt indicator display flag.
    pub const FLAGS_3: usize = 2;
    /// 1-bit water in diesel fault flag,
    /// 1-bit OBD fault flag (displays the engine fault indicator),
    /// 1-bit worn brake pad fault flag,
    /// 1-bit gearbox fault flag,
    /// 1-bit ESP/ASR fault flag,
    /// 1-bit ABS fault flag,
    /// 1-bit suspension failure flag,
    /// 1-bit EBD fault flag.
    pub const FLAGS_4: usize = 3;
    /// 1-bit engine fault flag,
    /// 1-bit empty,
    /// 1-bit turn lights fault flag,
    /// 2-bit automatic levelling indicator state field,
    /// 4-bit gearbox drive mode engaged gear field,
    /// 1-bit electrical generator fault flag,
    /// 1-bit battery charge fault flag,
    /// 1-bit empty,
    /// 1-bit anti-emission system fault flag,
    /// 1-bit passive safety fault flag,
    /// 2-bit AdBlue indicator state flag,
    pub const FLAGS_5_6: Field = 4..6;
    /// 2-bit Stop & Start indicator state field,
    /// 1-bit engine fault indicator blinking flag,
    /// 1-bit electrical parking brake fault flag,
    /// 1-bit steering assistance fault flag,
    /// 3-bit empty.
    pub const FLAGS_7: usize = 6;
    /// 2-bit empty,
    /// 2-bit ZEV indicator state field,
    /// 2-bit empty,
    /// 1-bit OBD codes readiness flag,
    /// 1-bit fuse fault alert flag.
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

    /// Return the foot on clutch pedal indicator state field.
    #[inline]
    pub fn foot_on_clutch_pedal_indicator(&self) -> IndicatorState {
        let data = self.buffer.as_ref();
        let raw = data[field::FLAGS_3] & 0x03;
        IndicatorState::from(raw)
    }

    /// Return the engine fault flag.
    #[inline]
    pub fn engine_fault(&self) -> bool {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        raw & 0x0001 != 0
    }

    /// Return the turn lights fault flag.
    #[inline]
    pub fn turn_lights_fault(&self) -> bool {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        raw & 0x0004 != 0
    }

    /// Return the automatic levelling indicator state field.
    #[inline]
    pub fn automatic_levelling_indicator(&self) -> IndicatorState {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        let raw = (raw & 0x0018) >> 3;
        IndicatorState::from(raw as u8)
    }

    /// Return the gearbox drive mode engaged gear field.
    #[inline]
    pub fn gearbox_drive_mode_gear(&self) -> GearboxDriveModeGear {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        let raw = (raw & 0x01e8) >> 5;
        GearboxDriveModeGear::from(raw as u8)
    }

    /// Return the electrical generator fault flag.
    #[inline]
    pub fn electrical_generator_fault(&self) -> bool {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        raw & 0x0200 != 0
    }

    /// Return the battery charge fault flag.
    #[inline]
    pub fn battery_charge_fault(&self) -> bool {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        raw & 0x0400 != 0
    }

    /// Return the anti-emission system fault flag.
    #[inline]
    pub fn anti_emission_fault(&self) -> bool {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        raw & 0x1000 != 0
    }

    /// Return the passive safety fault flag.
    #[inline]
    pub fn passive_safety_fault(&self) -> bool {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        raw & 0x2000 != 0
    }

    /// Return the AdBlue indicator state field.
    #[inline]
    pub fn adblue_indicator(&self) -> AdBlueIndicatorState {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        let raw = raw >> 14;
        AdBlueIndicatorState::from(raw as u8)
    }

    /// Return the Stop & Start indicator state field.
    #[inline]
    pub fn stop_start_indicator(&self) -> IndicatorState {
        let data = self.buffer.as_ref();
        let raw = data[field::FLAGS_7] & 0x03;
        IndicatorState::from(raw)
    }

    /// Return the ZEV indicator state field.
    #[inline]
    pub fn zev_indicator(&self) -> IndicatorState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_8] & 0x0c) >> 2;
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

    /// Set the foot on clutch pedal indicator state field.
    #[inline]
    pub fn set_foot_on_clutch_pedal_indicator(&mut self, value: IndicatorState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_3] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::FLAGS_3] = raw;
    }

    /// Set the engine fault flag.
    #[inline]
    pub fn set_engine_fault(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        let raw = if value { raw | 0x0001 } else { raw & !0x0001 };
        NetworkEndian::write_u16(&mut data[field::FLAGS_5_6], raw);
    }

    /// Set the turn lights fault flag.
    #[inline]
    pub fn set_turn_lights_fault(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        let raw = if value { raw | 0x0004 } else { raw & !0x0004 };
        NetworkEndian::write_u16(&mut data[field::FLAGS_5_6], raw);
    }

    /// Set the automatic levelling indicator state field.
    #[inline]
    pub fn set_automatic_levelling_indicator(&mut self, value: IndicatorState) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]) & !0x0018;
        let value = (u8::from(value) as u16) << 3;
        let raw = raw | (value & 0x0018);
        NetworkEndian::write_u16(&mut data[field::FLAGS_5_6], raw);
    }

    /// Set the gearbox drive mode engaged gear field.
    #[inline]
    pub fn set_gearbox_drive_mode_gear(&mut self, value: GearboxDriveModeGear) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]) & !0x01e8;
        let value = (u8::from(value) as u16) << 5;
        let raw = raw | (value & 0x01e8);
        NetworkEndian::write_u16(&mut data[field::FLAGS_5_6], raw);
    }

    /// Set the electrical generator fault flag.
    #[inline]
    pub fn set_electrical_generator_fault(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        let raw = if value { raw | 0x0200 } else { raw & !0x0200 };
        NetworkEndian::write_u16(&mut data[field::FLAGS_5_6], raw);
    }

    /// Set the battery charge fault flag.
    #[inline]
    pub fn set_battery_charge_fault(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        let raw = if value { raw | 0x0400 } else { raw & !0x0400 };
        NetworkEndian::write_u16(&mut data[field::FLAGS_5_6], raw);
    }

    /// Set the anti-emission system fault flag.
    #[inline]
    pub fn set_anti_emission_fault(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        let raw = if value { raw | 0x1000 } else { raw & !0x1000 };
        NetworkEndian::write_u16(&mut data[field::FLAGS_5_6], raw);
    }

    /// Set the passive safety fault flag.
    #[inline]
    pub fn set_passive_safety_fault(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]);
        let raw = if value { raw | 0x2000 } else { raw & !0x2000 };
        NetworkEndian::write_u16(&mut data[field::FLAGS_5_6], raw);
    }

    /// Set the AdBlue indicator state field.
    #[inline]
    pub fn set_adblue_indicator(&mut self, value: AdBlueIndicatorState) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_5_6]) & !0xc000;
        let value = (u8::from(value) as u16) << 14;
        let raw = raw | value & 0xc000;
        NetworkEndian::write_u16(&mut data[field::FLAGS_5_6], raw);
    }

    /// Set the Stop & Start indicator state field.
    #[inline]
    pub fn set_stop_start_indicator(&mut self, value: IndicatorState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_7] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::FLAGS_7] = raw;
    }

    /// Set the ZEV indicator state field.
    #[inline]
    pub fn set_zev_indicator(&mut self, value: IndicatorState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_8] & !0x0c;
        let raw = raw | ((u8::from(value) << 2) & 0x0c);
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
    pub oil_temperature_alert: bool,
    pub coolant_temperature_alert: bool,
    pub max_engine_rpm_level2_indicator: bool,
    pub low_fuel_level_alert: bool,
    pub max_engine_rpm_level1_indicator: bool,
    pub automatic_wipers_enabled: bool,
    pub particulate_filter_indicator: bool,
    pub automatic_stop_indicator: bool,
    pub tyre_puncture_alert: bool,
    pub under_inflation_alert_flag: bool,
    pub foot_on_clutch_pedal_indicator: IndicatorState,
    pub rear_right_seat_belt_indicator_blinking: bool,
    pub rear_right_seat_belt_indicator: bool,
    pub rear_middle_seat_belt_indicator_blinking: bool,
    pub rear_middle_seat_belt_indicator: bool,
    pub rear_left_seat_belt_indicator_blinking: bool,
    pub rear_left_seat_belt_indicator: bool,
    pub water_in_diesel: bool,
    pub obd_fault: bool,
    pub worn_brake_pad_fault: bool,
    pub gearbox_fault: bool,
    pub esp_asr_fault: bool,
    pub abs_fault: bool,
    pub suspension_fault: bool,
    pub ebd_fault: bool,
    pub engine_fault: bool,
    pub turn_lights_fault: bool,
    pub automatic_levelling_indicator: IndicatorState,
    pub gearbox_drive_mode_gear: GearboxDriveModeGear,
    pub electrical_generator_fault: bool,
    pub battery_charge_fault: bool,
    pub anti_emission_fault: bool,
    pub passive_safety_fault: bool,
    pub adblue_indicator: AdBlueIndicatorState,
    pub stop_start_indicator: IndicatorState,
    pub engine_fault_indicator_blinking: bool,
    pub electrical_parking_brake_fault: bool,
    pub steering_assistance_fault: bool,
    pub zev_indicator: IndicatorState,
    pub obd_code_readiness: bool,
    pub fuse_fault: bool,
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
            oil_temperature_alert: frame.read_bit::<{ field::FLAGS_1 }, 6>(),
            coolant_temperature_alert: frame.read_bit::<{ field::FLAGS_1 }, 7>(),
            max_engine_rpm_level2_indicator: frame.read_bit::<{ field::FLAGS_2 }, 0>(),
            low_fuel_level_alert: frame.read_bit::<{ field::FLAGS_2 }, 1>(),
            max_engine_rpm_level1_indicator: frame.read_bit::<{ field::FLAGS_2 }, 2>(),
            automatic_wipers_enabled: frame.read_bit::<{ field::FLAGS_2 }, 3>(),
            particulate_filter_indicator: frame.read_bit::<{ field::FLAGS_2 }, 4>(),
            automatic_stop_indicator: frame.read_bit::<{ field::FLAGS_2 }, 5>(),
            tyre_puncture_alert: frame.read_bit::<{ field::FLAGS_2 }, 6>(),
            under_inflation_alert_flag: frame.read_bit::<{ field::FLAGS_2 }, 7>(),
            foot_on_clutch_pedal_indicator: frame.foot_on_clutch_pedal_indicator(),
            rear_right_seat_belt_indicator_blinking: frame.read_bit::<{ field::FLAGS_3 }, 2>(),
            rear_right_seat_belt_indicator: frame.read_bit::<{ field::FLAGS_3 }, 3>(),
            rear_middle_seat_belt_indicator_blinking: frame.read_bit::<{ field::FLAGS_3 }, 4>(),
            rear_middle_seat_belt_indicator: frame.read_bit::<{ field::FLAGS_3 }, 5>(),
            rear_left_seat_belt_indicator_blinking: frame.read_bit::<{ field::FLAGS_3 }, 6>(),
            rear_left_seat_belt_indicator: frame.read_bit::<{ field::FLAGS_3 }, 7>(),
            water_in_diesel: frame.read_bit::<{ field::FLAGS_4 }, 0>(),
            obd_fault: frame.read_bit::<{ field::FLAGS_4 }, 1>(),
            worn_brake_pad_fault: frame.read_bit::<{ field::FLAGS_4 }, 2>(),
            gearbox_fault: frame.read_bit::<{ field::FLAGS_4 }, 3>(),
            esp_asr_fault: frame.read_bit::<{ field::FLAGS_4 }, 4>(),
            abs_fault: frame.read_bit::<{ field::FLAGS_4 }, 5>(),
            suspension_fault: frame.read_bit::<{ field::FLAGS_4 }, 6>(),
            ebd_fault: frame.read_bit::<{ field::FLAGS_4 }, 7>(),
            engine_fault: frame.engine_fault(),
            turn_lights_fault: frame.turn_lights_fault(),
            automatic_levelling_indicator: frame.automatic_levelling_indicator(),
            gearbox_drive_mode_gear: frame.gearbox_drive_mode_gear(),
            electrical_generator_fault: frame.electrical_generator_fault(),
            battery_charge_fault: frame.battery_charge_fault(),
            anti_emission_fault: frame.anti_emission_fault(),
            passive_safety_fault: frame.passive_safety_fault(),
            adblue_indicator: frame.adblue_indicator(),
            stop_start_indicator: frame.stop_start_indicator(),
            engine_fault_indicator_blinking: frame.read_bit::<{ field::FLAGS_7 }, 2>(),
            electrical_parking_brake_fault: frame.read_bit::<{ field::FLAGS_7 }, 3>(),
            steering_assistance_fault: frame.read_bit::<{ field::FLAGS_7 }, 4>(),
            zev_indicator: frame.zev_indicator(),
            obd_code_readiness: frame.read_bit::<{ field::FLAGS_8 }, 6>(),
            fuse_fault: frame.read_bit::<{ field::FLAGS_8 }, 7>(),
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
        frame.write_bit::<{ field::FLAGS_1 }, 6>(self.oil_temperature_alert);
        frame.write_bit::<{ field::FLAGS_1 }, 7>(self.coolant_temperature_alert);
        frame.write_bit::<{ field::FLAGS_2 }, 0>(self.max_engine_rpm_level2_indicator);
        frame.write_bit::<{ field::FLAGS_2 }, 1>(self.low_fuel_level_alert);
        frame.write_bit::<{ field::FLAGS_2 }, 2>(self.max_engine_rpm_level1_indicator);
        frame.write_bit::<{ field::FLAGS_2 }, 3>(self.automatic_wipers_enabled);
        frame.write_bit::<{ field::FLAGS_2 }, 4>(self.particulate_filter_indicator);
        frame.write_bit::<{ field::FLAGS_2 }, 5>(self.automatic_stop_indicator);
        frame.write_bit::<{ field::FLAGS_2 }, 6>(self.tyre_puncture_alert);
        frame.write_bit::<{ field::FLAGS_2 }, 7>(self.under_inflation_alert_flag);
        frame.set_foot_on_clutch_pedal_indicator(self.foot_on_clutch_pedal_indicator);
        frame.write_bit::<{ field::FLAGS_3 }, 2>(self.rear_right_seat_belt_indicator_blinking);
        frame.write_bit::<{ field::FLAGS_3 }, 3>(self.rear_right_seat_belt_indicator);
        frame.write_bit::<{ field::FLAGS_3 }, 4>(self.rear_middle_seat_belt_indicator_blinking);
        frame.write_bit::<{ field::FLAGS_3 }, 5>(self.rear_middle_seat_belt_indicator);
        frame.write_bit::<{ field::FLAGS_3 }, 6>(self.rear_left_seat_belt_indicator_blinking);
        frame.write_bit::<{ field::FLAGS_3 }, 7>(self.rear_left_seat_belt_indicator);
        frame.write_bit::<{ field::FLAGS_4 }, 0>(self.water_in_diesel);
        frame.write_bit::<{ field::FLAGS_4 }, 1>(self.obd_fault);
        frame.write_bit::<{ field::FLAGS_4 }, 2>(self.worn_brake_pad_fault);
        frame.write_bit::<{ field::FLAGS_4 }, 3>(self.gearbox_fault);
        frame.write_bit::<{ field::FLAGS_4 }, 4>(self.esp_asr_fault);
        frame.write_bit::<{ field::FLAGS_4 }, 5>(self.abs_fault);
        frame.write_bit::<{ field::FLAGS_4 }, 6>(self.suspension_fault);
        frame.write_bit::<{ field::FLAGS_4 }, 7>(self.ebd_fault);
        frame.set_engine_fault(self.engine_fault);
        frame.set_turn_lights_fault(self.turn_lights_fault);
        frame.set_automatic_levelling_indicator(self.automatic_levelling_indicator);
        frame.set_gearbox_drive_mode_gear(self.gearbox_drive_mode_gear);
        frame.set_electrical_generator_fault(self.electrical_generator_fault);
        frame.set_battery_charge_fault(self.battery_charge_fault);
        frame.set_anti_emission_fault(self.anti_emission_fault);
        frame.set_passive_safety_fault(self.passive_safety_fault);
        frame.set_adblue_indicator(self.adblue_indicator);
        frame.set_stop_start_indicator(self.stop_start_indicator);
        frame.write_bit::<{ field::FLAGS_7 }, 2>(self.engine_fault_indicator_blinking);
        frame.write_bit::<{ field::FLAGS_7 }, 3>(self.electrical_parking_brake_fault);
        frame.write_bit::<{ field::FLAGS_7 }, 4>(self.steering_assistance_fault);
        frame.set_zev_indicator(self.zev_indicator);
        frame.write_bit::<{ field::FLAGS_8 }, 6>(self.obd_code_readiness);
        frame.write_bit::<{ field::FLAGS_8 }, 7>(self.fuse_fault);
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
        writeln!(f, " oil_temperature_alert={}", self.oil_temperature_alert)?;
        writeln!(
            f,
            " coolant_temperature_alert={}",
            self.coolant_temperature_alert
        )?;
        writeln!(
            f,
            " max_engine_rpm_level2_indicator={}",
            self.max_engine_rpm_level2_indicator
        )?;
        writeln!(f, " low_fuel_level_alert={}", self.low_fuel_level_alert)?;
        writeln!(
            f,
            " max_engine_rpm_level1_indicator={}",
            self.max_engine_rpm_level1_indicator
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
        writeln!(
            f,
            " automatic_stop_indicator={}",
            self.automatic_stop_indicator
        )?;
        writeln!(f, " tyre_puncture_alert={}", self.tyre_puncture_alert)?;
        writeln!(
            f,
            " under_inflation_alert_flag={}",
            self.under_inflation_alert_flag
        )?;
        writeln!(
            f,
            " foot_on_clutch_pedal_indicator={}",
            self.foot_on_clutch_pedal_indicator
        )?;
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
            " rear_left_seat_belt_indicator={}",
            self.rear_left_seat_belt_indicator
        )?;
        writeln!(f, " water_in_diesel={}", self.water_in_diesel)?;
        writeln!(f, " obd_fault={}", self.obd_fault)?;
        writeln!(f, " worn_brake_pad_fault={}", self.worn_brake_pad_fault)?;
        writeln!(f, " gearbox_fault={}", self.gearbox_fault)?;
        writeln!(f, " esp_asr_fault={}", self.esp_asr_fault)?;
        writeln!(f, " abs_fault={}", self.abs_fault)?;
        writeln!(f, " suspension_fault={}", self.suspension_fault)?;
        writeln!(f, " ebd_fault={}", self.ebd_fault)?;
        writeln!(f, " engine_fault={}", self.engine_fault)?;
        writeln!(f, " turn_lights_fault={}", self.turn_lights_fault)?;
        writeln!(
            f,
            " automatic_levelling_indicator={}",
            self.automatic_levelling_indicator
        )?;
        writeln!(
            f,
            " gearbox_drive_mode_gear={}",
            self.gearbox_drive_mode_gear
        )?;
        writeln!(
            f,
            " electrical_generator_fault={}",
            self.electrical_generator_fault
        )?;
        writeln!(f, " battery_charge_fault={}", self.battery_charge_fault)?;
        writeln!(f, " anti_emission_fault={}", self.anti_emission_fault)?;
        writeln!(f, " passive_safety_fault={}", self.passive_safety_fault)?;
        writeln!(f, " adblue_indicator={}", self.adblue_indicator)?;
        writeln!(f, " stop_start_indicator={}", self.stop_start_indicator)?;
        writeln!(
            f,
            " engine_fault_indicator_blinking={}",
            self.engine_fault_indicator_blinking
        )?;
        writeln!(
            f,
            " electrical_parking_brake_fault={}",
            self.electrical_parking_brake_fault
        )?;
        writeln!(
            f,
            " steering_assistance_fault={}",
            self.steering_assistance_fault
        )?;
        writeln!(f, " zev_indicator={}", self.zev_indicator)?;
        writeln!(f, " obd_code_readiness={}", self.obd_code_readiness)?;
        writeln!(f, " fuse_fault={}", self.fuse_fault)
    }
}

#[cfg(test)]
mod test {
    use super::{field, Frame, Repr};
    use crate::{
        vehicle::{AdBlueIndicatorState, GearboxDriveModeGear, IndicatorState},
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x55, 0x55, 0x55, 0x55, 0x93, 0x11, 0x16, 0x80];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0xaa, 0xaa, 0xa8, 0xaa, 0x64, 0x64, 0x08, 0x44];

    fn frame_1_repr() -> Repr {
        Repr {
            under_inflation_failure: true,
            cold_engine_alert: false,
            low_brake_fluid_level_alert: true,
            low_oil_pressure_alert: false,
            low_oil_level_alert: true,
            low_coolant_level_alert: false,
            oil_temperature_alert: true,
            coolant_temperature_alert: false,
            max_engine_rpm_level2_indicator: true,
            low_fuel_level_alert: false,
            max_engine_rpm_level1_indicator: true,
            automatic_wipers_enabled: false,
            particulate_filter_indicator: true,
            automatic_stop_indicator: false,
            tyre_puncture_alert: true,
            under_inflation_alert_flag: false,
            foot_on_clutch_pedal_indicator: IndicatorState::On,
            rear_right_seat_belt_indicator_blinking: true,
            rear_right_seat_belt_indicator: false,
            rear_middle_seat_belt_indicator_blinking: true,
            rear_middle_seat_belt_indicator: false,
            rear_left_seat_belt_indicator_blinking: true,
            rear_left_seat_belt_indicator: false,
            water_in_diesel: true,
            obd_fault: false,
            worn_brake_pad_fault: true,
            gearbox_fault: false,
            esp_asr_fault: true,
            abs_fault: false,
            suspension_fault: true,
            ebd_fault: false,
            engine_fault: true,
            turn_lights_fault: false,
            automatic_levelling_indicator: IndicatorState::Blinking,
            gearbox_drive_mode_gear: GearboxDriveModeGear::Gear8,
            electrical_generator_fault: true,
            battery_charge_fault: false,
            anti_emission_fault: true,
            passive_safety_fault: false,
            adblue_indicator: AdBlueIndicatorState::On,
            stop_start_indicator: IndicatorState::Blinking,
            engine_fault_indicator_blinking: true,
            electrical_parking_brake_fault: false,
            steering_assistance_fault: true,
            zev_indicator: IndicatorState::Off,
            obd_code_readiness: false,
            fuse_fault: true,
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
            oil_temperature_alert: false,
            coolant_temperature_alert: true,
            max_engine_rpm_level2_indicator: false,
            low_fuel_level_alert: true,
            max_engine_rpm_level1_indicator: false,
            automatic_wipers_enabled: true,
            particulate_filter_indicator: false,
            automatic_stop_indicator: true,
            tyre_puncture_alert: false,
            under_inflation_alert_flag: true,
            foot_on_clutch_pedal_indicator: IndicatorState::Off,
            rear_right_seat_belt_indicator_blinking: false,
            rear_right_seat_belt_indicator: true,
            rear_middle_seat_belt_indicator_blinking: false,
            rear_middle_seat_belt_indicator: true,
            rear_left_seat_belt_indicator_blinking: false,
            rear_left_seat_belt_indicator: true,
            water_in_diesel: false,
            obd_fault: true,
            worn_brake_pad_fault: false,
            gearbox_fault: true,
            esp_asr_fault: false,
            abs_fault: true,
            suspension_fault: false,
            ebd_fault: true,
            engine_fault: false,
            turn_lights_fault: true,
            automatic_levelling_indicator: IndicatorState::Off,
            gearbox_drive_mode_gear: GearboxDriveModeGear::Gear3,
            electrical_generator_fault: false,
            battery_charge_fault: true,
            anti_emission_fault: false,
            passive_safety_fault: true,
            adblue_indicator: AdBlueIndicatorState::Blinking,
            stop_start_indicator: IndicatorState::Off,
            engine_fault_indicator_blinking: false,
            electrical_parking_brake_fault: true,
            steering_assistance_fault: false,
            zev_indicator: IndicatorState::On,
            obd_code_readiness: true,
            fuse_fault: false,
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
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 1>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 2>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 3>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 4>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 5>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 6>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 7>(), false);
        assert_eq!(frame.foot_on_clutch_pedal_indicator(), IndicatorState::On);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 2>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 3>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 4>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 5>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 6>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 7>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 0>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 1>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 2>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 3>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 4>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 5>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 6>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 7>(), false);
        assert_eq!(frame.engine_fault(), true);
        assert_eq!(frame.turn_lights_fault(), false);
        assert_eq!(
            frame.automatic_levelling_indicator(),
            IndicatorState::Blinking
        );
        assert_eq!(frame.gearbox_drive_mode_gear(), GearboxDriveModeGear::Gear8);
        assert_eq!(frame.electrical_generator_fault(), true);
        assert_eq!(frame.battery_charge_fault(), false);
        assert_eq!(frame.anti_emission_fault(), true);
        assert_eq!(frame.passive_safety_fault(), false);
        assert_eq!(frame.adblue_indicator(), AdBlueIndicatorState::On);
        assert_eq!(frame.stop_start_indicator(), IndicatorState::Blinking);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 2>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 3>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 4>(), true);
        assert_eq!(frame.zev_indicator(), IndicatorState::Off);
        assert_eq!(frame.read_bit::<{ field::FLAGS_8 }, 6>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_8 }, 7>(), true);
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
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 1>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 2>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 3>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 4>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 5>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 6>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_2 }, 7>(), true);
        assert_eq!(frame.foot_on_clutch_pedal_indicator(), IndicatorState::Off);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 2>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 3>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 4>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 5>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 6>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_3 }, 7>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 0>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 1>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 2>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 3>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 4>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 5>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 6>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_4 }, 7>(), true);
        assert_eq!(frame.engine_fault(), false);
        assert_eq!(frame.turn_lights_fault(), true);
        assert_eq!(frame.automatic_levelling_indicator(), IndicatorState::Off);
        assert_eq!(frame.gearbox_drive_mode_gear(), GearboxDriveModeGear::Gear3);
        assert_eq!(frame.electrical_generator_fault(), false);
        assert_eq!(frame.battery_charge_fault(), true);
        assert_eq!(frame.anti_emission_fault(), false);
        assert_eq!(frame.passive_safety_fault(), true);
        assert_eq!(frame.adblue_indicator(), AdBlueIndicatorState::Blinking);
        assert_eq!(frame.stop_start_indicator(), IndicatorState::Off);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 2>(), false);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 3>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_7 }, 4>(), false);
        assert_eq!(frame.zev_indicator(), IndicatorState::On);
        assert_eq!(frame.read_bit::<{ field::FLAGS_8 }, 6>(), true);
        assert_eq!(frame.read_bit::<{ field::FLAGS_8 }, 7>(), false);
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
        frame.write_bit::<{ field::FLAGS_2 }, 1>(false);
        frame.write_bit::<{ field::FLAGS_2 }, 2>(true);
        frame.write_bit::<{ field::FLAGS_2 }, 3>(false);
        frame.write_bit::<{ field::FLAGS_2 }, 4>(true);
        frame.write_bit::<{ field::FLAGS_2 }, 5>(false);
        frame.write_bit::<{ field::FLAGS_2 }, 6>(true);
        frame.write_bit::<{ field::FLAGS_2 }, 7>(false);
        frame.set_foot_on_clutch_pedal_indicator(IndicatorState::On);
        frame.write_bit::<{ field::FLAGS_3 }, 2>(true);
        frame.write_bit::<{ field::FLAGS_3 }, 3>(false);
        frame.write_bit::<{ field::FLAGS_3 }, 4>(true);
        frame.write_bit::<{ field::FLAGS_3 }, 5>(false);
        frame.write_bit::<{ field::FLAGS_3 }, 6>(true);
        frame.write_bit::<{ field::FLAGS_3 }, 7>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 0>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 1>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 2>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 3>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 4>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 5>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 6>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 7>(false);
        frame.set_engine_fault(true);
        frame.set_turn_lights_fault(false);
        frame.set_automatic_levelling_indicator(IndicatorState::Blinking);
        frame.set_gearbox_drive_mode_gear(GearboxDriveModeGear::Gear8);
        frame.set_electrical_generator_fault(true);
        frame.set_battery_charge_fault(false);
        frame.set_anti_emission_fault(true);
        frame.set_passive_safety_fault(false);
        frame.set_adblue_indicator(AdBlueIndicatorState::On);
        frame.set_stop_start_indicator(IndicatorState::Blinking);
        frame.write_bit::<{ field::FLAGS_7 }, 2>(true);
        frame.write_bit::<{ field::FLAGS_7 }, 3>(false);
        frame.write_bit::<{ field::FLAGS_7 }, 4>(true);
        frame.set_zev_indicator(IndicatorState::Off);
        frame.write_bit::<{ field::FLAGS_8 }, 6>(false);
        frame.write_bit::<{ field::FLAGS_8 }, 7>(true);

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
        frame.write_bit::<{ field::FLAGS_2 }, 1>(true);
        frame.write_bit::<{ field::FLAGS_2 }, 2>(false);
        frame.write_bit::<{ field::FLAGS_2 }, 3>(true);
        frame.write_bit::<{ field::FLAGS_2 }, 4>(false);
        frame.write_bit::<{ field::FLAGS_2 }, 5>(true);
        frame.write_bit::<{ field::FLAGS_2 }, 6>(false);
        frame.write_bit::<{ field::FLAGS_2 }, 7>(true);
        frame.set_foot_on_clutch_pedal_indicator(IndicatorState::Off);
        frame.write_bit::<{ field::FLAGS_3 }, 2>(false);
        frame.write_bit::<{ field::FLAGS_3 }, 3>(true);
        frame.write_bit::<{ field::FLAGS_3 }, 4>(false);
        frame.write_bit::<{ field::FLAGS_3 }, 5>(true);
        frame.write_bit::<{ field::FLAGS_3 }, 6>(false);
        frame.write_bit::<{ field::FLAGS_3 }, 7>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 0>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 1>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 2>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 3>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 4>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 5>(true);
        frame.write_bit::<{ field::FLAGS_4 }, 6>(false);
        frame.write_bit::<{ field::FLAGS_4 }, 7>(true);
        frame.set_engine_fault(false);
        frame.set_turn_lights_fault(true);
        frame.set_automatic_levelling_indicator(IndicatorState::Off);
        frame.set_gearbox_drive_mode_gear(GearboxDriveModeGear::Gear3);
        frame.set_electrical_generator_fault(false);
        frame.set_battery_charge_fault(true);
        frame.set_anti_emission_fault(false);
        frame.set_passive_safety_fault(true);
        frame.set_adblue_indicator(AdBlueIndicatorState::Blinking);
        frame.set_stop_start_indicator(IndicatorState::Off);
        frame.write_bit::<{ field::FLAGS_7 }, 2>(false);
        frame.write_bit::<{ field::FLAGS_7 }, 3>(true);
        frame.write_bit::<{ field::FLAGS_7 }, 4>(false);
        frame.set_zev_indicator(IndicatorState::On);
        frame.write_bit::<{ field::FLAGS_8 }, 6>(true);
        frame.write_bit::<{ field::FLAGS_8 }, 7>(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x55, 0x55, 0x55, 0x55, 0x93, 0x11, 0x16, 0x80, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x55, 0x55, 0x55, 0x55, 0x93, 0x11, 0x16];
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
