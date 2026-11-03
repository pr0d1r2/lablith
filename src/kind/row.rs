//! A `## workflows` row checked for its kind by pipelith's `Row::check`
//! (systems, cache, args, timeout and row rules; `src/registry:V28`).
//! A kind builds only from a checked row. A `-` row keeps only the
//! systems the runner table covers (`src/system:V75`).

use super::contract::Kind;
use super::error::KindError;
use crate::core::kind::{Extras, Row};
use crate::registry::WorkflowRow;
use crate::system::RunnerTable;

/// The kinds and arg keys only lablith knows: none, so every row lablith
/// renders works on every forge (`.:V4`, `src/check:V21`).
pub(crate) const EXTRAS: Extras<'static> = Extras {
    kinds: &[],
    keys: &[],
};

/// `row` checked against `kind`; for `-`, pipelith's default systems
/// that `runners` covers, so `x86_64-darwin` joins only with its own
/// `## runners` row (`src/system:V75`).
pub(crate) fn check(
    kind: &dyn Kind,
    row: &WorkflowRow,
    runners: &RunnerTable,
) -> Result<Row, KindError> {
    let mut checked = Row::check(kind.spec(), row, &EXTRAS)?;
    if row.systems.is_empty() {
        checked.systems.retain(|&s| runners.covers(s));
    }
    Ok(checked)
}

#[cfg(test)]
mod tests;
