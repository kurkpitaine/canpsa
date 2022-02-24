use core::fmt;

#[macro_use]
mod macros;

pub mod aee2004;
pub mod aee2010;
pub mod config;
pub mod mfd;
pub mod vehicle;

mod field {
    pub type Field = ::core::ops::Range<usize>;
    pub type Rest = ::core::ops::RangeFrom<usize>;
}

/// Year value offset. Stellantis CAN time origin is January 1st 2000 0:00.
/// To get human year, add this constant to the CAN bus value.
pub const YEAR_OFFSET: i32 = 2000;

/// The error type for the networking stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// An operation cannot proceed because a buffer is empty or full.
    Exhausted,
    /// An operation is not permitted in the current state.
    Illegal,
    /// An incoming frame could not be parsed because some of its fields were out of bounds
    /// of the received data.
    Truncated,
    /// An incoming frame could not be parsed because its size is too long.
    Overlong,
    /// An incoming frame was recognized but contains invalid values.
    /// E.g. a datetime frame with impossible values.
    Invalid,
    /// An incoming frame was recognized but contradicted internal state.
    Dropped,
}

/// The result type for the networking stack.
pub type Result<T> = core::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Exhausted => write!(f, "buffer space exhausted"),
            Error::Illegal => write!(f, "illegal operation"),
            Error::Truncated => write!(f, "truncated frame"),
            Error::Overlong => write!(f, "overlong frame"),
            Error::Invalid => write!(f, "invalid frame"),
            Error::Dropped => write!(f, "dropped by socket"),
        }
    }
}
