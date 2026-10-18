use super::text;
use crate::report::model::Report;

#[test]
fn an_empty_report_shows_a_dash_per_section() {
    assert_eq!(text(&Report::default()), "rows\n  -\nhand-written\n  -\n");
}

/// A report of one gate row on two systems, `aarch64-linux` skipped.
fn skipped() -> Report {
    let text = "format 1\n## workflows\nid|kind|systems|cache|args\n\
                ci|gate|x86_64-linux, aarch64-linux|-|-\n## runners\n\
                system|tags\naarch64-linux|-\n";
    let reg = crate::registry::parse(text).unwrap_or_default();
    crate::report::collect::collect(&reg).unwrap_or_default()
}

/// `src/system:V72`: a skipped system says so instead of listing tags.
#[test]
fn a_skipped_system_reads_as_skipped() {
    let out = text(&skipped());
    assert!(out.contains("x86_64-linux tags nix, x86_64-linux"), "{out}");
    assert!(out.contains("aarch64-linux skipped: no runner"), "{out}");
}
