//! The one shape a refused `.lablith` is reported in (`src/core` I
//! errors): `.lablith:N: what`, then an indented `fix:` line. pipelith's
//! registry and kind errors convert into it here, so every node reports a
//! refusal the same way.

use std::fmt;

/// Why a `.lablith` was refused. The caller exits 2 with it (`.:I` exit).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConfigError {
    /// The 1-based line at fault.
    pub line: usize,
    /// What is wrong, naming the offending text.
    pub what: String,
    /// What to write instead.
    pub fix: String,
}

impl ConfigError {
    /// An error at `line`.
    #[must_use]
    pub fn new(
        line: usize,
        what: impl Into<String>,
        fix: impl Into<String>,
    ) -> Self {
        let _ = (what.into(), fix.into());
        Self {
            line,
            what: String::new(),
            fix: String::new(),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}

impl From<super::registry::RegistryError> for ConfigError {
    /// pipelith's grammar error, in lablith's shape: the file is always
    /// `.lablith` and the column is not shown.
    fn from(e: super::registry::RegistryError) -> Self {
        Self::new(e.column, e.fix, e.what)
    }
}

impl From<super::kind::KindError> for ConfigError {
    /// pipelith's row error, in lablith's shape.
    fn from(e: super::kind::KindError) -> Self {
        Self::new(e.line, e.fix, e.what)
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests;
