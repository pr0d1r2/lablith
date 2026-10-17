//! `src/gate`: kind `gate`: the hk gate on every CI system.
//!
//! Composition only: the kind lives in `shape`; `src/render` picks it up.
#![allow(dead_code, reason = "used by src/render in a later wave")]

mod shape;

#[allow(unused_imports, reason = "used by src/render in a later wave")]
pub(crate) use shape::Gate;
