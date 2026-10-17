//! `detect` and `seed --init`: the verbs that propose `## workflows` rows
//! (`.:I`, `src/cli:V34`). Dispatch only: the rows come from `src/detect`.

use std::fs;
use std::path::Path;

use super::exit::{Outcome, Ran, USAGE, lines};
use super::ls_files::tracked;
use crate::detect::{SeedError, detect as propose, proposal, seed_init};
use crate::registry::WorkflowRow;

/// `detect`: the proposed rows, one per line; a row off by default
/// comes commented, under its note (`src/detect:V22`).
///
/// # Errors
///
/// Exit 2: the tracked file list cannot be read.
pub(crate) fn detect(root: &Path) -> Ran {
    let rows = proposed(root)?;
    Ok(Outcome::ok(lines(rows.iter().map(proposal))))
}

/// `seed --init`: write the first `.lablith` from `detect`'s rows.
///
/// # Errors
///
/// Exit 2 without `--init` or when the file cannot be written; exit 1
/// when `.lablith` is already there (`src/detect:V35`).
pub(crate) fn seed(root: &Path, init: bool) -> Ran {
    if !init {
        return Err(Outcome::fail(
            USAGE,
            "seed: nothing to do without --init\n  fix: run `lablith seed \
             --init`",
        ));
    }
    let rows = proposed(root)?;
    let path = seed_init(root, &rows).map_err(|e| refused(&e))?;
    Ok(Outcome::ok(format!("wrote {}\n", path.display())))
}

/// The rows `detect` proposes for the files tracked under `root`.
fn proposed(root: &Path) -> Result<Vec<WorkflowRow>, Outcome> {
    let files = tracked(root)?;
    let lock = fs::read_to_string(root.join("flake.lock")).ok();
    Ok(propose(&files, lock.as_deref()))
}

/// Why `seed --init` wrote nothing, with the fix.
fn refused(error: &SeedError) -> Outcome {
    let fix = match error {
        SeedError::Exists(_) => "edit it by hand, or delete it to seed again",
        SeedError::Write(..) => "make the root writable, or pass --root DIR",
    };
    Outcome::fail(error.code(), format!("{error}\n  fix: {fix}"))
}

#[cfg(test)]
mod tests;
