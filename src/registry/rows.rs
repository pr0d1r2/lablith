//! GitLab's sections, typed from the cells pipelith read
//! (`src/registry:T12`): `## images`, `## runners`, `## hand-written`.

use super::model::{HandWritten, ImagePin, RunnerRow};
use super::parse::FILE;
use crate::core::ConfigError;
use crate::core::registry::{Row, list, required};

#[cfg(test)]
mod tests;

/// An `image|version|digest` row; the digest is `sha256:` + 64 hex.
pub(super) fn image(row: &Row) -> Result<ImagePin, ConfigError> {
    Ok(ImagePin {
        line: row.line,
        image: required(FILE, row, 0, "image")?,
        version: required(FILE, row, 1, "version")?,
        digest: digest(row.line, required(FILE, row, 2, "digest")?)?,
    })
}

/// A `system|tags` row; at least one tag.
pub(super) fn runner(row: &Row) -> Result<RunnerRow, ConfigError> {
    let system = required(FILE, row, 0, "system")?;
    let tags = list(FILE, row, 1, "tags")?;
    if tags.is_empty() {
        let what = format!("runner `{system}` has no tags");
        let fix = "list the tags a job on this system carries, e.g. `nix`";
        return Err(ConfigError::new(row.line, what, fix));
    }
    Ok(RunnerRow {
        line: row.line,
        system,
        tags,
    })
}

/// A `path|reason` row; both are required.
pub(super) fn hand(row: &Row) -> Result<HandWritten, ConfigError> {
    Ok(HandWritten {
        line: row.line,
        path: required(FILE, row, 0, "path")?,
        reason: required(FILE, row, 1, "reason")?,
    })
}

/// `digest` when it is `sha256:` and 64 lowercase hex digits.
fn digest(line: usize, digest: String) -> Result<String, ConfigError> {
    let hex = |c: char| c.is_ascii_digit() || ('a'..='f').contains(&c);
    let ok = digest
        .strip_prefix("sha256:")
        .is_some_and(|h| h.len() == 64 && h.chars().all(hex));
    if ok {
        return Ok(digest);
    }
    let what = format!("digest `{digest}` is not `sha256:` and 64 hex digits");
    let fix = "write the image index digest (`lablith pin` resolves it)";
    Err(ConfigError::new(line, what, fix))
}
