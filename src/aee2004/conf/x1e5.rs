use core::{cmp::Ordering, fmt};

use crate::{
    config::{MusicalAmbiance, SpeedDependentVolumeLaw},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    /// 7-bit balance level,
    /// 1-bit balance under adjustment flag.
    pub const BALANCE_ADJ: usize = 0;
    /// 7-bit fader level,
    /// 1-bit fader under adjustment flag.
    pub const FADER_ADJ: usize = 1;
    /// 7-bit bass level,
    /// 1-bit bass under adjustment flag.
    pub const BASS_ADJ: usize = 2;
    /// 7-bit middle level,
    /// 1-bit middle under adjustment flag.
    pub const MIDDLE_ADJ: usize = 3;
    /// 7-bit treble level,
    /// 1-bit treble under adjustment flag.
    pub const TREBLE_ADJ: usize = 4;
    /// 3-bit speed-dependent volume control law,
    /// 1-bit empty,
    /// 1-bit speed-dependent volume control under adjustment flag,
    /// 1-bit empty,
    /// 1-bit loudness activation flag,
    /// 1-bit loudness under adjustment flag.
    pub const SPD_VOL_ADJ_LOUD_ADJ: usize = 5;
    /// 1-bit loudness activation flag (via diagnostic session),
    /// 1-bit fader activation flag (via diagnostic session),
    /// 3-bit musical ambiance setting,
    /// 1-bit impossible audio setting with phone as audio source,
    /// 1-bit musical ambiance under adjustment flag,
    /// 1-bit empty.
    pub const FLAGS_AMBIANCE: usize = 6;
}

