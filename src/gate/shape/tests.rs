use super::Gate;
use crate::core::kind::Purpose;
use crate::kind::{EXTRAS, Kind, Row, Tables};
use crate::pin::PinTable;
use crate::pipeline::{Job, row_document};
use crate::registry::parse;
use crate::script::Script;
use crate::system::RunnerTable;
use crate::yaml::GITLAB;

fn row(cells: &str) -> Option<Row> {
    let text = format!(
        "format 1\n## workflows\nid|kind|systems|cache|args\n{cells}\n"
    );
    let raw = parse(&text).ok()?.workflows.first().cloned()?;
    Row::check(Gate.spec(), &raw, &EXTRAS).ok()
}

fn jobs(cells: &str) -> Vec<Job> {
    let tables = Tables {
        pins: PinTable::builtin(),
        runners: RunnerTable::builtin(),
    };
    row(cells)
        .and_then(|r| Gate.jobs(&r, &tables).ok())
        .unwrap_or_default()
}

fn scripts(jobs: &[Job]) -> Vec<String> {
    jobs.iter()
        .flat_map(|j| j.script.iter().map(|t| t.render(&GITLAB)))
        .collect()
}

#[test]
fn a_default_row_gates_every_default_system() {
    let names: Vec<String> =
        jobs("ci|gate|-|-|-").into_iter().map(|j| j.name).collect();
    assert_eq!(
        names,
        [
            "ci:gate:x86_64-linux",
            "ci:gate:aarch64-linux",
            "ci:gate:aarch64-darwin"
        ]
    );
}

#[test]
fn without_a_cache_the_command_runs_alone() {
    let got = scripts(&jobs("ci|gate|x86_64-linux|-|-"));
    assert_eq!(got, ["nix develop --command hk check --all --check"]);
    let row = row("ci|gate|-|-|-");
    let none =
        row.map(|r| (Gate.secrets(&r).is_empty(), Gate.scripts(&r).is_empty()));
    assert_eq!(none, Some((true, true)));
}

#[test]
fn with_a_cache_the_command_runs_through_cachix_sh_as_one_word() {
    let got = scripts(&jobs(
        "ci|gate|x86_64-linux|pr0d1r2|command=hk check \\| tee log",
    ));
    let want = "bash scripts/lablith/cachix.sh pr0d1r2 'hk check | tee log'";
    assert_eq!(got, [want]);
}

#[test]
fn a_cached_row_declares_the_cachix_secret_and_script() {
    let row = row("ci|gate|-|pr0d1r2|-");
    let secrets = row.as_ref().map(|r| {
        Gate.secrets(r)
            .into_iter()
            .map(|s| s.purpose)
            .collect::<Vec<_>>()
    });
    assert_eq!(secrets, Some(vec![Purpose::CachePush]));
    assert_eq!(row.map(|r| Gate.scripts(&r)), Some(vec![Script::Cachix]));
}

/// The row file of `ci|gate|x86_64-linux|pr0d1r2|-`, every byte.
const GOLDEN: &str = include_str!("../../../tests/fixtures/gate/ci.yml");

#[test]
fn the_row_file_matches_its_golden_bytes() {
    let out = row_document(&jobs("ci|gate|x86_64-linux|pr0d1r2|-"));
    if std::env::var_os("UPDATE_GOLDEN").is_some() {
        let _ = std::fs::write("tests/fixtures/gate/ci.yml", &out);
    }
    assert_eq!(out, GOLDEN);
}
