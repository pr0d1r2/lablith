//! lablith: materialize GitLab CI pipelines from a small config.
//!
//! This file only declares node modules and re-exports their public
//! surface (`src:V37`); every node is a private module. The design lives
//! in `SPEC.md` and the `SPEC.md` of each node.

mod core;
mod registry;

pub use crate::core::ConfigError;
