//! `src/cache`: pipelith's `## caches` on GitLab (`src/cache:T72`): the
//! nix settings a job trusts a cache with, the line that pushes to it,
//! the line that verifies it and the secrets each needs. A secret value
//! never reaches a rendered file: a line names the variable, as
//! `secret:NAME` for the seeded scripts to read (`src/cache:C`).

use crate::core::cache::{
    BinaryCache, CacheKind, CacheValue, Push, SECRET_PREFIX,
};
use crate::kind::{Purpose, Secret, quote};
use crate::script::Script;

/// Where a push may happen (`src/cache:V83`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Branches {
    /// Any ref the pipeline runs on (`gate`).
    Any,
    /// The default branch only (`build`).
    Default,
}

impl Branches {
    /// The word `cache-push.sh` reads.
    fn word(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Default => "default",
        }
    }
}

/// The `NIX_CONFIG` lines that trust `cache` when its url and key are
/// literals; empty for no cache or a secret setting (`src/cache:V81`).
pub(crate) fn nix_config(cache: Option<&BinaryCache>) -> String {
    cache.and_then(BinaryCache::nix_config).unwrap_or_default()
}

/// The script line that adds a secret url or key to the job's nix
/// config at its start, or `None` when both are literals (`src/cache:V81`).
pub(crate) fn read_line(cache: Option<&BinaryCache>) -> Option<String> {
    let cache = cache.filter(|c| !c.read_secrets().is_empty())?;
    let [(_, url), (_, key)] = cache.nix_settings();
    let script = Script::CacheRead.path();
    let (url, key) = (quote(&arg(url)), quote(&arg(key)));
    Some(format!(
        "export NIX_CONFIG=\"$NIX_CONFIG$(bash {script} {url} {key})\""
    ))
}

/// `command`, pushing to `cache` as its `push` says: alone for a read-only
/// cache, through `cachix.sh` for the built-in one as before, through
/// `cache-push.sh` for a push secret, copying `installable` to an https
/// cache first (`src/cache:V82`).
pub(crate) fn push_line(
    cache: &BinaryCache,
    branches: Branches,
    command: &str,
    installable: &str,
) -> String {
    match cache.push() {
        Push::Forge => {
            let script = Script::Cachix.path();
            format!("bash {script} {} {}", cache.name(), quote(command))
        }
        Push::Secret(secret) => {
            let line = secret_push(cache, secret, branches, command);
            match cache.kind() {
                CacheKind::Https => format!("{line} {installable}"),
                _ => line,
            }
        }
        _ => command.to_owned(),
    }
}

/// `cache-push.sh <kind> <target> <secret> <branches> <command>`.
fn secret_push(
    cache: &BinaryCache,
    secret: &str,
    branches: Branches,
    command: &str,
) -> String {
    let script = Script::CachePush.path();
    let (kind, branches) = (cache.kind().name(), branches.word());
    let target = target(cache);
    format!(
        "bash {script} {kind} {target} {secret} {branches} {}",
        quote(command)
    )
}

/// The `build` line that asks the cache back for each built system's
/// package (`src/cache:V84`).
pub(crate) fn verify_line(
    cache: &BinaryCache,
    package: &str,
    systems: &[&str],
) -> String {
    let systems = systems.join(" ");
    if cache.kind() == CacheKind::Https {
        let script = Script::HttpsVerify.path();
        let netrc = match cache.push() {
            Push::Secret(secret) => secret.as_str(),
            _ => "-",
        };
        let url = quote(&arg(cache.url()));
        format!("bash {script} {url} {netrc} {package} {systems}")
    } else {
        let script = Script::VerifyCache.path();
        format!("bash {script} {} {package} {systems}", cache.name())
    }
}

/// The seeded scripts reading and pushing to `cache` need, and the one
/// that verifies it when `verify` (`src/render:V30`).
pub(crate) fn scripts(
    cache: Option<&BinaryCache>,
    verify: bool,
) -> Vec<Script> {
    let Some(cache) = cache else {
        return Vec::new();
    };
    let read = read_line(Some(cache)).map(|_| Script::CacheRead);
    let push = match cache.push() {
        Push::Forge => Some(Script::Cachix),
        Push::Secret(_) => Some(Script::CachePush),
        _ => None,
    };
    let check = verify.then(|| match cache.kind() {
        CacheKind::Https => Script::HttpsVerify,
        _ => Script::VerifyCache,
    });
    [read, push, check].into_iter().flatten().collect()
}

/// The secrets reading and pushing to `cache` need (`src/cache:V85`):
/// the push secret, then each read secret.
pub(crate) fn secrets(cache: Option<&BinaryCache>) -> Vec<Secret> {
    let Some(cache) = cache else {
        return Vec::new();
    };
    let read = cache
        .read_secrets()
        .into_iter()
        .map(|name| Secret::new(name, Purpose::CacheRead));
    push_secret(cache).into_iter().chain(read).collect()
}

/// The secret pushing to `cache` needs, if it is pushed.
fn push_secret(cache: &BinaryCache) -> Option<Secret> {
    match cache.push() {
        Push::Forge => Some(Secret::cachix()),
        Push::Secret(name) if cache.kind() == CacheKind::Https => {
            Some(Secret::new(name, Purpose::NetrcPush))
        }
        Push::Secret(name) => Some(Secret::new(name, Purpose::CachePush)),
        _ => None,
    }
}

/// What `cache-push.sh` pushes to: a cachix cache's name, an https
/// cache's url.
fn target(cache: &BinaryCache) -> String {
    match cache.kind() {
        CacheKind::Https => quote(&arg(cache.url())),
        _ => cache.name().to_owned(),
    }
}

/// A setting as a script argument: the literal, or `secret:NAME`.
fn arg(value: &CacheValue) -> String {
    value.literal().map_or_else(
        || format!("{SECRET_PREFIX}{}", value.secret().unwrap_or_default()),
        str::to_owned,
    )
}

#[cfg(test)]
mod tests;
