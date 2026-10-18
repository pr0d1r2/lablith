//! `src/kind`: what every kind shares: the `Kind` trait, the row check,
//! declared secrets (`src/kind:V32`), the hardened nix job
//! (`src/kind:V11` to `src/kind:V15`) and its rules (`src/kind:V50`).
//!
//! Composition only: the logic lives in the submodules.
#![allow(
    dead_code,
    unused_imports,
    reason = "used by the kind nodes and src/render in a later wave"
)]

mod contract;
mod error;
mod job;
mod row;
mod rules;
mod secret;
mod shell;

pub(crate) use crate::core::kind::{
    Args, BUILD, GATE, KindSpec, Purpose, Row, UPDATE_PINS, UPSTREAM,
};
pub(crate) use contract::{Kind, Tables};
pub(crate) use error::KindError;
pub(crate) use job::{Depth, NIX_IMAGE, NixJob, nix_job, runnable};
pub(crate) use row::{EXTRAS, check};
pub(crate) use rules::{ROW, bump_rules, change_rules, default_branch_rules};
pub(crate) use secret::{DEFAULT_TOKEN, Secret};
pub(crate) use shell::quote;
