use core::fmt;

enum_with_unknown! {
    /// Vehicle configuration option.
    pub enum ConfigOption(u8) {
        /// Unavailable option, not mounted on vehicle.
        Unavailable = 0,
        /// Unselectable option, vehicle hardware is capable
        /// but lacks parts to support it, ie: audio fader option with no rear speakers.
        UnselectableOption = 1,
        /// Selectable option, vehicle supports the option.
        SelectableOption = 2,
    }
}

impl fmt::Display for ConfigOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigOption::Unavailable => write!(f, "unavailable"),
            ConfigOption::UnselectableOption => write!(f, "unselectable option"),
            ConfigOption::SelectableOption => write!(f, "selectable option"),
            ConfigOption::Unknown(opt) => write!(f, "0x{:02x}", opt),
        }
    }
}

enum_with_unknown! {
    /// Speed unit.
    pub enum SpeedUnit(u8) {
        /// Kilometer per hour speed unit.
        Kph = 0,
        /// Mile per hour speed unit.
        Mph = 1,
    }
}

impl fmt::Display for SpeedUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpeedUnit::Kph => write!(f, "kph"),
            SpeedUnit::Mph => write!(f, "mph"),
            SpeedUnit::Unknown(unit) => write!(f, "0x{:02x}", unit),
        }
    }
}

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
    /// Display color mode.
    pub enum DisplayColorMode(u8) {
        /// Negative display mode, ie: clear characters
        /// on a dark background.
        Negative = 0,
        /// Positive display mode, ie: dark characters
        /// on a clear background.
        Positive = 1,
    }
}

impl fmt::Display for DisplayColorMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayColorMode::Negative => write!(f, "negative"),
            DisplayColorMode::Positive => write!(f, "positive"),
            DisplayColorMode::Unknown(mode) => write!(f, "0x{:02x}", mode),
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
    /// Generic display mode.
    pub enum DisplayMode(u8) {
        /// Steady display mode.
        Steady = 0,
        /// Blinking display mode.
        Blinking = 1,
    }
}

impl fmt::Display for DisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayMode::Steady => write!(f, "steady"),
            DisplayMode::Blinking => write!(f, "blinking"),
            DisplayMode::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

impl From<MaintenanceDisplayMode> for DisplayMode {
    fn from(maintenance_mode: MaintenanceDisplayMode) -> Self {
        match maintenance_mode {
            MaintenanceDisplayMode::Off => DisplayMode::Steady,
            MaintenanceDisplayMode::Steady => DisplayMode::Steady,
            MaintenanceDisplayMode::Blinking => DisplayMode::Blinking,
            MaintenanceDisplayMode::Unknown(mode) => DisplayMode::Unknown(mode),
        }
    }
}

enum_with_unknown! {
    /// Maintenance display mode. AEE 2004 only.
    pub enum MaintenanceDisplayMode(u8) {
        /// Off, no display.
        Off = 0,
        /// Steady display mode.
        Steady = 1,
        /// Blinking display mode.
        Blinking = 2,
    }
}

impl fmt::Display for MaintenanceDisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MaintenanceDisplayMode::Off => write!(f, "off"),
            MaintenanceDisplayMode::Steady => write!(f, "steady"),
            MaintenanceDisplayMode::Blinking => write!(f, "blinking"),
            MaintenanceDisplayMode::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
    /// Maintenance type.
    pub enum MaintenanceType(u8) {
        /// Maintenance deadline is from distance.
        Distance = 0,
        /// Maintenance deadline is from time.
        Time = 1,
    }
}

impl fmt::Display for MaintenanceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MaintenanceType::Distance => write!(f, "distance"),
            MaintenanceType::Time => write!(f, "time"),
            MaintenanceType::Unknown(mode) => write!(f, "0x{:02x}", mode),
        }
    }
}

