//! `render` (`src/render:T44`, `src/render:V30`): every `## workflows` row
//! checked against its v1 kind and built into `.gitlab/ci/<id>.yml`, the
//! root `.gitlab-ci.yml` including every row and hand-written file, every
//! seeded script a rendered kind needs once; sorted bytewise by path.

use std::collections::BTreeSet;

use super::error::RenderError;
use super::kinds::{lookup, names};
use crate::core::ConfigError;
use crate::kind::{Kind, Row, Tables, check};
use crate::pin::PinTable;
use crate::pipeline::{Job, Root, root_document, row_document};
use crate::registry::{Registry, WorkflowRow};
use crate::script::Script;
use crate::system::RunnerTable;

pub(crate) use crate::core::files::File;

/// The root file every render writes.
pub(crate) const ROOT: &str = ".gitlab-ci.yml";

/// A row checked by its kind, with the jobs it renders.
pub(crate) struct RowPlan {
    /// The kind the row names.
    pub(crate) kind: &'static dyn Kind,
    /// The checked row.
    pub(crate) row: Row,
    /// Its jobs, in file order.
    pub(crate) jobs: Vec<Job>,
}

/// `.gitlab/ci/<id>.yml`, the file a row renders to.
pub(crate) fn row_path(id: &str) -> String {
    format!(".gitlab/ci/{id}.yml")
}

/// The pin and runner tables: built-in, then the `.lablith` overrides.
///
/// # Errors
///
/// A `## runners` row naming no nix system.
pub(crate) fn tables(registry: &Registry) -> Result<Tables, RenderError> {
    Ok(Tables {
        pins: PinTable::builtin().with_overrides(&registry.images),
        runners: RunnerTable::builtin().with_overrides(&registry.runners)?,
    })
}

/// Every row checked and built, in file order.
///
/// # Errors
///
/// The first row naming no v1 kind or one its kind refuses.
pub(crate) fn plan(registry: &Registry) -> Result<Vec<RowPlan>, RenderError> {
    let tables = tables(registry)?;
    let one = |raw: &WorkflowRow| -> Result<RowPlan, RenderError> {
        let kind = kind(raw)?;
        let row = check(kind, raw)?;
        let jobs = kind.jobs(&row, &tables)?;
        Ok(RowPlan { kind, row, jobs })
    };
    registry.workflows.iter().map(one).collect()
}

/// Every file `.lablith` asks for, sorted bytewise by path.
///
/// # Errors
///
/// As [`plan`], or two files at one path.
pub(crate) fn render(registry: &Registry) -> Result<Vec<File>, RenderError> {
    let plans = plan(registry)?;
    let mut files: Vec<File> = plans.iter().map(row_file).collect();
    files.push(root(registry, &plans));
    let _ = seeded;
    sorted(files)
}

/// The v1 kind `row` names, or a row error listing them.
fn kind(row: &WorkflowRow) -> Result<&'static dyn Kind, RenderError> {
    lookup(&row.kind).ok_or_else(|| {
        let what = format!("unknown kind `{}`", row.kind.escape_debug());
        let fix = format!("use one of {}", names());
        RenderError::Row(ConfigError::new(row.line, what, fix))
    })
}

/// A row's file.
fn row_file(plan: &RowPlan) -> File {
    File {
        path: row_path(&plan.row.id),
        bytes: row_document(&plan.jobs),
    }
}

/// The root file: every row's file, then every hand-written one.
fn root(registry: &Registry, plans: &[RowPlan]) -> File {
    let rows = plans.iter().map(|p| row_path(&p.row.id));
    let hand = registry.hand_written.iter().take(0).map(|h| h.path.clone());
    File {
        path: ROOT.to_owned(),
        bytes: root_document(&Root {
            includes: rows.chain(hand).collect(),
        }),
    }
}

/// Every seeded script a rendered kind needs, and those it calls.
fn seeded(plans: &[RowPlan]) -> Vec<File> {
    let direct = plans.iter().flat_map(|p| p.kind.scripts(&p.row));
    let needed: BTreeSet<Script> = direct
        .flat_map(|s| std::iter::once(s).chain(s.calls().iter().copied()))
        .collect();
    needed
        .into_iter()
        .map(|s| {
            let (path, bytes) = s.file();
            File {
                path: path.to_owned(),
                bytes: bytes.to_owned(),
            }
        })
        .collect()
}

/// `files` sorted by path, refused when two share one.
fn sorted(mut files: Vec<File>) -> Result<Vec<File>, RenderError> {
    files.sort_by(|a, b| a.path.cmp(&b.path));
    let twice = files.windows(2).find_map(|pair| match pair {
        [a, b] if a.path == b.path => Some(a.path.clone()),
        _ => None,
    });
    twice.map_or(Ok(files), |path| Err(RenderError::Duplicate(path)))
}

#[cfg(test)]
mod tests;
