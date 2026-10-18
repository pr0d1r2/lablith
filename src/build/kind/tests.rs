use super::Build;
use crate::kind::{EXTRAS, Kind, Row, Tables};
use crate::pin::PinTable;
use crate::pipeline::{Job, Stage, row_document};
use crate::registry::{RunnerRow, parse};
use crate::script::Script;
use crate::system::RunnerTable;
use crate::yaml::GITLAB;

fn row(cells: &str) -> Option<Row> {
    let text = format!(
        "format 1\n## workflows\nid|kind|systems|cache|args\n{cells}\n"
    );
    let raw = parse(&text).ok()?.workflows.first().cloned()?;
    Row::check(Build.spec(), &raw, &EXTRAS).ok()
}

fn tables(extra: &[RunnerRow]) -> Tables {
    let runners = RunnerTable::builtin().with_overrides(extra);
    Tables {
        pins: PinTable::builtin(),
        runners: runners.unwrap_or_else(|_| RunnerTable::builtin()),
    }
}

fn jobs(cells: &str, extra: &[RunnerRow]) -> Vec<Job> {
    let t = tables(extra);
    row(cells)
        .and_then(|r| Build.jobs(&r, &t).ok())
        .unwrap_or_default()
}

const ROW: &str = "pkg|build|x86_64-linux, aarch64-linux|pr0d1r2|package=hk";

fn script(job: Option<&Job>) -> Vec<String> {
    job.map(|j| j.script.iter().map(|t| t.render(&GITLAB)).collect())
        .unwrap_or_default()
}

#[test]
fn a_row_builds_each_system_then_checks_then_verifies() {
    let got: Vec<(String, Stage)> = jobs(ROW, &[])
        .into_iter()
        .map(|j| (j.name, j.stage))
        .collect();
    let want = [
        ("pkg:build:x86_64-linux".to_owned(), Stage::Build),
        ("pkg:build:aarch64-linux".to_owned(), Stage::Build),
        ("pkg:check".to_owned(), Stage::Build),
        ("pkg:verify-cache".to_owned(), Stage::Verify),
    ];
    assert_eq!(got, want);
}

#[test]
fn a_build_runs_through_cachix_sh() {
    let all = jobs(ROW, &[]);
    let want = "bash scripts/lablith/cachix.sh pr0d1r2 'nix build \
                --print-build-logs .#packages.x86_64-linux.hk'";
    assert_eq!(script(all.first()), [want]);
}

#[test]
fn check_evaluates_the_systems_without_tags() {
    let all = jobs(ROW, &[]);
    let want = [
        "nix flake check --all-systems --print-build-logs",
        "nix eval --raw .#packages.x86_64-darwin.hk.drvPath",
    ];
    assert_eq!(script(all.get(2)), want);
}

#[test]
fn a_system_given_tags_is_no_longer_evaluated_only() {
    let mac = RunnerRow {
        line: 1,
        system: "x86_64-darwin".into(),
        tags: vec!["mac".into()],
    };
    let all = jobs(ROW, &[mac]);
    assert_eq!(script(all.get(2)).len(), 1);
}

#[test]
fn verify_cache_needs_every_build_and_runs_on_the_default_branch() {
    let all = jobs(ROW, &[]);
    let verify = all.last();
    let needs = verify.map(|j| j.needs.clone()).unwrap_or_default();
    assert_eq!(needs, ["pkg:build:x86_64-linux", "pkg:build:aarch64-linux"]);
    let want = "bash scripts/lablith/verify-cache.sh pr0d1r2 hk x86_64-linux aarch64-linux";
    assert_eq!(script(verify), [want]);
}

#[test]
fn build_declares_the_cachix_secret_and_its_two_scripts() {
    let r = row(ROW);
    assert_eq!(r.as_ref().map(|r| Build.secrets(r).len()), Some(1));
    assert_eq!(
        r.map(|r| Build.scripts(&r)),
        Some(vec![Script::Cachix, Script::VerifyCache])
    );
}

/// The row file of `ROW`, every byte.
const GOLDEN: &str = include_str!("../../../tests/fixtures/build/pkg.yml");

#[test]
fn the_row_file_matches_its_golden_bytes() {
    let out = row_document(&jobs(ROW, &[]));
    if std::env::var_os("UPDATE_GOLDEN").is_some() {
        let _ = std::fs::write("tests/fixtures/build/pkg.yml", &out);
    }
    assert_eq!(out, GOLDEN);
}

#[test]
fn a_row_without_package_or_cache_is_refused_naming_it() {
    let bare = Row {
        line: 6,
        id: "pkg".into(),
        systems: Vec::new(),
        cache: None,
        args: crate::kind::Args::default(),
    };
    let e = Build.jobs(&bare, &tables(&[])).err().map(|e| e.to_string());
    let e = e.unwrap_or_default();
    assert!(
        e.starts_with(".lablith:6: build needs package and cache"),
        "{e}"
    );
}

fn skip(system: &str) -> RunnerRow {
    RunnerRow {
        line: 1,
        system: system.into(),
        tags: Vec::new(),
    }
}

/// `src/system:V72`: a skipped system is evaluated, not built, and the
/// cache is not asked for it.
#[test]
fn a_skipped_system_is_evaluated_and_not_verified() {
    let all = jobs(ROW, &[skip("aarch64-linux")]);
    let names: Vec<&str> = all.iter().map(|j| j.name.as_str()).collect();
    assert_eq!(
        names,
        ["pkg:build:x86_64-linux", "pkg:check", "pkg:verify-cache"]
    );
    let check = script(all.get(1));
    let eval = "nix eval --raw .#packages.aarch64-linux.hk.drvPath";
    assert!(check.iter().any(|l| l == eval), "{check:?}");
    let verify = script(all.get(2)).concat();
    assert!(!verify.contains("aarch64-linux"), "{verify}");
}

/// `src/system:V72`: system-free jobs run on `x86_64-linux`, so skipping
/// it refuses the row rather than rendering a job no runner takes.
#[test]
fn skipping_the_system_free_runner_refuses_the_row() {
    let t = tables(&[skip("x86_64-linux")]);
    let r = row("pkg|build|aarch64-linux|pr0d1r2|package=hk");
    let e = r.map(|r| Build.jobs(&r, &t).err().map(|e| e.to_string()));
    let e = e.flatten().unwrap_or_default();
    assert!(e.contains("`x86_64-linux` is skipped"), "{e}");
}
