use core::{cmp::Ordering, fmt};

use byteorder::{ByteOrder, NetworkEndian};

use crate::{
    mfd::{Menu, Popup, TripComputerPage, UserAction2010},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
167 DEMANDES_EMF_APP_URG_HS7_167                    // OK
167 DEMANDES_EMF_CHECK_DDES_EMF_HS7_167             // OK
167 DEMANDES_EMF_DDE_PRECOND_CLIM_HS7_167           // OK
167 DEMANDES_EMF_DELAI_HS7_167                      // OK
167 DEMANDES_EMF_DMD_RAZ_HS7_167                    // OK
167 DEMANDES_EMF_ETAT_BP_DARK_HS7_167               // OK
167 DEMANDES_EMF_HMI_POP_UP_ID_ACK_HS7_167          // OK
167 DEMANDES_EMF_HMI_USER_ACTION_HS7_167            // OK
167 DEMANDES_EMF_HMI_WIFI_SETTING_UPD_ACK_HS7_167   // OK
167 DEMANDES_EMF_KM_TOT_PARC_HS7_167                // OK
167 DEMANDES_EMF_MAIN_MENU_SELECTION_HS7_167        // OK
167 DEMANDES_EMF_POINT_MESS_INTERACTIF_HS7_167      // OK
167 DEMANDES_EMF_RAZ_CUMT1_DDES_EMF_HS7_167         // OK
167 DEMANDES_EMF_RAZ_CUMT2_DDES_EMF_HS7_167         // OK
167 DEMANDES_EMF_S_FCT_TELE_HS7_167                 // OK
167 DEMANDES_EMF_STATE_HMI_SALC_AUTH_HS7_167
167 DEMANDES_EMF_STATE_HMI_SALC_CONFIRM_HS7_167
167 DEMANDES_EMF_STOP_CHECK_EMF_HS7_167             // OK
*/

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
    /// 1-bit MFD stop check request.
    pub const INTERACTIVE_MSG_STOP_CHK: Field = 4..6;
    /// 8-bit popup id to display acknowledge.
    pub const POPUP_ID: usize = 6;
    /// 2-bit empty,
    /// 3-bit user selected menu,
    /// 1-bit wifi parameters reception acknowledge.
    /// 2-bit user action on MFD.
    pub const MENU_ACTION: usize = 7;
}

