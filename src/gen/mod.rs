//! `src/gen`: `gen` writes rendered files; `gen --check` proves they are
//! current (`src/gen:V5`).
//!
//! Composition only: the write, the compare and the rule that a rendered
//! path stays plain and under the root are pipelith's `files`, through
//! `src/core`. The only node besides `pin` that writes files, and only
//! rendered paths under the root.
#![allow(unused_imports, reason = "used by src/cli in a later wave")]

pub(crate) use crate::core::files::{Stale, check, write as r#gen};

#[cfg(test)]
mod tests;
