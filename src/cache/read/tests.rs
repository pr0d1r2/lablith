use super::{nix_config, read_line};
use crate::cache::scripts;
use crate::core::cache::BinaryCache;
use crate::registry::parse;
use crate::script::Script;

const KEY: &str =
    "example-cache-1:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

/// The `## caches` row `row`, checked by pipelith.
fn cache(row: &str) -> Option<BinaryCache> {
    let text = format!("format 1\n## caches\nname|kind|url|key|push\n{row}\n");
    let reg = parse(&text).ok()?;
    let name = row.split('|').next()?;
    reg.caches.find(name)
}

fn hidden() -> Option<BinaryCache> {
    cache("hidden|https|secret:HIDDEN_URL|secret:HIDDEN_KEY|-")
}

/// An https cache read with its own netrc and pushed with another.
fn private() -> Option<BinaryCache> {
    let push = "read=secret:READ_NETRC, push=secret:PUSH_NETRC";
    cache(&format!(
        "private|https|https://cache.example.org|{KEY}|{push}"
    ))
}

/// `src/cache/read:V81`: a secret setting leaves `NIX_CONFIG` empty and is
/// read by `cache-read.sh`.
#[test]
fn a_secret_setting_is_read_by_cache_read() {
    let c = hidden();
    assert_eq!(nix_config(c.as_ref()), "");
    let line = read_line(c.as_ref()).unwrap_or_default();
    assert!(
        line.contains("cache-read.sh 'secret:HIDDEN_URL' 'secret:HIDDEN_KEY'"),
        "{line}"
    );
    assert_eq!(nix_config(None), "");
}

/// `src/cache/read:V86`: a read netrc keeps the settings out of
/// `NIX_CONFIG` and goes to `cache-read.sh` with the cache's name, on a
/// line that assigns rather than exports, so a refusal stops the job.
#[test]
fn a_read_netrc_is_passed_to_cache_read() {
    let c = private();
    assert_eq!(nix_config(c.as_ref()), "");
    let line = read_line(c.as_ref()).unwrap_or_default();
    let want = format!(
        "NIX_CONFIG=\"$NIX_CONFIG$(bash scripts/lablith/cache-read.sh \
         'https://cache.example.org' '{KEY}' READ_NETRC private)\""
    );
    assert_eq!(line, want);
    assert_eq!(
        scripts(c.as_ref(), false),
        [Script::CacheRead, Script::CachePush]
    );
}

/// `src/cache/read:V81`: a secret setting is read on an assignment too.
#[test]
fn a_secret_setting_is_assigned_not_exported() {
    let line = read_line(hidden().as_ref()).unwrap_or_default();
    assert!(
        line.starts_with("NIX_CONFIG=\"$NIX_CONFIG$(bash "),
        "{line}"
    );
}
