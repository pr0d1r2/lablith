//! `src/check`: `check`: totality over `.gitlab-ci.yml` and `.gitlab/ci/`
//! (`src/check:V6`, `src/check:V65`), and the portability findings that
//! `check` and `portable` report (`src/check:V21`).
//!
//! Composition only: the listing read from disk lives in `listing`, the
//! pure totality in `totality`, the finding in `finding`, portability in
//! `portable`.
#![allow(dead_code, unused_imports, reason = "used by src/cli in a later wave")]

mod finding;
mod listing;
mod portable;
mod totality;

pub(crate) use finding::{Finding, Problem};
pub(crate) use listing::{CI_DIR, listing};
pub(crate) use portable::{PortFinding, portable};
pub(crate) use totality::totality;
