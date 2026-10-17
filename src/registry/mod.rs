//! `src/registry`: parse `.lablith` into typed rows; every row error named.
//!
//! Composition only: the logic lives in the submodules. Until a consumer
//! lands, the lib build sees the node as unused; dead code is `allow`ed,
//! not `expect`ed, as in actlith.
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/render in a later wave")
)]

mod model;
mod parse;
mod rows;

#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/render in a later wave")
)]
pub(crate) use model::{
    Arg, HandWritten, ImagePin, Registry, RunnerRow, WorkflowRow,
};
pub(crate) use parse::{FILE, parse};
