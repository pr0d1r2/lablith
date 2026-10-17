//! `listing` in temp dirs: the root file and every file under
//! `.gitlab/ci/`, sorted.

use std::fs;
use std::path::{Path, PathBuf};

use crate::check::listing;

fn scratch(name: &str) -> PathBuf {
    let id = std::process::id();
    let dir = std::env::temp_dir().join(format!("lablith-check-{id}-{name}"));
    let _ = fs::remove_dir_all(&dir);
    assert!(fs::create_dir_all(&dir).is_ok());
    dir
}

fn touch(root: &Path, path: &str) {
    let file = root.join(path);
    if let Some(parent) = file.parent() {
        assert!(fs::create_dir_all(parent).is_ok());
    }
    assert!(fs::write(file, "x\n").is_ok());
}

/// What `lists_the_root_and_every_file_under_gitlab_ci_sorted` writes.
const WRITTEN: [&str; 5] = [
    ".gitlab/ci/z.yml",
    ".gitlab/ci/deep/a.yml",
    ".gitlab-ci.yml",
    "README.md",
    ".gitlab/issue_templates/x.md",
];

/// What it lists, sorted.
const LISTED: [&str; 3] = [
    ".gitlab-ci.yml",
    ".gitlab/ci/deep/a.yml",
    ".gitlab/ci/z.yml",
];

#[test]
fn lists_the_root_and_every_file_under_gitlab_ci_sorted() {
    let dir = scratch("all");
    for p in WRITTEN {
        touch(&dir, p);
    }
    let got = listing(&dir).unwrap_or_default();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(got, LISTED);
}

#[test]
fn nothing_there_lists_nothing() {
    let dir = scratch("empty");
    let got = listing(&dir).ok();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(got, Some(Vec::new()));
}

#[test]
fn an_unreadable_ci_path_is_an_error() {
    let dir = scratch("notdir");
    touch(&dir, ".gitlab/ci");
    let got = listing(&dir);
    let _ = fs::remove_dir_all(&dir);
    assert!(got.is_err(), "{got:?}");
}
