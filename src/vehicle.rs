use core::fmt;

enum_with_unknown! {
   /// Generic function state. Describes a vehicle function state.
   pub enum FunctionState(u8) {
       /// Function is absent on vehicle.
       Absent = 0,
       /// Function is disabled.
       Disabled = 1,
       /// Function is enabled.
       Enabled = 3,
   }
}

impl fmt::Display for FunctionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FunctionState::Absent => write!(f, "absent"),
            FunctionState::Disabled => write!(f, "disabled"),
            FunctionState::Enabled => write!(f, "enabled"),
            FunctionState::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Stop & Start system state.
   pub enum StopAndStartSystemState(u8) {
       /// Stop & Start system is unavailable.
       Unavailable = 0,
       /// Stop & Start system is enabled.
       Enabled = 1,
       /// Stop & Start system is disabled.
       Disabled = 2,
   }
}

impl fmt::Display for StopAndStartSystemState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopAndStartSystemState::Unavailable => write!(f, "unavailable"),
            StopAndStartSystemState::Enabled => write!(f, "enabled"),
            StopAndStartSystemState::Disabled => write!(f, "disabled"),
            StopAndStartSystemState::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Generic engine state.
   pub enum EngineState(u8) {
       /// Engine is disabled.
       Disabled = 0,
       /// Engine driving.
       Driving = 1,
       /// Engine braking.
       Braking = 2,
   }
}

impl fmt::Display for EngineState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EngineState::Disabled => write!(f, "disabled"),
            EngineState::Driving => write!(f, "driving"),
            EngineState::Braking => write!(f, "braking"),
            EngineState::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
/// Traction battery charge state.
pub enum TractionBatteryChargeState(u8) {
    /// Engine is disabled.
    Disabled = 0,
    /// Engine driving.
    Recharge = 1,
    /// Engine braking.
    Discharge = 2,
}
}

impl fmt::Display for TractionBatteryChargeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TractionBatteryChargeState::Disabled => write!(f, "disabled"),
            TractionBatteryChargeState::Recharge => write!(f, "recharge"),
            TractionBatteryChargeState::Discharge => write!(f, "discharge"),
            TractionBatteryChargeState::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Vehicle Supervision Module configuration mode.
   pub enum VsmConfigMode(u8) {
       /// Vehicle is configured in factory mode.
       Factory = 0,
       /// Vehicle is configured in showroom mode.
       Showroom = 1,
       /// Vehicle is configured in customer mode.
       Customer = 2,
   }
}

impl fmt::Display for VsmConfigMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VsmConfigMode::Factory => write!(f, "factory"),
            VsmConfigMode::Showroom => write!(f, "showroom"),
            VsmConfigMode::Customer => write!(f, "customer"),
            VsmConfigMode::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Vehicle configuration mode.
   pub enum VehicleConfigMode(u8) {
       /// Vehicle is configured in assembly-line mode.
       Assembly = 0,
       /// Vehicle is configured in factory mode.
       Factory = 1,
       /// Vehicle is configured in control mode.
       Control = 2,
       /// Vehicle is configured in storage mode.
       Storage = 3,
       /// Vehicle is configured in customer mode.
       Customer = 4,
       /// Vehicle is configured in showroom mode.
       Showroom = 5,
       /// Vehicle is configured in workshop mode.
       Workshop = 6,
   }
}

impl fmt::Display for VehicleConfigMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VehicleConfigMode::Assembly => write!(f, "assembly"),
            VehicleConfigMode::Factory => write!(f, "factory"),
            VehicleConfigMode::Control => write!(f, "control"),
            VehicleConfigMode::Storage => write!(f, "storage"),
            VehicleConfigMode::Customer => write!(f, "customer"),
            VehicleConfigMode::Showroom => write!(f, "showroom"),
            VehicleConfigMode::Workshop => write!(f, "workshop"),
            VehicleConfigMode::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Vehicle steering wheel position on the dashboard.
   pub enum SteeringWheelPosition(u8) {
       /// Vehicle steering wheel is on the right of the dashboard.
       Right = 1,
       /// Vehicle steering wheel is on the left of the dashboard.
       Left = 2,
   }
}

impl fmt::Display for SteeringWheelPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SteeringWheelPosition::Right => write!(f, "right"),
            SteeringWheelPosition::Left => write!(f, "left"),
            SteeringWheelPosition::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Vehicle main state.
   pub enum MainStatus(u8) {
       /// Vehicle is Off.
       Off = 0,
       /// Vehicle is On.
       On = 1,
       /// Vehicle is starting its motor.
       Cranking = 2,
   }
}

impl fmt::Display for MainStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MainStatus::Off => write!(f, "off"),
            MainStatus::On => write!(f, "on"),
            MainStatus::Cranking => write!(f, "cranking"),
            MainStatus::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Vehicle main state validity. Only [valid] value should be considered
   /// as a valid vehicle main state, everything else is invalid.
   ///
   /// [valid]: #variant.Valid
   pub enum MainStatusValidity(u8) {
       /// Main status is valid.
       Valid = 0x0A,
   }
}

