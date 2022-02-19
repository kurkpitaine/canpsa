use core::fmt;

use byteorder::{ByteOrder, NetworkEndian};

use crate::{
    vehicle::{ElectricalNetworkState, VehicleConfigMode},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
236 DONNEES_VSM_LENTES_2_COMPTEUR_RAZ_GCT_HS4_236      // OK
236 DONNEES_VSM_LENTES_2_CONF_STT_DRIVER_INHIB_HS4_236 // NOPE
236 DONNEES_VSM_LENTES_2_CONTACT_FREIN1_HS4_236        // NOPE
236 DONNEES_VSM_LENTES_2_CPT_TEMPOREL_HS4_236          // OK
236 DONNEES_VSM_LENTES_2_CTX_JDD_HS4_236               // OK
236 DONNEES_VSM_LENTES_2_ETAT_COFFRE_HS4_236           // OK
236 DONNEES_VSM_LENTES_2_ETAT_CPO_ARD_HS4_236          // NOPE
236 DONNEES_VSM_LENTES_2_ETAT_CPO_ARG_HS4_236          // NOPE
236 DONNEES_VSM_LENTES_2_ETAT_CPO_COND_HS4_236         // OK
236 DONNEES_VSM_LENTES_2_ETAT_CPO_COND_REPLI_HS4_236   // NOPE
236 DONNEES_VSM_LENTES_2_ETAT_CPO_LUNETTE_HS4_236      // NOPE
236 DONNEES_VSM_LENTES_2_ETAT_CPO_PASS_HS4_236         // NOPE
236 DONNEES_VSM_LENTES_2_ETAT_RESEAU_ELEC_HS4_236      // OK
236 DONNEES_VSM_LENTES_2_MODE_CONFIG_VHL_HS4_236       // OK
236 DONNEES_VSM_LENTES_2_ON_REFUSE_HS4_236             // OK
236 DONNEES_VSM_LENTES_2_PRESENCE_CRT_HS4_236          // NOPE
*/

mod field {
    use crate::field::*;
    /// 4-bit vehicle config mode,
    /// 4-bit electrical network status.
    pub const CONFIG_MODE_ELEC_NET: usize = 0;
    /// 32-bit vehicle supervision module temporal counter, 0xFFFFFFFE if unavailable.
    pub const TEMPORAL_COUNTER: Field = 1..5;
    /// 5-bit 'JDD' context field,
    /// 1-bit unknown,
    /// 1-bit driver door opened event flag,
    /// 1-bit opened boot flag.
    pub const CTX_FLAGS: usize = 5;
    /// 8-bit 'GCT' reset counter. 0xfe if unavailable.
    pub const RESET_COUNTER: usize = 6;
    /// 7-bit unknown,
    /// 1-bit Power-On request denied flag, when car is in economy mode.
    pub const UNKNOWN_FLAGS: usize = 7;
}

/// Length of a x236 CAN frame.
pub const FRAME_LEN: usize = field::UNKNOWN_FLAGS + 1;

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

    /// Return the vehicle configuration mode field.
    #[inline]
    pub fn vehicle_config_mode(&self) -> VehicleConfigMode {
        let data = self.buffer.as_ref();
        let raw = data[field::CONFIG_MODE_ELEC_NET] & 0x0f;
        VehicleConfigMode::from(raw)
    }

    /// Return the electrical network status field.
    #[inline]
    pub fn electrical_network_status(&self) -> ElectricalNetworkState {
        let data = self.buffer.as_ref();
        let raw = (data[field::CONFIG_MODE_ELEC_NET] & 0xf0) >> 4;
        ElectricalNetworkState::from(raw)
    }

    /// Return the vehicle supervision module temporal counter field.
    #[inline]
    pub fn vsm_temporal_counter(&self) -> u32 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u32(&data[field::TEMPORAL_COUNTER])
    }

    /// Return the 'JDD' context field.
    #[inline]
    pub fn jdd_context(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::CTX_FLAGS] & 0x1f
    }

    /// Return the driver door opened event flag.
    #[inline]
    pub fn driver_door_open_evt(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::CTX_FLAGS] & 0x40 != 0
    }

    /// Return the opened boot flag.
    #[inline]
    pub fn boot_open(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::CTX_FLAGS] & 0x80 != 0
    }

    /// Return the 'GCT' reset counter field.
    #[inline]
    pub fn gct_reset_counter(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::RESET_COUNTER]
    }

    /// Return the Power-On request denied flag.
    #[inline]
    pub fn power_on_req_denied(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::UNKNOWN_FLAGS] & 0x80 != 0
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the vehicle configuration mode field.
    #[inline]
    pub fn set_vehicle_config_mode(&mut self, value: VehicleConfigMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::CONFIG_MODE_ELEC_NET] & !0x0f;
        let raw = raw | (u8::from(value) & 0x0f);
        data[field::CONFIG_MODE_ELEC_NET] = raw;
    }

    /// Set the electrical network status field.
    #[inline]
    pub fn set_electrical_network_status(&mut self, value: ElectricalNetworkState) {
        let data = self.buffer.as_mut();
        let raw = data[field::CONFIG_MODE_ELEC_NET] & !0xf0;
        let raw = raw | ((u8::from(value) << 4) & 0xf0);
        data[field::CONFIG_MODE_ELEC_NET] = raw;
    }

    /// Set the vehicle supervision module temporal counter field.
    #[inline]
    pub fn set_vsm_temporal_counter(&mut self, value: u32) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u32(&mut data[field::TEMPORAL_COUNTER], value);
    }

    /// Set the 'JDD' context field.
    #[inline]
    pub fn set_jdd_context(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::CTX_FLAGS] & !0x1f;
        let raw = raw | (value & 0x1f);
        data[field::CTX_FLAGS] = raw;
    }

    /// Set the driver door opened event flag.
    #[inline]
    pub fn set_driver_door_open_evt(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::CTX_FLAGS] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::CTX_FLAGS] = raw;
    }

    /// Set the opened boot flag.
    #[inline]
    pub fn set_boot_open(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::CTX_FLAGS] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::CTX_FLAGS] = raw;
    }

    /// Set the 'GCT' reset counter field.
    #[inline]
    pub fn set_gct_reset_counter(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::RESET_COUNTER] = value;
    }

    /// Set the Power-On request denied flag.
    #[inline]
    pub fn set_power_on_req_denied(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::UNKNOWN_FLAGS] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::UNKNOWN_FLAGS] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x236 ({})", err)?;
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

