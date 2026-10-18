use super::{SCHEMA_VERSION, document};
use crate::report::json::{Json, write};
use crate::report::model::Report;

#[test]
fn an_empty_report_leads_with_its_schema_version() {
    assert_eq!(SCHEMA_VERSION, 1);
    let out = write(&document(&Report::default()));
    assert_eq!(
        out,
        "{\n  \"schema_version\": 1,\n  \"rows\": [],\n  \"hand_written\": [],\n  \"token\": null\n}\n"
    );
    assert!(matches!(document(&Report::default()), Json::Object(_)));
}

/// A report of one gate row on two systems, `aarch64-linux` skipped.
fn skipped() -> Report {
    let text = "format 1\n## workflows\nid|kind|systems|cache|args\n\
                ci|gate|x86_64-linux, aarch64-linux|-|-\n## runners\n\
                system|tags\naarch64-linux|-\n";
    let reg = crate::registry::parse(text).unwrap_or_default();
    crate::report::collect::collect(&reg).unwrap_or_default()
}

/// `src/system:V72`: each system carries `skipped`; an added field, so
/// `schema_version` stays (`src/report:V36`).
#[test]
fn a_skipped_system_carries_skipped_true() {
    let out = write(&document(&skipped()));
    let want = "\"system\": \"aarch64-linux\",\n          \"tags\": [],\n          \"skipped\": true";
    assert!(out.contains(want), "{out}");
    assert!(out.contains("\"skipped\": false"), "{out}");
}
