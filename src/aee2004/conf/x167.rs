use core::fmt;

use crate::{
    mfd::{TripComputerPage, UserAction},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    use crate::field::*;
    /// 3-bit multi-function display trip computer displayed page,
    /// 1-bit maintenance reset request,
    /// 1-bit emergency call in progress,
    /// 1-bit fault check recall request,
    /// 1-bit trip computer secondary trip reset request,
    /// 1-bit trip computer primary trip reset request.
    pub const REQ_0: usize = 0;
    /// 4-bit pre-conditioning time,
    /// 1-bit telematics state,
    /// 1-bit black panel function state,
    /// 1-bit indirect under-inflation detection reset request,
    /// 1-bit thermal pre-conditioning request.
    pub const REQ_1: usize = 1;
    /// 16-bit total trip distance.
    pub const TOTAL_TRIP_DISTANCE: Field = 2..4;
    /// 15-bit interactive message.
    /// 1-bit empty
    pub const INTERACTIVE_MSG: Field = 4..6;
    /// 1-bit stop and start push button state,
    /// 1-bit lane centering push button state,
    /// 1-bit parking sensors push button state,
    /// 1-bit empty
    /// 4-bit user action on MFD.
    pub const PUSHS_ACTION: usize = 6;
    /// 8-bit value set by user.
    pub const VALUE: usize = 7;
}

/// Length of a x167 CAN frame.
pub const FRAME_LEN: usize = field::VALUE + 1;

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

        /// Return the multi-function display trip computer displayed page field.

    #[inline]
    pub fn mfd_trip_computer_page(&self) -> TripComputerPage {
        let data = self.buffer.as_ref();
        let raw = data[field::REQ_0] & 0x07;
        TripComputerPage::from(raw)
    }

    /// Return the maintenance reset request field.
    #[inline]
    pub fn maintenance_reset_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x08 != 0
    }

    /// Return the emergency call in progress field.
    #[inline]
    pub fn emergency_call_in_progress(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x10 != 0
    }

    /// Return the fault check recall request field.
    #[inline]
    pub fn fault_recall_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x20 != 0
    }

    /// Return the trip computer secondary trip reset request field.
    #[inline]
    pub fn trip_computer_secondary_trip_reset_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x40 != 0
    }

    /// Return the trip computer secondary trip reset request field.
    #[inline]
    pub fn trip_computer_primary_trip_reset_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x80 != 0
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
                write!(f, "x167 ({})", err)?;
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

/// A high-level representation of a x167 CAN frame.
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

    /// Emit a high-level representation into a x167 CAN frame.
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