//! `src/script`: seeded shell scripts shipped in the crate, written by
//! `gen` with the generated header (`src/script:V16`).
//!
//! The scripts are real files beside this module, embedded by `embed`.
#![allow(dead_code, reason = "used by src/render in a later wave")]

mod embed;

#[allow(unused_imports, reason = "used by src/kind in a later wave")]
pub(crate) use embed::{Script, scripts};
