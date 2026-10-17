use std::fs;

use super::report;
use crate::cli::scratch::{put, scratch};
use crate::report::Format;

const REPO: &str = include_str!("../../../tests/fixtures/report/repo.lablith");
const JSON: &str = include_str!("../../../tests/fixtures/report/report.json");

#[test]
fn report_prints_the_registry_in_the_format_asked() {
    let root = scratch("report-json");
    put(&root, ".lablith", REPO);
    let out = report(&root, Format::Json).map(|o| o.out);
    let _ = fs::remove_dir_all(&root);
    assert_eq!(out, Ok(JSON.to_owned()));
}

#[test]
fn a_refused_row_is_exit_two() {
    let root = scratch("report-bad");
    put(
        &root,
        ".lablith",
        "format 1\n## workflows\nid|kind|systems|cache|args\nx|lint|-|-|-\n",
    );
    let code = report(&root, Format::Text).err().map(|o| o.code);
    let _ = fs::remove_dir_all(&root);
    assert_eq!(code, Some(2));
}
