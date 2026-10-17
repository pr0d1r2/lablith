//! Portability (`src/check:V21`): each row, kind or arg key actlith could
//! not render, from pipelith's `kind::portable`. lablith declares no
//! forge-only kind or key (`src/kind` `EXTRAS`), so a v1 `.lablith` has
//! none; `## images` and `## runners` are sections, never findings.

use std::fmt;

use crate::core::kind::{Finding, portable as neutral};
use crate::kind::EXTRAS;
use crate::registry::Registry;

/// One reason a row would not survive the move to GitHub.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PortFinding(pub(crate) Finding);

/// Every portability finding of `registry`'s rows, in file order.
pub(crate) fn portable(registry: &Registry) -> Vec<PortFinding> {
    let row = |raw| neutral(raw, &EXTRAS).into_iter().map(PortFinding);
    registry.workflows.iter().flat_map(row).collect()
}

impl fmt::Display for PortFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (line, what) = match &self.0 {
            Finding::Kind { line, kind } => (line, format!("kind `{kind}`")),
            Finding::Key { line, key } => (line, format!("arg `{key}`")),
        };
        write!(
            f,
            ".lablith:{line}: {what} is GitLab-only; actlith cannot render \
             it\n  fix: use a catalogue kind and its args (docs/KINDS.md)"
        )
    }
}

#[cfg(test)]
mod tests;
