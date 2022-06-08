use core::{cmp::Ordering, fmt};

use byteorder::{ByteOrder, NetworkEndian};
use time::Duration;

use crate::{config::*, Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    use crate::field::*;
    /// 20-bit seconds, 12-bit days.
    pub const RUNNING_SEC_DAYS: Field = 0..4;
    /// 8-bit years.
    pub const RUNNING_YEARS: usize = 4;
    /// Each field is 1-bit.
    pub const DISPLAY_CONFIG_FLAGS: usize = 5;
    /// 4-bit language code, 4-bit empty.
    pub const LANGUAGE: usize = 6;
}

/// Raw x3f6 CAN frame identifier.
pub const FRAME_ID: u16 = 0x3f6;
/// Length of a x3f6 CAN frame.
pub const FRAME_LEN: usize = field::LANGUAGE + 1;

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

    /// Return the running seconds field.
    #[inline]
    pub fn running_seconds(&self) -> u32 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u32(&data[field::RUNNING_SEC_DAYS]) >> 12
    }

    /// Return the running days field.
    #[inline]
    pub fn running_days(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[2..4]) & 0x7f
    }

    /// Return the running years field.
    #[inline]
    pub fn running_years(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::RUNNING_YEARS]
    }

    /// Return the distance unit field.
    #[inline]
    pub fn distance_unit(&self) -> DistanceUnit {
        let data = self.buffer.as_ref();
        let raw = data[field::DISPLAY_CONFIG_FLAGS] & 0x01;
        DistanceUnit::from(raw)
    }

    /// Return the volume unit field.
    #[inline]
    pub fn volume_unit(&self) -> VolumeUnit {
        let data = self.buffer.as_ref();
        let raw = (data[field::DISPLAY_CONFIG_FLAGS] & 0x02) >> 1;
        VolumeUnit::from(raw)
    }

    /// Return the consumption unit field.
    #[inline]
    pub fn consumption_unit(&self) -> ConsumptionUnit {
        let data = self.buffer.as_ref();
        let raw = (data[field::DISPLAY_CONFIG_FLAGS] & 0x04) >> 2;
        ConsumptionUnit::from(raw)
    }

    /// Return the pressure unit field.
    #[inline]
    pub fn pressure_unit(&self) -> PressureUnit {
        let data = self.buffer.as_ref();
        let raw = (data[field::DISPLAY_CONFIG_FLAGS] & 0x08) >> 3;
        PressureUnit::from(raw)
    }

    /// Return the display charset field.
    #[inline]
    pub fn display_charset(&self) -> DisplayCharset {
        let data = self.buffer.as_ref();
        let raw = (data[field::DISPLAY_CONFIG_FLAGS] & 0x10) >> 4;
        DisplayCharset::from(raw)
    }

    /// Return the temperature unit field.
    #[inline]
    pub fn temperature_unit(&self) -> TemperatureUnit {
        let data = self.buffer.as_ref();
        let raw = (data[field::DISPLAY_CONFIG_FLAGS] & 0x20) >> 5;
        TemperatureUnit::from(raw)
    }

    /// Return the display mode field.
    #[inline]
    pub fn display_mode(&self) -> DisplayColorMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::DISPLAY_CONFIG_FLAGS] & 0x40) >> 6;
        DisplayColorMode::from(raw)
    }

    /// Return the clock format field.
    #[inline]
    pub fn clock_format(&self) -> ClockFormat {
        let data = self.buffer.as_ref();
        let raw = (data[field::DISPLAY_CONFIG_FLAGS] & 0x80) >> 7;
        ClockFormat::from(raw)
    }

    /// Return the language field.
    #[inline]
    pub fn language(&self) -> Language {
        let data = self.buffer.as_ref();
        Language::from(data[field::LANGUAGE] & 0x0f)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the running seconds field.
    #[inline]
    pub fn set_running_seconds(&mut self, value: u32) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u32(&data[field::RUNNING_SEC_DAYS]);
        let raw = (raw & 0x0000_0fff) | (value << 12);
        NetworkEndian::write_u32(&mut data[field::RUNNING_SEC_DAYS], raw);
    }

    /// Set the running days field.
    #[inline]
    pub fn set_running_days(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u32(&data[field::RUNNING_SEC_DAYS]);
        let raw = (raw & !0x0000_0fff) | (u32::from(value) & 0x0000_0fff);
        NetworkEndian::write_u32(&mut data[field::RUNNING_SEC_DAYS], raw);
    }

    /// Set the running years field.
    #[inline]
    pub fn set_running_years(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::RUNNING_YEARS] = value;
    }

    /// Clear the entire display config flags field.
    #[inline]
    pub fn clear_display_config(&mut self) {
        let data = self.buffer.as_mut();
        data[field::DISPLAY_CONFIG_FLAGS] = 0;
    }

    /// Set the distance unit field.
    #[inline]
    pub fn set_distance_unit(&mut self, value: DistanceUnit) {
        let data = self.buffer.as_mut();
        let raw = data[field::DISPLAY_CONFIG_FLAGS] & !0x01;
        let raw = raw | u8::from(value);
        data[field::DISPLAY_CONFIG_FLAGS] = raw;
    }

    /// Set the volume unit field.
    #[inline]
    pub fn set_volume_unit(&mut self, value: VolumeUnit) {
        let data = self.buffer.as_mut();
        let raw = data[field::DISPLAY_CONFIG_FLAGS] & !0x02;
        let raw = raw | (u8::from(value) << 1);
        data[field::DISPLAY_CONFIG_FLAGS] = raw;
    }

    /// Set the consumption unit field.
    #[inline]
    pub fn set_consumption_unit(&mut self, value: ConsumptionUnit) {
        let data = self.buffer.as_mut();
        let raw = data[field::DISPLAY_CONFIG_FLAGS] & !0x04;
        let raw = raw | (u8::from(value) << 2);
        data[field::DISPLAY_CONFIG_FLAGS] = raw;
    }

    /// Set the pressure unit field.
    #[inline]
    pub fn set_pressure_unit(&mut self, value: PressureUnit) {
        let data = self.buffer.as_mut();
        let raw = data[field::DISPLAY_CONFIG_FLAGS] & !0x08;
        let raw = raw | (u8::from(value) << 3);
        data[field::DISPLAY_CONFIG_FLAGS] = raw;
    }

    /// Set the display charset field.
    #[inline]
    pub fn set_display_charset(&mut self, value: DisplayCharset) {
        let data = self.buffer.as_mut();
        let raw = data[field::DISPLAY_CONFIG_FLAGS] & !0x10;
        let raw = raw | (u8::from(value) << 4);
        data[field::DISPLAY_CONFIG_FLAGS] = raw;
    }

    /// Set the temperature unit field.
    #[inline]
    pub fn set_temperature_unit(&mut self, value: TemperatureUnit) {
        let data = self.buffer.as_mut();
        let raw = data[field::DISPLAY_CONFIG_FLAGS] & !0x20;
        let raw = raw | (u8::from(value) << 5);
        data[field::DISPLAY_CONFIG_FLAGS] = raw;
    }

    /// Set the display mode field.
    #[inline]
    pub fn set_display_mode(&mut self, value: DisplayColorMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::DISPLAY_CONFIG_FLAGS] & !0x40;
        let raw = raw | (u8::from(value) << 6);
        data[field::DISPLAY_CONFIG_FLAGS] = raw;
    }

    /// Set the clock format field.
    #[inline]
    pub fn set_clock_format(&mut self, value: ClockFormat) {
        let data = self.buffer.as_mut();
        let raw = data[field::DISPLAY_CONFIG_FLAGS] & !0x80;
        let raw = raw | (u8::from(value) << 7);
        data[field::DISPLAY_CONFIG_FLAGS] = raw;
    }

    /// Set the language field.
    #[inline]
    pub fn set_language(&mut self, value: Language) {
        let data = self.buffer.as_mut();
        let raw = data[field::LANGUAGE] & !0x0f;
        let raw = raw | (u8::from(value) & 0x0f);
        data[field::LANGUAGE] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x3f6 ({})", err)?;
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

/// A high-level representation of a x3f6 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub running_duration: Duration,
    pub distance_unit: DistanceUnit,
    pub volume_unit: VolumeUnit,
    pub consumption_unit: ConsumptionUnit,
    pub pressure_unit: PressureUnit,
    pub display_charset: DisplayCharset,
    pub temperature_unit: TemperatureUnit,
    pub display_mode: DisplayColorMode,
    pub clock_format: ClockFormat,
    pub language: Language,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        if frame.running_seconds() >= 86400
            || frame.running_days() >= 365
            || frame.running_years() >= 100
        {
            Err(Error::Invalid)
        } else {
            let running_duration = Duration::seconds(frame.running_seconds().into())
                + Duration::days(frame.running_days().into())
                + Duration::days(365 * i64::from(frame.running_years()));

            Ok(Repr {
                running_duration,
                distance_unit: frame.distance_unit(),
                volume_unit: frame.volume_unit(),
                consumption_unit: frame.consumption_unit(),
                pressure_unit: frame.pressure_unit(),
                display_charset: frame.display_charset(),
                temperature_unit: frame.temperature_unit(),
                display_mode: frame.display_mode(),
                clock_format: frame.clock_format(),
                language: frame.language(),
            })
        }
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x3f6 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        let mut whole_days = self.running_duration.whole_days();
        let mut whole_seconds = self.running_duration.whole_seconds();

        let (running_years, running_days, running_seconds) = if whole_days >= 365 {
            let mut years = 0;
            while whole_days >= 365 {
                whole_days -= 365;
                whole_seconds -= 365 * 86400;
                years += 1;
            }
            (years, whole_days, whole_seconds - whole_days * 86400)
        } else {
            (0, whole_days, whole_seconds - whole_days * 86400)
        };

        frame.set_running_seconds(running_seconds as u32);
        frame.set_running_days(running_days as u16);
        frame.set_running_years(running_years);
        frame.clear_display_config();
        frame.set_distance_unit(self.distance_unit);
        frame.set_volume_unit(self.volume_unit);
        frame.set_consumption_unit(self.consumption_unit);
        frame.set_pressure_unit(self.pressure_unit);
        frame.set_display_charset(self.display_charset);
        frame.set_temperature_unit(self.temperature_unit);
        frame.set_display_mode(self.display_mode);
        frame.set_clock_format(self.clock_format);
        frame.set_language(self.language);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "x3f6 running_duration={}",
            self.running_duration.whole_seconds()
        )?;
        writeln!(f, " distance_unit={}", self.distance_unit)?;
        writeln!(f, " volume_unit={}", self.volume_unit)?;
        writeln!(f, " consumption_unit={}", self.consumption_unit)?;
        writeln!(f, " pressure_unit={}", self.pressure_unit)?;
        writeln!(f, " display_charset={}", self.display_charset)?;
        writeln!(f, " temperature_unit={}", self.temperature_unit)?;
        writeln!(f, " display_mode={}", self.display_mode)?;
        writeln!(f, " time_format={}", self.clock_format)?;
        writeln!(f, " language={}", self.language)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{config::*, Error};

    use time::Duration;

    static REPR_FRAME_BYTES_1: [u8; 7] = [0x00, 0xde, 0x80, 0x17, 0x00, 0x80, 0x04];
    static REPR_FRAME_BYTES_2: [u8; 7] = [0x14, 0x28, 0xc0, 0x1f, 0x01, 0x7f, 0x01];

    fn frame_1_repr() -> Repr {
        Repr {
            running_duration: Duration::seconds(3560) + Duration::days(23),
            distance_unit: DistanceUnit::Kilometer,
            volume_unit: VolumeUnit::Liter,
            consumption_unit: ConsumptionUnit::VolumePerDistance,
            pressure_unit: PressureUnit::Bar,
            display_charset: DisplayCharset::ASCII,
            temperature_unit: TemperatureUnit::Celsius,
            display_mode: DisplayColorMode::Negative,
            clock_format: ClockFormat::H24,
            language: Language::Italian,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            running_duration: Duration::seconds(82572) + Duration::days(31) + Duration::days(365),
            distance_unit: DistanceUnit::Mile,
            volume_unit: VolumeUnit::Gallon,
            consumption_unit: ConsumptionUnit::DistancePerVolume,
            pressure_unit: PressureUnit::PSI,
            display_charset: DisplayCharset::UTF8,
            temperature_unit: TemperatureUnit::Fahrenheit,
            display_mode: DisplayColorMode::Positive,
            clock_format: ClockFormat::H12,
            language: Language::English,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.running_seconds(), 3560);
        assert_eq!(frame.running_days(), 23);
        assert_eq!(frame.running_years(), 0);
        assert_eq!(frame.distance_unit(), DistanceUnit::Kilometer);
        assert_eq!(frame.volume_unit(), VolumeUnit::Liter);
        assert_eq!(frame.consumption_unit(), ConsumptionUnit::VolumePerDistance);
        assert_eq!(frame.pressure_unit(), PressureUnit::Bar);
        assert_eq!(frame.display_charset(), DisplayCharset::ASCII);
        assert_eq!(frame.temperature_unit(), TemperatureUnit::Celsius);
        assert_eq!(frame.display_mode(), DisplayColorMode::Negative);
        assert_eq!(frame.clock_format(), ClockFormat::H24);
        assert_eq!(frame.language(), Language::Italian);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.running_seconds(), 82572);
        assert_eq!(frame.running_days(), 31);
        assert_eq!(frame.running_years(), 1);
        assert_eq!(frame.distance_unit(), DistanceUnit::Mile);
        assert_eq!(frame.volume_unit(), VolumeUnit::Gallon);
        assert_eq!(frame.consumption_unit(), ConsumptionUnit::DistancePerVolume);
        assert_eq!(frame.pressure_unit(), PressureUnit::PSI);
        assert_eq!(frame.display_charset(), DisplayCharset::UTF8);
        assert_eq!(frame.temperature_unit(), TemperatureUnit::Fahrenheit);
        assert_eq!(frame.display_mode(), DisplayColorMode::Positive);
        assert_eq!(frame.clock_format(), ClockFormat::H12);
        assert_eq!(frame.language(), Language::English);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_running_seconds(3560);
        frame.set_running_days(23);
        frame.set_running_years(0);
        frame.set_distance_unit(DistanceUnit::Kilometer);
        frame.set_volume_unit(VolumeUnit::Liter);
        frame.set_consumption_unit(ConsumptionUnit::VolumePerDistance);
        frame.set_pressure_unit(PressureUnit::Bar);
        frame.set_display_charset(DisplayCharset::ASCII);
        frame.set_temperature_unit(TemperatureUnit::Celsius);
        frame.set_display_mode(DisplayColorMode::Negative);
        frame.set_clock_format(ClockFormat::H24);
        frame.set_language(Language::Italian);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_running_seconds(82572);
        frame.set_running_days(31);
        frame.set_running_years(1);
        frame.set_distance_unit(DistanceUnit::Mile);
        frame.set_volume_unit(VolumeUnit::Gallon);
        frame.set_consumption_unit(ConsumptionUnit::DistancePerVolume);
        frame.set_pressure_unit(PressureUnit::PSI);
        frame.set_display_charset(DisplayCharset::UTF8);
        frame.set_temperature_unit(TemperatureUnit::Fahrenheit);
        frame.set_display_mode(DisplayColorMode::Positive);
        frame.set_clock_format(ClockFormat::H12);
        frame.set_language(Language::English);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 8] = [0x00, 0xde, 0x80, 0x17, 0x00, 0x80, 0x04, 0xfe];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 6] = [0x00, 0xde, 0x80, 0x17, 0x00, 0x80];
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
        let mut buf = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_basic_repr_2_emit() {
        let mut buf = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_2_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }
}
