use core::{fmt, time::Duration};

use crate::{
    vehicle::{
        BootAndConvertibleRoofPosition, EnhancedTractionControlMode, FunctionState, SuspensionMode,
        SuspensionMovement, SuspensionPosition,
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
    /// 2-bit automatic door locking when driving function state field,
    /// 2-bit automatic headlamps function state field,
    /// 2-bit passenger airbag function state field,
    /// 2-bit parking sensors function state field.
    pub const FN_0: usize = 0;
    /// 2-bit settable suspension function mode field,
    /// 2-bit automatic wipers function state field,
    /// 2-bit ESP function state field,
    /// 2-bit door locking function state field.
    pub const FN_1: usize = 1;
    /// 1-bit empty,
    /// 3-bit boot and convertible roof position field,
    /// 2-bit Stop & Start function state field,
    /// 2-bit rear doors child lock function state field.
    pub const FN_2: usize = 2;
    /// 2-bit settable suspension movement type field,
    /// 3-bit final settable suspension position field,
    /// 3-bit initial settable suspension position field.
    pub const FN_3: usize = 3;
    /// 1-bit empty,
    /// 3-bit enhanced traction control function state field,
    /// 1-bit settable suspension warning flag,
    /// 3-bit real settable suspension position field.
    pub const FN_4: usize = 4;
}

/// Length of a x2e1 CAN frame.
pub const FRAME_LEN: usize = field::FN_4 + 1;

/// Periodicity of a x2e1 CAN frame.
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

    /// Return the automatic door locking when driving function state field.
    #[inline]
    pub fn auto_door_locking_when_driving_state(&self) -> FunctionState {
        let data = self.buffer.as_ref();
        let raw = data[field::FN_0] & 0x03;
        FunctionState::from(raw)
    }

    /// Return the automatic headlamps function state field.
    #[inline]
    pub fn automatic_headlamps_state(&self) -> FunctionState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_0] & 0x0c) >> 2;
        FunctionState::from(raw)
    }

    /// Return the passenger airbag function state field.
    #[inline]
    pub fn passenger_airbag_state(&self) -> FunctionState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_0] & 0x30) >> 4;
        FunctionState::from(raw)
    }

    /// Return the parking sensors function state field.
    #[inline]
    pub fn park_sensors_state(&self) -> FunctionState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_0] & 0xc0) >> 6;
        FunctionState::from(raw)
    }

    /// Return the settable suspension mode field.
    #[inline]
    pub fn settable_suspension_mode(&self) -> SuspensionMode {
        let data = self.buffer.as_ref();
        let raw = data[field::FN_1] & 0x03;
        SuspensionMode::from(raw)
    }

    /// Return automatic wipers function state field.
    #[inline]
    pub fn automatic_wipers_state(&self) -> FunctionState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_1] & 0x0c) >> 2;
        FunctionState::from(raw)
    }

    /// Return the ESP function state field.
    #[inline]
    pub fn esp_state(&self) -> FunctionState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_1] & 0x30) >> 4;
        FunctionState::from(raw)
    }

    /// Return the door locking function state field.
    #[inline]
    pub fn door_locking_state(&self) -> FunctionState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_1] & 0xc0) >> 6;
        FunctionState::from(raw)
    }

    /// Return the boot and convertible roof position field.
    #[inline]
    pub fn boot_and_convertible_roof_position(&self) -> BootAndConvertibleRoofPosition {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_2] & 0x0e) >> 1;
        BootAndConvertibleRoofPosition::from(raw)
    }

    /// Return the Stop & Start function state field.
    #[inline]
    pub fn stop_start_state(&self) -> FunctionState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_2] & 0x30) >> 4;
        FunctionState::from(raw)
    }

    /// Return the rear doors child lock function state field.
    #[inline]
    pub fn rear_doors_child_lock(&self) -> FunctionState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_2] & 0xc0) >> 6;
        FunctionState::from(raw)
    }

    /// Return the settable suspension movement type field.
    #[inline]
    pub fn settable_suspension_movement_type(&self) -> SuspensionMovement {
        let data = self.buffer.as_ref();
        let raw = data[field::FN_3] & 0x03;
        SuspensionMovement::from(raw)
    }

    /// Return the final settable suspension position field.
    #[inline]
    pub fn final_settable_suspension_position(&self) -> SuspensionPosition {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_3] & 0x1c) >> 2;
        SuspensionPosition::from(raw)
    }

    /// Return the initial settable suspension position field.
    #[inline]
    pub fn initial_settable_suspension_position(&self) -> SuspensionPosition {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_3] & 0xe0) >> 5;
        SuspensionPosition::from(raw)
    }

    /// Return the enhanced traction control function state field.
    #[inline]
    pub fn enhanced_asr_state(&self) -> EnhancedTractionControlMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_4] & 0x0e) >> 1;
        EnhancedTractionControlMode::from(raw)
    }

    /// Return the settable suspension warning flag.
    #[inline]
    pub fn settable_suspension_warning(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FN_4] & 0x10 != 0
    }

    /// Return the real settable suspension position field.
    #[inline]
    pub fn current_settable_suspension_position(&self) -> SuspensionPosition {
        let data = self.buffer.as_ref();
        let raw = (data[field::FN_4] & 0xe0) >> 5;
        SuspensionPosition::from(raw)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the automatic door locking when driving function state field.
    #[inline]
    pub fn set_auto_door_locking_when_driving_state(&mut self, value: FunctionState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_0] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::FN_0] = raw;
    }

    /// Set the automatic headlamps function state field.
    #[inline]
    pub fn set_automatic_headlamps_state(&mut self, value: FunctionState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_0] & !0x0c;
        let raw = raw | ((u8::from(value) << 2) & 0x0c);
        data[field::FN_0] = raw;
    }

    /// Set the passenger airbag function state field.
    #[inline]
    pub fn set_passenger_airbag_state(&mut self, value: FunctionState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_0] & !0x30;
        let raw = raw | ((u8::from(value) << 4) & 0x30);
        data[field::FN_0] = raw;
    }

    /// Set the parking sensors function state field.
    #[inline]
    pub fn set_park_sensors_state(&mut self, value: FunctionState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_0] & !0xc0;
        let raw = raw | ((u8::from(value) << 6) & 0xc0);
        data[field::FN_0] = raw;
    }

    /// Set the settable suspension mode field.
    #[inline]
    pub fn set_settable_suspension_mode(&mut self, value: SuspensionMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_1] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::FN_1] = raw;
    }

    /// Set the automatic wipers function state field.
    #[inline]
    pub fn set_automatic_wipers_state(&mut self, value: FunctionState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_1] & !0x0c;
        let raw = raw | ((u8::from(value) << 2) & 0x0c);
        data[field::FN_1] = raw;
    }

    /// Set the ESP function state field.
    #[inline]
    pub fn set_esp_state(&mut self, value: FunctionState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_1] & !0x30;
        let raw = raw | ((u8::from(value) << 4) & 0x30);
        data[field::FN_1] = raw;
    }

    /// Set the door locking function state field.
    #[inline]
    pub fn set_door_locking_state(&mut self, value: FunctionState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_1] & !0xc0;
        let raw = raw | ((u8::from(value) << 6) & 0xc0);
        data[field::FN_1] = raw;
    }

    /// Set the boot and convertible roof position field.
    #[inline]
    pub fn set_boot_and_convertible_roof_position(
        &mut self,
        value: BootAndConvertibleRoofPosition,
    ) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_2] & !0x0e;
        let raw = raw | ((u8::from(value) << 1) & 0x0e);
        data[field::FN_2] = raw;
    }

    /// Set the Stop & Start function state field.
    #[inline]
    pub fn set_stop_start_state(&mut self, value: FunctionState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_2] & !0x30;
        let raw = raw | ((u8::from(value) << 4) & 0x30);
        data[field::FN_2] = raw;
    }

    /// Set the rear doors child lock function state field.
    #[inline]
    pub fn set_rear_doors_child_lock(&mut self, value: FunctionState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_2] & !0xc0;
        let raw = raw | ((u8::from(value) << 6) & 0xc0);
        data[field::FN_2] = raw;
    }

    /// Set the settable suspension movement type field.
    #[inline]
    pub fn set_settable_suspension_movement_type(&mut self, value: SuspensionMovement) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_3] & !0x03;
        let raw = raw | (u8::from(value) & 0x03);
        data[field::FN_3] = raw;
    }

    /// Set the final settable suspension position field.
    #[inline]
    pub fn set_final_settable_suspension_position(&mut self, value: SuspensionPosition) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_3] & !0x1c;
        let raw = raw | ((u8::from(value) << 2) & 0x1c);
        data[field::FN_3] = raw;
    }

    /// Set the initial settable suspension position field.
    #[inline]
    pub fn set_initial_settable_suspension_position(&mut self, value: SuspensionPosition) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_3] & !0xe0;
        let raw = raw | ((u8::from(value) << 5) & 0xe0);
        data[field::FN_3] = raw;
    }

    /// Set the enhanced traction control function state field.
    #[inline]
    pub fn set_enhanced_asr_state(&mut self, value: EnhancedTractionControlMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_4] & !0x0e;
        let raw = raw | ((u8::from(value) << 1) & 0x0e);
        data[field::FN_4] = raw;
    }

    /// Set the settable suspension warning flag.
    #[inline]
    pub fn set_settable_suspension_warning(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_4];
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::FN_4] = raw;
    }

    /// Set the real settable suspension position field.
    #[inline]
    pub fn set_current_settable_suspension_position(&mut self, value: SuspensionPosition) {
        let data = self.buffer.as_mut();
        let raw = data[field::FN_4] & !0xe0;
        let raw = raw | ((u8::from(value) << 5) & 0xe0);
        data[field::FN_4] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x2e1 ({})", err)?;
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

/// A high-level representation of a x2e1 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub automatic_door_locking_when_driving_state: FunctionState,
    pub automatic_headlamps_state: FunctionState,
    pub passenger_airbag_state: FunctionState,
    pub parking_sensors_state: FunctionState,
    pub settable_suspension_mode: SuspensionMode,
    pub automatic_wipers_state: FunctionState,
    pub esp_state: FunctionState,
    pub door_locking_state: FunctionState,
    pub boot_and_convertible_roof_position: BootAndConvertibleRoofPosition,
    pub stop_start_state: FunctionState,
    pub rear_doors_child_lock: FunctionState,
    pub settable_suspension_movement_type: SuspensionMovement,
    pub final_settable_suspension_position: SuspensionPosition,
    pub initial_settable_suspension_position: SuspensionPosition,
    pub current_settable_suspension_position: SuspensionPosition,
    pub enhanced_asr_state: EnhancedTractionControlMode,
    pub settable_suspension_warning: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            automatic_door_locking_when_driving_state: frame.auto_door_locking_when_driving_state(),
            automatic_headlamps_state: frame.automatic_headlamps_state(),
            passenger_airbag_state: frame.passenger_airbag_state(),
            parking_sensors_state: frame.park_sensors_state(),
            settable_suspension_mode: frame.settable_suspension_mode(),
            automatic_wipers_state: frame.automatic_wipers_state(),
            esp_state: frame.esp_state(),
            door_locking_state: frame.door_locking_state(),
            boot_and_convertible_roof_position: frame.boot_and_convertible_roof_position(),
            stop_start_state: frame.stop_start_state(),
            rear_doors_child_lock: frame.rear_doors_child_lock(),
            settable_suspension_movement_type: frame.settable_suspension_movement_type(),
            final_settable_suspension_position: frame.final_settable_suspension_position(),
            initial_settable_suspension_position: frame.initial_settable_suspension_position(),
            current_settable_suspension_position: frame.current_settable_suspension_position(),
            enhanced_asr_state: frame.enhanced_asr_state(),
            settable_suspension_warning: frame.settable_suspension_warning(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x2e1 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_auto_door_locking_when_driving_state(
            self.automatic_door_locking_when_driving_state,
        );
        frame.set_automatic_headlamps_state(self.automatic_headlamps_state);
        frame.set_passenger_airbag_state(self.passenger_airbag_state);
        frame.set_park_sensors_state(self.parking_sensors_state);
        frame.set_settable_suspension_mode(self.settable_suspension_mode);
        frame.set_automatic_wipers_state(self.automatic_wipers_state);
        frame.set_esp_state(self.esp_state);
        frame.set_door_locking_state(self.door_locking_state);
        frame.set_boot_and_convertible_roof_position(self.boot_and_convertible_roof_position);
        frame.set_stop_start_state(self.stop_start_state);
        frame.set_rear_doors_child_lock(self.rear_doors_child_lock);
        frame.set_settable_suspension_movement_type(self.settable_suspension_movement_type);
        frame.set_final_settable_suspension_position(self.final_settable_suspension_position);
        frame.set_initial_settable_suspension_position(self.initial_settable_suspension_position);
        frame.set_current_settable_suspension_position(self.current_settable_suspension_position);
        frame.set_enhanced_asr_state(self.enhanced_asr_state);
        frame.set_settable_suspension_warning(self.settable_suspension_warning);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x2e1")?;
        writeln!(
            f,
            " auto_door_locking_when_driving_state={}",
            self.automatic_door_locking_when_driving_state
        )?;
        writeln!(
            f,
            " automatic_headlamps_state={}",
            self.automatic_headlamps_state
        )?;
        writeln!(f, " passenger_airbag_state={}", self.passenger_airbag_state)?;
        writeln!(f, " park_sensors_state={}", self.parking_sensors_state)?;
        writeln!(
            f,
            " settable_suspension_mode={}",
            self.settable_suspension_mode
        )?;
        writeln!(f, " automatic_wipers_state={}", self.automatic_wipers_state)?;
        writeln!(f, " esp_state={}", self.esp_state)?;
        writeln!(f, " door_locking={}", self.door_locking_state)?;
        writeln!(
            f,
            " boot_and_convertible_roof_position={}",
            self.boot_and_convertible_roof_position
        )?;
        writeln!(f, " stop_start_state={}", self.stop_start_state)?;
        writeln!(f, " rear_doors_child_lock={}", self.rear_doors_child_lock)?;
        writeln!(
            f,
            " settable_suspension_movement_type={}",
            self.settable_suspension_movement_type
        )?;
        writeln!(
            f,
            " final_settable_suspension_position={}",
            self.final_settable_suspension_position
        )?;
        writeln!(
            f,
            " initial_settable_suspension_position={}",
            self.initial_settable_suspension_position
        )?;
        writeln!(
            f,
            " current_settable_suspension_position={}",
            self.current_settable_suspension_position
        )?;
        writeln!(f, " enhanced_asr_state={}", self.enhanced_asr_state)?;
        writeln!(
            f,
            " settable_suspension_warning={}",
            self.settable_suspension_warning
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        vehicle::{
            BootAndConvertibleRoofPosition, EnhancedTractionControlMode, FunctionState,
            SuspensionMode, SuspensionMovement, SuspensionPosition,
        },
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 5] = [0x77, 0xdc, 0x70, 0xfc, 0xf0];
    static REPR_FRAME_BYTES_2: [u8; 5] = [0xdd, 0x75, 0xda, 0x4f, 0x28];

    fn frame_1_repr() -> Repr {
        Repr {
            automatic_door_locking_when_driving_state: FunctionState::Enabled,
            automatic_headlamps_state: FunctionState::Disabled,
            passenger_airbag_state: FunctionState::Enabled,
            parking_sensors_state: FunctionState::Disabled,
            settable_suspension_mode: SuspensionMode::Absent,
            automatic_wipers_state: FunctionState::Enabled,
            esp_state: FunctionState::Disabled,
            door_locking_state: FunctionState::Enabled,
            boot_and_convertible_roof_position: BootAndConvertibleRoofPosition::None,
            stop_start_state: FunctionState::Enabled,
            rear_doors_child_lock: FunctionState::Disabled,
            settable_suspension_movement_type: SuspensionMovement::Immobile,
            final_settable_suspension_position: SuspensionPosition::None,
            initial_settable_suspension_position: SuspensionPosition::None,
            current_settable_suspension_position: SuspensionPosition::None,
            enhanced_asr_state: EnhancedTractionControlMode::EspOff,
            settable_suspension_warning: true,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            automatic_door_locking_when_driving_state: FunctionState::Disabled,
            automatic_headlamps_state: FunctionState::Enabled,
            passenger_airbag_state: FunctionState::Disabled,
            parking_sensors_state: FunctionState::Enabled,
            settable_suspension_mode: SuspensionMode::Sport,
            automatic_wipers_state: FunctionState::Disabled,
            esp_state: FunctionState::Enabled,
            door_locking_state: FunctionState::Disabled,
            boot_and_convertible_roof_position:
                BootAndConvertibleRoofPosition::OpenBootAndRoofClosed,
            stop_start_state: FunctionState::Disabled,
            rear_doors_child_lock: FunctionState::Enabled,
            settable_suspension_movement_type: SuspensionMovement::Denied,
            final_settable_suspension_position: SuspensionPosition::High,
            initial_settable_suspension_position: SuspensionPosition::Low,
            current_settable_suspension_position: SuspensionPosition::MidHigh,
            enhanced_asr_state: EnhancedTractionControlMode::Sand,
            settable_suspension_warning: false,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(
            frame.auto_door_locking_when_driving_state(),
            FunctionState::Enabled
        );
        assert_eq!(frame.automatic_headlamps_state(), FunctionState::Disabled);
        assert_eq!(frame.passenger_airbag_state(), FunctionState::Enabled);
        assert_eq!(frame.park_sensors_state(), FunctionState::Disabled);
        assert_eq!(frame.settable_suspension_mode(), SuspensionMode::Absent);
        assert_eq!(frame.automatic_wipers_state(), FunctionState::Enabled);
        assert_eq!(frame.esp_state(), FunctionState::Disabled);
        assert_eq!(frame.door_locking_state(), FunctionState::Enabled);
        assert_eq!(
            frame.boot_and_convertible_roof_position(),
            BootAndConvertibleRoofPosition::None
        );
        assert_eq!(frame.stop_start_state(), FunctionState::Enabled);
        assert_eq!(frame.rear_doors_child_lock(), FunctionState::Disabled);
        assert_eq!(
            frame.settable_suspension_movement_type(),
            SuspensionMovement::Immobile
        );
        assert_eq!(
            frame.final_settable_suspension_position(),
            SuspensionPosition::None
        );
        assert_eq!(
            frame.initial_settable_suspension_position(),
            SuspensionPosition::None
        );
        assert_eq!(
            frame.current_settable_suspension_position(),
            SuspensionPosition::None
        );
        assert_eq!(
            frame.enhanced_asr_state(),
            EnhancedTractionControlMode::EspOff
        );
        assert_eq!(frame.settable_suspension_warning(), true);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(
            frame.auto_door_locking_when_driving_state(),
            FunctionState::Disabled
        );
        assert_eq!(frame.automatic_headlamps_state(), FunctionState::Enabled);
        assert_eq!(frame.passenger_airbag_state(), FunctionState::Disabled);
        assert_eq!(frame.park_sensors_state(), FunctionState::Enabled);
        assert_eq!(frame.settable_suspension_mode(), SuspensionMode::Sport);
        assert_eq!(frame.automatic_wipers_state(), FunctionState::Disabled);
        assert_eq!(frame.esp_state(), FunctionState::Enabled);
        assert_eq!(frame.door_locking_state(), FunctionState::Disabled);
        assert_eq!(
            frame.boot_and_convertible_roof_position(),
            BootAndConvertibleRoofPosition::OpenBootAndRoofClosed
        );
        assert_eq!(frame.stop_start_state(), FunctionState::Disabled);
        assert_eq!(frame.rear_doors_child_lock(), FunctionState::Enabled);
        assert_eq!(
            frame.settable_suspension_movement_type(),
            SuspensionMovement::Denied
        );
        assert_eq!(
            frame.final_settable_suspension_position(),
            SuspensionPosition::High
        );
        assert_eq!(
            frame.initial_settable_suspension_position(),
            SuspensionPosition::Low
        );
        assert_eq!(
            frame.current_settable_suspension_position(),
            SuspensionPosition::MidHigh
        );
        assert_eq!(
            frame.enhanced_asr_state(),
            EnhancedTractionControlMode::Sand
        );
        assert_eq!(frame.settable_suspension_warning(), false);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 5];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_auto_door_locking_when_driving_state(FunctionState::Enabled);
        frame.set_automatic_headlamps_state(FunctionState::Disabled);
        frame.set_passenger_airbag_state(FunctionState::Enabled);
        frame.set_park_sensors_state(FunctionState::Disabled);
        frame.set_settable_suspension_mode(SuspensionMode::Absent);
        frame.set_automatic_wipers_state(FunctionState::Enabled);
        frame.set_esp_state(FunctionState::Disabled);
        frame.set_door_locking_state(FunctionState::Enabled);
        frame.set_boot_and_convertible_roof_position(BootAndConvertibleRoofPosition::None);
        frame.set_stop_start_state(FunctionState::Enabled);
        frame.set_rear_doors_child_lock(FunctionState::Disabled);
        frame.set_settable_suspension_movement_type(SuspensionMovement::Immobile);
        frame.set_final_settable_suspension_position(SuspensionPosition::None);
        frame.set_initial_settable_suspension_position(SuspensionPosition::None);
        frame.set_current_settable_suspension_position(SuspensionPosition::None);
        frame.set_enhanced_asr_state(EnhancedTractionControlMode::EspOff);
        frame.set_settable_suspension_warning(true);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 5];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_auto_door_locking_when_driving_state(FunctionState::Disabled);
        frame.set_automatic_headlamps_state(FunctionState::Enabled);
        frame.set_passenger_airbag_state(FunctionState::Disabled);
        frame.set_park_sensors_state(FunctionState::Enabled);
        frame.set_settable_suspension_mode(SuspensionMode::Sport);
        frame.set_automatic_wipers_state(FunctionState::Disabled);
        frame.set_esp_state(FunctionState::Enabled);
        frame.set_door_locking_state(FunctionState::Disabled);
        frame.set_boot_and_convertible_roof_position(
            BootAndConvertibleRoofPosition::OpenBootAndRoofClosed,
        );
        frame.set_stop_start_state(FunctionState::Disabled);
        frame.set_rear_doors_child_lock(FunctionState::Enabled);
        frame.set_settable_suspension_movement_type(SuspensionMovement::Denied);
        frame.set_final_settable_suspension_position(SuspensionPosition::High);
        frame.set_initial_settable_suspension_position(SuspensionPosition::Low);
        frame.set_current_settable_suspension_position(SuspensionPosition::MidHigh);
        frame.set_enhanced_asr_state(EnhancedTractionControlMode::Sand);
        frame.set_settable_suspension_warning(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 6] = [0x01, 0x03, 0xb2, 0x00, 0x00, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 4] = [0x01, 0x03, 0xb2, 0x00];
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
