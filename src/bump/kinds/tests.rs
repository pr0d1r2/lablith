use super::{UpdatePins, Upstream};
use crate::core::kind::Purpose;
use crate::kind::{EXTRAS, Kind, Row, Tables};
use crate::pin::PinTable;
use crate::pipeline::{Job, Stage, row_document};
use crate::registry::parse;
use crate::script::Script;
use crate::system::RunnerTable;
use crate::yaml::GITLAB;

fn row(kind: &dyn Kind, cells: &str) -> Option<Row> {
    let text = format!(
        "format 1\n## workflows\nid|kind|systems|cache|args\n{cells}\n"
    );
    let raw = parse(&text).ok()?.workflows.first().cloned()?;
    Row::check(kind.spec(), &raw, &EXTRAS).ok()
}

fn job(kind: &dyn Kind, cells: &str) -> Option<Job> {
    let tables = Tables {
        pins: PinTable::builtin(),
        runners: RunnerTable::builtin(),
    };
    let jobs = kind.jobs(&row(kind, cells)?, &tables).ok()?;
    jobs.into_iter().next()
}

fn script(job: Option<&Job>) -> Vec<String> {
    job.map(|j| j.script.iter().map(|t| t.render(&GITLAB)).collect())
        .unwrap_or_default()
}

fn var(job: Option<&Job>, key: &str) -> Option<String> {
    let found = job?.variables.iter().find(|(k, _)| *k == key);
    found.map(|(_, v)| v.render(&GITLAB))
}

#[test]
fn update_pins_runs_its_script_in_the_dev_shell_as_one_bump_job() {
    let j = job(&UpdatePins, "pins|update-pins|-|-|-");
    let head = j
        .as_ref()
        .map(|j| (j.name.as_str(), j.stage, j.interruptible));
    assert_eq!(head, Some(("pins:update-pins", Stage::Bump, false)));
    let want =
        "nix develop --command bash scripts/lablith/update-pins.sh '.#default'";
    assert_eq!(script(j.as_ref()), [want]);
    assert_eq!(
        j.and_then(|j| j.resource_group),
        Some("lablith-pins".into())
    );
}

#[test]
fn update_pins_builds_the_attribute_the_row_names() {
    let j = job(&UpdatePins, "pins|update-pins|-|-|build=.#hk");
    let want =
        "nix develop --command bash scripts/lablith/update-pins.sh '.#hk'";
    assert_eq!(script(j.as_ref()), [want]);
}

#[test]
fn a_bump_job_names_its_token_variable_never_its_value() {
    let j = job(&UpdatePins, "pins|update-pins|-|-|-");
    assert_eq!(
        var(j.as_ref(), "LABLITH_TOKEN_NAME"),
        Some("LABLITH_TOKEN".into())
    );
    let j = job(&UpdatePins, "pins|update-pins|-|-|token=BUMP_TOKEN");
    assert_eq!(
        var(j.as_ref(), "LABLITH_TOKEN_NAME"),
        Some("BUMP_TOKEN".into())
    );
    assert_eq!(var(j.as_ref(), "GIT_DEPTH"), Some("0".into()));
}

#[test]
fn a_bump_job_runs_only_on_its_own_schedule_or_web_run() {
    let j = job(&UpdatePins, "pins|update-pins|-|-|-");
    let conds: Vec<String> = j
        .map(|j| {
            j.rules
                .iter()
                .filter_map(|r| r.cond.as_ref())
                .map(|c| c.render(&GITLAB))
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(conds.len(), 2);
    assert!(
        conds.iter().all(|c| c.contains("$LABLITH_ROW == \"pins\"")),
        "{conds:?}"
    );
}

#[test]
fn upstream_runs_the_row_script_with_its_branch_prefix() {
    let j = job(&Upstream, "up|upstream|-|-|script=bump.sh");
    let want = "nix develop --command bash scripts/lablith/upstream.sh 'bump.sh' 'upstream/'";
    assert_eq!(script(j.as_ref()), [want]);
    let j = job(
        &Upstream,
        "up|upstream|-|-|script=bump.sh, branch-prefix=v/",
    );
    assert!(
        script(j.as_ref())
            .first()
            .is_some_and(|s| s.ends_with("'v/'"))
    );
    assert_eq!(j.map(|j| j.name), Some("up:upstream".into()));
}

#[test]
fn update_pins_declares_the_row_token_and_its_scripts() {
    let pins = row(&UpdatePins, "pins|update-pins|-|-|token=T");
    let secret = |r: &Row| {
        UpdatePins
            .secrets(r)
            .into_iter()
            .map(|s| (s.name, s.purpose))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        pins.as_ref().map(secret),
        Some(vec![("T".to_owned(), Purpose::OpenChange)])
    );
    let scripts = pins.map(|r| UpdatePins.scripts(&r));
    assert_eq!(scripts, Some(vec![Script::UpdatePins, Script::OpenMr]));
}

#[test]
fn upstream_declares_the_token_and_its_scripts() {
    let up = row(&Upstream, "up|upstream|-|-|script=b.sh");
    assert_eq!(up.as_ref().map(|r| Upstream.secrets(r).len()), Some(1));
    let scripts = up.map(|r| Upstream.scripts(&r));
    assert_eq!(scripts, Some(vec![Script::Upstream, Script::OpenMr]));
}

/// The row file of `pins|update-pins|-|pr0d1r2|-`, every byte.
const GOLDEN: &str = include_str!("../../../tests/fixtures/bump/pins.yml");

#[test]
fn the_update_pins_row_file_matches_its_golden_bytes() {
    let out = row_document(
        &job(&UpdatePins, "pins|update-pins|-|pr0d1r2|-")
            .into_iter()
            .collect::<Vec<_>>(),
    );
    if std::env::var_os("UPDATE_GOLDEN").is_some() {
        let _ = std::fs::write("tests/fixtures/bump/pins.yml", &out);
    }
    assert_eq!(out, GOLDEN);
}