/// Raw x1e5 CAN frame identifier.
pub const FRAME_ID: u16 = 0x1e5;
/// Length of a x1e5 CAN frame.
pub const FRAME_LEN: usize = field::FLAGS_AMBIANCE + 1;

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

    /// Return the balance level field.
    #[inline]
    pub fn balance_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::BALANCE_ADJ] & 0x7f
    }

    /// Return the balance under adjustment flag.
    #[inline]
    pub fn balance_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::BALANCE_ADJ] & 0x80 != 0
    }

    /// Return the fader level field.
    #[inline]
    pub fn fader_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::FADER_ADJ] & 0x7f
    }

    /// Return the fader under adjustment flag.
    #[inline]
    pub fn fader_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FADER_ADJ] & 0x80 != 0
    }

    /// Return the bass level field.
    #[inline]
    pub fn bass_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::BASS_ADJ] & 0x7f
    }

    /// Return the bass under adjustment flag.
    #[inline]
    pub fn bass_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::BASS_ADJ] & 0x80 != 0
    }

    /// Return the middle level field.
    #[inline]
    pub fn middle_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::MIDDLE_ADJ] & 0x7f
    }

    /// Return the middle under adjustment flag.
    #[inline]
    pub fn middle_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::MIDDLE_ADJ] & 0x80 != 0
    }

    /// Return the treble level field.
    #[inline]
    pub fn treble_level(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::TREBLE_ADJ] & 0x7f
    }

    /// Return the middle under adjustment flag.
    #[inline]
    pub fn treble_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::TREBLE_ADJ] & 0x80 != 0
    }

    /// Return the speed-dependent volume law field.
    #[inline]
    pub fn speed_dependent_volume(&self) -> SpeedDependentVolumeLaw {
        let data = self.buffer.as_ref();
        let raw = data[field::SPD_VOL_ADJ_LOUD_ADJ] & 0x07;
        SpeedDependentVolumeLaw::from(raw)
    }

    /// Return the speed-dependent volume under adjustment flag.
    #[inline]
    pub fn speed_dependent_volume_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::SPD_VOL_ADJ_LOUD_ADJ] & 0x10 != 0
    }

    /// Return whether loudness is enabled or not.
    #[inline]
    pub fn loudness_enabled(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::SPD_VOL_ADJ_LOUD_ADJ] & 0x40 != 0
    }

    /// Return the loudness under adjustment flag.
    #[inline]
    pub fn loudness_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::SPD_VOL_ADJ_LOUD_ADJ] & 0x80 != 0
    }

    /// Return whether loudness is enabled or not (via diagnostic session).
    #[inline]
    pub fn loudness_enabled_diag(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_AMBIANCE] & 0x01 != 0
    }

    /// Return whether the fader is enabled or not (via diagnostic session).
    #[inline]
    pub fn fader_enabled_diag(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_AMBIANCE] & 0x02 != 0
    }

    /// Return the musical ambiance field.
    #[inline]
    pub fn musical_ambiance(&self) -> MusicalAmbiance {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_AMBIANCE] & 0x1c) >> 2;
        MusicalAmbiance::from(raw)
    }

    /// Return the impossible setting with phone as audio source flag.
    #[inline]
    pub fn impossible_setting(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_AMBIANCE] & 0x20 != 0
    }

    /// Return the musical ambiance under adjustment flag.
    #[inline]
    pub fn musical_ambiance_under_adjustment(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_AMBIANCE] & 0x40 != 0
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
    pub balance_level: u8,
    pub balance_under_adj: bool,
    pub fader_level: u8,
    pub fader_under_adj: bool,
    pub bass_level: u8,
    pub bass_under_adj: bool,
    pub middle_level: u8,
    pub middle_under_adj: bool,
    pub treble_level: u8,
    pub treble_under_adj: bool,
    pub speed_dependent_volume: SpeedDependentVolumeLaw,
    pub speed_dependent_volume_under_adj: bool,
    pub loudness_enabled: bool,
    pub loudness_under_adj: bool,
    pub loudness_enabled_diag: bool,
    pub fader_enabled_diag: bool,
    pub musical_ambiance: MusicalAmbiance,
    pub musical_ambiance_under_adj: bool,
    pub impossible_setting: bool,
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
        writeln!(f, "x1e5 balance_level={}", self.balance_level)?;
        writeln!(f, " balance_under_adj={}", self.balance_under_adj)?;
        writeln!(f, " fader_level={}", self.fader_level)?;
        writeln!(f, " fader_under_adj={}", self.fader_under_adj)?;
        writeln!(f, " bass_level={}", self.bass_level)?;
        writeln!(f, " bass_under_adj={}", self.bass_under_adj)?;
        writeln!(f, " middle_level={}", self.middle_level)?;
        writeln!(f, " middle_under_adj={}", self.middle_under_adj)?;
        writeln!(f, " treble_level={}", self.treble_level)?;
        writeln!(f, " treble_under_adj={}", self.treble_under_adj)?;
        writeln!(f, " speed_dependent_volume={}", self.speed_dependent_volume)?;
        writeln!(
            f,
            " speed_dependent_volume_under_adj={}",
            self.speed_dependent_volume_under_adj
        )?;
        writeln!(f, " loudness_enabled={}", self.loudness_enabled)?;
        writeln!(f, " loudness_under_adj={}", self.loudness_under_adj)?;
        writeln!(f, " loudness_enabled_diag={}", self.loudness_enabled_diag)?;
        writeln!(f, " fader_enabled_diag={}", self.fader_enabled_diag)?;
        writeln!(f, " musical_ambiance={}", self.musical_ambiance)?;
        writeln!(
            f,
            " musical_ambiance_under_adj={}",
            self.musical_ambiance_under_adj
        )?;
        writeln!(f, " impossible_setting={}", self.impossible_setting)
    }
}

impl From<&crate::aee2010::infodiv::x1e5::Repr> for Repr {
    fn from(repr_2010: &crate::aee2010::infodiv::x1e5::Repr) -> Self {
        Repr {
            balance_level: repr_2010.balance_level + 49,
            balance_under_adj: repr_2010.balance_under_adj,
            fader_level: repr_2010.fader_level + 49,
            fader_under_adj: repr_2010.fader_under_adj,
            bass_level: repr_2010.bass_level + 49,
            bass_under_adj: repr_2010.bass_under_adj,
            middle_level: 0x3f,
            middle_under_adj: false,
            treble_level: repr_2010.treble_level + 49,
            treble_under_adj: repr_2010.treble_under_adj,
            speed_dependent_volume: if repr_2010.speed_dependent_volume_enabled {
                SpeedDependentVolumeLaw::On
            } else {
                SpeedDependentVolumeLaw::Off
            },
            speed_dependent_volume_under_adj: repr_2010.speed_dependent_volume_under_adj,
            loudness_enabled: repr_2010.loudness_enabled,
            loudness_under_adj: repr_2010.loudness_under_adj,
            loudness_enabled_diag: false,
            fader_enabled_diag: repr_2010.fader_opt
                == crate::config::ConfigOption::SelectableOption,
            musical_ambiance: repr_2010.musical_ambiance,
            musical_ambiance_under_adj: repr_2010.musical_ambiance_under_adj,
            impossible_setting: repr_2010.impossible_setting,
        }
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