enum_with_unknown! {
    /// Speed-dependent volume law.
    /// LawX values are used for car radios with settable Speed-dependent law.
    /// On standard car radios, only the [off] and [on] values are used.
    ///
    /// [off]: #variant.Off
    /// [on]: #variant.On
    pub enum SpeedDependentVolumeLaw(u8) {
        /// Speed-dependent volume is disabled.
        Off = 0,
        /// Speed-dependent volume law 0.
        Law0 = 1,
        /// Speed-dependent volume law 1.
        Law1 = 2,
        /// Speed-dependent volume law 2.
        Law2 = 3,
        /// Speed-dependent volume law 3.
        Law3 = 4,
        /// Speed-dependent volume law 4.
        Law4 = 5,
        /// Speed-dependent volume law 5.
        Law5 = 6,
        /// Speed-dependent volume is enabled.
        On = 7,
    }
}

impl fmt::Display for SpeedDependentVolumeLaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpeedDependentVolumeLaw::Off => write!(f, "off"),
            SpeedDependentVolumeLaw::Law0 => write!(f, "law 0"),
            SpeedDependentVolumeLaw::Law1 => write!(f, "law 1"),
            SpeedDependentVolumeLaw::Law2 => write!(f, "law 2"),
            SpeedDependentVolumeLaw::Law3 => write!(f, "law 3"),
            SpeedDependentVolumeLaw::Law4 => write!(f, "law 4"),
            SpeedDependentVolumeLaw::Law5 => write!(f, "law 5"),
            SpeedDependentVolumeLaw::On => write!(f, "on"),
            SpeedDependentVolumeLaw::Unknown(law) => write!(f, "0x{:02x}", law),
        }
    }
}

enum_with_unknown! {
    /// Musical ambiance setting.
    pub enum MusicalAmbiance(u8) {
        /// No musical ambiance.
        None = 0,
        /// Classic music ambiance.
        Classic = 1,
        /// Jazz or blues music ambiance.
        JazzBlues = 2,
        /// Pop or rock music ambiance.
        PopRock = 3,
        /// Vocal music ambiance.
        Vocal = 4,
        /// Techno music ambiance.
        Techno = 5,
    }
}

impl fmt::Display for MusicalAmbiance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MusicalAmbiance::None => write!(f, "none"),
            MusicalAmbiance::Classic => write!(f, "classic"),
            MusicalAmbiance::JazzBlues => write!(f, "jazz or blues"),
            MusicalAmbiance::PopRock => write!(f, "pop or rock"),
            MusicalAmbiance::Vocal => write!(f, "vocal"),
            MusicalAmbiance::Techno => write!(f, "techno"),
            MusicalAmbiance::Unknown(ambiance) => write!(f, "0x{:02x}", ambiance),
        }
    }
}

enum_with_unknown! {
    /// Sound repartition setting.
    pub enum SoundRepartition(u8) {
        /// No sound repartition.
        Off = 0,
        /// Driver sound repartition.
        Driver = 1,
        /// Surround sound repartition.
        Surround = 2,
        /// All passengers sound repartition.
        AllPassengers = 7,
    }
}

impl fmt::Display for SoundRepartition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SoundRepartition::Off => write!(f, "off"),
            SoundRepartition::Driver => write!(f, "driver"),
            SoundRepartition::Surround => write!(f, "surround"),
            SoundRepartition::AllPassengers => write!(f, "all passengers"),
            SoundRepartition::Unknown(sound_rep) => write!(f, "0x{:02x}", sound_rep),
        }
    }
}

enum_with_unknown! {
    /// Under-inflation detection system type.
    pub enum UnderInflationDetectionSystem(u8) {
        /// No under-inflation detection system.
        None = 0,
        /// Under-inflation detection direct system
        /// with absolute pressure measurement
        /// (first generation).
        DirectWithAbsolutePressure = 1,
        /// Under-inflation detection direct system
        /// without absolute pressure measurement
        /// (second generation).
        DirectWithoutAbsolutePressure = 2,
        /// Under-inflation detection indirect system type.
        Indirect = 3,
        /// Under-inflation detection indirect system type
        /// manufactured by Borg Warner.
        IndirectBorgWarner = 4,
    }
}

