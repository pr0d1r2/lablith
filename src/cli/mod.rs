//! `src/cli`: clap verbs, flags and exit codes over the verb nodes.
//!
//! Composition only: the argv table in `args`, the exit codes and printed
//! outcome in `exit`, the entry point and dispatch in `entry`; each verb
//! group dispatches to its node: `files` (`gen`, `check`, `portable`),
//! `rows` (`detect`, `seed`), `report`, `pin`, `meta` (`completions`,
//! `man`). `load` reads `.lablith`, `ls_files` the tracked files. `clap`
//! lives here only (`.:C`).

mod args;
mod entry;
mod exit;
mod files;
mod load;
mod ls_files;
mod meta;
mod pin;
mod report;
mod rows;
#[cfg(test)]
mod scratch;

pub use entry::lablith;
