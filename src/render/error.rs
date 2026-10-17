//! Why `.lablith` could not be rendered: the caller exits 2 with it.

use std::fmt;

use crate::core::ConfigError;
use crate::kind::KindError;
use crate::system::SystemError;

/// A refused render.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RenderError {
    /// A row or section the registry or a kind refused.
    Row(ConfigError),
    /// A row its kind could not build (pin, tags, args).
    Kind(KindError),
    /// A `## runners` row naming no nix system.
    System(SystemError),
    /// Two files at one path.
    Duplicate(String),
}

impl From<ConfigError> for RenderError {
    fn from(e: ConfigError) -> Self {
        Self::Row(e)
    }
}

impl From<KindError> for RenderError {
    fn from(e: KindError) -> Self {
        Self::Kind(e)
    }
}

impl From<SystemError> for RenderError {
    fn from(e: SystemError) -> Self {
        Self::System(e)
    }
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Row(e) => e.fmt(f),
            Self::Kind(e) => e.fmt(f),
            Self::System(e) => e.fmt(f),
            Self::Duplicate(path) => write!(
                f,
                "two files at `{path}`\n  fix: give each row its own id, \
                 and list no generated path as hand-written"
            ),
        }
    }
}

impl std::error::Error for RenderError {}

#[cfg(test)]
mod tests;
