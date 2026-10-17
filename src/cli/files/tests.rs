//! `gen`, `gen --check`, `check` and `portable` over temp roots (`.:I`,
//! `src/cli:V34`).

use std::fs;

use super::{check, generate, portable};
use crate::cli::exit::Outcome;
use crate::cli::scratch::{put, scratch};

const REPO: &str = include_str!("../../../tests/fixtures/gen/repo.lablith");

/// Either side of a verb's result.
fn ran(result: Result<Outcome, Outcome>) -> Outcome {
    result.unwrap_or_else(|outcome| outcome)
}

/// A root holding the fixture `.lablith`.
fn repo(name: &str) -> std::path::PathBuf {
    let root = scratch(name);
    put(&root, ".lablith", REPO);
    root
}

#[test]
fn gen_writes_and_names_every_file_then_check_passes() {
    let root = repo("files-gen");
    let wrote = ran(generate(&root, false));
    assert_eq!(wrote.code, 0);
    assert!(
        wrote.out.contains("wrote .gitlab-ci.yml\n"),
        "{}",
        wrote.out
    );
    assert!(wrote.out.contains("wrote .gitlab/ci/pkg.yml\n"));
    assert_eq!(ran(generate(&root, true)), Outcome::ok(""));
    assert_eq!(ran(check(&root)), Outcome::ok(""));
    assert_eq!(ran(generate(&root, false)), Outcome::ok(""));
    let _ = fs::remove_dir_all(&root);
}

/// A generated root with `.gitlab/ci/ci.yml` edited and `cachix.sh`
/// deleted.
fn drifted(name: &str) -> std::path::PathBuf {
    let root = repo(name);
    let _ = ran(generate(&root, false));
    put(&root, ".gitlab/ci/ci.yml", "edited\n");
    assert!(fs::remove_file(root.join("scripts/lablith/cachix.sh")).is_ok());
    root
}

#[test]
fn gen_check_names_each_stale_and_missing_file_with_the_fix() {
    let root = drifted("files-stale");
    let stale = ran(generate(&root, true));
    let _ = fs::remove_dir_all(&root);
    assert_eq!(stale.code, 1);
    let edited = "stale: .gitlab/ci/ci.yml; fix: run `lablith gen`";
    assert!(stale.err.contains(edited), "{}", stale.err);
    let gone = "missing: scripts/lablith/cachix.sh; fix: run";
    assert!(stale.err.contains(gone), "{}", stale.err);
}

#[test]
fn gen_check_writes_nothing() {
    let root = drifted("files-nowrite");
    let _ = ran(generate(&root, true));
    let ci = fs::read_to_string(root.join(".gitlab/ci/ci.yml"));
    let _ = fs::remove_dir_all(&root);
    assert_eq!(ci.ok().as_deref(), Some("edited\n"));
}

#[test]
fn check_names_an_undeclared_ci_file_with_the_fix() {
    let root = repo("files-undeclared");
    let _ = ran(generate(&root, false));
    put(&root, ".gitlab/ci/extra.yml", "x: 1\n");
    let found = ran(check(&root));
    assert_eq!(found.code, 1);
    assert!(
        found
            .err
            .starts_with(".gitlab/ci/extra.yml: CI file is neither"),
        "{}",
        found.err
    );
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn portable_passes_every_v1_row() {
    let root = repo("files-portable");
    assert_eq!(ran(portable(&root)), Outcome::ok(""));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_missing_lablith_is_exit_two_naming_seed() {
    let root = scratch("files-missing");
    for result in [generate(&root, false), check(&root), portable(&root)] {
        let failed = ran(result);
        assert_eq!(failed.code, 2);
        assert!(
            failed.err.contains("run `lablith seed --init`"),
            "{}",
            failed.err
        );
    }
    let _ = fs::remove_dir_all(&root);
}
