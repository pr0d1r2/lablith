//! `src/pipeline`: the GitLab pipeline model, lowered to pipelith's YAML
//! tree and written by its writer (`src/pipeline:V7` to
//! `src/pipeline:V10`, `src/pipeline:V51`, `src/pipeline:V52`,
//! `src/pipeline:V66`). The root `.gitlab-ci.yml` holds only
//! `workflow: rules`, `stages:` and `include: local:`; every job lives in
//! its row's `.gitlab/ci/<id>.yml`, fully expanded.
#![allow(dead_code, reason = "used by src/kind in a later wave")]

mod job;
mod root;
mod text;

#[allow(unused_imports, reason = "used by src/kind in a later wave")]
pub(crate) use job::{Job, Rule, Stage, When, row_document};
#[allow(unused_imports, reason = "used by src/render in a later wave")]
pub(crate) use root::{Root, root_document};
#[allow(unused_imports, reason = "used by src/kind in a later wave")]
pub(crate) use text::{concat, eq, eq_var, var};
