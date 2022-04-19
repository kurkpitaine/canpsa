use crate::{Error, Result};
use core::{cmp::Ordering, fmt, time::Duration};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
329 DEMANDES_IVI_2_ACPK_ACTIVATION_DMD_HS7_329
329 DEMANDES_IVI_2_ACPK_MANEUVER_SIDE_DMD_HS7_329
329 DEMANDES_IVI_2_APPUI_CONSO_REINIT_HISTO_HS7_329
329 DEMANDES_IVI_2_APPUI_PUSH_ECLAIRAGE_ZEV_HS7_329
329 DEMANDES_IVI_2_BTNPSD_HEATGELMFORFRNTLEPASS_HS7_329
329 DEMANDES_IVI_2_BTNPSD_HEATGELMFORFRNTRIPASS_HS7_329
329 DEMANDES_IVI_2_BTNPSD_HEATGSTEERWHL_HS7_329
329 DEMANDES_IVI_2_CABIN_AIR_PURIFIER_CHG_RQST_HS7_329
329 DEMANDES_IVI_2_CARTRIDGE_MONO_FRAGRANCE_HS7_329        // OK
329 DEMANDES_IVI_2_CMD_ACTIV_MASS_HS7_329                  // OK
329 DEMANDES_IVI_2_CMD_INTENS_MASS_HS7_329                 // OK
329 DEMANDES_IVI_2_CMD_REG_MASS_AV_HS7_329                 // OK
329 DEMANDES_IVI_2_CMD_TYPE_MASS_HS7_329                   // OK
329 DEMANDES_IVI_2_DMD_FREIN_ASR_TACT_INHIB_HS7_329        // OK
329 DEMANDES_IVI_2_DMD_IONIZER_HS7_329
329 DEMANDES_IVI_2_DMDM_FRAGRANCE_DIFFUSER_HS7_329         // OK
329 DEMANDES_IVI_2_DMDM_FRAGRANCE_INTENSITY_HS7_329        // OK
329 DEMANDES_IVI_2_DMDM_FRAGRANCE_SELECTION_HS7_329        // OK
329 DEMANDES_IVI_2_DMD_REINIT_ALL_PARAM_HS7_329
329 DEMANDES_IVI_2_DMD_RESET_ECOACH_TRIP_HS7_329
329 DEMANDES_IVI_2_DMD_SELECT_ACHV_HS7_329
329 DEMANDES_IVI_2_ENT_AVERT_SON_VEH_SIL_HS7_329
329 DEMANDES_IVI_2_ENT_PUSH_AVN_HS7_329
329 DEMANDES_IVI_2_ENT_PUSH_ENERGY_RECOVER_HS7_329
329 DEMANDES_IVI_2_ENT_PUSH_LKA_HS7_329                    // OK
329 DEMANDES_IVI_2_HMISOUNDSPKRSTS_HS7_329
329 DEMANDES_IVI_2_MODE_DYN_SELECT_TACTILE_HS7_329         // OK
329 DEMANDES_IVI_2_PRESENCE_SHORTCUT_CLOSE_CMFT_HS7_329    // OK
329 DEMANDES_IVI_2_REINIT_CABIN_AIR_FILTER_HS7_329
329 DEMANDES_IVI_2_SELEC_CLIENT_ASR_P_HS7_329
329 DEMANDES_IVI_2_USRDMDCLEANCABIN_HS7_329
329 DEMANDES_IVI_2_VAL_CONSO_IHM_ZOOM_HISTO_HS7_329        // OK
 */

mod field {
    /// 2-bit fragrance diffuser mono-fragrance cartridge type field,
    /// 2-bit unknown,
    /// 1-bit lane-keep assist push button state flag,
    /// 3-bit unknown.
    pub const REQ_0: usize = 0;
    /// 2-bit front seats massage adjustment request field,
    /// 2-bit fragrance diffuser perfume selection request field,
    /// 2-bit fragrance diffuser intensity request field,
    /// 2-bit fragrance diffuser request field.
    pub const REQ_1: usize = 1;
    /// 1-bit unknown,
    /// 1-bit massage activation request flag,
    /// 2-bit massage intensity request field,
    /// 4-bit massage type field.
    pub const REQ_2: usize = 2;
    /// 1-bit traction control inhibit request flag,
    /// 2-bit consumption history zoomed value field,
    /// 2-bit dynamic mode selected mode value field,
    /// 3-bit unknown.
    pub const REQ_3: usize = 3;
    /// 8-bit unknown.
    pub const REQ_4: usize = 4;
    /// 8-bit unknown.
    pub const REQ_5: usize = 5;
    /// 8-bit unknown.
    pub const REQ_6: usize = 6;
}

