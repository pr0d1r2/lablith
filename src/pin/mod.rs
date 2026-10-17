//! `src/pin`: the image pin table: every `image:` lablith writes is a
//! name pinned by digest, with its version (`src/pin:V2`).
//!
//! Composition only: the built-in table and the `## images` override
//! merge live in `table`, one resolved pin in `image`.
#![allow(dead_code, reason = "used by src/kind in a later wave")]

mod image;
mod table;

#[allow(unused_imports, reason = "used by src/kind in a later wave")]
pub(crate) use image::{Image, PinError};
#[allow(unused_imports, reason = "used by src/kind in a later wave")]
pub(crate) use table::PinTable;