impl fmt::Display for UnderInflationDetectionSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnderInflationDetectionSystem::None => write!(f, "none"),
            UnderInflationDetectionSystem::DirectWithAbsolutePressure => {
                write!(f, "direct with absolute pressure")
            }
            UnderInflationDetectionSystem::DirectWithoutAbsolutePressure => {
                write!(f, "direct without absolute pressure")
            }
            UnderInflationDetectionSystem::Indirect => write!(f, "indirect"),
            UnderInflationDetectionSystem::IndirectBorgWarner => write!(f, "indirect Borg Warner"),
            UnderInflationDetectionSystem::Unknown(uid) => write!(f, "0x{:02x}", uid),
        }
    }
}

enum_with_unknown! {
    /// User profile number. AEE 2004 only.
    pub enum UserProfile(u8) {
        /// No user profile.
        None = 0,
        /// Profile number one.
        Profile1 = 1,
        /// Profile number two.
        Profile2 = 2,
        /// Profile number three.
        Profile3 = 4,
        /// Default profile.
        Default = 7,
    }
}

impl fmt::Display for UserProfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UserProfile::None => write!(f, "none"),
            UserProfile::Profile1 => write!(f, "profile 1"),
            UserProfile::Profile2 => write!(f, "profile 2"),
            UserProfile::Profile3 => write!(f, "profile 3"),
            UserProfile::Default => write!(f, "default profile"),
            UserProfile::Unknown(uid) => write!(f, "0x{:02x}", uid),
        }
    }
}

enum_with_unknown! {
    /// Sound harmony setting. AEE 2010 only.
    pub enum SoundHarmony(u8) {
        /// Harmony setting 1.
        Harmony1 = 0,
        /// Harmony setting 2.
        Harmony2 = 1,
        /// Harmony setting 3.
        Harmony3 = 2,
        /// Harmony setting 4.
        Harmony4 = 3,
    }
}

impl fmt::Display for SoundHarmony {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SoundHarmony::Harmony1 => write!(f, "harmony 1"),
            SoundHarmony::Harmony2 => write!(f, "harmony 2"),
            SoundHarmony::Harmony3 => write!(f, "harmony 3"),
            SoundHarmony::Harmony4 => write!(f, "harmony 4"),
            SoundHarmony::Unknown(harmony) => write!(f, "0x{:02x}", harmony),
        }
    }
}

enum_with_unknown! {
    /// Mood lighting level. AEE 2010 only.
    pub enum MoodLightingLevel(u8) {
        /// Mood lighting level 1.
        Level1 = 0,
        /// Mood lighting level 2.
        Level2 = 1,
        /// Mood lighting level 3.
        Level3 = 2,
        /// Mood lighting level 4.
        Level4 = 3,
        /// Mood lighting level 5.
        Level5 = 5,
        /// Mood lighting level 6.
        Level6 = 6,
    }
}

impl fmt::Display for MoodLightingLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MoodLightingLevel::Level1 => write!(f, "level 1"),
            MoodLightingLevel::Level2 => write!(f, "level 2"),
            MoodLightingLevel::Level3 => write!(f, "level 3"),
            MoodLightingLevel::Level4 => write!(f, "level 4"),
            MoodLightingLevel::Level5 => write!(f, "level 5"),
            MoodLightingLevel::Level6 => write!(f, "level 6"),
            MoodLightingLevel::Unknown(mood_level) => write!(f, "0x{:02x}", mood_level),
        }
    }
}

enum_with_unknown! {
    /// Lighting duration for welcome/follow-me-home lighting. AEE 2010 only.
    pub enum LightingDuration(u8) {
        /// 15 sec lighting duration.
        FifteenSeconds = 0,
        /// 30 sec lighting duration.
        ThirtySeconds = 1,
        /// 60 sec lighting duration.
        SixtySeconds = 2,
    }
}

