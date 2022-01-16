use core::fmt;

/// Distance unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DistanceUnit {
    /// Kilometer distance unit.
    Kilometer,
    /// Mile distance unit.
    Mile
}

impl fmt::Display for DistanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DistanceUnit::Kilometer => write!(f, "kilometer"),
            DistanceUnit::Mile => write!(f, "mile"),
        }
    }
}

/// Volume unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VolumeUnit {
    /// Liter volume unit.
    Liter,
    /// Gallon volume unit.
    Gallon,
}

impl fmt::Display for VolumeUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VolumeUnit::Liter => write!(f, "liter"),
            VolumeUnit::Gallon => write!(f, "gallon"),
        }
    }
}

/// Consumption unit type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConsumptionUnit {
    /// Volume per distance unit, ie: liters/kilometers.
    VolumePerDistance,
    /// Distance per volumne unit, ie: miles/gallon.
    DistancePerVolume,
}

impl fmt::Display for ConsumptionUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConsumptionUnit::VolumePerDistance => write!(f, "volume per distance"),
            ConsumptionUnit::DistancePerVolume => write!(f, "distance per volume"),
        }
    }
}

/// Pressure unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PressureUnit {
    /// Bar pressure unit.
    Bar,
    /// PSI pressure unit.
    PSI,
}

impl fmt::Display for PressureUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PressureUnit::Bar => write!(f, "bar"),
            PressureUnit::PSI => write!(f, "PSI"),
        }
    }
}

/// Temperature unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TemperatureUnit {
    /// Celsius temperature unit.
    Celsius,
    /// Fahrenheit temperature unit.
    Fahrenheit,
}

impl fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TemperatureUnit::Celsius => write!(f, "celsius"),
            TemperatureUnit::Fahrenheit => write!(f, "fahrenheit"),
        }
    }
}

/// Display mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DisplayMode {
    /// Negative display mode, ie: clear characters
    /// on a dark background.
    Negative,
    /// Positive display mode, ie: dark characters
    /// on a clear background.
    Positive,
}

impl fmt::Display for DisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayMode::Negative => write!(f, "negative"),
            DisplayMode::Positive => write!(f, "positive"),
        }
    }
}

/// Time format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TimeFormat {
    /// Twelve hour time format.
    H12,
    /// Twenty-four hour time format.
    H24,
}

impl fmt::Display for TimeFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TimeFormat::H12 => write!(f, "12h"),
            TimeFormat::H24 => write!(f, "24h"),
        }
    }
}
