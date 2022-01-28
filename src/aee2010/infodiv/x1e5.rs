use core::fmt;

use crate::{
    config::{ConfigOption, MusicalAmbiance, SpeedDependentVolumeLaw},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 2-bit balance option activation, 5-bit balance level,
    /// 1-bit balance under adjustment flag.
    pub const BALANCE_OPT_ADJ: usize = 0;
    /// 2-bit fader option activation (via diagnostic session),
    /// 5-bit fader level, 1-bit fader under adjustment flag.
    pub const FADER_OPT_ADJ: usize = 1;
    /// 2-bit bass option activation, 5-bit bass level,
    /// 1-bit bass under adjustment flag.
    pub const BASS_OPT_ADJ: usize = 2;
    /// 2-bit treble option activation, 5-bit treble level,
    /// 1-bit treble under adjustment flag.
    pub const TREBLE_OPT_ADJ: usize = 3;
    /// 2-bit speed-dependent volume control option activation (via diagnostic session),
    /// 1-bit speed-dependent volume control activation flag,
    /// 1-bit speed-dependent volume control under adjustment flag,
    /// 1-bit loudness activation flag,
    /// 2-bit loudness option activation,
    /// 1-bit loudness under adjustment flag.
    pub const SPD_VOL_ADJ_LOUD_ADJ: usize = 4;
    /// 2-bit sound repartition option activation,
    /// 1-bit musical ambiance under adjustment flag,
    /// 3-bit musical ambiance setting,
    /// 2-bit musical ambiance option activation.
    pub const REPARTITION_AMBIANCE: usize = 5;
    /// 1-bit unknown/empty,
    /// 1-bit spatial sound setting under adjustment flag,
    /// 1-bit spectral sound setting under adjustment flag,
    /// 1-bit impossible audio setting with phone as audio source,
    /// 3-bit sound repartition field,
    /// 1-bit sound repartition under adjustment flag.
    pub const SPATIAL_SPECTRAL_REPARTITION: usize = 6;
}

