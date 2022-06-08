use core::{cmp::Ordering, fmt, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};

use crate::{
    vehicle::{
        AdaptiveCruiseControlState, SpeedRegulationMode, SpeedRegulationModeState,
        SpeedRegulationSettingPage,
    },
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
228 ACC_XVV_IHM_ETAT_ACC_STOP_LIGHT_REQUEST_HS7_228
228 ACC_XVV_IHM_ETAT_AUTO_HOLD_ACC_HS7_228
228 ACC_XVV_IHM_ETAT_CONS_TIV_ACC_HS7_228               // OK
228 ACC_XVV_IHM_ETAT_DMD_MEM_CSV_HS7_228                // OK
228 ACC_XVV_IHM_ETAT_DMD_PAGE_CONS_XVV_HS7_228          // OK
228 ACC_XVV_IHM_ETAT_ETAT_FONCT_LVV_RVV_HS7_228         // OK
228 ACC_XVV_IHM_ETAT_ETAT_IHM_RVVI_HS7_228              // OK
228 ACC_XVV_IHM_ETAT_FONCT_ACT_LVV_RVV_HS7_228          // OK
228 ACC_XVV_IHM_ETAT_P_INFO_XVV_INCIT_PLV_HS7_228       // OK
228 ACC_XVV_IHM_ETAT_PRESENCE_CIBLE_HS7_228
228 ACC_XVV_IHM_ETAT_REGL_CONS_TIV_ACC_HS7_228
228 ACC_XVV_IHM_ETAT_TENT_ACT_LVV_RVV_HS7_228           // OK
228 ACC_XVV_IHM_ETAT_TIV_ACC_HS7_228
228 ACC_XVV_IHM_ETAT_VIT_CONS_LVV_RVV_HS7_228           // OK
228 ACC_XVV_IHM_ETAT_VIT_CONS_RVVI_AJUST_HS7_228        // OK
228 ACC_XVV_IHM_ETAT_XVV_APPEL_REGL_TIV_HS7_228
228 ACC_XVV_IHM_ETAT_XVV_DISPONIBLE_HS7_228             // OK
228 ACC_XVV_IHM_ETAT_XVV_REGL_CONS_VIT_HS7_228          // OK
 */

mod field {
    use crate::field::Field;
    /// 16-bit cruise-control/speed-limiter/acc speed setting field.
    pub const SPD_INST: Field = 0..2;
    /// 1-bit speed setting adjustment in progress flag,
    /// 1-bit unknown,
    /// 1-bit cruise-control/speed-limiter/acc try enable flag,
    /// 3-bit cruise-control/speed-limiter/acc activated mode state field,
    /// 2-bit cruise-control/speed-limiter/acc activated mode field.
    pub const XVV_1: usize = 2;
    /// 7-bit unknown
    /// 1-bit cruise-control/speed-limiter/acc availability flag.
    pub const XVV_2: usize = 3;
    /// 6-bit adaptive cruise-control time setting field,
    /// 2-bit cruise-control/speed-limiter/acc setting page request field.
    pub const XVV_3: usize = 4;
    /// 16-bit adaptive cruise-control adjusted speed setting field.
    pub const ACC_ADJ_SPD: Field = 5..7;
    /// 1-bit cruise-control/speed-limiter/acc speed setting from traffic sign recognition allowed flag,
    /// 1-bit 'mem' key (on the steering-wheel control) pressed flag,
    /// 2-bit unknown,
    /// 4-bit adaptive cruise-control displayed state field.
    pub const XVV_4: usize = 7;
}

/// Raw x228 CAN frame identifier.
pub const FRAME_ID: u16 = 0x228;
/// Length of a x228 CAN frame.
pub const FRAME_LEN: usize = field::XVV_4 + 1;

/// Periodicity of a x228 CAN frame.
pub const PERIODICITY: Duration = Duration::from_millis(1000);

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

    /// Return the cruise-control/speed-limiter/acc speed setting field.
    #[inline]
    pub fn speed_setting(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::SPD_INST])
    }

    /// Return the speed setting adjustment in progress flag.
    #[inline]
    pub fn speed_setting_adjustment_in_progress(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::XVV_1] & 0x01 != 0
    }

    /// Return the cruise-control/speed-limiter/acc try enable flag.
    #[inline]
    pub fn try_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::XVV_1] & 0x04 != 0
    }

    /// Return the cruise-control/speed-limiter/acc activated mode state field.
    #[inline]
    pub fn speed_regulation_mode_state(&self) -> SpeedRegulationModeState {
        let data = self.buffer.as_ref();
        let raw = (data[field::XVV_1] & 0x38) >> 3;
        SpeedRegulationModeState::from(raw)
    }

    /// Return the cruise-control/speed-limiter/acc activated mode field.
    #[inline]
    pub fn speed_regulation_mode(&self) -> SpeedRegulationMode {
        let data = self.buffer.as_ref();
        let raw = data[field::XVV_1] >> 6;
        SpeedRegulationMode::from(raw)
    }

    /// Return the cruise-control/speed-limiter/acc availability flag.
    #[inline]
    pub fn speed_regulation_available(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::XVV_2] & 0x80 != 0
    }

    /// Return the adaptive cruise-control time setting field.
    #[inline]
    pub fn acc_time(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::XVV_3] & 0x3f
    }

    /// Return the cruise-control/speed-limiter/acc setting page request field.
    #[inline]
    pub fn speed_regulation_page_req(&self) -> SpeedRegulationSettingPage {
        let data = self.buffer.as_ref();
        SpeedRegulationSettingPage::from(data[field::XVV_3] >> 6)
    }

    /// Return the adaptive cruise-control adjusted speed setting field.
    #[inline]
    pub fn acc_adjusted_speed(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::ACC_ADJ_SPD])
    }

    /// Return the cruise-control/speed-limiter/acc speed setting from traffic sign recognition allowed flag.
    #[inline]
    pub fn set_speed_from_traffic_sign_recognition_allowed(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::XVV_4] & 0x01 != 0
    }

    /// Return the 'mem' key (on the steering-wheel control) pressed flag.
    #[inline]
    pub fn mem_key_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::XVV_4] & 0x02 != 0
    }

    /// Return adaptive cruise-control displayed state field.
    #[inline]
    pub fn acc_displayed_state(&self) -> AdaptiveCruiseControlState {
        let data = self.buffer.as_ref();
        AdaptiveCruiseControlState::from(data[field::XVV_4] >> 4)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the cruise-control/speed-limiter/acc speed setting field.
    #[inline]
    pub fn set_speed_setting(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::SPD_INST], value);
    }

    /// Set the speed setting adjustment in progress flag.
    #[inline]
    pub fn set_speed_setting_adjustment_in_progress(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::XVV_1];
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::XVV_1] = raw;
    }

    /// Set the cruise-control/speed-limiter/acc try enable flag.
    #[inline]
    pub fn set_try_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::XVV_1];
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::XVV_1] = raw;
    }

    /// Set the cruise-control/speed-limiter/acc activated mode state field.
    #[inline]
    pub fn set_speed_regulation_mode_state(&mut self, value: SpeedRegulationModeState) {
        let data = self.buffer.as_mut();
        let raw = data[field::XVV_1] & !0x38;
        let raw = raw | ((u8::from(value) << 3) & 0x38);
        data[field::XVV_1] = raw;
    }

    /// Set the cruise-control/speed-limiter/acc activated mode field.
    #[inline]
    pub fn set_speed_regulation_mode(&mut self, value: SpeedRegulationMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::XVV_1] & !0xc0;
        let raw = raw | (u8::from(value) << 6);
        data[field::XVV_1] = raw;
    }

    /// Set the cruise-control/speed-limiter/acc availability flag.
    #[inline]
    pub fn set_speed_regulation_available(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::XVV_2];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::XVV_2] = raw;
    }

    /// Set the adaptive cruise-control time setting field.
    #[inline]
    pub fn set_acc_time(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::XVV_3] & !0x3f;
        let raw = raw | (value & 0x3f);
        data[field::XVV_3] = raw;
    }

    /// Set the cruise-control/speed-limiter/acc setting page request field.
    #[inline]
    pub fn set_speed_regulation_page_req(&mut self, value: SpeedRegulationSettingPage) {
        let data = self.buffer.as_mut();
        let raw = data[field::XVV_3] & !0xc0;
        let raw = raw | (u8::from(value) << 6);
        data[field::XVV_3] = raw;
    }

    /// Set the adaptive cruise-control adjusted speed setting field.
    #[inline]
    pub fn set_acc_adjusted_speed(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::ACC_ADJ_SPD], value);
    }

    /// Set the cruise-control/speed-limiter/acc speed setting from traffic sign recognition allowed flag.
    #[inline]
    pub fn set_set_speed_from_traffic_sign_recognition_allowed(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::XVV_4];
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::XVV_4] = raw;
    }

    /// Set the 'mem' key (on the steering-wheel control) pressed flag.
    #[inline]
    pub fn set_mem_key_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::XVV_4];
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::XVV_4] = raw;
    }

    /// Set the adaptive cruise-control displayed state field.
    #[inline]
    pub fn set_acc_displayed_state(&mut self, value: AdaptiveCruiseControlState) {
        let data = self.buffer.as_mut();
        let raw = data[field::XVV_4] & !0xf0;
        let raw = raw | (u8::from(value) << 4);
        data[field::XVV_4] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x228 ({})", err)?;
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

/// A high-level representation of a x228 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub speed_setting: u16,
    pub speed_setting_adjustment_in_progress: bool,
    pub try_enable: bool,
    pub speed_regulation_mode_state: SpeedRegulationModeState,
    pub speed_regulation_mode: SpeedRegulationMode,
    pub speed_regulation_available: bool,
    pub acc_time: u8,
    pub speed_regulation_page_req: SpeedRegulationSettingPage,
    pub acc_adjusted_speed: u16,
    pub set_speed_from_traffic_sign_recognition_allowed: bool,
    pub mem_key_state: bool,
    pub acc_displayed_state: AdaptiveCruiseControlState,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            speed_setting: frame.speed_setting(),
            speed_setting_adjustment_in_progress: frame.speed_setting_adjustment_in_progress(),
            try_enable: frame.try_enable(),
            speed_regulation_mode_state: frame.speed_regulation_mode_state(),
            speed_regulation_mode: frame.speed_regulation_mode(),
            speed_regulation_available: frame.speed_regulation_available(),
            acc_time: frame.acc_time(),
            speed_regulation_page_req: frame.speed_regulation_page_req(),
            acc_adjusted_speed: frame.acc_adjusted_speed(),
            set_speed_from_traffic_sign_recognition_allowed: frame
                .set_speed_from_traffic_sign_recognition_allowed(),
            mem_key_state: frame.mem_key_state(),
            acc_displayed_state: frame.acc_displayed_state(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x228 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_speed_setting(self.speed_setting);
        frame.set_speed_setting_adjustment_in_progress(self.speed_setting_adjustment_in_progress);
        frame.set_try_enable(self.try_enable);
        frame.set_speed_regulation_mode_state(self.speed_regulation_mode_state);
        frame.set_speed_regulation_mode(self.speed_regulation_mode);
        frame.set_speed_regulation_available(self.speed_regulation_available);
        frame.set_acc_time(self.acc_time);
        frame.set_speed_regulation_page_req(self.speed_regulation_page_req);
        frame.set_acc_adjusted_speed(self.acc_adjusted_speed);
        frame.set_set_speed_from_traffic_sign_recognition_allowed(
            self.set_speed_from_traffic_sign_recognition_allowed,
        );
        frame.set_mem_key_state(self.mem_key_state);
        frame.set_acc_displayed_state(self.acc_displayed_state);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x228")?;
        writeln!(f, " speed_setting={}", self.speed_setting)?;
        writeln!(
            f,
            " speed_setting_adjustment_in_progress={}",
            self.speed_setting_adjustment_in_progress
        )?;
        writeln!(f, " try_enable={}", self.try_enable)?;
        writeln!(
            f,
            " speed_regulation_mode_state={}",
            self.speed_regulation_mode_state
        )?;
        writeln!(f, " speed_regulation_mode={}", self.speed_regulation_mode)?;
        writeln!(
            f,
            " speed_regulation_available={}",
            self.speed_regulation_available
        )?;
        writeln!(f, " acc_time={}", self.acc_time)?;
        writeln!(
            f,
            " speed_regulation_page_req={}",
            self.speed_regulation_page_req
        )?;
        writeln!(f, " acc_adjusted_speed={}", self.acc_adjusted_speed)?;
        writeln!(
            f,
            " set_speed_from_traffic_sign_recognition_allowed={}",
            self.set_speed_from_traffic_sign_recognition_allowed
        )?;
        writeln!(f, " mem_key_state={}", self.mem_key_state)?;
        writeln!(f, " acc_displayed_state={}", self.acc_displayed_state)
    }
}

