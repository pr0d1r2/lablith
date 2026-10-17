//! `src/fetch`: image tag -> index digest for `lablith pin`; the only
//! network code (`src/fetch:V34`).
//!
//! Composition only: the resolver seam and the `curl` subprocess in
//! `curl` (no HTTP crate, `.:C`), the OCI distribution flow in `oci`, the
//! `## images` rewrite in `section`, the verb in `pin`, the refusal in
//! `error`.
#![allow(dead_code, unused_imports, reason = "used by src/cli in a later wave")]

mod curl;
mod error;
mod oci;
mod pin;
mod section;

pub(crate) use curl::{Curl, Http};
pub(crate) use error::FetchError;
pub(crate) use oci::{Oci, Resolver};
pub(crate) use pin::pin;
