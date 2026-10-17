//! Why `lablith pin` stopped (`src/fetch:V59`): exit 1 when an image
//! could not be resolved, 2 when `.lablith` could not be read, parsed or
//! written. `.lablith` is written only after every image resolved.

use std::fmt;
use std::path::{Path, PathBuf};

use crate::core::ConfigError;

/// A refused or failed `pin`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FetchError {
    /// `.lablith` could not be read.
    Read { path: PathBuf, why: String },
    /// `.lablith` could not be written.
    Write { path: PathBuf, why: String },
    /// `.lablith` does not parse.
    Config(ConfigError),
    /// An image name or tag unsafe to put in a registry URL.
    Name { image: String, tag: String },
    /// The registry did not give an index digest for the tag.
    Resolve {
        image: String,
        tag: String,
        why: String,
    },
}

impl FetchError {
    /// The exit code `src/cli` returns (`.:I` exit).
    pub(crate) fn code(&self) -> u8 {
        match self {
            Self::Name { .. } | Self::Resolve { .. } => 1,
            _ => 2,
        }
    }
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, why } => io(f, "read", path, why),
            Self::Write { path, why } => io(f, "write", path, why),
            Self::Config(e) => e.fmt(f),
            Self::Name { image, tag } => {
                let what = "is not a registry name and tag";
                write!(f, "image `{image}` tag `{tag}` {what}{NAME_FIX}")
            }
            Self::Resolve { image, tag, why } => {
                write!(
                    f,
                    "image `{image}:{tag}` unresolved: {why}{RESOLVE_FIX}"
                )
            }
        }
    }
}

const NAME_FIX: &str = "\n  fix: write `## images` rows as `name|tag|digest`, \
    e.g. `nixos/nix|2.35.3|...`";

const RESOLVE_FIX: &str = "\n  fix: check the tag exists and the registry is \
    reachable; .lablith was not changed";

/// `<path>: cannot <verb>: <why>`.
fn io(
    f: &mut fmt::Formatter<'_>,
    verb: &str,
    path: &Path,
    why: &str,
) -> fmt::Result {
    write!(f, "{}: cannot {verb}: {why}", path.display())
}

impl std::error::Error for FetchError {}

#[cfg(test)]
mod tests;
