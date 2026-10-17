//! lablith: materialize GitLab CI pipelines from a small config.
//!
//! This file only declares node modules and re-exports their public
//! surface (`src:V37`); every node is a private module. The design lives
//! in `SPEC.md` and the `SPEC.md` of each node.

mod build;
mod bump;
mod check;
mod cli;
mod core;
mod detect;
mod fetch;
mod gate;
mod r#gen;
mod kind;
mod pin;
mod pipeline;
mod registry;
mod render;
mod report;
mod script;
mod system;
mod yaml;

pub use crate::core::ConfigError;
pub use cli::lablith;
