use super::md;
use crate::report::model::Report;

#[test]
fn an_empty_report_says_none_per_table() {
    let out = md(&Report::default());
    assert_eq!(out.matches("None.\n").count(), 4, "{out}");
    assert!(!out.contains("## Token"), "{out}");
}
