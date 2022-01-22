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
    /// Display charset.
    pub enum DisplayCharset(u8) {
        /// ASCII charset.
        ASCII = 0,
        /// UTF8 charset.
        UTF8 = 1,
    }
}

impl fmt::Display for DisplayCharset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayCharset::ASCII => write!(f, "ASCII"),
            DisplayCharset::UTF8 => write!(f, "UTF8"),
            DisplayCharset::Unknown(unit) => write!(f, "0x{:02x}", unit),
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
    /// Clock time format.
    pub enum ClockFormat(u8) {
        /// Twelve hour time clock format.
        H12 = 0,
        /// Twenty-four hour time clock format.
        H24 = 1,
    }
}

impl fmt::Display for ClockFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClockFormat::H12 => write!(f, "12h"),
            ClockFormat::H24 => write!(f, "24h"),
            ClockFormat::Unknown(time) => write!(f, "0x{:02x}", time),
        }
    }
}

enum_with_unknown! {
    /// User interface language.
    pub enum Language(u8) {
        /// French language - Available in AEE 2004 and AEE 2010.
        French = 0x00,
        /// English language - Available in AEE 2004 and AEE 2010.
        English = 0x01,
        /// German language - Available in AEE 2004 and AEE 2010.
        German = 0x02,
        /// Spanish language - Available in AEE 2004 and AEE 2010.
        Spanish = 0x03,
        /// Italian language - Available in AEE 2004 and AEE 2010.
        Italian = 0x04,
        /// Portuguese language - Available in AEE 2004 and AEE 2010.
        Portuguese = 0x05,
        /// Dutch language - Available in AEE 2004 and AEE 2010.
        Dutch = 0x06,
        /// Greek language - Available in AEE 2004 and AEE 2010.
        Greek = 0x07,
        /// Brazilian Portuguese language - Available in AEE 2004 and AEE 2010.
        BrazilianPortuguese = 0x08,
        /// Polish language - Available in AEE 2004 and AEE 2010.
        Polish = 0x09,
        /// Traditional Chinese language - Available in AEE 2004 and AEE 2010.
        TraditionalChinese = 0x0a,
        /// Simplified Chinese language - Available in AEE 2004 and AEE 2010.
        SimplifiedChinese = 0x0b,
        /// Turkish language - Available in AEE 2004 and AEE 2010.
        Turkish = 0x0c,
        /// Japanese language - Maybe available in AEE 2004? Available in AEE 2010.
        Japanese = 0x0d,
        /// Russian language - Available in AEE 2004 and AEE 2010.
        Russian = 0x0e,
        /// Invalid language value - Available in AEE 2004 and maybe in AEE 2010?.
        Invalid = 0x0f,
        /// Arabic language - Only available in AEE 2010.
        Arabic = 0x12,
        /// Farsi - Persian language - Only available in AEE 2010.
        Farsi = 0x17,
        /// Swedish language - Only available in AEE 2010.
        Swedish = 0x1d,
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
            Language::BrazilianPortuguese => write!(f, "brazilian portuguese"),
            Language::Polish => write!(f, "polish"),
            Language::TraditionalChinese => write!(f, "traditional chinese"),
            Language::SimplifiedChinese => write!(f, "simplified chinese"),
            Language::Turkish => write!(f, "turkish"),
            Language::Russian => write!(f, "russian"),
            Language::Invalid => write!(f, "invalid value"),
            Language::Japanese => write!(f, "japanese"),
            Language::Arabic => write!(f, "arabic"),
            Language::Farsi => write!(f, "farsi"),
            Language::Swedish => write!(f, "swedish"),
            Language::Unknown(lang) => write!(f, "0x{:02x}", lang),
        }
    }
}

enum_with_unknown! {
    /// Clock display mode.
    pub enum ClockDisplayMode(u8) {
        /// Steady clock display mode.
        Steady = 0,
        /// Blinking clock display mode.
        Blinking = 1,
    }
}

impl fmt::Display for ClockDisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClockDisplayMode::Steady => write!(f, "steady"),
            ClockDisplayMode::Blinking => write!(f, "blinking"),
            ClockDisplayMode::Unknown(time) => write!(f, "0x{:02x}", time),
        }
    }
}
