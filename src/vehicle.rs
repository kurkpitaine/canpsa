use core::fmt;

enum_with_unknown! {
   /// Vehicle configuration mode.
   pub enum ConfigMode(u8) {
       /// Vehicle is configured in factory mode.
       Factory = 0,
       /// Vehicle is configured in showroom mode.
       Showroom = 1,
       /// Vehicle is configured in customer mode.
       Customer = 2,
   }
}

impl fmt::Display for ConfigMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigMode::Factory => write!(f, "factory"),
            ConfigMode::Showroom => write!(f, "showroom"),
            ConfigMode::Customer => write!(f, "customer"),
            ConfigMode::Unknown(state) => write!(f, "0x{:02x}", state),
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
