//! `report [--format text|json|md]` (`.:I`, `src/cli:V34`). Dispatch
//! only: the report is `src/report`'s.

use std::path::Path;

use super::exit::{Outcome, Ran, USAGE};
use super::load::registry;
use crate::report::{Format, report as reported};

/// `report`: the `.lablith` under `root` in `format`, on stdout.
///
/// # Errors
///
/// Exit 2: `.lablith` missing or refused, or a row its kind refuses.
pub(crate) fn report(root: &Path, format: Format) -> Ran {
    let text = reported(&registry(root)?, format)
        .map_err(|e| Outcome::fail(USAGE, e))?;
    Ok(Outcome::ok(text))
}

#[cfg(test)]
mod tests;
