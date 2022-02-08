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
   /// User action on multi-function display.
   pub enum UserAction(u8) {
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

impl fmt::Display for UserAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UserAction::NoAction => write!(f, "no action"),
            UserAction::Yes => write!(f, "yes"),
            UserAction::No => write!(f, "no"),
            UserAction::Esc => write!(f, "esc"),
            UserAction::ValueReturn => write!(f, "value return"),
            UserAction::Timeout => write!(f, "timeout"),
            UserAction::Unknown(opt) => write!(f, "0x{:02x}", opt),
        }
    }
}