impl fmt::Display for MainStatusValidity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MainStatusValidity::Valid => write!(f, "valid"),
            MainStatusValidity::Unknown(state) => write!(f, "invalid: 0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Vehicle speed validity. Only [valid] value should be considered
   /// as a valid speed valid, everything else is invalid.
   ///
   /// [valid]: #variant.Valid
   pub enum SpeedValidity(u8) {
       /// Speed is valid.
       Valid = 0x0A,
   }
}

impl fmt::Display for SpeedValidity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpeedValidity::Valid => write!(f, "valid"),
            SpeedValidity::Unknown(state) => write!(f, "invalid: 0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Vehicle powertrain state.
   pub enum PowertrainStatus(u8) {
       /// Motor is stopped.
       Stopped = 0,
       /// Motor is starting.
       Cranking = 1,
       /// Motor is running.
       Running = 2,
       /// Motor is stopping.
       Stopping = 3
   }
}

impl fmt::Display for PowertrainStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PowertrainStatus::Stopped => write!(f, "stopped"),
            PowertrainStatus::Cranking => write!(f, "cranking"),
            PowertrainStatus::Running => write!(f, "running"),
            PowertrainStatus::Stopping => write!(f, "stopping"),
            PowertrainStatus::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Blinkers state.
   pub enum BlinkersStatus(u8) {
       /// Blinkers are off.
       Off = 0,
       /// Right blinker is on.
       Right = 1,
       /// Left blinker is on.
       Left = 2,
       /// Left and Right blinkers are on.
       LeftAndRight = 3
   }
}

impl fmt::Display for BlinkersStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlinkersStatus::Off => write!(f, "off"),
            BlinkersStatus::Right => write!(f, "right"),
            BlinkersStatus::Left => write!(f, "left"),
            BlinkersStatus::LeftAndRight => write!(f, "left and right"),
            BlinkersStatus::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Vehicle driving direction.
   pub enum DrivingDirection(u8) {
       /// Forward driving direction.
       Forward = 1,
       /// Reverse driving direction.
       Reverse = 2,
   }
}

impl fmt::Display for DrivingDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DrivingDirection::Forward => write!(f, "forward"),
            DrivingDirection::Reverse => write!(f, "reverse"),
            DrivingDirection::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Hybrid powertrain working mode.
   pub enum HybridPowertrainMode(u8) {
       /// 4X4 mode.
       FourWheelDrive = 0,
       /// Sport mode.
       Sport = 1,
       /// Hybrid mode.
       Hybrid = 2,
       /// Zero emission mode.
       ZeroEmission = 3,
       /// Invalid mode.
       Invalid = 7,
   }
}

impl fmt::Display for HybridPowertrainMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HybridPowertrainMode::FourWheelDrive => write!(f, "4x4"),
            HybridPowertrainMode::Sport => write!(f, "sport"),
            HybridPowertrainMode::Hybrid => write!(f, "hybrid"),
            HybridPowertrainMode::ZeroEmission => write!(f, "zero emission"),
            HybridPowertrainMode::Invalid => write!(f, "invalid"),
            HybridPowertrainMode::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
   /// Hybrid powertrain working state.
   pub enum HybridPowertrainState(u8) {
       /// Indefinite mode.
       Indefinite = 0,
       /// Inactive mode.
       Inactive = 1,
       /// Activated at stop mode (vehicle is not moving).
       ActivatedAtStop = 2,
       /// Front wheel drive mode.
       FrontWheelDrive = 3,
       /// Rear wheel drive mode.
       RearWheelDrive = 4,
       /// Hybrid mode.
       Hybrid = 5,
   }
}

impl fmt::Display for HybridPowertrainState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HybridPowertrainState::Indefinite => write!(f, "indefinite"),
            HybridPowertrainState::Inactive => write!(f, "inactive"),
            HybridPowertrainState::ActivatedAtStop => write!(f, "activated at stop"),
            HybridPowertrainState::FrontWheelDrive => write!(f, "front-wheel drive"),
            HybridPowertrainState::RearWheelDrive => write!(f, "rear-wheel drive"),
            HybridPowertrainState::Hybrid => write!(f, "hybrid"),
            HybridPowertrainState::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
   /// Day or night state.
   pub enum DayNightStatus(u8) {
       /// Day state.
       Day = 0,
       /// Night state.
       Night = 1,
   }
}

impl fmt::Display for DayNightStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DayNightStatus::Day => write!(f, "day"),
            DayNightStatus::Night => write!(f, "night"),
            DayNightStatus::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Rheostat mode, used for dashboard panel lighting intensity.
   pub enum RheostatMode(u8) {
       /// Manual mode.
       Manual = 0,
       /// Automatic mode.
       Automatic = 1,
   }
}

