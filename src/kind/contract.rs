//! The `Kind` trait: what each kind node implements, and the tables a
//! kind builds jobs from.

use super::error::KindError;
use super::secret::Secret;
use crate::core::kind::{KindSpec, Row};
use crate::pin::PinTable;
use crate::pipeline::Job;
use crate::script::Script;
use crate::system::RunnerTable;

/// The pin and runner tables, built-in plus the `.lablith` overrides.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Tables {
    /// Image pins (`src/pin:V2`).
    pub(crate) pins: PinTable,
    /// Runner tags (`src/system:V56`).
    pub(crate) runners: RunnerTable,
}

/// One kind: its pipelith catalogue entry and how it renders on GitLab.
pub(crate) trait Kind {
    /// pipelith's catalogue entry (`docs/KINDS.md`).
    fn spec(&self) -> &'static KindSpec;

    /// Every secret the row's jobs read (`src/kind:V32`).
    fn secrets(&self, row: &Row) -> Vec<Secret>;

    /// The seeded scripts the row's jobs call (`src/render:V30`).
    fn scripts(&self, row: &Row) -> Vec<Script>;

    /// The row's jobs, in file order.
    fn jobs(&self, row: &Row, tables: &Tables) -> Result<Vec<Job>, KindError>;
}
