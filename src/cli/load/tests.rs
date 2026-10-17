//! `.lablith` loading: missing and refused files are exit 2 with a fix
//! (`src/registry:V29`, `src/cli:V34`).

use std::fs;

use super::{registry, rendered};
use crate::cli::scratch::{put, scratch};

const REPO: &str = include_str!("../../../tests/fixtures/gen/repo.lablith");

#[test]
fn a_missing_file_is_exit_two_naming_seed() {
    let root = scratch("load-missing");
    let failed = registry(&root).err().unwrap_or_default();
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains(".lablith: not found"));
    assert!(failed.err.contains("fix: run `lablith seed --init`"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn an_unreadable_file_is_exit_two() {
    let root = scratch("load-dir");
    assert!(fs::create_dir_all(root.join(".lablith")).is_ok());
    let failed = registry(&root).err().unwrap_or_default();
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains("cannot read"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_refused_row_is_exit_two_with_its_line() {
    let root = scratch("load-refused");
    put(&root, ".lablith", "format 9\n");
    let failed = registry(&root).err().unwrap_or_default();
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains(".lablith:1:"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn an_unknown_kind_is_refused_at_render() {
    let root = scratch("load-kind");
    let rows = "format 1\n\n## workflows\nid|kind|systems|cache|args\n";
    put(&root, ".lablith", &format!("{rows}x|nope|-|-|-\n"));
    let failed = rendered(&root).err().unwrap_or_default();
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains("fix:"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_good_file_renders_its_rows() {
    let root = scratch("load-good");
    put(&root, ".lablith", REPO);
    let files = rendered(&root).map(|(_, files)| files).unwrap_or_default();
    assert!(files.iter().any(|f| f.path == ".gitlab/ci/ci.yml"));
    let _ = fs::remove_dir_all(&root);
}
