//! `report` over `tests/fixtures/report/repo.lablith`, every format
//! against its golden file (`UPDATE_GOLDEN=1` rewrites them).

use super::report;
use crate::registry::parse;
use crate::report::Format;

const REPO: &str = include_str!("../../../tests/fixtures/report/repo.lablith");

fn golden(format: Format, path: &str, want: &str) {
    let reg = parse(REPO).unwrap_or_default();
    let out = report(&reg, format).unwrap_or_default();
    if std::env::var_os("UPDATE_GOLDEN").is_some() {
        let _ = std::fs::write(path, &out);
    }
    assert_eq!(out, want);
}

#[test]
fn text_matches_its_golden_file() {
    let want = include_str!("../../../tests/fixtures/report/report.txt");
    golden(Format::Text, "tests/fixtures/report/report.txt", want);
}

#[test]
fn json_matches_its_golden_file() {
    let want = include_str!("../../../tests/fixtures/report/report.json");
    golden(Format::Json, "tests/fixtures/report/report.json", want);
}

#[test]
fn md_matches_its_golden_file() {
    let want = include_str!("../../../tests/fixtures/report/report.md");
    golden(Format::Md, "tests/fixtures/report/report.md", want);
}

#[test]
fn a_refused_row_is_an_error() {
    let text =
        "format 1\n## workflows\nid|kind|systems|cache|args\nx|lint|-|-|-\n";
    let reg = parse(text).unwrap_or_default();
    assert!(report(&reg, Format::Text).is_err());
}
