//! A `## workflows` row checked for its kind by pipelith's
//! `Row::check_with` (systems, cache, args, timeout and row rules;
//! `src/registry:V28`) against the file's `## caches` rows. A kind builds
//! only from a checked row. A `-` row keeps only the systems the runner
//! table covers (`src/system:V75`).

use std::ops::Deref;

use super::contract::Kind;
use super::error::KindError;
use crate::core::cache::{BinaryCache, Caches};
use crate::core::kind::{Checked, Extras, KindSpec, Row as Base};
use crate::registry::WorkflowRow;
use crate::system::RunnerTable;

/// The kinds and arg keys only lablith knows: none, so every row lablith
/// renders works on every forge (`.:V4`, `src/check:V21`).
pub(crate) const EXTRAS: Extras<'static> = Extras {
    kinds: &[],
    keys: &[],
};

/// A checked row: pipelith's row, read through `Deref`, and the cache it
/// trusts. Cache steps render from `cache` only, never from the 0.2.0
/// `Base::cache`, which a `## caches` row leaves empty (`src/cache:V80`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Row {
    /// pipelith's checked row.
    pub(crate) base: Base,
    /// The cache the row trusts, from `## caches` or built in.
    pub(crate) cache: Option<BinaryCache>,
}

impl Deref for Row {
    type Target = Base;

    fn deref(&self) -> &Base {
        &self.base
    }
}

impl From<Checked> for Row {
    fn from(checked: Checked) -> Self {
        Self {
            base: checked.row,
            cache: checked.cache,
        }
    }
}

#[cfg(test)]
impl Row {
    /// `row` checked against `spec` with the built-in caches only, as a
    /// `.lablith` without `## caches` is. Tests build rows with it;
    /// rendering checks every row with the file's caches (`check`).
    ///
    /// # Errors
    ///
    /// As pipelith's `Row::check_with`.
    pub(crate) fn check(
        spec: &KindSpec,
        row: &WorkflowRow,
        extras: &Extras<'_>,
    ) -> Result<Self, KindError> {
        let checked = Base::check_with(spec, row, extras, &Caches::default())?;
        Ok(checked.into())
    }
}

/// `row` checked against `kind` and the file's `caches`; for `-`,
/// pipelith's default systems that `runners` covers, so `x86_64-darwin`
/// joins only with its own `## runners` row (`src/system:V75`).
pub(crate) fn check(
    kind: &dyn Kind,
    row: &WorkflowRow,
    runners: &RunnerTable,
    caches: &Caches,
) -> Result<Row, KindError> {
    let mut checked: Row =
        Base::check_with(kind.spec(), row, &EXTRAS, caches)?.into();
    if row.systems.is_empty() {
        checked.base.systems.retain(|&s| runners.covers(s));
    }
    Ok(checked)
}

#[cfg(test)]
mod tests;
