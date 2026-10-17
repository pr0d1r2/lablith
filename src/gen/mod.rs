//! `src/gen`: `gen` writes rendered files; `gen --check` proves they are
//! current (`src/gen:V5`).
//!
//! Composition only: the write, the compare and the rule that a rendered
//! path stays plain and under the root are pipelith's `files`, through
//! `src/core`. The only node besides `pin` that writes files, and only
//! rendered paths under the root.
#![allow(unused_imports, reason = "used by src/cli in a later wave")]

pub(crate) use crate::core::files::Stale;

/// RED stub (src/gen:T25): checks nothing.
#[allow(dead_code, clippy::unnecessary_wraps, reason = "RED stub")]
pub(crate) fn check(
    _: &std::path::Path,
    _: &[crate::core::files::File],
) -> std::io::Result<Vec<Stale>> {
    Ok(Vec::new())
}

/// RED stub (src/gen:T25): writes nothing.
#[allow(dead_code, clippy::unnecessary_wraps, reason = "RED stub")]
pub(crate) fn r#gen(
    _: &std::path::Path,
    _: &[crate::core::files::File],
) -> std::io::Result<Vec<String>> {
    Ok(Vec::new())
}

#[cfg(test)]
mod tests;
