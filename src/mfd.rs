use core::fmt;

enum_with_unknown! {
   /// Trip computer displayed page on multi-function display.
   pub enum TripComputerPage(u8) {
       /// Nothing is displayed.
       Nothing = 0,
       /// General parameters page is displayed.
       GeneralParameters = 1,
       /// First trip page is displayed.
       Trip1 = 2,
       /// Second trip page is displayed.
       Trip2 = 4,
       /// Page 4 is displayed.
       Page4 = 5,
       /// Trip computer is not managed by the multi-function display.
       NotManagedByMFD = 7,
   }
}

impl fmt::Display for TripComputerPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TripComputerPage::Nothing => write!(f, "nothing"),
            TripComputerPage::GeneralParameters => write!(f, "general parameters"),
            TripComputerPage::Trip1 => write!(f, "trip 1"),
            TripComputerPage::Trip2 => write!(f, "trip 2"),
            TripComputerPage::Page4 => write!(f, "page 4"),
            TripComputerPage::NotManagedByMFD => write!(f, "not managed by MFD"),
            TripComputerPage::Unknown(opt) => write!(f, "0x{:02x}", opt),
        }
    }
}

enum_with_unknown! {
   /// User action on AEE 2004 multi-function display.
   pub enum UserAction2004(u8) {
       /// No action from user.
       NoAction = 0,
       /// 'Yes' action from user.
       Yes = 1,
       /// 'No' action from user.
       No = 2,
       /// 'Esc' action from user.
       Esc = 4,
       /// Value has been set by the user.
       ValueReturn = 5,
       /// Timeout on waiting for user.
       Timeout = 15,
   }
}

impl fmt::Display for UserAction2004 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UserAction2004::NoAction => write!(f, "no action"),
            UserAction2004::Yes => write!(f, "yes"),
            UserAction2004::No => write!(f, "no"),
            UserAction2004::Esc => write!(f, "esc"),
            UserAction2004::ValueReturn => write!(f, "value return"),
            UserAction2004::Timeout => write!(f, "timeout"),
            UserAction2004::Unknown(opt) => write!(f, "0x{:02x}", opt),
        }
    }
}

enum_with_unknown! {
   /// User action on AEE 2010 multi-function display.
   pub enum UserAction2010(u8) {
       /// No action from user.
       NoAction = 0,
       /// 'Dismiss' action from user.
       Dismiss = 1,
       /// 'Yes' action from user.
       Yes = 2,
   }
}

impl fmt::Display for UserAction2010 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UserAction2010::NoAction => write!(f, "no action"),
            UserAction2010::Dismiss => write!(f, "dismiss"),
            UserAction2010::Yes => write!(f, "yes"),
            UserAction2010::Unknown(opt) => write!(f, "0x{:02x}", opt),
        }
    }
}

enum_with_unknown! {
   /// Popup to display on MFD.
   pub enum Popup(u8) {
       /// No popup display.
       NoDisplay = 0,
       /// Incoming advisor call popup.
       IncomingAdvisorCall = 1,
       /// Initiated advisor call popup.
       InitiatedAdvisorCall = 2,
       /// Initiated emergency call popup.
       InitiatedEmergencyCall = 3,
       /// Connected advisor call popup.
       ConnectedAdvisorCall = 4,
       /// Connected emergency call popup.
       ConnectedEmergencyCall = 5,
       /// Sending vehicle location popup.
       SendingVehicleLocation = 6,
       /// Onstar main menu popup.
       OnstarMainMenu = 7,
       /// Roaming active popup.
       RoamingActive = 8,
       /// Roaming ended popup.
       RoamingEnded = 9,
   }
}

impl fmt::Display for Popup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Popup::NoDisplay => write!(f, "no display"),
            Popup::IncomingAdvisorCall => write!(f, "incoming advisor call"),
            Popup::InitiatedAdvisorCall => write!(f, "initiated advisor call"),
            Popup::InitiatedEmergencyCall => write!(f, "initiated emergency call"),
            Popup::ConnectedAdvisorCall => write!(f, "connected advisor call"),
            Popup::ConnectedEmergencyCall => write!(f, "connected emergency call"),
            Popup::SendingVehicleLocation => write!(f, "sending vehicle location"),
            Popup::OnstarMainMenu => write!(f, "onstar main menu"),
            Popup::RoamingActive => write!(f, "roaming active"),
            Popup::RoamingEnded => write!(f, "roaming ended"),
            Popup::Unknown(popup) => write!(f, "0x{:02x}", popup),
        }
    }
}

enum_with_unknown! {
   /// Menu selected on MFD.
   pub enum Menu(u8) {
       /// Wifi settings menu.
       WifiSettings = 0,
       /// Data roaming menu.
       DataRoaming = 1,
       /// Privacy settings menu.
       PrivacySettings = 2,
       /// Cancel menu.
       Cancel = 3,
       /// No menu display.
       NoDisplay = 7,
   }
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Menu::WifiSettings => write!(f, "wifi settings"),
            Menu::DataRoaming => write!(f, "data roaming"),
            Menu::PrivacySettings => write!(f, "privacy settings"),
            Menu::Cancel => write!(f, "cancel"),
            Menu::NoDisplay => write!(f, "no display"),
            Menu::Unknown(menu) => write!(f, "0x{:02x}", menu),
        }
    }
}
