//! Totality (`src/check:V6`, `src/check:V65`): pure, over a listing
//! already read.

use std::collections::BTreeSet;
use std::path::Path;

use super::finding::{Finding, Problem};
use super::listing::{CI_DIR, ROOT};
use crate::core::files::File;
use crate::registry::HandWritten;

/// Every finding for `listing` against the `rendered` files and the
/// `hand_written` rows, sorted by path.
pub(crate) fn totality(
    listing: &[String],
    rendered: &[File],
    hand_written: &[HandWritten],
) -> Vec<Finding> {
    let made: BTreeSet<&str> = rendered.iter().map(|f| &*f.path).collect();
    let listed: BTreeSet<&str> = listing.iter().map(|p| &**p).collect();
    let hand: BTreeSet<&str> = hand_written.iter().map(|h| &*h.path).collect();
    let mut found: Vec<Finding> = listing
        .iter()
        .filter(|p| is_ci(p) && !made.contains(&***p) && !hand.contains(&***p))
        .map(|p| finding(p, None, Problem::Undeclared))
        .collect();
    for row in hand_written {
        found.extend(rows(row, &made, &listed));
    }
    found.sort();
    found
}

/// The findings for one `## hand-written` row.
fn rows(
    row: &HandWritten,
    made: &BTreeSet<&str>,
    listed: &BTreeSet<&str>,
) -> Vec<Finding> {
    let path = row.path.as_str();
    let checks = [
        (row.reason.trim().is_empty(), Problem::NoReason),
        (!listed.contains(path), Problem::Missing),
        (made.contains(path), Problem::AlsoRendered),
    ];
    checks
        .into_iter()
        .filter(|(hit, _)| *hit)
        .map(|(_, problem)| finding(path, Some(row.line), problem))
        .collect()
}

/// A CI file GitLab may read: the root file, or a `.yml` / `.yaml` under
/// `.gitlab/ci/`, the extension in any case.
fn is_ci(path: &str) -> bool {
    if path == ROOT {
        return true;
    }
    let ext = Path::new(path).extension().and_then(|e| e.to_str());
    path.starts_with(CI_DIR)
        && ext.is_some_and(|e| {
            e.eq_ignore_ascii_case("yml") || e.eq_ignore_ascii_case("yaml")
        })
}

/// One finding on `path`.
fn finding(path: &str, line: Option<usize>, problem: Problem) -> Finding {
    Finding {
        path: path.to_owned(),
        line,
        problem,
    }
}

#[cfg(test)]
mod tests;
