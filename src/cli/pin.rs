//! `pin [--dry-run]`: the one networked verb (`.:I`, `src/fetch:V34`).
//! Dispatch only: the resolve and the `## images` rewrite are
//! `src/fetch`'s; the resolver is passed in so tests need no network.

use std::path::Path;

use super::exit::{Outcome, Ran};
use crate::fetch::{Resolver, pin as resolve};

/// `pin`: the fresh `## images` rows on stdout, written into `.lablith`
/// unless `dry_run`.
///
/// # Errors
///
/// Exit 1: an image did not resolve, and `.lablith` is unchanged
/// (`src/fetch:V59`). Exit 2: `.lablith` unreadable, refused or
/// unwritable. Each names its fix.
pub(crate) fn pin(root: &Path, dry_run: bool, resolver: &dyn Resolver) -> Ran {
    let rows = resolve(root, dry_run, resolver)
        .map_err(|e| Outcome::fail(e.code(), e))?;
    Ok(Outcome::ok(rows))
}

#[cfg(test)]
mod tests;