impl fmt::Display for RheostatMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RheostatMode::Manual => write!(f, "manual"),
            RheostatMode::Automatic => write!(f, "automatic"),
            RheostatMode::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
   /// CAN Network state.
   pub enum NetworkState(u8) {
       /// Network is sleeping.
       Sleep = 0,
       /// Network is in normal mode.
       Normal = 1,
       /// Network is going to sleep.
       GoingToSleep = 2,
       /// Network is waking-up.
       WakeUp = 3,
       /// Network is off.
       Off = 4,
   }
}

impl fmt::Display for NetworkState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NetworkState::Sleep => write!(f, "sleep"),
            NetworkState::Normal => write!(f, "normal"),
            NetworkState::GoingToSleep => write!(f, "qoing to sleep"),
            NetworkState::WakeUp => write!(f, "wake up"),
            NetworkState::Off => write!(f, "off"),
            NetworkState::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Convertible roof position.
   pub enum ConvertibleRoofPosition(u8) {
       /// Coupe position.
       Coupe = 0,
       /// Convertible (open-top) position.
       Convertible = 1,
   }
}

impl fmt::Display for ConvertibleRoofPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConvertibleRoofPosition::Coupe => write!(f, "coupe"),
            ConvertibleRoofPosition::Convertible => write!(f, "convertible"),
            ConvertibleRoofPosition::Unknown(pos) => write!(f, "0x{:02x}", pos),
        }
    }
}

enum_with_unknown! {
   /// Vehicle electrical network state.
   pub enum ElectricalNetworkState(u8) {
       /// Electrical network is running on battery in normal mode.
       BatteryNormal = 0,
       /// Electrical network is running on battery in fail-soft mode.
       BatteryFailSoftMode = 1,
       /// Electrical network is running on battery and cranking is available.
       BatteryCrankingAvailable = 2,
       /// Electrical network is starting.
       Starting = 3,
       /// Electrical network is restarting.
       Restart = 4,
       /// Electrical network is running on generator in normal mode.
       GeneratorNormal = 5,
       /// Electrical network is running on generator in fail-soft mode.
       GeneratorFailSoftMode = 6,
       /// Electrical network is running on generator in secured mode.
       GeneratorSecured = 7,
       /// Electrical network is running on generator in urgent mode.
       GeneratorUrgent = 8,
   }
}

impl fmt::Display for ElectricalNetworkState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ElectricalNetworkState::BatteryNormal => write!(f, "battery normal"),
            ElectricalNetworkState::BatteryFailSoftMode => write!(f, "battery fail-soft mode"),
            ElectricalNetworkState::BatteryCrankingAvailable => {
                write!(f, "battery cranking available")
            }
            ElectricalNetworkState::Starting => write!(f, "starting"),
            ElectricalNetworkState::Restart => write!(f, "restart"),
            ElectricalNetworkState::GeneratorNormal => write!(f, "generator normal"),
            ElectricalNetworkState::GeneratorFailSoftMode => write!(f, "generator fail-soft mode"),
            ElectricalNetworkState::GeneratorSecured => write!(f, "generator secured"),
            ElectricalNetworkState::GeneratorUrgent => write!(f, "generator urgent"),
            ElectricalNetworkState::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
   /// Volume level origin.
   pub enum VolumeLevelOrigin(u8) {
       /// Volume level origin is from user setting.
       User = 0,
       /// Volume level origin is from a source change.
       SourceChange = 1,
       /// Volume level origin is from speed dependent volume.
       SpeedDependentVolume = 2,
       /// Volume level origin is from parking sensors source mix.
       ParkSensorsSourceMix = 3,
       /// Volume level origin is from thermal protection.
       ThermalProtection = 4,
       /// Volume level origin if from overtake.
       Overtake = 5,
       /// Volume level origin is from user phone.
       Phone = 6,
       /// Volume level origin is other or sleep.
       OtherOrSleep = 7,
   }
}

impl fmt::Display for VolumeLevelOrigin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VolumeLevelOrigin::User => write!(f, "user"),
            VolumeLevelOrigin::SourceChange => write!(f, "source change"),
            VolumeLevelOrigin::SpeedDependentVolume => {
                write!(f, "speed-dependent volume")
            }
            VolumeLevelOrigin::ParkSensorsSourceMix => write!(f, "park sensors source mix"),
            VolumeLevelOrigin::ThermalProtection => write!(f, "thermal protection"),
            VolumeLevelOrigin::Overtake => write!(f, "overtake"),
            VolumeLevelOrigin::Phone => write!(f, "phone"),
            VolumeLevelOrigin::OtherOrSleep => write!(f, "other or sleep"),
            VolumeLevelOrigin::Unknown(origin) => write!(f, "0x{:02x}", origin),
        }
    }
}

enum_with_unknown! {
   /// Automatic parking mode.
   pub enum AutomaticParkingMode(u8) {
       /// SCP 6 mode.
       SCP6 = 0,
       /// SCP 9 mode.
       SCP9 = 1,
   }
}