/// Raw x167 CAN frame identifier.
pub const FRAME_ID: u16 = 0x167;
/// Length of a x167 CAN frame.
pub const FRAME_LEN: usize = field::MENU_ACTION + 1;

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

    /// Return the multi-function display trip computer displayed page field.
    #[inline]
    pub fn mfd_trip_computer_page(&self) -> TripComputerPage {
        let data = self.buffer.as_ref();
        let raw = data[field::REQ_0] & 0x07;
        TripComputerPage::from(raw)
    }

    /// Return the maintenance reset request flag.
    /// logic is inverted here, 0 means requested...
    #[inline]
    pub fn maintenance_reset_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x08 == 0
    }

    /// Return the emergency call in progress flag.
    #[inline]
    pub fn emergency_call_in_progress(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x10 != 0
    }

    /// Return the fault check recall request flag.
    #[inline]
    pub fn fault_recall_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x20 != 0
    }

    /// Return the trip computer secondary trip reset request flag.
    #[inline]
    pub fn trip_computer_secondary_trip_reset_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x40 != 0
    }

    /// Return the trip computer primary trip reset request flag.
    #[inline]
    pub fn trip_computer_primary_trip_reset_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_0] & 0x80 != 0
    }

    /// Return the pre-conditioning time field (units: minutes).
    #[inline]
    pub fn pre_conditioning_time(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x08
    }

    /// Return the telematics enabled flag.
    #[inline]
    pub fn telematics_enabled(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x10 != 0
    }

    /// Return the black panel function state flag.
    #[inline]
    pub fn black_panel_enabled(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x20 != 0
    }

    /// Return the indirect under-inflation detection reset request flag.
    #[inline]
    pub fn indirect_under_inflation_reset_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x40 != 0
    }

    /// Return the thermal pre-conditioning request flag.
    #[inline]
    pub fn pre_conditioning_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::REQ_1] & 0x80 != 0
    }

    /// Return the total trip distance field.
    #[inline]
    pub fn total_trip_distance(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::TOTAL_TRIP_DISTANCE])
    }

    /// Return the interactive message field.
    #[inline]
    pub fn interactive_message(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::INTERACTIVE_MSG_STOP_CHK]) & 0x7fff
    }

    /// Return the MFD stop check request field.
    #[inline]
    pub fn stop_check_request(&self) -> bool {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::INTERACTIVE_MSG_STOP_CHK]);
        raw & !0x7fff != 0
    }

    /// Return the popup id to display acknowledge field.
    #[inline]
    pub fn popup_id_ack(&self) -> Popup {
        let data = self.buffer.as_ref();
        Popup::from(data[field::POPUP_ID])
    }

    /// Return the user selected menu field.
    #[inline]
    pub fn selected_menu(&self) -> Menu {
        let data = self.buffer.as_ref();
        let raw = (data[field::MENU_ACTION] & 0x1c) >> 2;
        Menu::from(raw)
    }

    /// Return the wifi parameters reception acknowledge flag.
    #[inline]
    pub fn wifi_parameters_ack(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::MENU_ACTION] & 0x20 != 0
    }

    /// Return the user action on MFD field.
    #[inline]
    pub fn user_action_on_mfd(&self) -> UserAction2010 {
        let data = self.buffer.as_ref();
        let raw = data[field::MENU_ACTION] >> 6;
        UserAction2010::from(raw)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the multi-function display trip computer displayed page field.
    #[inline]
    pub fn set_mfd_trip_computer_page(&mut self, value: TripComputerPage) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0] & !0x07;
        let raw = raw | (u8::from(value) & 0x07);
        data[field::REQ_0] = raw;
    }

    /// Set the maintenance reset request flag.
    /// logic is inverted here, 0 means requested...
    #[inline]
    pub fn set_maintenance_reset_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0];
        let raw = if !value { raw | 0x08 } else { raw & !0x08 };
        data[field::REQ_0] = raw;
    }

    /// Set the emergency call in progress flag.
    #[inline]
    pub fn set_emergency_call_in_progress(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0];
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::REQ_0] = raw;
    }

    /// Set the fault check recall request flag.
    #[inline]
    pub fn set_fault_check_recall_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0];
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::REQ_0] = raw;
    }

    /// Set the trip computer secondary trip reset request flag.
    #[inline]
    pub fn set_trip_computer_secondary_trip_reset_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0];
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::REQ_0] = raw;
    }

    /// Set the trip computer primary trip reset request flag.
    #[inline]
    pub fn set_trip_computer_primary_trip_reset_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_0];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::REQ_0] = raw;
    }

    /// Set the pre-conditioning time field (units: minutes).
    #[inline]
    pub fn set_pre_conditioning_time(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1] & !0x08;
        let raw = raw | (value & 0x08);
        data[field::REQ_1] = raw;
    }

    /// Set the telematics enabled flag.
    #[inline]
    pub fn set_telematics_enabled(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1];
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::REQ_1] = raw;
    }

    /// Set the black panel function state flag.
    #[inline]
    pub fn set_black_panel_enabled(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1];
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::REQ_1] = raw;
    }

    /// Set the indirect under-inflation detection reset request flag.
    #[inline]
    pub fn set_indirect_under_inflation_reset_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1];
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::REQ_1] = raw;
    }

    /// Set the thermal pre-conditioning request flag.
    #[inline]
    pub fn set_pre_conditioning_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::REQ_1];
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::REQ_1] = raw;
    }

    /// Set the total trip distance field.
    #[inline]
    pub fn set_total_trip_distance(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::TOTAL_TRIP_DISTANCE], value);
    }

    /// Set the interactive message field.
    #[inline]
    pub fn set_interactive_message(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::INTERACTIVE_MSG_STOP_CHK]);
        let raw = raw | (value & 0x7fff);
        NetworkEndian::write_u16(&mut data[field::INTERACTIVE_MSG_STOP_CHK], raw);
    }

    /// Set the MFD stop check request field.
    #[inline]
    pub fn set_stop_check_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::INTERACTIVE_MSG_STOP_CHK]);
        let raw = if value { raw | 0x8000 } else { raw & !0x8000 };
        NetworkEndian::write_u16(&mut data[field::INTERACTIVE_MSG_STOP_CHK], raw);
    }

    /// Set the popup id to display acknowledge field.
    #[inline]
    pub fn set_popup_id_ack(&mut self, value: Popup) {
        let data = self.buffer.as_mut();
        data[field::POPUP_ID] = u8::from(value);
    }

    /// Set the user selected menu field.
    #[inline]
    pub fn set_selected_menu(&mut self, value: Menu) {
        let data = self.buffer.as_mut();
        let raw = data[field::MENU_ACTION] & !0x1c;
        let raw = raw | ((u8::from(value) << 2) & 0x1c);
        data[field::MENU_ACTION] = raw;
    }

    /// Set the wifi parameters reception acknowledge flag.
    #[inline]
    pub fn set_wifi_parameters_ack(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::MENU_ACTION];
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::MENU_ACTION] = raw;
    }

    /// Set the user action on MFD field.
    #[inline]
    pub fn set_user_action_on_mfd(&mut self, value: UserAction2010) {
        let data = self.buffer.as_mut();
        let raw = data[field::MENU_ACTION];
        let raw = raw | (u8::from(value) << 6);
        data[field::MENU_ACTION] = raw;
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
    pub mfd_trip_computer_page: TripComputerPage,
    pub maintenance_reset_request: bool,
    pub emergency_call_in_progress: bool,
    pub fault_recall_request: bool,
    pub trip_computer_secondary_trip_reset_request: bool,
    pub trip_computer_primary_trip_reset_request: bool,
    pub pre_conditioning_time: u8,
    pub telematics_enabled: bool,
    pub black_panel_enabled: bool,
    pub indirect_under_inflation_reset_request: bool,
    pub pre_conditioning_request: bool,
    pub total_trip_distance: u16,
    pub interactive_message: u16,
    pub stop_check_request: bool,
    pub popup_id_acknowledge: Popup,
    pub selected_menu: Menu,
    pub wifi_parameters_acknowledge: bool,
    pub user_action_on_mfd: UserAction2010,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            mfd_trip_computer_page: frame.mfd_trip_computer_page(),
            maintenance_reset_request: frame.maintenance_reset_request(),
            emergency_call_in_progress: frame.emergency_call_in_progress(),
            fault_recall_request: frame.fault_recall_request(),
            trip_computer_secondary_trip_reset_request: frame
                .trip_computer_secondary_trip_reset_request(),
            trip_computer_primary_trip_reset_request: frame
                .trip_computer_primary_trip_reset_request(),
            pre_conditioning_time: frame.pre_conditioning_time() / 5,
            telematics_enabled: frame.telematics_enabled(),
            black_panel_enabled: frame.black_panel_enabled(),
            indirect_under_inflation_reset_request: frame.indirect_under_inflation_reset_request(),
            pre_conditioning_request: frame.pre_conditioning_request(),
            total_trip_distance: frame.total_trip_distance() * 2,
            interactive_message: frame.interactive_message(),
            stop_check_request: frame.stop_check_request(),
            popup_id_acknowledge: frame.popup_id_ack(),
            selected_menu: frame.selected_menu(),
            wifi_parameters_acknowledge: frame.wifi_parameters_ack(),
            user_action_on_mfd: frame.user_action_on_mfd(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x167 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_mfd_trip_computer_page(self.mfd_trip_computer_page);
        frame.set_maintenance_reset_request(self.maintenance_reset_request);
        frame.set_emergency_call_in_progress(self.emergency_call_in_progress);
        frame.set_fault_check_recall_request(self.fault_recall_request);
        frame.set_trip_computer_secondary_trip_reset_request(
            self.trip_computer_secondary_trip_reset_request,
        );
        frame.set_trip_computer_primary_trip_reset_request(
            self.trip_computer_primary_trip_reset_request,
        );
        frame.set_pre_conditioning_time(self.pre_conditioning_time * 5);
        frame.set_telematics_enabled(self.telematics_enabled);
        frame.set_black_panel_enabled(self.black_panel_enabled);
        frame.set_indirect_under_inflation_reset_request(
            self.indirect_under_inflation_reset_request,
        );
        frame.set_pre_conditioning_request(self.pre_conditioning_request);
        frame.set_total_trip_distance(self.total_trip_distance / 2);
        frame.set_interactive_message(self.interactive_message);
        frame.set_stop_check_request(self.stop_check_request);
        frame.set_popup_id_ack(self.popup_id_acknowledge);
        frame.set_selected_menu(self.selected_menu);
        frame.set_wifi_parameters_ack(self.wifi_parameters_acknowledge);
        frame.set_user_action_on_mfd(self.user_action_on_mfd);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "x167 mfd_trip_computer_page={}",
            self.mfd_trip_computer_page
        )?;
        writeln!(
            f,
            " maintenance_reset_request={}",
            self.maintenance_reset_request
        )?;
        writeln!(
            f,
            " emergency_call_in_progress={}",
            self.emergency_call_in_progress
        )?;
        writeln!(f, " fault_recall_request={}", self.fault_recall_request)?;
        writeln!(
            f,
            " trip_computer_secondary_trip_reset_request={}",
            self.trip_computer_secondary_trip_reset_request
        )?;
        writeln!(
            f,
            " trip_computer_primary_trip_reset_request={}",
            self.trip_computer_primary_trip_reset_request
        )?;
        writeln!(f, " preconditioning_time={}", self.pre_conditioning_time)?;
        writeln!(f, " telematics_enabled={}", self.telematics_enabled)?;
        writeln!(f, " black_panel_enabled={}", self.black_panel_enabled)?;
        writeln!(
            f,
            " indirect_under_inflation_reset_request={}",
            self.indirect_under_inflation_reset_request
        )?;
        writeln!(
            f,
            " pre_conditioning_request={}",
            self.pre_conditioning_request
        )?;
        writeln!(f, " total_trip_distance={}", self.total_trip_distance)?;
        writeln!(f, " interactive_message={}", self.interactive_message)?;
        writeln!(f, " stopcheck_request={}", self.stop_check_request)?;
        writeln!(f, " popup_id_acknowledge={}", self.popup_id_acknowledge)?;
        writeln!(f, " selected_menu={}", self.selected_menu)?;
        writeln!(
            f,
            " wifi_parameters_acknowledge={}",
            self.wifi_parameters_acknowledge
        )?;
        writeln!(f, " user_action_on_mfd={}", self.user_action_on_mfd)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        mfd::{Menu, Popup, TripComputerPage, UserAction2010},
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x08, 0x00, 0x00, 0x00, 0x7f, 0xff, 0x00, 0x00];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0x08, 0x10, 0x00, 0x00, 0xff, 0xff, 0x05, 0xa8];

    fn frame_1_repr() -> Repr {
        Repr {
            mfd_trip_computer_page: TripComputerPage::Nothing,
            maintenance_reset_request: false,
            emergency_call_in_progress: false,
            fault_recall_request: false,
            trip_computer_secondary_trip_reset_request: false,
            trip_computer_primary_trip_reset_request: false,
            pre_conditioning_time: 0,
            telematics_enabled: false,
            black_panel_enabled: false,
            indirect_under_inflation_reset_request: false,
            pre_conditioning_request: false,
            total_trip_distance: 0,
            interactive_message: 32767,
            stop_check_request: false,
            popup_id_acknowledge: Popup::NoDisplay,
            selected_menu: Menu::WifiSettings,
            wifi_parameters_acknowledge: false,
            user_action_on_mfd: UserAction2010::NoAction,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            mfd_trip_computer_page: TripComputerPage::Nothing,
            maintenance_reset_request: false,
            emergency_call_in_progress: false,
            fault_recall_request: false,
            trip_computer_secondary_trip_reset_request: false,
            trip_computer_primary_trip_reset_request: false,
            pre_conditioning_time: 0,
            telematics_enabled: true,
            black_panel_enabled: false,
            indirect_under_inflation_reset_request: false,
            pre_conditioning_request: false,
            total_trip_distance: 0,
            interactive_message: 32767,
            stop_check_request: true,
            popup_id_acknowledge: Popup::ConnectedEmergencyCall,
            selected_menu: Menu::PrivacySettings,
            wifi_parameters_acknowledge: true,
            user_action_on_mfd: UserAction2010::Yes,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.mfd_trip_computer_page(), TripComputerPage::Nothing);
        assert_eq!(frame.maintenance_reset_request(), false);
        assert_eq!(frame.emergency_call_in_progress(), false);
        assert_eq!(frame.fault_recall_request(), false);
        assert_eq!(frame.trip_computer_secondary_trip_reset_request(), false);
        assert_eq!(frame.trip_computer_primary_trip_reset_request(), false);
        assert_eq!(frame.pre_conditioning_time(), 0);
        assert_eq!(frame.telematics_enabled(), false);
        assert_eq!(frame.black_panel_enabled(), false);
        assert_eq!(frame.indirect_under_inflation_reset_request(), false);
        assert_eq!(frame.pre_conditioning_request(), false);
        assert_eq!(frame.total_trip_distance(), 0);
        assert_eq!(frame.interactive_message(), 32767);
        assert_eq!(frame.stop_check_request(), false);
        assert_eq!(frame.popup_id_ack(), Popup::NoDisplay);
        assert_eq!(frame.selected_menu(), Menu::WifiSettings);
        assert_eq!(frame.wifi_parameters_ack(), false);
        assert_eq!(frame.user_action_on_mfd(), UserAction2010::NoAction);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.mfd_trip_computer_page(), TripComputerPage::Nothing);
        assert_eq!(frame.maintenance_reset_request(), false);
        assert_eq!(frame.emergency_call_in_progress(), false);
        assert_eq!(frame.fault_recall_request(), false);
        assert_eq!(frame.trip_computer_secondary_trip_reset_request(), false);
        assert_eq!(frame.trip_computer_primary_trip_reset_request(), false);
        assert_eq!(frame.pre_conditioning_time(), 0);
        assert_eq!(frame.telematics_enabled(), true);
        assert_eq!(frame.black_panel_enabled(), false);
        assert_eq!(frame.indirect_under_inflation_reset_request(), false);
        assert_eq!(frame.pre_conditioning_request(), false);
        assert_eq!(frame.total_trip_distance(), 0);
        assert_eq!(frame.interactive_message(), 32767);
        assert_eq!(frame.stop_check_request(), true);
        assert_eq!(frame.popup_id_ack(), Popup::ConnectedEmergencyCall);
        assert_eq!(frame.selected_menu(), Menu::PrivacySettings);
        assert_eq!(frame.wifi_parameters_ack(), true);
        assert_eq!(frame.user_action_on_mfd(), UserAction2010::Yes);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_mfd_trip_computer_page(TripComputerPage::Nothing);
        frame.set_maintenance_reset_request(false);
        frame.set_emergency_call_in_progress(false);
        frame.set_fault_check_recall_request(false);
        frame.set_trip_computer_secondary_trip_reset_request(false);
        frame.set_trip_computer_primary_trip_reset_request(false);
        frame.set_pre_conditioning_time(0);
        frame.set_telematics_enabled(false);
        frame.set_black_panel_enabled(false);
        frame.set_indirect_under_inflation_reset_request(false);
        frame.set_pre_conditioning_request(false);
        frame.set_total_trip_distance(0);
        frame.set_interactive_message(32767);
        frame.set_stop_check_request(false);
        frame.set_popup_id_ack(Popup::NoDisplay);
        frame.set_selected_menu(Menu::WifiSettings);
        frame.set_wifi_parameters_ack(false);
        frame.set_user_action_on_mfd(UserAction2010::NoAction);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_mfd_trip_computer_page(TripComputerPage::Nothing);
        frame.set_maintenance_reset_request(false);
        frame.set_emergency_call_in_progress(false);
        frame.set_fault_check_recall_request(false);
        frame.set_trip_computer_secondary_trip_reset_request(false);
        frame.set_trip_computer_primary_trip_reset_request(false);
        frame.set_pre_conditioning_time(0);
        frame.set_telematics_enabled(true);
        frame.set_black_panel_enabled(false);
        frame.set_indirect_under_inflation_reset_request(false);
        frame.set_pre_conditioning_request(false);
        frame.set_total_trip_distance(0);
        frame.set_interactive_message(32767);
        frame.set_stop_check_request(true);
        frame.set_popup_id_ack(Popup::ConnectedEmergencyCall);
        frame.set_selected_menu(Menu::PrivacySettings);
        frame.set_wifi_parameters_ack(true);
        frame.set_user_action_on_mfd(UserAction2010::Yes);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x08, 0x00, 0x00, 0x00, 0x7f, 0xff, 0x00, 0x00, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x08, 0x00, 0x00, 0x00, 0x7f, 0xff, 0x00];
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
