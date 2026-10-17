//! `src/render`: `.lablith` rows -> the ordered set of files to write
//! (`src/render:V30`). Pure: no I/O (`.:V1`).
//!
//! Composition only: the closed list of v1 kinds lives in `kinds`, the
//! refusal in `error`, the plan and the render in `files`.
#![allow(dead_code, reason = "used by src/gen in a later wave")]

mod error;
mod files;
mod kinds;

#[allow(unused_imports, reason = "used by src/gen in a later wave")]
pub(crate) use error::RenderError;
#[allow(unused_imports, reason = "used by src/gen in a later wave")]
pub(crate) use files::{File, ROOT, RowPlan, plan, render, row_path, tables};
#[allow(unused_imports, reason = "used by src/report in a later wave")]
pub(crate) use kinds::{entries, lookup};

#[cfg(test)]
mod tests;
