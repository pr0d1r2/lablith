//! `src/yaml`: GitLab's expression dialect for pipelith's YAML writer
//! (`src/yaml:V18`). The writer itself, its key order and its quoting
//! are pipelith's.
#![allow(
    dead_code,
    unused_imports,
    reason = "used by src/pipeline in a later wave"
)]

mod dialect;

pub(crate) use dialect::GITLAB;
