use super::text;
use crate::report::model::Report;

#[test]
fn an_empty_report_shows_a_dash_per_section() {
    assert_eq!(text(&Report::default()), "rows\n  -\nhand-written\n  -\n");
}
