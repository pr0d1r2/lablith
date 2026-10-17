//! `lablith pin` (`src/fetch:T30`, `src/fetch:T49`): resolve every pinned
//! image's tag to its index digest and rewrite `## images`
//! (`src/fetch:V34`). Every lookup runs before anything is written, so a
//! failure leaves `.lablith` as it was (`src/fetch:V59`).

use std::fs;
use std::path::{Path, PathBuf};

use super::error::FetchError;
use super::oci::Resolver;
use super::section;
use crate::pin::{Image, PinTable};
use crate::registry::{FILE, parse};

/// Resolve each image of the pin table (built-in and `## images` rows)
/// at its recorded tag. Returns the new `## images` rows; writes them
/// into `<root>/.lablith` unless `dry_run`.
///
/// # Errors
///
/// `.lablith` unreadable, unparsable or unwritable; an image whose tag
/// the registry does not resolve to an index digest.
pub(crate) fn pin(
    root: &Path,
    dry_run: bool,
    resolver: &dyn Resolver,
) -> Result<String, FetchError> {
    let path = root.join(FILE);
    let text = read(&path)?;
    let reg = parse(&text).map_err(FetchError::Config)?;
    let table = PinTable::builtin().with_overrides(&reg.images);
    let fresh = table
        .all()
        .map(|i| resolve(i, resolver))
        .collect::<Result<Vec<_>, _>>()?;
    let rows = section::rows(&fresh);
    if !dry_run {
        write(path, &section::replace(&text, &rows))?;
    }
    Ok(rows)
}

/// `image` at its recorded version, with the digest the registry gives.
fn resolve(
    image: &Image,
    resolver: &dyn Resolver,
) -> Result<Image, FetchError> {
    Ok(Image {
        digest: resolver.digest(&image.name, &image.version)?,
        ..image.clone()
    })
}

/// The text of `.lablith`.
fn read(path: &Path) -> Result<String, FetchError> {
    fs::read_to_string(path).map_err(|e| FetchError::Read {
        path: path.to_owned(),
        why: e.to_string(),
    })
}

/// Replace `.lablith` with `text`.
fn write(path: PathBuf, text: &str) -> Result<(), FetchError> {
    fs::write(&path, text).map_err(|e| FetchError::Write {
        path,
        why: e.to_string(),
    })
}

#[cfg(test)]
mod tests;
