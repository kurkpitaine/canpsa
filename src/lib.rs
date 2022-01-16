use core::fmt;

mod aee2004;
mod aee2010;
mod config;

mod field {
    pub type Field = ::core::ops::Range<usize>;
    pub type Rest = ::core::ops::RangeFrom<usize>;
}

/// Year value offset. Year is transmitted on a single byte, from 0 to 255.
/// To get human year, add this constant to the CAN bus value.
pub const YEAR_OFFSET: i32 = 1872;

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
    /// An incoming frame was recognized but was malformed.
    /// E.g. a datetime frame with impossible values.
    Malformed,
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
            Error::Malformed => write!(f, "malformed frame"),
            Error::Dropped => write!(f, "dropped by socket"),
        }
    }
}
