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

pub(crate) use crate::core::detect::{detect, row_line};
pub(crate) use seed::{SeedError, seed_init};
pub(crate) use text::proposal;