impl fmt::Display for LightingDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LightingDuration::FifteenSeconds => write!(f, "15 seconds"),
            LightingDuration::ThirtySeconds => write!(f, "30 seconds"),
            LightingDuration::SixtySeconds => write!(f, "60 seconds"),
            LightingDuration::Unknown(duration) => write!(f, "0x{:02x}", duration),
        }
    }
}

enum_with_unknown! {
    /// Configurable key action. AEE 2004 only.
    pub enum ConfigurableKeyAction2004(u8) {
        /// Configurable key enables black panel.
        BlackPanel = 1,
        /// Configurable key enables ceiling lighting.
        CeilingLight = 2,
        /// Configurable key access to fault log.
        FaultLog = 5,
        /// Configurable key access to car functions state.
        FunctionState = 6,
        /// Configurable key access to cluster customization menu.
        ClusterCustomization = 8,
        /// Configurable key enables cluster color change.
        ClusterColor = 9,
    }
}

impl fmt::Display for ConfigurableKeyAction2004 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigurableKeyAction2004::BlackPanel => write!(f, "black panel"),
            ConfigurableKeyAction2004::CeilingLight => write!(f, "ceiling light"),
            ConfigurableKeyAction2004::FaultLog => write!(f, "fault log"),
            ConfigurableKeyAction2004::FunctionState => write!(f, "function state"),
            ConfigurableKeyAction2004::ClusterCustomization => write!(f, "cluster customization"),
            ConfigurableKeyAction2004::ClusterColor => write!(f, "cluster color"),
            ConfigurableKeyAction2004::Unknown(action) => write!(f, "0x{:02x}", action),
        }
    }
}

enum_with_unknown! {
    /// Configurable key action. AEE 2010 only.
    pub enum ConfigurableKeyAction2010(u8) {
        /// Configurable key enables ceiling lighting.
        CeilingLight = 0,
        /// Configurable key enables black panel.
        BlackPanel = 1,
        /// Configurable key access to fault log.
        FaultLog = 2,
        /// Configurable key access to cluster customization menu.
        ClusterCustomization = 3,
        /// Configurable key enables cluster color change.
        ClusterColor = 5,
        /// Configurable key set a manual fault check.
        ManualFaultCheck = 6,
    }
}

impl fmt::Display for ConfigurableKeyAction2010 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigurableKeyAction2010::CeilingLight => write!(f, "ceiling light"),
            ConfigurableKeyAction2010::BlackPanel => write!(f, "black panel"),
            ConfigurableKeyAction2010::FaultLog => write!(f, "fault log"),
            ConfigurableKeyAction2010::ClusterCustomization => write!(f, "cluster customization"),
            ConfigurableKeyAction2010::ClusterColor => write!(f, "cluster color"),
            ConfigurableKeyAction2010::ManualFaultCheck => write!(f, "manual fault check"),
            ConfigurableKeyAction2010::Unknown(action) => write!(f, "0x{:02x}", action),
        }
    }
}

enum_with_unknown! {
    /// Collision alert warning sensibility level. AEE 2010 only.
    pub enum CollisionAlertSensibilityLevel(u8) {
        /// Sensibility level 1 - close.
        Close = 1,
        /// Sensibility level 2 - normal.
        Normal = 2,
        /// Sensibility level 3 - distant.
        Distant = 3,
    }
}

impl fmt::Display for CollisionAlertSensibilityLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CollisionAlertSensibilityLevel::Close => write!(f, "close"),
            CollisionAlertSensibilityLevel::Normal => write!(f, "normal"),
            CollisionAlertSensibilityLevel::Distant => write!(f, "distant"),
            CollisionAlertSensibilityLevel::Unknown(level) => write!(f, "0x{:02x}", level),
        }
    }
}
