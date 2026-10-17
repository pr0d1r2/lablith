//! `src/core`: the one node that names `pipelith` (`src/core:V3`). It
//! re-exports pipelith's neutral modules as the nodes that use them land;
//! every other node reaches them through `crate::core`. It also owns the
//! one error shape lablith prints for a refused `.lablith`.

mod error;

pub(crate) use pipelith::{kind, registry};

pub use error::ConfigError;