impl From<&crate::aee2004::conf::x1a8::Repr> for Repr {
    fn from(repr_2004: &crate::aee2004::conf::x1a8::Repr) -> Self {
        Repr {
            speed_setting: repr_2004.speed_setting,
            speed_setting_adjustment_in_progress: false, // No known conversion.
            try_enable: repr_2004.try_enable,
            speed_regulation_mode_state: repr_2004.speed_regulation_mode_state,
            speed_regulation_mode: repr_2004.speed_regulation_mode,
            speed_regulation_available: true, // No known conversion.
            acc_time: 0x14,                   // Seems to be the default value.
            speed_regulation_page_req: SpeedRegulationSettingPage::Close, // No known conversion.
            acc_adjusted_speed: 0x7fff,       // Seems to be the default value.
            set_speed_from_traffic_sign_recognition_allowed: false, // No traffic sign recognition on AEE2004.
            mem_key_state: false, // No mem key on AEE2004 cruise control.
            acc_displayed_state: AdaptiveCruiseControlState::Disabled,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        vehicle::{
            AdaptiveCruiseControlState, SpeedRegulationMode, SpeedRegulationModeState,
            SpeedRegulationSettingPage,
        },
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x00, 0x82, 0x49, 0x80, 0x80, 0x00, 0x00, 0x92];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0x00, 0x49, 0xc4, 0x00, 0x0a, 0x00, 0x45, 0x11];

