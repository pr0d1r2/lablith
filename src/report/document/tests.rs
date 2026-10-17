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