/// Length of a x1e5 CAN frame.
pub const FRAME_LEN: usize = field::SPATIAL_SPECTRAL_REPARTITION + 1;

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

    /// Return the balance option activation field.
    #[inline]
    pub fn balance_option(&self) -> ConfigOption {
        let data = self.buffer.as_ref();
        let raw = data[field::BALANCE_OPT_ADJ] & 0x03;
        ConfigOption::from(raw)
    }

    /// Return the balance level field.
    #[inline]
    pub fn balance_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::BALANCE_OPT_ADJ] & 0x7c) >> 2
    }

    /// Return the balance under adjustment flag.
    #[inline]
    pub fn balance_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::BALANCE_OPT_ADJ] & 0x80 != 0
    }

    /// Return the fader option activation field.
    #[inline]
    pub fn fader_option(&self) -> ConfigOption {
        let data = self.buffer.as_ref();
        let raw = data[field::FADER_OPT_ADJ] & 0x03;
        ConfigOption::from(raw)
    }

    /// Return the fader level field.
    #[inline]
    pub fn fader_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::FADER_OPT_ADJ] & 0x7c) >> 2
    }

    /// Return the fader under adjustment flag.
    #[inline]
    pub fn fader_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FADER_OPT_ADJ] & 0x80 != 0
    }

    /// Return the bass option activation field.
    #[inline]
    pub fn bass_option(&self) -> ConfigOption {
        let data = self.buffer.as_ref();
        let raw = data[field::BASS_OPT_ADJ] & 0x03;
        ConfigOption::from(raw)
    }

    /// Return the bass level field.
    #[inline]
    pub fn bass_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::BASS_OPT_ADJ] & 0x7c) >> 2
    }

    /// Return the bass under adjustment flag.
    #[inline]
    pub fn bass_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::BASS_OPT_ADJ] & 0x80 != 0
    }

    /// Return the treble option activation field.
    #[inline]
    pub fn treble_option(&self) -> ConfigOption {
        let data = self.buffer.as_ref();
        let raw = data[field::TREBLE_OPT_ADJ] & 0x03;
        ConfigOption::from(raw)
    }

    /// Return the treble level field.
    #[inline]
    pub fn treble_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::TREBLE_OPT_ADJ] & 0x7c) >> 2
    }

    /// Return the middle under adjustment flag.
    #[inline]
    pub fn treble_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::TREBLE_OPT_ADJ] & 0x80 != 0
    }

    /// Return the speed dependent volume option activation field (via diagnostic session).
    #[inline]
    pub fn speed_dependent_volume_option(&self) -> ConfigOption {
        let data = self.buffer.as_ref();
        let raw = data[field::SPD_VOL_ADJ_LOUD_ADJ] & 0x03;
        ConfigOption::from(raw)
    }

    /// Return whether speed-dependent volume is enabled or not.
    #[inline]
    pub fn speed_dependent_volume_enabled(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::SPD_VOL_ADJ_LOUD_ADJ] & 0x04 != 0
    }

    /// Return the speed-dependent volume under adjustment flag.
    #[inline]
    pub fn speed_dependent_volume_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::SPD_VOL_ADJ_LOUD_ADJ] & 0x08 != 0
    }

    /// Return whether loudness is enabled or not.
    #[inline]
    pub fn loudness_enabled(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::SPD_VOL_ADJ_LOUD_ADJ] & 0x10 != 0
    }

    /// Return the speed dependent volume option activation field.
    #[inline]
    pub fn loudness_option(&self) -> ConfigOption {
        let data = self.buffer.as_ref();
        let raw = (data[field::SPD_VOL_ADJ_LOUD_ADJ] & 0x60) >> 5;
        ConfigOption::from(raw)
    }

    /// Return the loudness under adjustment flag.
    #[inline]
    pub fn loudness_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::SPD_VOL_ADJ_LOUD_ADJ] & 0x80 != 0
    }

    /// Return the sound repartition option activation field.
    #[inline]
    pub fn sound_repartition_option(&self) -> ConfigOption {
        let data = self.buffer.as_ref();
        let raw = data[field::REPARTITION_AMBIANCE] & 0x03;
        ConfigOption::from(raw)
    }

    /// Return the musical ambiance under adjustment flag.
    #[inline]
    pub fn musical_ambiance_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REPARTITION_AMBIANCE] & 0x04 != 0
    }

    /// Return the musical ambiance field.
    #[inline]
    pub fn musical_ambiance(&self) -> MusicalAmbiance {
        let data = self.buffer.as_ref();
        let raw = (data[field::REPARTITION_AMBIANCE] & 0x38) >> 3;
        MusicalAmbiance::from(raw)
    }

    /// Return the musical ambiance option activation field.
    #[inline]
    pub fn musical_ambiance_option(&self) -> ConfigOption {
        let data = self.buffer.as_ref();
        let raw = (data[field::REPARTITION_AMBIANCE] & 0xc0) >> 6;
        ConfigOption::from(raw)
    }

    /// Return the spatial sound under adjustment flag.
    #[inline]
    pub fn spatial_sound_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::SPATIAL_SPECTRAL_REPARTITION] & 0x02 != 0
    }

    /// Return the spectral sound under adjustment flag.
    #[inline]
    pub fn spectral_sound_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::SPATIAL_SPECTRAL_REPARTITION] & 0x04 != 0
    }

    /// Return the impossible setting with phone as audio source flag.
    #[inline]
    pub fn impossible_setting(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::SPATIAL_SPECTRAL_REPARTITION] & 0x08 != 0
    }

    /// Return the sound repartition field.
    #[inline]
    pub fn sound_repartition(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::SPATIAL_SPECTRAL_REPARTITION] & 0x70) >> 4
    }

    /// Return the sound repartition under adjustment flag.
    #[inline]
    pub fn sound_repartition_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::SPATIAL_SPECTRAL_REPARTITION] & 0x80 != 0
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the balance level field.
    #[inline]
    pub fn set_balance_level(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::BALANCE_ADJ] & !0x7f;
        let raw = raw | (value & 0x7f);
        data[field::BALANCE_ADJ] = raw;
    }

    /// Set the balance under adjustment flag.
    #[inline]
    pub fn set_balance_under_adjustment(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::BALANCE_ADJ];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::BALANCE_ADJ] = raw;
    }

    /// Set the fader level field.
    #[inline]
    pub fn set_fader_level(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::FADER_ADJ] & !0x7f;
        let raw = raw | (value & 0x7f);
        data[field::FADER_ADJ] = raw;
    }

    /// Set the fader under adjustment flag.
    #[inline]
    pub fn set_fader_under_adjustment(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FADER_ADJ];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::FADER_ADJ] = raw;
    }

    /// Set the bass level field.
    #[inline]
    pub fn set_bass_level(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::BASS_ADJ] & !0x7f;
        let raw = raw | (value & 0x7f);
        data[field::BASS_ADJ] = raw;
    }

    /// Set the bass under adjustment flag.
    #[inline]
    pub fn set_bass_under_adjustment(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::BASS_ADJ];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::BASS_ADJ] = raw;
    }

    /// Set the middle level field.
    #[inline]
    pub fn set_middle_level(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::MIDDLE_ADJ] & !0x7f;
        let raw = raw | (value & 0x7f);
        data[field::MIDDLE_ADJ] = raw;
    }

    /// Set the bass under adjustment flag.
    #[inline]
    pub fn set_middle_under_adjustment(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::MIDDLE_ADJ];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::MIDDLE_ADJ] = raw;
    }

    /// Set the treble level field.
    #[inline]
    pub fn set_treble_level(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::TREBLE_ADJ] & !0x7f;
        let raw = raw | (value & 0x7f);
        data[field::TREBLE_ADJ] = raw;
    }

    /// Set the treble under adjustment flag.
    #[inline]
    pub fn set_treble_under_adjustment(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::TREBLE_ADJ];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::TREBLE_ADJ] = raw;
    }

    /// Set the speed-dependent volume level field.
    #[inline]
    pub fn set_speed_dependent_volume(&mut self, value: SpeedDependentVolumeLaw) {
        let data = self.buffer.as_mut();
        let raw = data[field::SPD_VOL_ADJ_LOUD_ADJ] & !0x07;
        let raw = raw | (u8::from(value) & 0x07);
        data[field::SPD_VOL_ADJ_LOUD_ADJ] = raw;
    }

    /// Set the speed-dependent volume under adjustment flag.
    #[inline]
    pub fn set_speed_dependent_volume_under_adjustment(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::SPD_VOL_ADJ_LOUD_ADJ];
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::SPD_VOL_ADJ_LOUD_ADJ] = raw;
    }

    /// Set the loudness enabled flag.
    #[inline]
    pub fn set_loudness_enabled(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::SPD_VOL_ADJ_LOUD_ADJ];
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::SPD_VOL_ADJ_LOUD_ADJ] = raw;
    }

    /// Set the loudness under adjustment flag.
    #[inline]
    pub fn set_loudness_under_adjustment(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::SPD_VOL_ADJ_LOUD_ADJ];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::SPD_VOL_ADJ_LOUD_ADJ] = raw;
    }

    /// Set the loudness enabled (via diagnostic session) flag.
    #[inline]
    pub fn set_loudness_enabled_diag(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_AMBIANCE];
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::FLAGS_AMBIANCE] = raw;
    }

    /// Set the fader enabled (via diagnostic session) flag.
    #[inline]
    pub fn set_fader_enabled_diag(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_AMBIANCE];
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::FLAGS_AMBIANCE] = raw;
    }

    /// Set the musical ambiance field.
    #[inline]
    pub fn set_musical_ambiance(&mut self, value: MusicalAmbiance) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_AMBIANCE] & !0x1c;
        let raw = raw | ((u8::from(value) << 2) & 0x1c);
        data[field::FLAGS_AMBIANCE] = raw;
    }

    /// Set the impossible setting with phone as audio source flag.
    #[inline]
    pub fn set_impossible_setting(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_AMBIANCE];
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::FLAGS_AMBIANCE] = raw;
    }

    /// Set the musical ambiance under adjustment flag.
    #[inline]
    pub fn set_musical_ambiance_under_adjustment(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_AMBIANCE];
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::FLAGS_AMBIANCE] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x1e5 ({})", err)?;
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

