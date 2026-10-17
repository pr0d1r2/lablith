//! One test per fixture in `tests/fixtures/registry/` (`src/registry:T12`).

use crate::core::ConfigError;
use crate::registry::{Registry, parse};

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            "../../../tests/fixtures/registry/",
            $name,
            ".lablith"
        ))
    };
}

fn refused(text: &str) -> ConfigError {
    parse(text).err().unwrap_or_default()
}

fn ids(reg: &Registry) -> Vec<&str> {
    reg.workflows.iter().map(|w| w.id.as_str()).collect()
}

#[test]
fn full_fills_the_workflows() {
    let reg = parse(fixture!("full")).unwrap_or_default();
    assert_eq!(ids(&reg), ["ci", "update-pins", "check"]);
    let lines: Vec<usize> = reg.workflows.iter().map(|w| w.line).collect();
    assert_eq!(lines, [8, 10, 11]);
    let ci = reg.workflows.first().map(|w| w.systems.len());
    assert_eq!(ci, Some(3));
    let check = reg.workflows.last().and_then(|w| w.args.first());
    let check = check.map(|a| a.value.as_str());
    assert_eq!(check, Some("hk check --all | tee log"));
}

#[test]
fn full_fills_the_gitlab_sections() {
    let reg = parse(fixture!("full")).unwrap_or_default();
    let pin = reg.images.first().map(|i| (i.line, i.version.as_str()));
    assert_eq!(pin, Some((15, "2.28.3")));
    let runners: Vec<(&str, usize)> = reg
        .runners
        .iter()
        .map(|r| (r.system.as_str(), r.tags.len()))
        .collect();
    assert_eq!(runners, [("x86_64-linux", 2), ("aarch64-linux", 2)]);
    assert_eq!(reg.hand_written.len(), 1);
}

#[test]
fn format_only_has_empty_sections() {
    assert_eq!(parse(fixture!("format-only")), Ok(Registry::default()));
}

#[test]
fn no_format_is_refused() {
    let e = refused(fixture!("no-format"));
    assert_eq!((e.line, e.what.as_str()), (1, "no `format` line"));
    assert_eq!(refused("").what, "no `format` line");
    assert_eq!(refused("# only a comment\n\n").what, "no `format` line");
}

#[test]
fn a_newer_format_names_both_versions() {
    let e = refused(fixture!("newer-format"));
    assert_eq!(e.line, 1);
    assert_eq!(e.what, "`.lablith` is format 2; this build reads format 1");
    assert_eq!(e.fix, "upgrade to a release that reads format 2");
}

#[test]
fn duplicate_id_names_both_lines() {
    let e = refused(fixture!("duplicate-id"));
    assert_eq!(e.line, 7);
    assert_eq!(e.what, "id `ci` given twice, first on line 5");
}

#[test]
fn empty_reason_is_refused() {
    let e = refused(fixture!("empty-reason"));
    assert_eq!((e.line, e.what.as_str()), (5, "`reason` is empty"));
}

#[test]
fn bad_cell_count_is_refused() {
    let e = refused(fixture!("bad-cell-count"));
    assert_eq!(e.line, 5);
    assert_eq!(e.what, "4 cells where `## workflows` rows have 5");
}

#[test]
fn bad_header_is_refused() {
    let e = refused(fixture!("bad-header"));
    assert_eq!(e.line, 4);
    assert_eq!(
        e.what,
        "`## workflows` header is `id|kind|systems|gate|cache`"
    );
}

#[test]
fn unknown_section_is_refused_naming_the_known_ones() {
    let e = refused(fixture!("unknown-section"));
    assert_eq!(
        (e.line, e.what.as_str()),
        (3, "unknown section `## checks`")
    );
    for known in ["## images", "## runners", "## hand-written"] {
        assert!(e.fix.contains(known), "{known}: {}", e.fix);
    }
}

#[test]
fn duplicate_section_names_both_lines() {
    let e = refused(fixture!("duplicate-section"));
    assert_eq!(e.line, 7);
    assert_eq!(e.what, "`## runners` given twice, first on line 3");
}

#[test]
fn row_outside_section_is_refused() {
    let e = refused(fixture!("row-outside-section"));
    assert_eq!(e.line, 2);
    assert_eq!(e.what, "row `ci|gate|-|-|-` is outside any section");
}

#[test]
fn a_section_may_be_empty_or_header_only() {
    let reg =
        parse("format 1\n## workflows\n## images\nimage|version|digest\n");
    assert_eq!(reg, Ok(Registry::default()));
}

#[test]
fn a_workflow_row_is_pipelith_raw_row() {
    use std::any::TypeId;
    let ours = TypeId::of::<crate::registry::WorkflowRow>();
    assert_eq!(ours, TypeId::of::<crate::core::kind::RawRow>());
}
