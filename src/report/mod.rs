//! `src/report`: `report`: rows x systems x image pins, the secrets each
//! row needs, the pipeline schedules to create and the token they need
//! (`src/report:T29`, `src/report:T53`), as text, JSON
//! (`src/report:V36`) or Markdown.
//!
//! Composition only: the sorted model in `model` (built by `collect`
//! from `src/render`'s plan), the formats in `text`, `json` with
//! `document`, and `md`, the verb in `output`.
#![allow(dead_code, unused_imports, reason = "used by src/cli in a later wave")]

mod collect;
mod document;
mod format;
mod json;
mod md;
mod model;
mod output;
mod text;

pub(crate) use format::Format;
pub(crate) use output::report;
