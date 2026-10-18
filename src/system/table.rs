//! The one runner table (`src/system:V56`): each nix system beside the
//! runner tags a job on it carries. No tag is spelled anywhere else. The
//! systems themselves, their names and the defaults are pipelith's
//! (`src/core`).

use std::collections::BTreeMap;
use std::fmt;

use crate::core::system as neutral;
use crate::registry::RunnerRow;
pub(crate) use neutral::System;

#[cfg(test)]
mod tests;

/// The built-in table: the nix system name plus `nix`, so no private
/// runner tag ships with lablith (`.:V20`). `x86_64-darwin` has no row:
/// pipelith refuses it in every row.
const BUILTIN: [(System, [&str; 2]); 3] = [
    (System::X86_64Linux, ["nix", "x86_64-linux"]),
    (System::Aarch64Linux, ["nix", "aarch64-linux"]),
    (System::Aarch64Darwin, ["nix", "aarch64-darwin"]),
];

/// Why a system cannot be given runner tags.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SystemError {
    /// A `## runners` row names no nix system.
    Unknown {
        /// The `.lablith` line of the row.
        line: usize,
        /// The name as written.
        name: String,
    },
    /// No row of the table covers the system.
    NoTags(System),
    /// `## runners` skips the system, but a job must run there (`V72`).
    Skipped(System),
}

/// Runner tags per system: built-in, then `## runners` overrides.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RunnerTable(BTreeMap<&'static str, Vec<String>>);

impl RunnerTable {
    /// The built-in table.
    pub(crate) fn builtin() -> Self {
        let row = |&(s, tags): &(System, [&str; 2])| {
            (s.name(), tags.iter().map(|&t| t.to_owned()).collect())
        };
        Self(BUILTIN.iter().map(row).collect())
    }

    /// This table with each `## runners` row replacing its system's tags.
    pub(crate) fn with_overrides(
        mut self,
        rows: &[RunnerRow],
    ) -> Result<Self, SystemError> {
        for row in rows {
            let system = neutral::parse_system(&row.system).map_err(|_| {
                SystemError::Unknown {
                    line: row.line,
                    name: row.system.clone(),
                }
            })?;
            self.0.insert(system.name(), row.tags.clone());
        }
        Ok(self)
    }

    /// Whether `## runners` skips `system` on this forge (`V72`).
    pub(crate) fn skips(&self, system: System) -> bool {
        self.0.get(system.name()).is_some_and(Vec::is_empty)
    }

    /// The tags a job on `system` carries, or an error naming the fix.
    pub(crate) fn tags(
        &self,
        system: System,
    ) -> Result<&[String], SystemError> {
        match self.0.get(system.name()) {
            None => Err(SystemError::NoTags(system)),
            Some(tags) if tags.is_empty() => Err(SystemError::Skipped(system)),
            Some(tags) => Ok(tags),
        }
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (what, fix) = match self {
            Self::Unknown { line, name } => (
                format!(".lablith:{line}: unknown system `{name}`"),
                format!("use one of {}", known()),
            ),
            Self::NoTags(system) => no_tags(system.name()),
            Self::Skipped(system) => skipped(system.name()),
        };
        write!(f, "{what}\n  fix: {fix}")
    }
}

/// A system no row of the table covers.
fn no_tags(s: &str) -> (String, String) {
    (
        format!("system `{s}` has no GitLab runner tags"),
        format!("add a `## runners` row `{s}|<tags>`"),
    )
}

/// A system `## runners` skips, asked for by a job (`V72`).
fn skipped(s: &str) -> (String, String) {
    (
        format!(
            "system `{s}` is skipped in `## runners`, but a job must run there"
        ),
        "give it runner tags, or move that row to a system this forge runs"
            .to_owned(),
    )
}

/// Every nix system name, comma-separated.
fn known() -> String {
    let names: Vec<&str> = neutral::ALL.iter().map(|s| s.name()).collect();
    names.join(", ")
}

impl std::error::Error for SystemError {}
