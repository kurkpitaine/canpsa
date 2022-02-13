pub mod x167;
pub use x167 as ID_DEMANDES_EMF;

pub mod x1e5;
pub use x1e5 as ID_ETAT_RADIO_GEN_AUD;

pub mod x260;
pub use x260 as ID_BSI_INF_PROFILS;

pub mod x276;
pub use x276 as ID_DONNEES_BSI_LENTES_3;

pub mod x361;
pub use x361 as ID_BSI_INF_CFG;

pub mod x39b;
pub use x39b as ID_DMD_MAJ_DATE_HEURE;

// When x15b is written on CAN bus, BSI should answer in BSI_INF_PROFILS.
// length = 5
// pub mod x15b;
// pub use x15b as MSG_ECRAN_INFO_PROFILS;

// length =
// pub mod x151;
// pub use x151 as BSI_INF_PROFILS;
