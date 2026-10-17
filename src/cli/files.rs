//! `gen [--check]`, `check` and `portable`: the verbs over rendered
//! files and rows (`.:I`, `src/cli:V34`). Dispatch only: the write, the
//! compare, totality and portability live in `src/gen` and `src/check`.

use std::path::Path;

use super::exit::{FINDING, Outcome, Ran, lines};
use super::load::{registry, rendered};
use crate::check::{listing, portable as moves, totality};
use crate::core::files::File;
use crate::r#gen::{Stale, check as compare, r#gen as write};

/// The fix every stale or missing file names.
const REGEN: &str = "fix: run `lablith gen` and commit the result";

/// `gen`: write each rendered file that differs, printing its path;
/// `gen --check`: write nothing, exit 1 naming each stale or missing
/// file.
///
/// # Errors
///
/// Exit 2: `.lablith` missing or refused, or a read or write failing.
pub(crate) fn generate(root: &Path, check: bool) -> Ran {
    let (_, files) = rendered(root)?;
    if check {
        return stale(root, &files);
    }
    let written =
        write(root, &files).map_err(|e| Outcome::io(root, "write", &e))?;
    Ok(Outcome::ok(lines(
        written.iter().map(|path| format!("wrote {path}")),
    )))
}

/// Exit 1 naming each of `files` stale or missing under `root`.
fn stale(root: &Path, files: &[File]) -> Ran {
    let stale =
        compare(root, files).map_err(|e| Outcome::io(root, "read", &e))?;
    if stale.is_empty() {
        return Ok(Outcome::ok(""));
    }
    let named = |s: &Stale| format!("{s}; {REGEN}");
    Err(Outcome::fail(FINDING, lines(stale.iter().map(named))))
}

/// `check`: exit 1 naming each CI file no row generates and no
/// `## hand-written` row lists, each bad `## hand-written` row, and each
/// row actlith could not render (`src/check:V21`).
///
/// # Errors
///
/// Exit 2: `.lablith` missing or refused, or the listing unreadable.
pub(crate) fn check(root: &Path) -> Ran {
    let (registry, files) = rendered(root)?;
    let on_disk = listing(root).map_err(|e| Outcome::io(root, "list", &e))?;
    let found = totality(&on_disk, &files, &registry.hand_written);
    let unported = moves(&registry);
    if found.is_empty() && unported.is_empty() {
        return Ok(Outcome::ok(""));
    }
    let text = lines(found) + &lines(unported);
    Err(Outcome::fail(FINDING, text))
}

/// `portable`: exit 1 naming each row, kind or arg key actlith cannot
/// render; `## images` and `## runners` never count (`src/check:V21`).
///
/// # Errors
///
/// Exit 2: `.lablith` missing or refused.
pub(crate) fn portable(root: &Path) -> Ran {
    let unported = moves(&registry(root)?);
    if unported.is_empty() {
        return Ok(Outcome::ok(""));
    }
    Err(Outcome::fail(FINDING, lines(unported)))
}

#[cfg(test)]
mod tests;