/// A high-level representation of a x1e5 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    balance_level: u8,
    balance_under_adj: bool,
    fader_level: u8,
    fader_under_adj: bool,
    bass_level: u8,
    bass_under_adj: bool,
    middle_level: u8,
    middle_under_adj: bool,
    treble_level: u8,
    treble_under_adj: bool,
    speed_dependent_volume: SpeedDependentVolumeLaw,
    speed_dependent_volume_under_adj: bool,
    loudness_enabled: bool,
    loudness_under_adj: bool,
    loudness_enabled_diag: bool,
    fader_enabled_diag: bool,
    musical_ambiance: MusicalAmbiance,
    musical_ambiance_under_adj: bool,
    impossible_setting: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            balance_level: frame.balance_level(),
            balance_under_adj: frame.balance_under_adjustment(),
            fader_level: frame.fader_level(),
            fader_under_adj: frame.fader_under_adjustment(),
            bass_level: frame.bass_level(),
            bass_under_adj: frame.bass_under_adjustment(),
            middle_level: frame.middle_level(),
            middle_under_adj: frame.middle_under_adjustment(),
            treble_level: frame.treble_level(),
            treble_under_adj: frame.treble_under_adjustment(),
            speed_dependent_volume: frame.speed_dependent_volume(),
            speed_dependent_volume_under_adj: frame.speed_dependent_volume_under_adjustment(),
            loudness_enabled: frame.loudness_enabled(),
            loudness_under_adj: frame.loudness_under_adjustment(),
            loudness_enabled_diag: frame.loudness_enabled_diag(),
            fader_enabled_diag: frame.fader_enabled_diag(),
            musical_ambiance: frame.musical_ambiance(),
            musical_ambiance_under_adj: frame.musical_ambiance_under_adjustment(),
            impossible_setting: frame.impossible_setting(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x1e5 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_balance_level(self.balance_level);
        frame.set_balance_under_adjustment(self.balance_under_adj);
        frame.set_fader_level(self.fader_level);
        frame.set_fader_under_adjustment(self.fader_under_adj);
        frame.set_bass_level(self.bass_level);
        frame.set_bass_under_adjustment(self.bass_under_adj);
        frame.set_middle_level(self.middle_level);
        frame.set_middle_under_adjustment(self.middle_under_adj);
        frame.set_treble_level(self.treble_level);
        frame.set_treble_under_adjustment(self.treble_under_adj);
        frame.set_speed_dependent_volume(self.speed_dependent_volume);
        frame.set_speed_dependent_volume_under_adjustment(self.speed_dependent_volume_under_adj);
        frame.set_loudness_enabled(self.loudness_enabled);
        frame.set_loudness_under_adjustment(self.loudness_under_adj);
        frame.set_loudness_enabled_diag(self.loudness_enabled_diag);
        frame.set_fader_enabled_diag(self.fader_enabled_diag);
        frame.set_musical_ambiance(self.musical_ambiance);
        frame.set_musical_ambiance_under_adjustment(self.musical_ambiance_under_adj);
        frame.set_impossible_setting(self.impossible_setting);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "balance level={}", self.balance_level)?;
        write!(f, "balance under adj={}", self.balance_under_adj)?;
        write!(f, "fader level={}", self.fader_level)?;
        write!(f, "fader under adj={}", self.fader_under_adj)?;
        write!(f, "bass level={}", self.bass_level)?;
        write!(f, "bass under adj={}", self.bass_under_adj)?;
        write!(f, "middle level={}", self.middle_level)?;
        write!(f, "middle under adj={}", self.middle_under_adj)?;
        write!(f, "treble level={}", self.treble_level)?;
        write!(f, "treble under adj={}", self.treble_under_adj)?;
        write!(f, "speed dependent volume={}", self.speed_dependent_volume)?;
        write!(
            f,
            "speed dependent volume under adj={}",
            self.speed_dependent_volume_under_adj
        )?;
        write!(f, "loudness enabled={}", self.loudness_enabled)?;
        write!(f, "loudness under adj={}", self.loudness_under_adj)?;
        write!(f, "loudness enabled diag={}", self.loudness_enabled_diag)?;
        write!(f, "fader enabled diag={}", self.fader_enabled_diag)?;
        write!(f, "musical ambiance={}", self.musical_ambiance)?;
        write!(
            f,
            "musical ambiance under adj={}",
            self.musical_ambiance_under_adj
        )?;
        write!(f, "impossible setting={}", self.impossible_setting)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        config::{MusicalAmbiance, SpeedDependentVolumeLaw},
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 7] = [0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x47, 0x00];
    static REPR_FRAME_BYTES_2: [u8; 7] = [0xbf, 0xbf, 0xbf, 0xbf, 0xbf, 0xd7, 0x6f];

    fn frame_1_repr() -> Repr {
        Repr {
            balance_level: 63,
            balance_under_adj: false,
            fader_level: 63,
            fader_under_adj: false,
            bass_level: 63,
            bass_under_adj: false,
            middle_level: 63,
            middle_under_adj: false,
            treble_level: 63,
            treble_under_adj: false,
            speed_dependent_volume: SpeedDependentVolumeLaw::On,
            speed_dependent_volume_under_adj: false,
            loudness_enabled: true,
            loudness_under_adj: false,
            loudness_enabled_diag: false,
            fader_enabled_diag: false,
            musical_ambiance: MusicalAmbiance::None,
            musical_ambiance_under_adj: false,
            impossible_setting: false,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            balance_level: 63,
            balance_under_adj: true,
            fader_level: 63,
            fader_under_adj: true,
            bass_level: 63,
            bass_under_adj: true,
            middle_level: 63,
            middle_under_adj: true,
            treble_level: 63,
            treble_under_adj: true,
            speed_dependent_volume: SpeedDependentVolumeLaw::On,
            speed_dependent_volume_under_adj: true,
            loudness_enabled: true,
            loudness_under_adj: true,
            loudness_enabled_diag: true,
            fader_enabled_diag: true,
            musical_ambiance: MusicalAmbiance::PopRock,
            musical_ambiance_under_adj: true,
            impossible_setting: true,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.balance_level(), 63);
        assert_eq!(frame.balance_under_adjustment(), false);
        assert_eq!(frame.fader_level(), 63);
        assert_eq!(frame.fader_under_adjustment(), false);
        assert_eq!(frame.bass_level(), 63);
        assert_eq!(frame.bass_under_adjustment(), false);
        assert_eq!(frame.middle_level(), 63);
        assert_eq!(frame.middle_under_adjustment(), false);
        assert_eq!(frame.treble_level(), 63);
        assert_eq!(frame.treble_under_adjustment(), false);
        assert_eq!(frame.speed_dependent_volume(), SpeedDependentVolumeLaw::On);
        assert_eq!(frame.speed_dependent_volume_under_adjustment(), false);
        assert_eq!(frame.loudness_enabled(), true);
        assert_eq!(frame.loudness_under_adjustment(), false);
        assert_eq!(frame.loudness_enabled_diag(), false);
        assert_eq!(frame.fader_enabled_diag(), false);
        assert_eq!(frame.musical_ambiance(), MusicalAmbiance::None);
        assert_eq!(frame.musical_ambiance_under_adjustment(), false);
        assert_eq!(frame.impossible_setting(), false);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.balance_level(), 63);
        assert_eq!(frame.balance_under_adjustment(), true);
        assert_eq!(frame.fader_level(), 63);
        assert_eq!(frame.fader_under_adjustment(), true);
        assert_eq!(frame.bass_level(), 63);
        assert_eq!(frame.bass_under_adjustment(), true);
        assert_eq!(frame.middle_level(), 63);
        assert_eq!(frame.middle_under_adjustment(), true);
        assert_eq!(frame.treble_level(), 63);
        assert_eq!(frame.treble_under_adjustment(), true);
        assert_eq!(frame.speed_dependent_volume(), SpeedDependentVolumeLaw::On);
        assert_eq!(frame.speed_dependent_volume_under_adjustment(), true);
        assert_eq!(frame.loudness_enabled(), true);
        assert_eq!(frame.loudness_under_adjustment(), true);
        assert_eq!(frame.loudness_enabled_diag(), true);
        assert_eq!(frame.fader_enabled_diag(), true);
        assert_eq!(frame.musical_ambiance(), MusicalAmbiance::PopRock);
        assert_eq!(frame.musical_ambiance_under_adjustment(), true);
        assert_eq!(frame.impossible_setting(), true);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_balance_level(63);
        frame.set_balance_under_adjustment(false);
        frame.set_fader_level(63);
        frame.set_fader_under_adjustment(false);
        frame.set_bass_level(63);
        frame.set_bass_under_adjustment(false);
        frame.set_middle_level(63);
        frame.set_middle_under_adjustment(false);
        frame.set_treble_level(63);
        frame.set_treble_under_adjustment(false);
        frame.set_speed_dependent_volume(SpeedDependentVolumeLaw::On);
        frame.set_speed_dependent_volume_under_adjustment(false);
        frame.set_loudness_enabled(true);
        frame.set_loudness_under_adjustment(false);
        frame.set_loudness_enabled_diag(false);
        frame.set_fader_enabled_diag(false);
        frame.set_musical_ambiance(MusicalAmbiance::None);
        frame.set_musical_ambiance_under_adjustment(false);
        frame.set_impossible_setting(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_balance_level(63);
        frame.set_balance_under_adjustment(true);
        frame.set_fader_level(63);
        frame.set_fader_under_adjustment(true);
        frame.set_bass_level(63);
        frame.set_bass_under_adjustment(true);
        frame.set_middle_level(63);
        frame.set_middle_under_adjustment(true);
        frame.set_treble_level(63);
        frame.set_treble_under_adjustment(true);
        frame.set_speed_dependent_volume(SpeedDependentVolumeLaw::On);
        frame.set_speed_dependent_volume_under_adjustment(true);
        frame.set_loudness_enabled(true);
        frame.set_loudness_under_adjustment(true);
        frame.set_loudness_enabled_diag(true);
        frame.set_fader_enabled_diag(true);
        frame.set_musical_ambiance(MusicalAmbiance::PopRock);
        frame.set_musical_ambiance_under_adjustment(true);
        frame.set_impossible_setting(true);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 8] = [0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x47, 0x00, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 6] = [0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x47];
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
