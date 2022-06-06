use core::{cmp::Ordering, fmt, time::Duration};

use time::Duration as TimeDuration;

use crate::{
    vehicle::{EngineState, StopAndStartSystemState, TractionBatteryChargeState},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 6-bit Stop & Start number of minutes engine stopped counter field,
    /// 2-bit Stop & Start number of hours engine stopped counter field.
    pub const STT_CNT_1: usize = 0;
    /// 2-bit unknown,
    /// 6-bit Stop & Start number of seconds engine stopped counter field.
    pub const STT_CNT_2: usize = 1;
    /// 8-bit unknown.
    pub const _UNKNOWN: usize = 2;
    /// 2-bit electrical engine state field,
    /// 2-bit petrol engine state field,
    /// 1-bit zero emission request flag,
    /// 1-bit Stop & Start presence flag,
    /// 2-bit Stop & Start system state field.
    pub const ENGINE_STATE: usize = 3;
    /// 6-bit unknown,
    /// 2-bit traction battery charge state field.
    pub const BATT_CHARGE: usize = 4;
}

/// Length of a x3e1 CAN frame.
pub const FRAME_LEN: usize = field::BATT_CHARGE + 1;

/// Periodicity of a x3e1 CAN frame.
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

    /// Return the Stop & Start number of minutes engine stopped counter field.
    #[inline]
    pub fn stop_start_minutes_stopped(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::STT_CNT_1] & 0x3f
    }

    /// Return the Stop & Start number of hours engine stopped counter field.
    #[inline]
    pub fn stop_start_hours_stopped(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::STT_CNT_1] & 0xc0) >> 6
    }

    /// Return the Stop & Start number of seconds engine stopped counter field.
    #[inline]
    pub fn stop_start_seconds_stopped(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::STT_CNT_2] >> 2
    }

    /// Return the electrical engine state field.
    #[inline]
    pub fn electrical_engine_state(&self) -> EngineState {
        let data = self.buffer.as_ref();
        let raw = data[field::ENGINE_STATE] & 0x03;
        EngineState::from(raw)
    }

    /// Return the petrol engine state field.
    #[inline]
    pub fn petrol_engine_state(&self) -> EngineState {
        let data = self.buffer.as_ref();
        let raw = (data[field::ENGINE_STATE] & 0x0c) >> 2;
        EngineState::from(raw)
    }

    /// Return the zero emission request flag.
    #[inline]
    pub fn zero_emission_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::ENGINE_STATE] & 0x10 != 0
    }

    /// Return the Stop & Start presence flag.
    #[inline]
    pub fn stop_and_start_present(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::ENGINE_STATE] & 0x20 != 0
    }

    /// Return the Stop & Start system state field.
    #[inline]
    pub fn stop_and_start_state(&self) -> StopAndStartSystemState {
        let data = self.buffer.as_ref();
        let raw = data[field::ENGINE_STATE] >> 6;
        StopAndStartSystemState::from(raw)
    }

    /// Return the traction battery recharge state.
    #[inline]
    pub fn traction_battery_charge_state(&self) -> TractionBatteryChargeState {
        let data = self.buffer.as_ref();
        let raw = data[field::BATT_CHARGE] >> 6;
        TractionBatteryChargeState::from(raw)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the Stop & Start number of minutes engine stopped counter field.
    #[inline]
    pub fn set_stop_start_minutes_stopped(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::STT_CNT_1] & !0x3f;
        let raw = raw | (value & 0x3f);
        data[field::STT_CNT_1] = raw;
    }

    /// Set the Stop & Start number of hours engine stopped counter field.
    #[inline]
    pub fn set_stop_start_hours_stopped(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::STT_CNT_1] & !0xc0;
        let raw = raw | ((value << 6) & 0xc0);
        data[field::STT_CNT_1] = raw;
    }

    /// Set the Stop & Start number of seconds engine stopped counter field.
    #[inline]
    pub fn set_stop_start_seconds_stopped(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::STT_CNT_2] & !0xfc;
        let raw = raw | ((value << 2) & 0xfc);
        data[field::STT_CNT_2] = raw;
    }

    /// Set the electrical engine state field.
    #[inline]
    pub fn set_electrical_engine_state(&mut self, value: EngineState) {
        let data = self.buffer.as_mut();
        let raw = data[field::ENGINE_STATE] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::ENGINE_STATE] = raw;
    }

    /// Set the petrol engine state field.
    #[inline]
    pub fn set_petrol_engine_state(&mut self, value: EngineState) {
        let data = self.buffer.as_mut();
        let raw = data[field::ENGINE_STATE] & !0x0c;
        let raw = raw | ((u8::from(value) << 2) & 0x0c);
        data[field::ENGINE_STATE] = raw;
    }

    /// Set the zero emission request flag.
    #[inline]
    pub fn set_zero_emission_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::ENGINE_STATE] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::ENGINE_STATE] = raw;
    }

    /// Set the Stop & Start presence flag.
    #[inline]
    pub fn set_stop_and_start_present(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::ENGINE_STATE] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::ENGINE_STATE] = raw;
    }

    /// Set the vehicle supervision module config mode field.
    #[inline]
    pub fn set_stop_and_start_state(&mut self, value: StopAndStartSystemState) {
        let data = self.buffer.as_mut();
        let raw = data[field::ENGINE_STATE] & !0xc0;
        let raw = raw | ((u8::from(value) << 6) & 0xc0);
        data[field::ENGINE_STATE] = raw;
    }

    /// Set the traction battery recharge state.
    #[inline]
    pub fn set_traction_battery_charge_state(&mut self, value: TractionBatteryChargeState) {
        let data = self.buffer.as_mut();
        let raw = data[field::BATT_CHARGE] & !0xc0;
        let raw = raw | ((u8::from(value) << 6) & 0xc0);
        data[field::BATT_CHARGE] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x3e1 ({})", err)?;
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

/// A high-level representation of a x3e1 CAN frame.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub stop_start_stopped_duration: TimeDuration,
    pub electrical_engine_state: EngineState,
    pub petrol_engine_state: EngineState,
    pub zero_emission_request: bool,
    pub stop_start_presence: bool,
    pub stop_start_state: StopAndStartSystemState,
    pub traction_battery_charge_state: TractionBatteryChargeState,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        if frame.stop_start_seconds_stopped() >= 60 || frame.stop_start_minutes_stopped() >= 60 {
            Err(Error::Invalid)
        } else {
            let stt_duration = TimeDuration::seconds(frame.stop_start_seconds_stopped().into())
                + TimeDuration::minutes(frame.stop_start_minutes_stopped().into())
                + TimeDuration::hours(frame.stop_start_hours_stopped().into());

            Ok(Repr {
                stop_start_stopped_duration: stt_duration,
                electrical_engine_state: frame.electrical_engine_state(),
                petrol_engine_state: frame.petrol_engine_state(),
                zero_emission_request: frame.zero_emission_request(),
                stop_start_presence: frame.stop_and_start_present(),
                stop_start_state: frame.stop_and_start_state(),
                traction_battery_charge_state: frame.traction_battery_charge_state(),
            })
        }
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x3e1 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        let mut whole_seconds = self.stop_start_stopped_duration.whole_seconds();
        let mut whole_minutes = self.stop_start_stopped_duration.whole_minutes();
        let whole_hours = self.stop_start_stopped_duration.whole_hours();

        let (stt_stopped_hours, stt_stopped_minutes, stt_stopped_seconds) = if whole_hours >= 1 {
            whole_minutes -= whole_hours * 60;
            whole_seconds -= (whole_hours * 3600) + (whole_minutes * 60);

            (whole_hours, whole_minutes, whole_seconds)
        } else {
            whole_seconds -= whole_minutes * 60;
            (whole_hours, whole_minutes, whole_seconds)
        };

        frame.set_stop_start_minutes_stopped(stt_stopped_minutes as u8);
        frame.set_stop_start_hours_stopped(stt_stopped_hours as u8);
        frame.set_stop_start_seconds_stopped(stt_stopped_seconds as u8);
        frame.set_electrical_engine_state(self.electrical_engine_state);
        frame.set_petrol_engine_state(self.petrol_engine_state);
        frame.set_zero_emission_request(self.zero_emission_request);
        frame.set_stop_and_start_present(self.stop_start_presence);
        frame.set_stop_and_start_state(self.stop_start_state);
        frame.set_traction_battery_charge_state(self.traction_battery_charge_state);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x3e1")?;

        writeln!(
            f,
            " stop_start_stopped_duration={}",
            self.stop_start_stopped_duration.whole_seconds()
        )?;
        writeln!(
            f,
            " electrical_engine_state={}",
            self.electrical_engine_state
        )?;
        writeln!(f, " petrol_engine_state={}", self.petrol_engine_state)?;
        writeln!(f, " zero_emission_request={}", self.zero_emission_request)?;
        writeln!(f, " stop_start_presence={}", self.stop_start_presence)?;
        writeln!(f, " stop_start_state={}", self.stop_start_state)?;
        writeln!(
            f,
            " traction_battery_charge_state={}",
            self.traction_battery_charge_state
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        vehicle::{EngineState, StopAndStartSystemState, TractionBatteryChargeState},
        Error,
    };

    use time::Duration as TimeDuration;

    static REPR_FRAME_BYTES_1: [u8; 5] = [0x02, 0xe0, 0x00, 0x64, 0x40];
    static REPR_FRAME_BYTES_2: [u8; 5] = [0x42, 0xe0, 0x00, 0x12, 0x80];

    fn frame_1_repr() -> Repr {
        Repr {
            stop_start_stopped_duration: TimeDuration::seconds(56) + TimeDuration::minutes(2),
            electrical_engine_state: EngineState::Disabled,
            petrol_engine_state: EngineState::Driving,
            zero_emission_request: false,
            stop_start_presence: true,
            stop_start_state: StopAndStartSystemState::Enabled,
            traction_battery_charge_state: TractionBatteryChargeState::Recharge,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            stop_start_stopped_duration: TimeDuration::seconds(56)
                + TimeDuration::minutes(2)
                + TimeDuration::hours(1),
            electrical_engine_state: EngineState::Braking,
            petrol_engine_state: EngineState::Disabled,
            zero_emission_request: true,
            stop_start_presence: false,
            stop_start_state: StopAndStartSystemState::Unavailable,
            traction_battery_charge_state: TractionBatteryChargeState::Discharge,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.stop_start_seconds_stopped(), 56);
        assert_eq!(frame.stop_start_minutes_stopped(), 2);
        assert_eq!(frame.stop_start_hours_stopped(), 0);
        assert_eq!(frame.electrical_engine_state(), EngineState::Disabled);
        assert_eq!(frame.petrol_engine_state(), EngineState::Driving);
        assert_eq!(frame.zero_emission_request(), false);
        assert_eq!(frame.stop_and_start_present(), true);
        assert_eq!(
            frame.stop_and_start_state(),
            StopAndStartSystemState::Enabled
        );
        assert_eq!(
            frame.traction_battery_charge_state(),
            TractionBatteryChargeState::Recharge
        );
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.stop_start_seconds_stopped(), 56);
        assert_eq!(frame.stop_start_minutes_stopped(), 2);
        assert_eq!(frame.stop_start_hours_stopped(), 1);
        assert_eq!(frame.electrical_engine_state(), EngineState::Braking);
        assert_eq!(frame.petrol_engine_state(), EngineState::Disabled);
        assert_eq!(frame.zero_emission_request(), true);
        assert_eq!(frame.stop_and_start_present(), false);
        assert_eq!(
            frame.stop_and_start_state(),
            StopAndStartSystemState::Unavailable
        );
        assert_eq!(
            frame.traction_battery_charge_state(),
            TractionBatteryChargeState::Discharge
        );
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 5];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_stop_start_minutes_stopped(2);
        frame.set_stop_start_hours_stopped(0);
        frame.set_stop_start_seconds_stopped(56);
        frame.set_electrical_engine_state(EngineState::Disabled);
        frame.set_petrol_engine_state(EngineState::Driving);
        frame.set_zero_emission_request(false);
        frame.set_stop_and_start_present(true);
        frame.set_stop_and_start_state(StopAndStartSystemState::Enabled);
        frame.set_traction_battery_charge_state(TractionBatteryChargeState::Recharge);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 5];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_stop_start_minutes_stopped(2);
        frame.set_stop_start_hours_stopped(1);
        frame.set_stop_start_seconds_stopped(56);
        frame.set_electrical_engine_state(EngineState::Braking);
        frame.set_petrol_engine_state(EngineState::Disabled);
        frame.set_zero_emission_request(true);
        frame.set_stop_and_start_present(false);
        frame.set_stop_and_start_state(StopAndStartSystemState::Unavailable);
        frame.set_traction_battery_charge_state(TractionBatteryChargeState::Discharge);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 6] = [0x02, 0xe0, 0x00, 0xa4, 0x40, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 4] = [0x02, 0xe0, 0x00, 0xa4];
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
        let mut buf = [0u8; 5];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_basic_repr_2_emit() {
        let mut buf = [0u8; 5];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_2_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }
}
