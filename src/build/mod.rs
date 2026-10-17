//! `src/build`: kind `build`: build and cache a flake package on every
//! system, check the flake, then ask the cache for every built path.
//!
//! Composition only: the kind lives in `kind`.
#![allow(dead_code, reason = "used by src/render in a later wave")]

mod kind;

#[allow(unused_imports, reason = "used by src/render in a later wave")]
pub(crate) use kind::Build;
