use super::{Depth, NixJob, nix_job};
use crate::core::kind::Row;
use crate::core::yaml::Text;
use crate::kind::{EXTRAS, GATE, KindError, Tables, change_rules};
use crate::pin::PinTable;
use crate::pipeline::{Job, Stage};
use crate::registry::parse;
use crate::system::{RunnerTable, System, SystemError};
use crate::yaml::GITLAB;

fn tables() -> Tables {
    Tables {
        pins: PinTable::builtin(),
        runners: RunnerTable::builtin(),
    }
}

fn row(cells: &str) -> Option<Row> {
    let text = format!(
        "format 1\n## workflows\nid|kind|systems|cache|args\n{cells}\n"
    );
    let raw = parse(&text).ok()?.workflows.first().cloned()?;
    Row::check(&GATE, &raw, &EXTRAS).ok()
}

fn spec(stage: Stage, system: System) -> NixJob<'static> {
    NixJob {
        name: "gate:x86_64-linux".into(),
        stage,
        system,
        depth: Depth::Full,
        rules: change_rules(),
        script: vec![Text::lit("true")],
        why: Some("why"),
    }
}

fn job(cells: &str, stage: Stage, system: System) -> Result<Job, KindError> {
    let row = row(cells).ok_or_else(|| KindError::row(0, "no row", "-"))?;
    nix_job(&row, &tables(), spec(stage, system))
}

fn vars(job: &Job) -> Vec<(&'static str, String)> {
    job.variables
        .iter()
        .map(|(k, v)| (*k, v.render(&GITLAB)))
        .collect()
}

#[test]
fn a_job_is_named_after_its_row_and_runs_the_pinned_nix_image() {
    let j = job("ci|gate|-|-|timeout=12", Stage::Gate, System::X86_64Linux);
    let got =
        j.map(|j| (j.name, j.image.name, j.tags, j.timeout, j.interruptible));
    let tags = vec!["nix".to_owned(), "x86_64-linux".to_owned()];
    assert_eq!(
        got,
        Ok((
            "ci:gate:x86_64-linux".into(),
            "nixos/nix".into(),
            tags,
            12,
            true
        ))
    );
}

#[test]
fn a_cached_row_adds_its_substituter_to_nix_config() {
    let j = job("ci|gate|-|pr0d1r2|-", Stage::Gate, System::X86_64Linux);
    let got = j.map(|j| {
        vars(&j)
            .into_iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
    });
    let config = "NIX_CONFIG=experimental-features = nix-command flakes\n\
                  extra-substituters = https://pr0d1r2.cachix.org\n\
                  extra-trusted-public-keys = \
                  pr0d1r2.cachix.org-1:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=\n";
    assert_eq!(got, Ok(vec!["GIT_DEPTH=0".to_owned(), config.to_owned()]));
}

#[test]
fn an_uncached_shallow_job_sets_flakes_and_depth_one() {
    let row = row("ci|gate|-|-|-");
    let shallow = NixJob {
        depth: Depth::Shallow,
        ..spec(Stage::Build, System::X86_64Linux)
    };
    let j = row.map(|r| nix_job(&r, &tables(), shallow));
    let got = j.map(|j| j.map(|j| vars(&j)));
    let flakes = "experimental-features = nix-command flakes\n".to_owned();
    assert_eq!(
        got,
        Some(Ok(vec![("GIT_DEPTH", "1".into()), ("NIX_CONFIG", flakes)]))
    );
}

#[test]
fn a_bump_job_is_not_interruptible_and_takes_the_row_resource_group() {
    let j = job("pins|gate|-|-|-", Stage::Bump, System::X86_64Linux);
    let got = j.map(|j| (j.interruptible, j.resource_group));
    assert_eq!(got, Ok((false, Some("lablith-pins".into()))));
}

#[test]
fn a_system_with_no_tags_is_refused_naming_the_fix() {
    let j = job("ci|gate|-|-|-", Stage::Gate, System::X86_64Darwin);
    assert_eq!(
        j,
        Err(KindError::System(SystemError::NoTags(System::X86_64Darwin)))
    );
}
