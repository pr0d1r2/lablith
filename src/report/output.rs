//! `report` (`src/report:T29`): the `.lablith` reported in one
//! [`Format`]. Pure: no I/O (`.:V1`).

use super::collect::collect;
use super::document::document;
use super::format::Format;
use super::json::write;
use super::md::md;
use super::text::text;
use crate::registry::Registry;
use crate::render::RenderError;

/// `registry` reported in `format`.
///
/// # Errors
///
/// A row naming no v1 kind or one its kind refuses.
pub(crate) fn report(
    registry: &Registry,
    format: Format,
) -> Result<String, RenderError> {
    let report = collect(registry)?;
    Ok(match format {
        Format::Text => text(&report),
        Format::Json => write(&document(&report)),
        Format::Md => md(&report),
    })
}

#[cfg(test)]
mod tests;
