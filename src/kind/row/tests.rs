use super::{EXTRAS, check};
use crate::kind::{GATE, Kind, KindError, KindSpec, Row, Secret, Tables};
use crate::pipeline::Job;
use crate::registry::parse;
use crate::script::Script;

/// A kind for the check alone: pipelith's `gate`, no jobs.
struct Probe;

impl Kind for Probe {
    fn spec(&self) -> &'static KindSpec {
        &GATE
    }
    fn secrets(&self, _: &Row) -> Vec<Secret> {
        Vec::new()
    }
    fn scripts(&self, _: &Row) -> Vec<Script> {
        Vec::new()
    }
    fn jobs(&self, _: &Row, _: &Tables) -> Result<Vec<Job>, KindError> {
        Ok(Vec::new())
    }
}

fn row(cells: &str) -> Result<Row, KindError> {
    let text = format!(
        "format 1\n## workflows\nid|kind|systems|cache|args\n{cells}\n"
    );
    let reg = parse(&text).unwrap_or_default();
    let raw = reg.workflows.first().cloned().unwrap_or_default();
    check(&Probe, &raw)
}

#[test]
fn lablith_declares_no_forge_only_kind_or_key() {
    assert!(EXTRAS.kinds.is_empty() && EXTRAS.keys.is_empty());
}

#[test]
fn a_valid_row_is_checked_with_its_defaults() {
    let r = row("ci|gate|-|pr0d1r2|-");
    let got =
        r.map(|r| (r.systems.len(), r.args.get("command").map(str::to_owned)));
    let command = "nix develop --command hk check --all --check".to_owned();
    assert_eq!(got, Ok((3, Some(command))));
}

#[test]
fn an_unknown_key_is_refused_in_lablith_shape() {
    let e = row("ci|gate|-|-|nope=1").err().map(|e| e.to_string());
    let e = e.unwrap_or_default();
    assert!(e.starts_with(".lablith:4: "), "{e}");
    assert!(e.contains("nope"), "{e}");
}
