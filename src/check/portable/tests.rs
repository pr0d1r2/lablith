use super::{PortFinding, portable};
use crate::core::kind::Finding;
use crate::registry::parse;

#[test]
fn every_v1_row_and_gitlab_section_is_portable() {
    let text = "format 1\n## workflows\nid|kind|systems|cache|args\n\
                ci|gate|-|pr0d1r2|-\npins|update-pins|-|-|token=T\n\
                ## runners\nsystem|tags\nx86_64-linux|big\n";
    let reg = parse(text).unwrap_or_default();
    assert_eq!(portable(&reg), []);
}

const KIND: &str = ".lablith:3: kind `deploy` is GitLab-only; actlith \
    cannot render it\n  fix: use a catalogue kind and its args \
    (docs/KINDS.md)";

#[test]
fn a_kind_finding_names_its_line_and_the_kind() {
    let kind = PortFinding(Finding::Kind {
        line: 3,
        kind: "deploy".into(),
    });
    assert_eq!(kind.to_string(), KIND);
}

#[test]
fn a_key_finding_names_its_line_and_the_key() {
    let key = PortFinding(Finding::Key {
        line: 4,
        key: "env".into(),
    });
    let shown = key.to_string();
    assert!(
        shown.starts_with(".lablith:4: arg `env` is GitLab-only"),
        "{shown}"
    );
}
