//! The typed rows of a parsed `.lablith` (`.:I` file). Shape only: kind,
//! system and arg names are checked by the nodes that own them
//! (`src/registry:V28`).

/// A parsed `.lablith`. A section the file leaves out is empty
/// (`src/registry:V29`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Registry {
    /// `## workflows` rows, in file order.
    pub(crate) workflows: Vec<WorkflowRow>,
    /// `## images` rows: pins overriding the built-in table.
    pub(crate) images: Vec<ImagePin>,
    /// `## runners` rows: tags overriding the built-in system table.
    pub(crate) runners: Vec<RunnerRow>,
    /// `## hand-written` rows: CI files lablith does not generate.
    pub(crate) hand_written: Vec<HandWritten>,
}

/// A `key=value` arg: pipelith's (`src/core`).
pub(crate) use crate::core::registry::Arg;

/// A `## workflows` row: pipelith's neutral `kind::RawRow` (`src/core`).
pub(crate) type WorkflowRow = crate::core::kind::RawRow;

/// An `## images` row: `image|version|digest`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ImagePin {
    /// The 1-based line the row is on.
    pub(crate) line: usize,
    /// The image name without tag or digest, e.g. `nixos/nix`.
    pub(crate) image: String,
    /// The tag the digest was resolved from.
    pub(crate) version: String,
    /// `sha256:` and 64 lowercase hex digits.
    pub(crate) digest: String,
}

/// A `## runners` row: `system|tags`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RunnerRow {
    /// The 1-based line the row is on.
    pub(crate) line: usize,
    /// The nix system, validated by `src/system`.
    pub(crate) system: String,
    /// The runner tags a job on that system carries; empty skips the
    /// system on this forge (`src/system:V72`).
    pub(crate) tags: Vec<String>,
}

/// A `## hand-written` row: `path|reason`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HandWritten {
    /// The 1-based line the row is on.
    pub(crate) line: usize,
    /// Repository-relative path of the CI file.
    pub(crate) path: String,
    /// Why it is not generated; never empty.
    pub(crate) reason: String,
}