    fn frame_1_repr() -> Repr {
        Repr {
            speed_setting: 130,
            speed_setting_adjustment_in_progress: true,
            try_enable: false,
            speed_regulation_mode_state: SpeedRegulationModeState::Up,
            speed_regulation_mode: SpeedRegulationMode::CruiseControl,
            speed_regulation_available: true,
            acc_time: 0,
            speed_regulation_page_req: SpeedRegulationSettingPage::CruiseControl,
            acc_adjusted_speed: 0,
            set_speed_from_traffic_sign_recognition_allowed: false,
            mem_key_state: true,
            acc_displayed_state: AdaptiveCruiseControlState::Disabled,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            speed_setting: 73,
            speed_setting_adjustment_in_progress: false,
            try_enable: true,
            speed_regulation_mode_state: SpeedRegulationModeState::Standby,
            speed_regulation_mode: SpeedRegulationMode::AdaptiveCruiseControl,
            speed_regulation_available: false,
            acc_time: 10,
            speed_regulation_page_req: SpeedRegulationSettingPage::Close,
            acc_adjusted_speed: 69,
            set_speed_from_traffic_sign_recognition_allowed: true,
            mem_key_state: false,
            acc_displayed_state: AdaptiveCruiseControlState::AdjustInProgress,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.speed_setting(), 130);
        assert_eq!(frame.speed_setting_adjustment_in_progress(), true);
        assert_eq!(frame.try_enable(), false);
        assert_eq!(
            frame.speed_regulation_mode_state(),
            SpeedRegulationModeState::Up
        );
        assert_eq!(
            frame.speed_regulation_mode(),
            SpeedRegulationMode::CruiseControl
        );
        assert_eq!(frame.speed_regulation_available(), true);
        assert_eq!(frame.acc_time(), 0);
        assert_eq!(
            frame.speed_regulation_page_req(),
            SpeedRegulationSettingPage::CruiseControl
        );
        assert_eq!(frame.acc_adjusted_speed(), 0);
        assert_eq!(
            frame.set_speed_from_traffic_sign_recognition_allowed(),
            false
        );
        assert_eq!(frame.mem_key_state(), true);
        assert_eq!(
            frame.acc_displayed_state(),
            AdaptiveCruiseControlState::Disabled
        );
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.speed_setting(), 73);
        assert_eq!(frame.speed_setting_adjustment_in_progress(), false);
        assert_eq!(frame.try_enable(), true);
        assert_eq!(
            frame.speed_regulation_mode_state(),
            SpeedRegulationModeState::Standby
        );
        assert_eq!(
            frame.speed_regulation_mode(),
            SpeedRegulationMode::AdaptiveCruiseControl
        );
        assert_eq!(frame.speed_regulation_available(), false);
        assert_eq!(frame.acc_time(), 10);
        assert_eq!(
            frame.speed_regulation_page_req(),
            SpeedRegulationSettingPage::Close
        );
        assert_eq!(frame.acc_adjusted_speed(), 69);
        assert_eq!(
            frame.set_speed_from_traffic_sign_recognition_allowed(),
            true
        );
        assert_eq!(frame.mem_key_state(), false);
        assert_eq!(
            frame.acc_displayed_state(),
            AdaptiveCruiseControlState::AdjustInProgress
        );
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_speed_setting(130);
        frame.set_speed_setting_adjustment_in_progress(true);
        frame.set_try_enable(false);
        frame.set_speed_regulation_mode_state(SpeedRegulationModeState::Up);
        frame.set_speed_regulation_mode(SpeedRegulationMode::CruiseControl);
        frame.set_speed_regulation_available(true);
        frame.set_acc_time(0);
        frame.set_speed_regulation_page_req(SpeedRegulationSettingPage::CruiseControl);
        frame.set_acc_adjusted_speed(0);
        frame.set_set_speed_from_traffic_sign_recognition_allowed(false);
        frame.set_mem_key_state(true);
        frame.set_acc_displayed_state(AdaptiveCruiseControlState::Disabled);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_speed_setting(73);
        frame.set_speed_setting_adjustment_in_progress(false);
        frame.set_try_enable(true);
        frame.set_speed_regulation_mode_state(SpeedRegulationModeState::Standby);
        frame.set_speed_regulation_mode(SpeedRegulationMode::AdaptiveCruiseControl);
        frame.set_speed_regulation_available(false);
        frame.set_acc_time(10);
        frame.set_speed_regulation_page_req(SpeedRegulationSettingPage::Close);
        frame.set_acc_adjusted_speed(69);
        frame.set_set_speed_from_traffic_sign_recognition_allowed(true);
        frame.set_mem_key_state(false);
        frame.set_acc_displayed_state(AdaptiveCruiseControlState::AdjustInProgress);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x00, 0x82, 0x49, 0x80, 0x80, 0x00, 0x00, 0x92, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x00, 0x82, 0x49, 0x80, 0x80, 0x00, 0x00];
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
        let mut buf = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_basic_repr_2_emit() {
        let mut buf = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_2_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }
}
