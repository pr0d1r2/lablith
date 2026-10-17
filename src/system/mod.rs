//! `src/system`: nix system -> GitLab runner tags, in one table
//! (`src/system:V56`). Built-in tags are neutral (`.:V20`); a repository
//! overrides them in `## runners`.
#![allow(dead_code, reason = "used by src/kind in a later wave")]

mod table;

#[allow(unused_imports, reason = "used by src/kind in a later wave")]
pub(crate) use table::{RunnerTable, System, SystemError};