/// Length of a x329 CAN frame.
pub const FRAME_LEN: usize = field::REQ_6 + 1;

/// Periodicity of a x329 CAN frame.
pub const PERIODICITY: Duration = Duration::from_millis(200);

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

    /// Return the fragrance diffuser mono-fragrance cartridge type field.
    #[inline]
    pub fn fragrance_diffuser_mono_type(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x03
    }

    /// Return the lane-keep assist push button state flag.
    #[inline]
    pub fn lane_keep_assist_button_state(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x10 != 0
    }

    /// Return the front seats massage adjustment request field.
    #[inline]
    pub fn front_massage_adjustment(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x03
    }

    /// Return the fragrance diffuser perfume selection request field.
    #[inline]
    pub fn fragrance_diffuser_perfume_selection(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::REQ_1] & 0x0c) >> 2
    }

    /// Return the fragrance diffuser intensity request field.
    #[inline]
    pub fn fragrance_diffuser_intensity(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::REQ_1] & 0x30) >> 4
    }

    /// Return the fragrance diffuser request field.
    #[inline]
    pub fn fragrance_diffuser_request(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::REQ_1] & 0xc0) >> 6
    }

    /// Return the massage activation request flag.
    #[inline]
    pub fn massage_activation_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_2] & 0x02 != 0
    }

    /// Return the massage intensity request field.
    #[inline]
    pub fn massage_intensity(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::REQ_2] & 0x0c) >> 2
    }

    /// Return the massage type field.
    #[inline]
    pub fn massage_type(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::REQ_2] & 0xf0) >> 4
    }

    /// Return the traction control inhibit request flag.
    #[inline]
    pub fn asr_inhibit(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_3] & 0x01 != 0
    }

    /// Return the consumption history zoomed value field.
    #[inline]
    pub fn consumption_history_zoomed_value(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::REQ_3] & 0x06) >> 1
    }

    /// Return the dynamic mode selected mode value field.
    #[inline]
    pub fn dynamic_mode_selected_mode(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::REQ_3] & 0x18) >> 3
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set fragrance diffuser mono-fragrance cartridge type field.
    #[inline]
    pub fn set_fragrance_diffuser_mono_type(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0] & !0x03;
        let raw = raw | (value & 0x03);
        data[field::REQ_0] = raw;
    }

    /// Set the lane-keep assist push button state flag.
    #[inline]
    pub fn set_lane_keep_assist_button_state(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0];
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::REQ_0] = raw;
    }

    /// Set the front seats massage adjustment request field.
    #[inline]
    pub fn set_front_massage_adjustment(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1] & !0x03;
        let raw = raw | (value & 0x03);
        data[field::REQ_1] = raw;
    }

    /// Set the fragrance diffuser perfume selection request field.
    #[inline]
    pub fn set_fragrance_diffuser_perfume_selection(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1] & !0x0c;
        let raw = raw | ((value << 2) & 0x0c);
        data[field::REQ_1] = raw;
    }

    /// Set the fragrance diffuser intensity request field.
    #[inline]
    pub fn set_fragrance_diffuser_intensity(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1] & !0x30;
        let raw = raw | ((value << 4) & 0x30);
        data[field::REQ_1] = raw;
    }

    /// Set the fragrance diffuser request field.
    #[inline]
    pub fn set_fragrance_diffuser_request(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1] & !0xc0;
        let raw = raw | ((value << 6) & 0xc0);
        data[field::REQ_1] = raw;
    }

    /// Set the massage activation request flag.
    #[inline]
    pub fn set_massage_activation_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_2];
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::REQ_2] = raw;
    }

    /// Set the massage intensity request field.
    #[inline]
    pub fn set_massage_intensity(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_2] & !0x0c;
        let raw = raw | ((value << 2) & 0x0c);
        data[field::REQ_2] = raw;
    }

    /// Set the massage type request field.
    #[inline]
    pub fn set_massage_type(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_2] & !0xf0;
        let raw = raw | ((value << 4) & 0xf0);
        data[field::REQ_2] = raw;
    }

    /// Set the traction control inhibit request flag.
    #[inline]
    pub fn set_asr_inhibit(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_3];
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::REQ_3] = raw;
    }

    /// Set the consumption history zoomed value field.
    #[inline]
    pub fn set_consumption_history_zoomed_value(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_3] & !0x06;
        let raw = raw | ((value << 1) & 0x06);
        data[field::REQ_3] = raw;
    }

    /// Set the dynamic mode selected mode value field.
    #[inline]
    pub fn set_dynamic_mode_selected_mode(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_3] & !0x18;
        let raw = raw | ((value << 3) & 0x18);
        data[field::REQ_3] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x329 ({})", err)?;
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

