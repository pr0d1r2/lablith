//! `detect` and `seed --init` over temp roots (`.:I`, `src/cli:V34`).

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

use super::{detect, seed};
use crate::cli::exit::Outcome;
use crate::cli::scratch::{git, put, scratch};

const LOCK: &str =
    include_str!("../../../tests/fixtures/detect/with-lock.lock");

/// Either side of a verb's result.
fn ran(result: Result<Outcome, Outcome>) -> Outcome {
    result.unwrap_or_else(|outcome| outcome)
}

/// A git repository tracking `flake.nix` and a lock with `nixpkgs-lock`.
fn flake(name: &str) -> PathBuf {
    let root = scratch(name);
    put(&root, "flake.nix", "{ }\n");
    put(&root, "flake.lock", LOCK);
    assert!(git(&root, &["init", "-q"]));
    assert!(git(&root, &["add", "flake.nix", "flake.lock"]));
    root
}

#[test]
fn detect_prints_one_row_per_line() {
    let root = flake("rows-detect");
    let rows = ran(detect(&root));
    let want = "ci|gate|-|-|-\n# off by default: uncomment to get daily \
                nixpkgs-lock bump MRs\n# update-pins|update-pins|-|-|-\n";
    assert_eq!(rows, Outcome::ok(want));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn seed_leaves_update_pins_off() {
    let root = flake("rows-seed-off");
    assert_eq!(ran(seed(&root, true)).code, 0);
    let text = fs::read_to_string(root.join(".lablith")).unwrap_or_default();
    let on = crate::registry::parse(&text).map(|r| r.workflows.len());
    assert_eq!(on, Ok(1), "only `ci` is live; update-pins is commented");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn detect_outside_a_repository_is_exit_two() {
    let root = scratch("rows-norepo");
    assert_eq!(ran(detect(&root)).code, 2);
    assert_eq!(ran(seed(&root, true)).code, 2);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn seed_writes_once_then_refuses_naming_the_file() {
    let root = flake("rows-seed");
    let first = ran(seed(&root, true));
    assert_eq!(first.code, 0);
    assert!(first.out.starts_with("wrote "));
    let text = fs::read_to_string(root.join(".lablith")).unwrap_or_default();
    assert!(text.contains("ci|gate|-|-|-\n"));
    let again = ran(seed(&root, true));
    assert_eq!(again.code, 1);
    assert!(again.err.contains(".lablith: already exists"));
    assert!(again.err.contains("fix: "));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn seed_without_init_is_a_usage_error() {
    let failed = ran(seed(Path::new("."), false));
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains("fix: run `lablith seed --init`"));
}

#[test]
fn an_unwritable_seed_is_exit_two() {
    let root = flake("rows-unwritable");
    let mode = |m| fs::set_permissions(&root, fs::Permissions::from_mode(m));
    assert!(mode(0o555).is_ok());
    let failed = ran(seed(&root, true));
    assert!(mode(0o755).is_ok());
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains("fix: make the root writable"));
    let _ = fs::remove_dir_all(&root);
}
