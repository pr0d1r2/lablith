//! A `## workflows` row checked for its kind by pipelith's `Row::check`
//! (systems, cache, args, timeout and row rules; `src/registry:V28`).
//! A kind builds only from a checked row.

use super::contract::Kind;
use super::error::KindError;
use crate::core::kind::{Extras, Row};
use crate::registry::WorkflowRow;

/// The kinds and arg keys only lablith knows: none, so every row lablith
/// renders works on every forge (`.:V4`, `src/check:V21`).
pub(crate) const EXTRAS: Extras<'static> = Extras {
    kinds: &[],
    keys: &[],
};

/// `row` checked against `kind`.
pub(crate) fn check(
    kind: &dyn Kind,
    row: &WorkflowRow,
) -> Result<Row, KindError> {
    Ok(Row::check(kind.spec(), row, &EXTRAS)?)
}

#[cfg(test)]
mod tests;
