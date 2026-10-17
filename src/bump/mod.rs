//! `src/bump`: kinds `update-pins` and `upstream`: scheduled bump merge
//! requests (`src/bump:V17`, `src/bump:V33`).
//!
//! Composition only: both kinds live in `kinds`.
#![allow(dead_code, reason = "used by src/render in a later wave")]

mod kinds;

#[allow(unused_imports, reason = "used by src/render in a later wave")]
pub(crate) use kinds::{UpdatePins, Upstream};