/// A high-level representation of a x329 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub fragrance_diffuser_mono_fragrance_cartridge_type: u8,
    pub lane_keep_assist_button_state: bool,
    pub front_seat_massage_adjustment: u8,
    pub fragrance_diffuser_perfume_selection: u8,
    pub fragrance_diffuser_intensity: u8,
    pub fragrance_diffuser_request: u8,
    pub massage_activation_request: bool,
    pub massage_intensity: u8,
    pub massage_type: u8,
    pub asr_inhibit: bool,
    pub consumption_history_zoomed_value: u8,
    pub dynamic_mode_selected_mode: u8,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            fragrance_diffuser_mono_fragrance_cartridge_type: frame.fragrance_diffuser_mono_type(),
            lane_keep_assist_button_state: frame.lane_keep_assist_button_state(),
            front_seat_massage_adjustment: frame.front_massage_adjustment(),
            fragrance_diffuser_perfume_selection: frame.fragrance_diffuser_perfume_selection(),
            fragrance_diffuser_intensity: frame.fragrance_diffuser_intensity(),
            fragrance_diffuser_request: frame.fragrance_diffuser_request(),
            massage_activation_request: frame.massage_activation_request(),
            massage_intensity: frame.massage_intensity(),
            massage_type: frame.massage_type(),
            asr_inhibit: frame.asr_inhibit(),
            consumption_history_zoomed_value: frame.consumption_history_zoomed_value(),
            dynamic_mode_selected_mode: frame.dynamic_mode_selected_mode(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x329 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_fragrance_diffuser_mono_type(
            self.fragrance_diffuser_mono_fragrance_cartridge_type,
        );
        frame.set_lane_keep_assist_button_state(self.lane_keep_assist_button_state);
        frame.set_front_massage_adjustment(self.front_seat_massage_adjustment);
        frame.set_fragrance_diffuser_perfume_selection(self.fragrance_diffuser_perfume_selection);
        frame.set_fragrance_diffuser_intensity(self.fragrance_diffuser_intensity);
        frame.set_fragrance_diffuser_request(self.fragrance_diffuser_request);
        frame.set_massage_activation_request(self.massage_activation_request);
        frame.set_massage_intensity(self.massage_intensity);
        frame.set_massage_type(self.massage_type);
        frame.set_asr_inhibit(self.asr_inhibit);
        frame.set_consumption_history_zoomed_value(self.consumption_history_zoomed_value);
        frame.set_dynamic_mode_selected_mode(self.dynamic_mode_selected_mode);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "x329 fragrance_diffuser_mono_fragrance_cartridge_type={}",
            self.fragrance_diffuser_mono_fragrance_cartridge_type
        )?;
        writeln!(
            f,
            " lane_keep_assist_button_state={}",
            self.lane_keep_assist_button_state
        )?;
        writeln!(
            f,
            " front_seat_massage_adjustment={}",
            self.front_seat_massage_adjustment
        )?;
        writeln!(
            f,
            " fragrance_diffuser_perfume_selection={}",
            self.fragrance_diffuser_perfume_selection
        )?;
        writeln!(
            f,
            " fragrance_diffuser_intensity={}",
            self.fragrance_diffuser_intensity
        )?;
        writeln!(
            f,
            " fragrance_diffuser_request={}",
            self.fragrance_diffuser_request
        )?;
        writeln!(
            f,
            " massage_activation_request={}",
            self.massage_activation_request
        )?;
        writeln!(f, " massage_intensity={}", self.massage_intensity)?;
        writeln!(f, " massage_type={}", self.massage_type)?;
        writeln!(f, " asr_inhibit={}", self.asr_inhibit)?;
        writeln!(
            f,
            " consumption_history_zoomed_value={}",
            self.consumption_history_zoomed_value
        )?;
        writeln!(
            f,
            " dynamic_mode_selected_mode={}",
            self.dynamic_mode_selected_mode
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::Error;

    static REPR_FRAME_BYTES_1: [u8; 7] = [0x10, 0x00, 0x02, 0x01, 0x00, 0x00, 0x00];
    static REPR_FRAME_BYTES_2: [u8; 7] = [0x03, 0xff, 0x3c, 0x1e, 0x00, 0x00, 0x00];

    fn frame_1_repr() -> Repr {
        Repr {
            fragrance_diffuser_mono_fragrance_cartridge_type: 0,
            lane_keep_assist_button_state: true,
            front_seat_massage_adjustment: 0,
            fragrance_diffuser_perfume_selection: 0,
            fragrance_diffuser_intensity: 0,
            fragrance_diffuser_request: 0,
            massage_activation_request: true,
            massage_intensity: 0,
            massage_type: 0,
            asr_inhibit: true,
            consumption_history_zoomed_value: 0,
            dynamic_mode_selected_mode: 0,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            fragrance_diffuser_mono_fragrance_cartridge_type: 3,
            lane_keep_assist_button_state: false,
            front_seat_massage_adjustment: 3,
            fragrance_diffuser_perfume_selection: 3,
            fragrance_diffuser_intensity: 3,
            fragrance_diffuser_request: 3,
            massage_activation_request: false,
            massage_intensity: 3,
            massage_type: 3,
            asr_inhibit: false,
            consumption_history_zoomed_value: 3,
            dynamic_mode_selected_mode: 3,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.fragrance_diffuser_mono_type(), 0);
        assert_eq!(frame.lane_keep_assist_button_state(), true);
        assert_eq!(frame.front_massage_adjustment(), 0);
        assert_eq!(frame.fragrance_diffuser_perfume_selection(), 0);
        assert_eq!(frame.fragrance_diffuser_intensity(), 0);
        assert_eq!(frame.fragrance_diffuser_request(), 0);
        assert_eq!(frame.massage_activation_request(), true);
        assert_eq!(frame.massage_intensity(), 0);
        assert_eq!(frame.massage_type(), 0);
        assert_eq!(frame.asr_inhibit(), true);
        assert_eq!(frame.consumption_history_zoomed_value(), 0);
        assert_eq!(frame.dynamic_mode_selected_mode(), 0);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.fragrance_diffuser_mono_type(), 3);
        assert_eq!(frame.lane_keep_assist_button_state(), false);
        assert_eq!(frame.front_massage_adjustment(), 3);
        assert_eq!(frame.fragrance_diffuser_perfume_selection(), 3);
        assert_eq!(frame.fragrance_diffuser_intensity(), 3);
        assert_eq!(frame.fragrance_diffuser_request(), 3);
        assert_eq!(frame.massage_activation_request(), false);
        assert_eq!(frame.massage_intensity(), 3);
        assert_eq!(frame.massage_type(), 3);
        assert_eq!(frame.asr_inhibit(), false);
        assert_eq!(frame.consumption_history_zoomed_value(), 3);
        assert_eq!(frame.dynamic_mode_selected_mode(), 3);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_fragrance_diffuser_mono_type(0);
        frame.set_lane_keep_assist_button_state(true);
        frame.set_front_massage_adjustment(0);
        frame.set_fragrance_diffuser_perfume_selection(0);
        frame.set_fragrance_diffuser_intensity(0);
        frame.set_fragrance_diffuser_request(0);
        frame.set_massage_activation_request(true);
        frame.set_massage_intensity(0);
        frame.set_massage_type(0);
        frame.set_asr_inhibit(true);
        frame.set_consumption_history_zoomed_value(0);
        frame.set_dynamic_mode_selected_mode(0);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_fragrance_diffuser_mono_type(3);
        frame.set_lane_keep_assist_button_state(false);
        frame.set_front_massage_adjustment(3);
        frame.set_fragrance_diffuser_perfume_selection(3);
        frame.set_fragrance_diffuser_intensity(3);
        frame.set_fragrance_diffuser_request(3);
        frame.set_massage_activation_request(false);
        frame.set_massage_intensity(3);
        frame.set_massage_type(3);
        frame.set_asr_inhibit(false);
        frame.set_consumption_history_zoomed_value(3);
        frame.set_dynamic_mode_selected_mode(3);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 8] = [0x10, 0x00, 0x02, 0x01, 0x00, 0x00, 0x00, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 6] = [0x10, 0x00, 0x02, 0x01, 0x00, 0x00];
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
