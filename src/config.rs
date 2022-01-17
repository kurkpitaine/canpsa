use core::fmt;

enum_with_unknown! {
    /// Distance unit.
    pub enum DistanceUnit(u8) {
        /// Kilometer distance unit.
        Kilometer = 0,
        /// Mile distance unit.
        Mile = 1,
    }
}

impl fmt::Display for DistanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DistanceUnit::Kilometer => write!(f, "kilometer"),
            DistanceUnit::Mile => write!(f, "mile"),
            DistanceUnit::Unknown(unit) => write!(f, "0x{:02x}", unit),
        }
    }
}

enum_with_unknown! {
    /// Volume unit.
    pub enum VolumeUnit(u8) {
        /// Liter volume unit.
        Liter = 0,
        /// Gallon volume unit.
        Gallon = 1,
    }
}

impl fmt::Display for VolumeUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VolumeUnit::Liter => write!(f, "liter"),
            VolumeUnit::Gallon => write!(f, "gallon"),
            VolumeUnit::Unknown(unit) => write!(f, "0x{:02x}", unit),
        }
    }
}

enum_with_unknown! {
    /// Consumption unit type.
    pub enum ConsumptionUnit(u8) {
        /// Volume per distance unit, ie: liters/kilometers.
        VolumePerDistance = 0,
        /// Distance per volume unit, ie: miles/gallon.
        DistancePerVolume = 1,
    }
}

impl fmt::Display for ConsumptionUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConsumptionUnit::VolumePerDistance => write!(f, "volume per distance"),
            ConsumptionUnit::DistancePerVolume => write!(f, "distance per volume"),
            ConsumptionUnit::Unknown(unit) => write!(f, "0x{:02x}", unit),
        }
    }
}

enum_with_unknown! {
    /// Pressure unit.
    pub enum PressureUnit(u8) {
        /// Bar pressure unit.
        Bar = 0,
        /// PSI pressure unit.
        PSI = 1,
    }
}

impl fmt::Display for PressureUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PressureUnit::Bar => write!(f, "bar"),
            PressureUnit::PSI => write!(f, "PSI"),
            PressureUnit::Unknown(unit) => write!(f, "0x{:02x}", unit),
        }
    }
}

enum_with_unknown! {
    /// Temperature unit.
    pub enum TemperatureUnit(u8) {
        /// Celsius temperature unit.
        Celsius = 0,
        /// Fahrenheit temperature unit.
        Fahrenheit = 1,
    }
}

impl fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TemperatureUnit::Celsius => write!(f, "celsius"),
            TemperatureUnit::Fahrenheit => write!(f, "fahrenheit"),
            TemperatureUnit::Unknown(unit) => write!(f, "0x{:02x}", unit),
        }
    }
}

enum_with_unknown! {
    /// Display mode.
    pub enum DisplayMode(u8) {
        /// Negative display mode, ie: clear characters
        /// on a dark background.
        Negative = 0,
        /// Positive display mode, ie: dark characters
        /// on a clear background.
        Positive = 1,
    }
}

impl fmt::Display for DisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayMode::Negative => write!(f, "negative"),
            DisplayMode::Positive => write!(f, "positive"),
            DisplayMode::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
    /// Time format.
    pub enum TimeFormat(u8) {
        /// Twelve hour time format.
        H12 = 0,
        /// Twenty-four hour time format.
        H24 = 1,
    }
}

impl fmt::Display for TimeFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TimeFormat::H12 => write!(f, "12h"),
            TimeFormat::H24 => write!(f, "24h"),
            TimeFormat::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
    /// User interface language.
    pub enum Language(u8) {
        /// French language.
        French = 0x00,
        /// English language.
        English = 0x01,
        /// German language.
        German = 0x02,
        /// Spanish language.
        Spanish = 0x03,
        /// Italian language.
        Italian = 0x04,
        /// Portuguese language.
        Portuguese = 0x05,
        /// Dutch language.
        Dutch = 0x06,
        /// Greek language.
        Greek = 0x07,
        /// Brazilian language.
        Brazilian = 0x08,
        /// Polish language.
        Polish = 0x09,
        /// Traditional Chinese language.
        TraditionalChinese = 0x0a,
        /// Simplified Chinese language.
        SimplifiedChinese = 0x0b,
        /// Turkish language.
        Turkish = 0x0c,
        /// Russian language.
        Russian = 0x0e,
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Language::French => write!(f, "french"),
            Language::English => write!(f, "english"),
            Language::German => write!(f, "german"),
            Language::Spanish => write!(f, "spanish"),
            Language::Italian => write!(f, "italian"),
            Language::Portuguese => write!(f, "portuguese"),
            Language::Dutch => write!(f, "dutch"),
            Language::Greek => write!(f, "greek"),
            Language::Brazilian => write!(f, "brazilian"),
            Language::Polish => write!(f, "polish"),
            Language::TraditionalChinese => write!(f, "traditional chinese"),
            Language::SimplifiedChinese => write!(f, "simplified chinese"),
            Language::Turkish => write!(f, "turkish"),
            Language::Russian => write!(f, "russian"),
            Language::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}
