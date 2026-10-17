//! Why a row could not become jobs: a row error naming its line, a
//! refused pin, or a system with no runner tags. The caller exits 2.

use std::fmt;

use crate::core::ConfigError;
use crate::core::kind as neutral;
use crate::pin::PinError;
use crate::system::SystemError;

/// A refused row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KindError {
    /// The row asks for something the kind does not take.
    Row(ConfigError),
    /// An image the kind uses has no pin (`src/pin:V2`).
    Pin(PinError),
    /// A system the row runs on has no runner tags (`src/system:V56`).
    System(SystemError),
}

impl KindError {
    /// A row error at `line`.
    pub(crate) fn row(
        line: usize,
        what: impl Into<String>,
        fix: impl Into<String>,
    ) -> Self {
        Self::Row(ConfigError::new(line, what, fix))
    }
}

impl From<PinError> for KindError {
    fn from(error: PinError) -> Self {
        Self::Pin(error)
    }
}

impl From<SystemError> for KindError {
    fn from(error: SystemError) -> Self {
        Self::System(error)
    }
}

/// pipelith's row refusal, in lablith's `.lablith:<line>` shape.
impl From<neutral::KindError> for KindError {
    fn from(error: neutral::KindError) -> Self {
        Self::Row(error.into())
    }
}

impl fmt::Display for KindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Row(e) => e.fmt(f),
            Self::Pin(e) => e.fmt(f),
            Self::System(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for KindError {}

#[cfg(test)]
mod tests;
