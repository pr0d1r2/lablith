//! Proposed rows as `.lablith` text (`.:I` file): pipelith's header and
//! `text_with`, each row written by [`proposal`], so a kind that is off by
//! default lands commented out under its note (src/detect:V22).

use crate::core::detect::{row_line, text_with};
use crate::registry::WorkflowRow;

/// A whole `.lablith`: pipelith's header, then each row's [`proposal`].
#[must_use]
pub(crate) fn text(rows: &[WorkflowRow]) -> String {
    text_with(rows, proposal)
}

/// The line above a row proposed off by default (src/detect:V22).
pub(crate) const OFF_NOTE: &str =
    "# off by default: uncomment to get daily nixpkgs-lock bump MRs";

/// Kinds proposed commented out: they push branches and open MRs on a
/// schedule, so a repository opts in by uncommenting (src/detect:V22).
const OFF_BY_DEFAULT: [&str; 1] = ["update-pins"];

/// A proposed row as `detect` and `seed` write it: the row, or for a kind
/// off by default, [`OFF_NOTE`] and the row commented out.
#[must_use]
pub(crate) fn proposal(row: &WorkflowRow) -> String {
    let line = row_line(row);
    if OFF_BY_DEFAULT.contains(&row.kind.as_str()) {
        return format!("{OFF_NOTE}\n# {line}");
    }
    line
}

#[cfg(test)]
mod tests;
