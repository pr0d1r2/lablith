//! `.lablith` text to a [`Registry`] (`src/registry:T12`): pipelith reads
//! the grammar against lablith's four sections, then each row is typed.
//! Every error names its line (`src/registry:V28`, `src/registry:V29`).

use super::model::Registry;
use super::rows::{hand, image, runner};
use crate::core::ConfigError;
use crate::core::kind::{self, RawRow};
use crate::core::registry::{self, Schema, SectionSpec};

/// The file name every error and every message names.
pub(crate) const FILE: &str = ".lablith";

/// lablith's sections: pipelith's neutral `## workflows`, then GitLab's,
/// each keyed by its first column.
const SCHEMA: Schema<'static> = Schema {
    sections: &[
        kind::WORKFLOWS,
        SectionSpec {
            name: "images",
            columns: &["image", "version", "digest"],
            required: false,
            key: Some(0),
        },
        SectionSpec {
            name: "runners",
            columns: &["system", "tags"],
            required: false,
            key: Some(0),
        },
        SectionSpec {
            name: "hand-written",
            columns: &["path", "reason"],
            required: false,
            key: Some(0),
        },
    ],
};

/// Parse a whole `.lablith`.
///
/// # Errors
///
/// The first problem found, with its line, what is wrong and the fix.
pub(crate) fn parse(text: &str) -> Result<Registry, ConfigError> {
    // RED stub (src/registry:T12): reads nothing yet.
    let _ = (registry::parse, RawRow::from_cells, &SCHEMA, text);
    let _ = (image, runner, hand);
    if text.is_empty() {
        return Err(ConfigError::default());
    }
    Ok(Registry::default())
}

#[cfg(test)]
mod tests;
