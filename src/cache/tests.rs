use super::{
    Branches, nix_config, push_line, read_line, scripts, secrets, verify_line,
};
use crate::core::cache::{BinaryCache, builtin};
use crate::kind::Purpose;
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

/// `src/cache:V80`: the built-in cache keeps its nix config and its
/// cachix line.
#[test]
fn the_built_in_cache_renders_as_before() {
    let c = builtin("pr0d1r2");
    assert!(
        nix_config(c.as_ref())
            .starts_with("extra-substituters = https://pr0d1r2.cachix.org\n")
    );
    assert_eq!(read_line(c.as_ref()), None);
    let line = c.map(|c| push_line(&c, Branches::Any, "true", "-"));
    assert_eq!(
        line.as_deref(),
        Some("bash scripts/lablith/cachix.sh pr0d1r2 'true'")
    );
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

/// `src/cache:V82`: a read-only cache adds no push and no push secret.
#[test]
fn a_read_only_cache_runs_the_command_alone() {
    let c = hidden();
    let line = c
        .as_ref()
        .map(|c| push_line(c, Branches::Default, "true", "-"));
    assert_eq!(line.as_deref(), Some("true"));
    let needs: Vec<Purpose> =
        secrets(c.as_ref()).into_iter().map(|s| s.purpose).collect();
    assert_eq!(needs, [Purpose::CacheRead, Purpose::CacheRead]);
    assert_eq!(scripts(c.as_ref(), false), [Script::CacheRead]);
}

/// `src/cache:V84`: a read-only https cache is verified without a netrc.
#[test]
fn a_read_only_https_cache_is_verified_without_a_netrc() {
    let row = format!("pub|https|https://cache.example.org|{KEY}|-");
    let line = cache(&row).map(|c| verify_line(&c, "hk", &["x86_64-linux"]));
    let want = "bash scripts/lablith/https-verify.sh 'https://cache.example.org' - hk x86_64-linux";
    assert_eq!(line.as_deref(), Some(want));
}

/// `src/cache:V85`: no cache needs no secret and no script.
#[test]
fn no_cache_needs_nothing() {
    assert!(secrets(None).is_empty());
    assert!(scripts(None, true).is_empty());
}

/// An https cache read with its own netrc and pushed with another.
fn private() -> Option<BinaryCache> {
    let push = "read=secret:READ_NETRC, push=secret:PUSH_NETRC";
    cache(&format!(
        "private|https|https://cache.example.org|{KEY}|{push}"
    ))
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

/// `src/cache/read:V86`: the push keeps its own netrc, and verify reads
/// with the read one.
#[test]
fn the_push_netrc_stays_apart_from_the_read_netrc() {
    let c = private();
    let push = c
        .as_ref()
        .map(|c| push_line(c, Branches::Any, "true", "-"))
        .unwrap_or_default();
    assert!(push.contains(" PUSH_NETRC any "), "{push}");
    assert!(!push.contains("READ_NETRC"), "{push}");
    let verify = c
        .as_ref()
        .map(|c| verify_line(c, "hk", &["x86_64-linux"]))
        .unwrap_or_default();
    assert!(verify.contains(" READ_NETRC hk "), "{verify}");
}

/// `src/cache/read:V86`: `report` lists the read netrc as one.
#[test]
fn the_read_netrc_is_reported_as_a_netrc() {
    let needs: Vec<(String, &str)> = secrets(private().as_ref())
        .into_iter()
        .map(|s| (s.name.clone(), s.why()))
        .collect();
    let want = [
        ("PUSH_NETRC".to_owned(), "https cache push (netrc file)"),
        ("READ_NETRC".to_owned(), "https cache read (netrc file)"),
    ];
    assert_eq!(needs, want);
}