impl fmt::Display for AutomaticParkingMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AutomaticParkingMode::SCP6 => write!(f, "SCP 6"),
            AutomaticParkingMode::SCP9 => write!(f, "SCP 9"),
            AutomaticParkingMode::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
   /// Automatic parking mode.
   pub enum CruiseControlCustomSettingPosition(u8) {
       /// No setting.
       None = 0,
       /// Position 1 cruise-control setting.
       Position1 = 1,
       /// Position 2 cruise-control setting.
       Position2 = 2,
        /// Position 3 cruise-control setting.
       Position3 = 3,
       /// Position 4 cruise-control setting.
       Position4 = 4,
       /// Position 5 cruise-control setting.
       Position5 = 5,
       /// Position 6 cruise-control setting.
       Position6 = 6,
   }
}

impl fmt::Display for CruiseControlCustomSettingPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CruiseControlCustomSettingPosition::None => write!(f, "none"),
            CruiseControlCustomSettingPosition::Position1 => write!(f, "position 1"),
            CruiseControlCustomSettingPosition::Position2 => write!(f, "position 2"),
            CruiseControlCustomSettingPosition::Position3 => write!(f, "position 3"),
            CruiseControlCustomSettingPosition::Position4 => write!(f, "position 4"),
            CruiseControlCustomSettingPosition::Position5 => write!(f, "position 5"),
            CruiseControlCustomSettingPosition::Position6 => write!(f, "position 6"),
            CruiseControlCustomSettingPosition::Unknown(pos) => write!(f, "0x{:02x}", pos),
        }
    }
}

enum_with_unknown! {
   /// Boot and convertible roof position.
   pub enum BootAndConvertibleRoofPosition(u8) {
       /// No display of this, ie: vehicle is not convertible.
       None = 0,
       /// Vehicle is in coupe, ie: convertible roof and boot are closed.
       Coupe = 1,
       /// Vehicle has boot and roof opened.
       OpenBootAndOpenRoof = 2,
       /// Vehicle has boot opened and roof is retracted inside boot.
       OpenBootAndRoofInsideBoot = 3,
       /// Vehicle is in convertible mode.
       Convertible = 4,
       /// Vehicle has boot opened and roof closed.
       OpenBootAndRoofClosed = 5,
   }
}

impl fmt::Display for BootAndConvertibleRoofPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BootAndConvertibleRoofPosition::None => write!(f, "none"),
            BootAndConvertibleRoofPosition::Coupe => write!(f, "coupe"),
            BootAndConvertibleRoofPosition::OpenBootAndOpenRoof => {
                write!(f, "open boot and open roof")
            }
            BootAndConvertibleRoofPosition::OpenBootAndRoofInsideBoot => {
                write!(f, "open boot and roof inside boot")
            }
            BootAndConvertibleRoofPosition::Convertible => write!(f, "convertible"),
            BootAndConvertibleRoofPosition::OpenBootAndRoofClosed => {
                write!(f, "open boot and roof closed")
            }
            BootAndConvertibleRoofPosition::Unknown(pos) => write!(f, "0x{:02x}", pos),
        }
    }
}

enum_with_unknown! {
   /// Vehicle suspension mode.
   pub enum SuspensionMode(u8) {
       /// Settable suspension mode is not mounted on vehicle.
       Absent = 0,
       /// Sport suspension mode.
       Sport = 1,
       /// Normal suspension mode.
       Normal = 2,
   }
}

impl fmt::Display for SuspensionMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SuspensionMode::Absent => write!(f, "absent"),
            SuspensionMode::Sport => write!(f, "sport"),
            SuspensionMode::Normal => write!(f, "normal"),
            SuspensionMode::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
   /// Vehicle settable suspension position.
   pub enum SuspensionPosition(u8) {
       /// Vehicle suspension is in normal position.
       Normal = 0,
       /// Vehicle suspension is in mid-high position.
       MidHigh = 1,
       /// Vehicle suspension is in low position.
       Low = 2,
       /// Vehicle suspension is in high position.
       High = 3,
       /// Vehicle suspension is not displayable.
       None = 7,
   }
}

impl fmt::Display for SuspensionPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SuspensionPosition::Normal => write!(f, "normal"),
            SuspensionPosition::MidHigh => write!(f, "mid-high"),
            SuspensionPosition::Low => write!(f, "low"),
            SuspensionPosition::High => write!(f, "high"),
            SuspensionPosition::None => write!(f, "none"),
            SuspensionPosition::Unknown(pos) => write!(f, "0x{:02x}", pos),
        }
    }
}

enum_with_unknown! {
   /// Vehicle settable suspension movement.
   pub enum SuspensionMovement(u8) {
       /// Suspension is immobile.
       Immobile = 0,
       /// Vehicle suspension ascending, ie: from low to high position.
       Ascent = 1,
       /// Vehicle suspension descending, ie: from high to low position.
       Descent = 2,
       /// Vehicle suspension movement request denied.
       Denied = 3,
   }
}

