//! `src/detect`: `detect` & `seed --init`: proposed rows from tracked files.
//!
//! Composition only: `detect` and the row text are pipelith's
//! (src/core); the off-by-default rule lives in `text`, the write in
//! `seed`. Pure except `seed`, which writes the one `.lablith` (src/detect:V35).
//!
//! Until `src/cli` lands, the lib build sees the node as unused: dead code
//! and the re-exports are `allow`ed.
#![allow(dead_code, unused_imports, reason = "used by src/cli in a later wave")]

mod seed;
mod text;

use crate::core::detect as neutral;
use crate::registry::WorkflowRow;

pub(crate) use neutral::row_line;
pub(crate) use seed::{SeedError, seed_init};
pub(crate) use text::{flake_systems, proposal};

/// Propose rows, specializing the gate row to systems declared by the flake.
#[must_use]
pub(crate) fn detect(
    tracked: &[String],
    flake_lock: Option<&str>,
    flake: Option<&str>,
) -> Vec<WorkflowRow> {
    let mut rows = neutral::detect(tracked, flake_lock);
    if let Some(systems) = flake
        .filter(|flake| flake.contains("devShells"))
        .map(text::flake_systems)
        .filter(|s| !s.is_empty())
        && let Some(row) = rows.iter_mut().find(|row| row.kind == "gate")
    {
        row.systems = systems;
    }
    rows
}

#[cfg(test)]
mod tests;
