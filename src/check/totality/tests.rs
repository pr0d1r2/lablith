//! `totality` (`src/check:V6`, `src/check:V65`): each finding, and none
//! when total.

use crate::check::{Finding, Problem, totality};
use crate::core::files::File;
use crate::registry::HandWritten;

fn paths(list: &[&str]) -> Vec<String> {
    list.iter().map(|p| (*p).to_owned()).collect()
}

fn rendered(list: &[&str]) -> Vec<File> {
    let file = |p: &&str| File {
        path: (*p).to_owned(),
        bytes: "x\n".to_owned(),
    };
    list.iter().map(file).collect()
}

fn hand(line: usize, path: &str, reason: &str) -> HandWritten {
    HandWritten {
        line,
        path: path.to_owned(),
        reason: reason.to_owned(),
    }
}

fn found(path: &str, line: Option<usize>, problem: Problem) -> Finding {
    Finding {
        path: path.to_owned(),
        line,
        problem,
    }
}

#[test]
fn a_total_repository_has_no_findings() {
    let listing = paths(&[
        ".gitlab-ci.yml",
        ".gitlab/ci/ci.yml",
        ".gitlab/ci/release.yml",
    ]);
    let files = rendered(&[
        ".gitlab-ci.yml",
        ".gitlab/ci/ci.yml",
        "scripts/lablith/cachix.sh",
    ]);
    let hw = [hand(9, ".gitlab/ci/release.yml", "release flow")];
    assert_eq!(totality(&listing, &files, &hw), []);
}

#[test]
fn an_undeclared_ci_file_is_a_finding_at_any_depth() {
    let listing = paths(&[
        ".gitlab-ci.yml",
        ".gitlab/ci/deep/x.yaml",
        ".gitlab/ci/stray.yml",
    ]);
    let want = [
        found(".gitlab-ci.yml", None, Problem::Undeclared),
        found(".gitlab/ci/deep/x.yaml", None, Problem::Undeclared),
        found(".gitlab/ci/stray.yml", None, Problem::Undeclared),
    ];
    assert_eq!(totality(&listing, &[], &[]), want);
}

#[test]
fn a_non_ci_file_is_ignored_and_the_extension_matches_in_any_case() {
    let listing = paths(&[
        ".gitlab/ci/README.md",
        ".gitlab/ci/noext",
        ".gitlab/ci/CI.YML",
    ]);
    let want = [found(".gitlab/ci/CI.YML", None, Problem::Undeclared)];
    assert_eq!(totality(&listing, &[], &[]), want);
}

#[test]
fn a_listed_but_missing_path_is_a_finding() {
    let hw = [hand(4, ".gitlab/ci/gone.yml", "old")];
    let want = [found(".gitlab/ci/gone.yml", Some(4), Problem::Missing)];
    assert_eq!(totality(&[], &[], &hw), want);
}

#[test]
fn a_hand_written_path_also_rendered_is_a_finding() {
    let listing = paths(&[".gitlab/ci/ci.yml"]);
    let files = rendered(&[".gitlab/ci/ci.yml"]);
    let hw = [hand(5, ".gitlab/ci/ci.yml", "mine")];
    let want = [found(".gitlab/ci/ci.yml", Some(5), Problem::AlsoRendered)];
    assert_eq!(totality(&listing, &files, &hw), want);
}

#[test]
fn a_hand_written_row_without_reason_is_a_finding() {
    let listing = paths(&[".gitlab/ci/r.yml"]);
    let hw = [hand(6, ".gitlab/ci/r.yml", " ")];
    let want = [found(".gitlab/ci/r.yml", Some(6), Problem::NoReason)];
    assert_eq!(totality(&listing, &[], &hw), want);
}

#[test]
fn findings_are_sorted_by_path() {
    let listing = paths(&[".gitlab/ci/b.yml"]);
    let hw = [
        hand(3, ".gitlab/ci/z.yml", "x"),
        hand(2, ".gitlab/ci/a.yml", "x"),
    ];
    let got: Vec<String> = totality(&listing, &[], &hw)
        .into_iter()
        .map(|f| f.path)
        .collect();
    assert_eq!(
        got,
        [".gitlab/ci/a.yml", ".gitlab/ci/b.yml", ".gitlab/ci/z.yml"]
    );
}