impl fmt::Display for SuspensionMovement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SuspensionMovement::Immobile => write!(f, "immobile"),
            SuspensionMovement::Ascent => write!(f, "ascent"),
            SuspensionMovement::Descent => write!(f, "descent"),
            SuspensionMovement::Denied => write!(f, "denied"),
            SuspensionMovement::Unknown(mov) => write!(f, "0x{:02x}", mov),
        }
    }
}

enum_with_unknown! {
   /// Enhanced traction control mode.
   pub enum EnhancedTractionControlMode(u8) {
       /// Enhanced traction control ESP is off.
       EspOff = 0,
       /// Enhanced traction control is in normal mode.
       Normal = 1,
       /// Enhanced traction control is in snow mode.
       Snow = 2,
        /// Enhanced traction control is in mud mode.
       Mud = 3,
       /// Enhanced traction control is in sand mode.
       Sand = 4,
       /// Enhanced traction control mode selector fault.
       ModeSelectorFault = 6,
       /// Enhanced traction control system fault.
       SystemFault = 7,
   }
}

impl fmt::Display for EnhancedTractionControlMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnhancedTractionControlMode::EspOff => write!(f, "esp off"),
            EnhancedTractionControlMode::Normal => write!(f, "normal"),
            EnhancedTractionControlMode::Snow => write!(f, "snow"),
            EnhancedTractionControlMode::Mud => write!(f, "mud"),
            EnhancedTractionControlMode::Sand => write!(f, "sand"),
            EnhancedTractionControlMode::ModeSelectorFault => write!(f, "mode selector fault"),
            EnhancedTractionControlMode::SystemFault => write!(f, "system fault"),
            EnhancedTractionControlMode::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
   /// Push button LED state.
   pub enum PushButtonLedState(u8) {
       /// LED is off.
       Off = 0,
       /// LED is on with a steady light.
       Steady = 1,
       /// LED is on with a blinking light.
       Blinking = 2,
   }
}

impl fmt::Display for PushButtonLedState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PushButtonLedState::Off => write!(f, "off"),
            PushButtonLedState::Steady => write!(f, "steady"),
            PushButtonLedState::Blinking => write!(f, "blinking"),
            PushButtonLedState::Unknown(led) => write!(f, "0x{:02x}", led),
        }
    }
}

enum_with_unknown! {
   /// Vehicle fuel type.
   pub enum FuelType(u8) {
       /// Petrol engine.
       Petrol = 0,
       /// Diesel engine.
       Diesel = 1,
   }
}

impl fmt::Display for FuelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FuelType::Petrol => write!(f, "petrol"),
            FuelType::Diesel => write!(f, "diesel"),
            FuelType::Unknown(led) => write!(f, "0x{:02x}", led),
        }
    }
}

enum_with_unknown! {
   /// A/C air recirculation state.
   pub enum ACRecirculationState(u8) {
       /// Exterior air source.
       ExteriorAir = 0,
       /// Partial air recirculation.
       PartialAirRecirculation = 1,
       /// Full air recirculation.
       FullAirRecirculation = 2,
       /// Stopped.
       Stopped = 3,
   }
}

impl fmt::Display for ACRecirculationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ACRecirculationState::ExteriorAir => write!(f, "exterior air"),
            ACRecirculationState::PartialAirRecirculation => write!(f, "partial air recirculation"),
            ACRecirculationState::FullAirRecirculation => write!(f, "full air recirculation"),
            ACRecirculationState::Stopped => write!(f, "stopped"),
            ACRecirculationState::Unknown(led) => write!(f, "0x{:02x}", led),
        }
    }
}

enum_with_unknown! {
   /// Cruise-control/speed-limiter/acc mode.
   pub enum SpeedRegulationMode(u8) {
       /// Off mode.
       Off = 0,
       /// Cruise-control mode.
       CruiseControl = 1,
       /// Speed limiter mode.
       SpeedLimiter = 2,
       /// Adaptive cruise-control mode.
       AdaptiveCruiseControl = 3,
   }
}

impl fmt::Display for SpeedRegulationMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpeedRegulationMode::Off => write!(f, "off"),
            SpeedRegulationMode::CruiseControl => write!(f, "cruise-control"),
            SpeedRegulationMode::SpeedLimiter => write!(f, "speed limiter"),
            SpeedRegulationMode::AdaptiveCruiseControl => write!(f, "adaptive cruise-control"),
            SpeedRegulationMode::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
   /// Cruise-control/speed-limiter/acc mode state.
   pub enum SpeedRegulationModeState(u8) {
       /// Mode is in standby state.
       Standby = 0,
       /// Mode is in up state.
       Up = 1,
       /// Speed limiter mode is up and running, limiting speed.
       LimiterUpAndRunning = 2,
       /// Mode is in up state, with overspeed from sloping.
       UpOverspeed = 3,
       /// Mode is is up state, with overspeed from driver action.
       UpOverspeedFromDriver = 4,
       /// Mode is forbidden.
       ForbiddenMode = 6,
       /// Mode is in failure state.
       Failure = 7,
   }
}