/// A high-level representation of a x236 CAN frame.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    vehicle_config_mode: VehicleConfigMode,
    electrical_network_status: ElectricalNetworkState,
    vsm_temporal_counter: u32,
    jdd_context: u8,
    driver_door_open_evt: bool,
    boot_open: bool,
    gct_reset_counter: u8,
    power_on_req_denied: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            vehicle_config_mode: frame.vehicle_config_mode(),
            electrical_network_status: frame.electrical_network_status(),
            vsm_temporal_counter: frame.vsm_temporal_counter(),
            jdd_context: frame.jdd_context(),
            driver_door_open_evt: frame.driver_door_open_evt(),
            boot_open: frame.boot_open(),
            gct_reset_counter: frame.gct_reset_counter(),
            power_on_req_denied: frame.power_on_req_denied(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x236 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_vehicle_config_mode(self.vehicle_config_mode);
        frame.set_electrical_network_status(self.electrical_network_status);
        frame.set_vsm_temporal_counter(self.vsm_temporal_counter);
        frame.set_jdd_context(self.jdd_context);
        frame.set_driver_door_open_evt(self.driver_door_open_evt);
        frame.set_boot_open(self.boot_open);
        frame.set_gct_reset_counter(self.gct_reset_counter);
        frame.set_power_on_req_denied(self.power_on_req_denied);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x236 vehicle_config_mode={}", self.vehicle_config_mode)?;
        write!(
            f,
            " electrical_network_status={}",
            self.electrical_network_status
        )?;
        write!(f, " vsm_temporal_counter={}", self.vsm_temporal_counter)?;
        write!(f, " jdd_context={}", self.jdd_context)?;
        write!(f, " driver_door_open_evt={}", self.driver_door_open_evt)?;
        write!(f, " boot_open={}", self.boot_open)?;
        write!(f, " gct_reset_counter={}", self.gct_reset_counter)?;
        write!(f, " power_on_req_denied={}", self.power_on_req_denied)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        vehicle::{ElectricalNetworkState, VehicleConfigMode},
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x54, 0x00, 0x01, 0xe2, 0x40, 0x80, 0xfe, 0x00];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0x06, 0x00, 0x74, 0xcb, 0xb1, 0x58, 0xfe, 0x80];

    fn frame_1_repr() -> Repr {
        Repr {
            vehicle_config_mode: VehicleConfigMode::Customer,
            electrical_network_status: ElectricalNetworkState::GeneratorNormal,
            vsm_temporal_counter: 123456,
            jdd_context: 0,
            driver_door_open_evt: false,
            boot_open: true,
            gct_reset_counter: 0xfe,
            power_on_req_denied: false,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            vehicle_config_mode: VehicleConfigMode::Workshop,
            electrical_network_status: ElectricalNetworkState::BatteryNormal,
            vsm_temporal_counter: 7654321,
            jdd_context: 24,
            driver_door_open_evt: true,
            boot_open: false,
            gct_reset_counter: 0xfe,
            power_on_req_denied: true,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.vehicle_config_mode(), VehicleConfigMode::Customer);
        assert_eq!(
            frame.electrical_network_status(),
            ElectricalNetworkState::GeneratorNormal
        );
        assert_eq!(frame.vsm_temporal_counter(), 123456);
        assert_eq!(frame.jdd_context(), 0);
        assert_eq!(frame.driver_door_open_evt(), false);
        assert_eq!(frame.boot_open(), true);
        assert_eq!(frame.gct_reset_counter(), 0xfe);
        assert_eq!(frame.power_on_req_denied(), false);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.vehicle_config_mode(), VehicleConfigMode::Workshop);
        assert_eq!(
            frame.electrical_network_status(),
            ElectricalNetworkState::BatteryNormal
        );
        assert_eq!(frame.vsm_temporal_counter(), 7654321);
        assert_eq!(frame.jdd_context(), 24);
        assert_eq!(frame.driver_door_open_evt(), true);
        assert_eq!(frame.boot_open(), false);
        assert_eq!(frame.gct_reset_counter(), 0xfe);
        assert_eq!(frame.power_on_req_denied(), true);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_vehicle_config_mode(VehicleConfigMode::Customer);
        frame.set_electrical_network_status(ElectricalNetworkState::GeneratorNormal);
        frame.set_vsm_temporal_counter(123456);
        frame.set_jdd_context(0);
        frame.set_driver_door_open_evt(false);
        frame.set_boot_open(true);
        frame.set_gct_reset_counter(0xfe);
        frame.set_power_on_req_denied(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_vehicle_config_mode(VehicleConfigMode::Workshop);
        frame.set_electrical_network_status(ElectricalNetworkState::BatteryNormal);
        frame.set_vsm_temporal_counter(7654321);
        frame.set_jdd_context(24);
        frame.set_driver_door_open_evt(true);
        frame.set_boot_open(false);
        frame.set_gct_reset_counter(0xfe);
        frame.set_power_on_req_denied(true);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x54, 0x00, 0x01, 0xe2, 0x40, 0x80, 0xfe, 0x00, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x54, 0x00, 0x01, 0xe2, 0x40, 0x80, 0xfe];
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
