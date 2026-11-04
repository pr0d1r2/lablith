//! `src/cache/read`: how a job reads a `## caches` cache
//! (`src/cache/read:T74`): the `NIX_CONFIG` lines of a cache with literal
//! settings, and the `cache-read.sh` line of one with a secret or a read
//! netrc.

use super::arg;
use crate::core::cache::{BinaryCache, CacheValue};
use crate::kind::quote;
use crate::script::Script;

/// The `NIX_CONFIG` lines that trust `cache` when its url and key are
/// literals; empty for no cache or a secret setting (`src/cache/read:V81`).
pub(crate) fn nix_config(cache: Option<&BinaryCache>) -> String {
    cache.and_then(BinaryCache::nix_config).unwrap_or_default()
}

/// The script line that adds a secret url or key, or a read netrc, to
/// the job's nix config at its start, or `None` when url and key are
/// literals and no netrc is read (`src/cache/read:V81`, V86). It assigns
/// rather than exports, so a refusal of `cache-read.sh` stops the job.
pub(crate) fn read_line(cache: Option<&BinaryCache>) -> Option<String> {
    let cache = cache.filter(|c| !c.read_secrets().is_empty())?;
    let [(_, url), (_, key)] = cache.nix_settings();
    let script = Script::CacheRead.path();
    let (url, key) = (quote(&arg(url)), quote(&arg(key)));
    let netrc = read_netrc(cache)
        .map(|netrc| format!(" {netrc} {}", cache.name()))
        .unwrap_or_default();
    Some(format!(
        "NIX_CONFIG=\"$NIX_CONFIG$(bash {script} {url} {key}{netrc})\""
    ))
}

/// The variable holding the netrc `cache` is read with, if any
/// (`src/cache/read:V86`).
pub(super) fn read_netrc(cache: &BinaryCache) -> Option<&str> {
    cache.read_netrc().and_then(CacheValue::secret)
}

#[cfg(test)]
mod tests;