impl fmt::Display for SpeedRegulationModeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpeedRegulationModeState::Standby => write!(f, "standby"),
            SpeedRegulationModeState::Up => write!(f, "up"),
            SpeedRegulationModeState::LimiterUpAndRunning => {
                write!(f, "limiter up and running")
            }
            SpeedRegulationModeState::UpOverspeed => write!(f, "up overspeed"),
            SpeedRegulationModeState::UpOverspeedFromDriver => {
                write!(f, "up overspeed from driver")
            }
            SpeedRegulationModeState::ForbiddenMode => write!(f, "forbidden mode"),
            SpeedRegulationModeState::Failure => write!(f, "failure"),
            SpeedRegulationModeState::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Cruise-control/speed-limiter/acc setting page.
   pub enum SpeedRegulationSettingPage(u8) {
       /// Close page.
       Close = 0,
       /// Speed limiter page.
       SpeedLimiter = 1,
       /// Cruise-control page.
       CruiseControl = 2,
   }
}

impl fmt::Display for SpeedRegulationSettingPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpeedRegulationSettingPage::Close => write!(f, "Close"),
            SpeedRegulationSettingPage::SpeedLimiter => write!(f, "speed limiter"),
            SpeedRegulationSettingPage::CruiseControl => write!(f, "cruise-control"),
            SpeedRegulationSettingPage::Unknown(page) => write!(f, "0x{:02x}", page),
        }
    }
}

enum_with_unknown! {
   /// Adaptive cruise-control state.
   pub enum AdaptiveCruiseControlState(u8) {
       /// No speed adjustment.
       NoAdjust = 0,
       /// Speed adjustment in progress.
       AdjustInProgress = 1,
       /// Speed adjustment reached high limit.
       HighLimit = 2,
       /// Speed adjustment reached low limit.
       LowLimit = 3,
       /// Disable speed adjustment.
       DisableSpeedAdjustment = 4,
       /// Disable automatic adjustment.
       DisableAutomaticAdjustment = 5,
       /// Overspeed state.
       Overspeed = 6,
       /// Radar has a low visibility ahead.
       RadarLowVisibility = 7,
       /// Radar is learning target.
       RadarLearning = 8,
       /// Adaptive cruise control is disabled.
       Disabled = 9,
       /// Desired speed reached limit value.
       AdjustmentLimit = 10,
   }
}

impl fmt::Display for AdaptiveCruiseControlState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdaptiveCruiseControlState::NoAdjust => write!(f, "no adjust"),
            AdaptiveCruiseControlState::AdjustInProgress => write!(f, "adjust in progress"),
            AdaptiveCruiseControlState::HighLimit => write!(f, "high limit"),
            AdaptiveCruiseControlState::LowLimit => write!(f, "low limit"),
            AdaptiveCruiseControlState::DisableSpeedAdjustment => {
                write!(f, "disable speed adjustment")
            }
            AdaptiveCruiseControlState::DisableAutomaticAdjustment => {
                write!(f, "disable automatic adjustment")
            }
            AdaptiveCruiseControlState::Overspeed => write!(f, "overspeed"),
            AdaptiveCruiseControlState::RadarLowVisibility => write!(f, "radar low visibility"),
            AdaptiveCruiseControlState::RadarLearning => write!(f, "radar learning"),
            AdaptiveCruiseControlState::Disabled => write!(f, "disabled"),
            AdaptiveCruiseControlState::AdjustmentLimit => write!(f, "adjustment limit"),
            AdaptiveCruiseControlState::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Wheel pressure state.
   pub enum WheelState(u8) {
       /// Wheel has a puncture.
       Puncture = 1,
       /// Lightly deflated wheel.
       LightlyDeflated = 2,
       /// Highly deflated wheel.
       HighlyDeflated = 4,
       /// Wheel is not monitored.
       NotMonitored = 8,
       /// Wheel is normal.
       Normal = 16,
   }
}

impl fmt::Display for WheelState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WheelState::Puncture => write!(f, "puncture"),
            WheelState::LightlyDeflated => write!(f, "lightly deflated"),
            WheelState::HighlyDeflated => write!(f, "highly deflated"),
            WheelState::NotMonitored => write!(f, "not monitored"),
            WheelState::Normal => write!(f, "normal"),
            WheelState::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// PAX Wheel pressure state. AEE 2004 only.
   /// PAX is a discontinued run-on-flat Michelin technology.
   pub enum PAXWheelState(u8) {
       /// Wheel is normal.
       Normal = 0,
       /// Wheel has a puncture.
       Puncture = 1,
       /// Unavailable wheel state.
       Unavailable = 2,
   }
}

