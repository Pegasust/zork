//! Boilerplate error definitions.
use std::fmt;

use crate::{lexer::Location, lexer::Range};

/// A type alias for std's Result with the Error as our error type.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
/// Error encountered during parser or so
pub enum Error {
    /// Originally given in ungrammar
    Simple {
        /// message to report
        message: String, 
        /// 0-indexed location of the error
        location: Option<Location>
    },
    /// LSP diagnostic ready variant
    Range {
        /// message to report
        message: String,
        /// 0-indexed range of the error, simulating an LSP diagnostic error
        range: Range
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Simple { message, location: Some(loc) } => {
                // Report 1-based indices, to match text editors
                write!(f, "{}; {}:{}: ", message, loc.line + 1, loc.column + 1)?;
            },
            Self::Simple {message, location: None} => {
                write!(f, "{}", message)?;
            },
            Error::Range { message, range } => {
                write!(
                    f, "{message}; {}:{} - {}:{}", 
                    range.begin.line + 1, range.begin.column+ 1,
                    range.ex_end.line + 1, range.ex_end.column + 1,
                )?;
            },
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

impl Error {
    pub(crate) fn with_location(self, location: Location) -> Error {
        match self {
            Self::Simple { message, location: _ } => Self::Simple {location: Some(location), message},
            _self@Error::Range {..} => _self,
        }
    }
}

macro_rules! _format_err {
    ($($tt:tt)*) => {
        $crate::error::Error::Simple {
            message: format!($($tt)*),
            location: None,
        }
    };
}
pub(crate) use _format_err as format_err;

macro_rules! _bail {
    ($($tt:tt)*) => { return Err($crate::error::format_err!($($tt)*)) };
}
pub(crate) use _bail as bail;