impl fmt::Display for PAXWheelState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PAXWheelState::Normal => write!(f, "normal"),
            PAXWheelState::Puncture => write!(f, "puncture"),
            PAXWheelState::Unavailable => write!(f, "unavailable"),
            PAXWheelState::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Under-inflation system state.
   pub enum UnderInflationSystemState(u8) {
       /// Pressure is not monitored.
       PressureNotMonitored = 0,
       /// System failure.
       SystemFailure = 1,
       /// Unsuitable wheel pressure for vehicle load.
       LoadUnsuitableWheelPressure = 2,
       /// Unsuitable wheel pressure for vehicle speed.
       SpeedUnsuitableWheelPressure = 3,
       /// Measure in progress.
       MeasureInProgress = 4,
       /// Ok system state.
       Ok = 7,
   }
}

impl fmt::Display for UnderInflationSystemState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnderInflationSystemState::PressureNotMonitored => write!(f, "pressure not monitored"),
            UnderInflationSystemState::SystemFailure => write!(f, "system failure"),
            UnderInflationSystemState::LoadUnsuitableWheelPressure => {
                write!(f, "load unsuitable wheel pressure")
            }
            UnderInflationSystemState::SpeedUnsuitableWheelPressure => {
                write!(f, "speed unsuitable wheel pressure")
            }
            UnderInflationSystemState::MeasureInProgress => write!(f, "measure in progress"),
            UnderInflationSystemState::Ok => write!(f, "ok"),
            UnderInflationSystemState::Unknown(state) => write!(f, "0x{:02x}", state),
        }
    }
}

enum_with_unknown! {
   /// Measured (by brake control unit) slope type.
   pub enum SlopeType(u8) {
       /// Light slope.
       Light = 0,
       /// Steep upward slope.
       SteepUpward = 1,
       /// Steep downward slope.
       SteepDownward = 2,
       /// Slope is not defined.
       Undefined = 3,
   }
}

impl fmt::Display for SlopeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SlopeType::Light => write!(f, "light"),
            SlopeType::SteepUpward => write!(f, "steep upward"),
            SlopeType::SteepDownward => write!(f, "steep downward"),
            SlopeType::Undefined => write!(f, "undefined"),
            SlopeType::Unknown(slope) => write!(f, "0x{:02x}", slope),
        }
    }
}

enum_with_unknown! {
   /// Stop and Start brake requirement
   pub enum StopAndStartBrakeRequirement(u8) {
       /// No requirement.
       Nothing = 0,
       /// Brake control unit require engine stop inhibition.
       StopInhibit = 1,
       /// Brake control unit require engine restart.
       Restart = 2,
       /// Brake control unit require engine stop and restart inhibition.
       StopAndRestartInhibit = 3,
   }
}

impl fmt::Display for StopAndStartBrakeRequirement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopAndStartBrakeRequirement::Nothing => write!(f, "nothing"),
            StopAndStartBrakeRequirement::StopInhibit => write!(f, "stop inhibit"),
            StopAndStartBrakeRequirement::Restart => write!(f, "restart"),
            StopAndStartBrakeRequirement::StopAndRestartInhibit => {
                write!(f, "stop and restart inhibit")
            }
            StopAndStartBrakeRequirement::Unknown(req) => write!(f, "0x{:02x}", req),
        }
    }
}

enum_with_unknown! {
   /// Gearbox type.
   pub enum GearboxType(u8) {
       /// Automatic gearbox (BVA).
       Automatic = 0,
       /// Manual gearbox (BVM).
       Manual = 1,
       /// Manual robotized gearbox (BVMP).
       ManualRobotized = 2,
       /// Automatic with dual-clutch gearbox (EAT).
       AutomaticDualClutch = 3,
   }
}

impl fmt::Display for GearboxType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GearboxType::Automatic => write!(f, "automatic"),
            GearboxType::Manual => write!(f, "manual"),
            GearboxType::ManualRobotized => write!(f, "manual robotized"),
            GearboxType::AutomaticDualClutch => {
                write!(f, "automatic dual clutch")
            }
            GearboxType::Unknown(req) => write!(f, "0x{:02x}", req),
        }
    }
}

enum_with_unknown! {
   /// Automatic gearbox mode.
   pub enum AutoGearboxMode(u8) {
       /// Automatic mode.
       Automatic = 0,
       /// Automatic sport mode.
       AutomaticSport = 2,
       /// Sequential mode.
       Sequential = 4,
       /// Sequential sport mode.
       SequentialSport = 5,
       /// Automatic snow mode.
       AutomaticSnow = 6,
       /// ASM mode.
       ASM = 7
   }
}

impl fmt::Display for AutoGearboxMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AutoGearboxMode::Automatic => write!(f, "automatic"),
            AutoGearboxMode::AutomaticSport => write!(f, "automatic sport"),
            AutoGearboxMode::Sequential => write!(f, "sequential"),
            AutoGearboxMode::SequentialSport => {
                write!(f, "sequential sport")
            }
            AutoGearboxMode::AutomaticSnow => {
                write!(f, "automatic snow")
            }
            AutoGearboxMode::ASM => write!(f, "asm"),

            AutoGearboxMode::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
   /// Gearbox gear when drive mode is engaged.
   pub enum GearboxDriveModeGear(u8) {
       /// Gear is disengaged.
       Disengaged = 0,
       /// Gear 1 is engaged.
       Gear1 = 1,
       /// Gear 2 is engaged.
       Gear2 = 2,
       /// Gear 3 is engaged.
       Gear3 = 3,
       /// Gear 4 is engaged.
       Gear4 = 4,
       /// Gear 5 is engaged.
       Gear5 = 5,
       /// Gear 6 is engaged.
       Gear6 = 6,

   }
}

impl fmt::Display for GearboxDriveModeGear {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GearboxDriveModeGear::Disengaged => write!(f, "disengaged"),
            GearboxDriveModeGear::Gear1 => write!(f, "gear 1"),
            GearboxDriveModeGear::Gear2 => write!(f, "gear 2"),
            GearboxDriveModeGear::Gear3 => write!(f, "gear 3"),
            GearboxDriveModeGear::Gear4 => write!(f, "gear 4"),
            GearboxDriveModeGear::Gear5 => write!(f, "gear 5"),
            GearboxDriveModeGear::Gear6 => write!(f, "gear 6"),
            GearboxDriveModeGear::Unknown(gear) => write!(f, "0x{:02x}", gear),
        }
    }
}

enum_with_unknown! {
   /// Gearbox engaged gear to display.
   pub enum GearboxGear(u8) {
       /// Parking mode is engaged.
       P = 0,
       /// Reverse gear is engaged.
       R = 1,
       /// Neutral is engaged.
       N = 2,
       /// Drive mode is engaged.
       D = 3,
       /// Gear 6 is engaged.
       Gear6 = 4,
       /// Gear 5 is engaged.
       Gear5 = 5,
       /// Gear 4 is engaged.
       Gear4 = 6,
       /// Gear 3 is engaged.
       Gear3 = 7,
       /// Gear 2 is engaged.
       Gear2 = 8,
       /// Gear 1 is engaged.
       Gear1 = 9,
       /// Nothing to display.
       Nothing = 0x0b,
       /// Gear 7 is engaged.
       Gear7 = 0x0c,
       /// Gear 8 is engaged.
       Gear8 = 0x0d,
       /// Gear 9 is engaged.
       Gear9 = 0x0e,
       /// Braking mode is engaged.
       B = 0x0f,
   }
}

impl fmt::Display for GearboxGear {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GearboxGear::P => write!(f, "P"),
            GearboxGear::R => write!(f, "R"),
            GearboxGear::N => write!(f, "N"),
            GearboxGear::D => write!(f, "D"),
            GearboxGear::B => write!(f, "B"),
            GearboxGear::Gear1 => write!(f, "gear 1"),
            GearboxGear::Gear2 => write!(f, "gear 2"),
            GearboxGear::Gear3 => write!(f, "gear 3"),
            GearboxGear::Gear4 => write!(f, "gear 4"),
            GearboxGear::Gear5 => write!(f, "gear 5"),
            GearboxGear::Gear6 => write!(f, "gear 6"),
            GearboxGear::Gear7 => write!(f, "gear 7"),
            GearboxGear::Gear8 => write!(f, "gear 8"),
            GearboxGear::Gear9 => write!(f, "gear 9"),
            GearboxGear::Nothing => write!(f, "nothing"),
            GearboxGear::Unknown(gear) => write!(f, "0x{:02x}", gear),
        }
    }
}

enum_with_unknown! {
   /// Gear efficiency indicator arrow type.
   pub enum GearEfficiencyArrowType(u8) {
       /// Nothing is displayed.
       Nothing = 0,
       /// Upward arrow type.
       Up = 1,
       /// Downwards arrow type.
       Down = 2,
       /// Upward and downward arrow type.
       UpAndDown = 3,
   }
}

impl fmt::Display for GearEfficiencyArrowType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GearEfficiencyArrowType::Nothing => write!(f, "nothing"),
            GearEfficiencyArrowType::Up => write!(f, "up"),
            GearEfficiencyArrowType::Down => write!(f, "down"),
            GearEfficiencyArrowType::UpAndDown => write!(f, "up and down"),
            GearEfficiencyArrowType::Unknown(arrow) => write!(f, "0x{:02x}", arrow),
        }
    }
}

enum_with_unknown! {
   /// Foot on brake pedal indicator state.
   pub enum FootOnBrakePedalIndicatorState(u8) {
       /// Indicator is off.
       Off = 0,
       /// Indicator is on.
       On = 1,
       /// Indicator is blinking.
       Blinking = 2,
   }
}

impl fmt::Display for FootOnBrakePedalIndicatorState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FootOnBrakePedalIndicatorState::Off => write!(f, "off"),
            FootOnBrakePedalIndicatorState::On => write!(f, "on"),
            FootOnBrakePedalIndicatorState::Blinking => write!(f, "blinking"),
            FootOnBrakePedalIndicatorState::Unknown(arrow) => write!(f, "0x{:02x}", arrow),
        }
    }
}
