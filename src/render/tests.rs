//! The hardening rules on the job model of every kind rendered from
//! every valid fixture (`tests:V62`, `tests:T51`), and the move to
//! actlith (`tests:V63`): one test per rule, over parsed structures,
//! never over grepped text.

use crate::core::kind::{self, Extras, RawRow, Row};
use crate::core::registry::{self, Schema, SectionSpec};
use crate::pipeline::{Job, Stage};
use crate::registry::parse;
use crate::render::{RowPlan, plan, render};
use crate::yaml::GITLAB;

/// Every fixture that renders.
const FIXTURES: [&str; 4] = [
    include_str!("../../tests/fixtures/gen/repo.lablith"),
    include_str!("../../tests/fixtures/report/repo.lablith"),
    include_str!("../../tests/fixtures/registry/full.lablith"),
    include_str!("../../tests/fixtures/fetch/full.lablith"),
];

/// Every row plan of every fixture.
fn plans() -> Vec<RowPlan> {
    let one = |text: &&str| parse(text).ok().and_then(|r| plan(&r).ok());
    FIXTURES.iter().filter_map(one).flatten().collect()
}

/// Every job of every plan, with the plan's row id.
fn jobs() -> Vec<(String, Job)> {
    let each =
        |p: RowPlan| p.jobs.into_iter().map(move |j| (p.row.id.clone(), j));
    plans().into_iter().flat_map(each).collect()
}

fn var(job: &Job, key: &str) -> Option<String> {
    job.variables
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| v.render(&GITLAB))
}

fn conds(job: &Job) -> Vec<String> {
    job.rules
        .iter()
        .filter_map(|r| r.cond.as_ref())
        .map(|c| c.render(&GITLAB))
        .collect()
}

#[test]
fn every_fixture_renders_and_every_kind_is_covered() {
    let planned = FIXTURES
        .iter()
        .filter(|t| parse(t).is_ok_and(|r| plan(&r).is_ok()));
    assert_eq!(planned.count(), FIXTURES.len());
    let kinds: std::collections::BTreeSet<&str> =
        plans().iter().map(|p| p.kind.spec().name).collect();
    let want = ["build", "gate", "update-pins", "upstream"];
    assert_eq!(kinds.into_iter().collect::<Vec<_>>(), want);
}

#[test]
fn v9_every_job_name_carries_its_row_prefix() {
    for (id, job) in jobs() {
        assert!(job.name.starts_with(&format!("{id}:")), "{}", job.name);
    }
}

#[test]
fn v2_every_image_is_pinned_by_digest() {
    for (_, job) in jobs() {
        let reference = job.image.reference();
        let digest = reference.split_once("@sha256:").map(|(_, d)| d.len());
        assert_eq!(digest, Some(64), "{reference}");
    }
}

#[test]
fn v11_every_job_has_a_timeout_in_range() {
    for (_, job) in jobs() {
        assert!(
            (1..=360).contains(&job.timeout),
            "{}: {}",
            job.name,
            job.timeout
        );
    }
}

#[test]
fn v12_change_jobs_are_interruptible_and_bumps_are_grouped_instead() {
    for (id, job) in jobs() {
        let bump = job.stage == Stage::Bump;
        assert_eq!(job.interruptible, !bump, "{}", job.name);
        let group = bump.then(|| format!("lablith-{id}"));
        assert_eq!(job.resource_group, group, "{}", job.name);
    }
}

#[test]
fn v13_every_job_sets_its_clone_depth() {
    for (_, job) in jobs() {
        let want = if job.stage == Stage::Gate || job.stage == Stage::Bump {
            "0"
        } else {
            "1"
        };
        assert_eq!(
            var(&job, "GIT_DEPTH").as_deref(),
            Some(want),
            "{}",
            job.name
        );
    }
}

#[test]
fn v13_no_script_line_prints_a_variable() {
    for (_, job) in jobs() {
        for line in job.script.iter().map(|t| t.render(&GITLAB)) {
            assert!(!(line.contains("echo") && line.contains('$')), "{line}");
        }
    }
}

#[test]
fn v14_a_cache_push_only_happens_through_cachix_sh() {
    for (_, job) in jobs() {
        for line in job.script.iter().map(|t| t.render(&GITLAB)) {
            let pushes =
                line.contains("cachix push") || line.contains("watch-");
            assert!(!pushes, "{line}");
        }
    }
}

#[test]
fn v15_v50_jobs_reading_a_token_run_only_on_their_own_protected_schedule() {
    for (id, job) in jobs().into_iter().filter(|(_, j)| j.stage == Stage::Bump)
    {
        let all = conds(&job);
        assert_eq!(all.len(), 2, "{}", job.name);
        for c in all {
            assert!(c.contains(&format!("$LABLITH_ROW == \"{id}\"")), "{c}");
            assert!(c.contains("$CI_COMMIT_REF_PROTECTED == \"true\""), "{c}");
        }
    }
}

#[test]
fn v50_change_jobs_never_run_on_a_schedule() {
    for (_, job) in jobs().into_iter().filter(|(_, j)| j.stage != Stage::Bump) {
        let first = job
            .rules
            .first()
            .and_then(|r| r.cond.as_ref())
            .map(|c| c.render(&GITLAB));
        assert_eq!(
            first.as_deref(),
            Some("$CI_PIPELINE_SOURCE == \"schedule\""),
            "{}",
            job.name
        );
    }
}

#[test]
fn v16_every_script_entry_is_one_line() {
    for (_, job) in jobs() {
        assert!(
            job.script.iter().all(|t| !t.render(&GITLAB).contains('\n')),
            "{}",
            job.name
        );
    }
}

/// Keys and YAML forms a generated file never holds (`src/pipeline:V7`,
/// `src/pipeline:V8`).
const BANNED: [&str; 7] = [
    "remote:",
    "project:",
    "template:",
    "component:",
    "extends:",
    "<<:",
    "&",
];

/// Every line of every rendered pipeline file of every fixture.
fn rendered_lines() -> Vec<(String, String)> {
    let files = FIXTURES
        .iter()
        .filter_map(|t| parse(t).ok().and_then(|r| render(&r).ok()));
    let yml = files
        .flatten()
        .filter(|f| f.path.to_ascii_lowercase().ends_with(".yml"));
    let lines = |f: crate::core::files::File| {
        let path = f.path.clone();
        f.bytes
            .lines()
            .map(move |l| (path.clone(), l.trim_start().to_owned()))
            .collect::<Vec<_>>()
    };
    yml.flat_map(lines).collect()
}

#[test]
fn v7_v8_files_include_only_local_and_expand_every_job() {
    for (path, line) in rendered_lines() {
        let key = line.trim_start_matches("- ");
        let hit = BANNED.iter().find(|b| {
            key.starts_with(*b) || key.contains(": &") || key.contains(": *")
        });
        assert!(hit.is_none(), "{path}: {line}");
    }
}

/// actlith's section set: pipelith's `## workflows` and `## hand-written`.
const ACTLITH: Schema<'static> = Schema {
    sections: &[
        kind::WORKFLOWS,
        SectionSpec {
            name: "hand-written",
            columns: &["path", "reason"],
            required: false,
            key: Some(0),
        },
    ],
};

/// `text` without its GitLab-only sections.
fn without_gitlab_sections(text: &str) -> String {
    let mut out = String::new();
    let mut skip = false;
    for line in text.lines() {
        if line.starts_with("## ") {
            skip = matches!(line.trim(), "## images" | "## runners");
        }
        if !skip {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// `text`'s rows as actlith would read them, GitLab sections dropped.
fn as_actlith(text: &str) -> Vec<RawRow> {
    let stripped = without_gitlab_sections(text);
    let doc = registry::parse(".actlith", &stripped, &ACTLITH);
    let doc = doc.unwrap_or_else(|e| panic_free(&e.to_string()));
    let row = |c: &registry::Row| RawRow::from_cells(".actlith", c).ok();
    doc.section(kind::SECTION).iter().filter_map(row).collect()
}

/// Whether pipelith's catalogue checks `raw` with no forge extras.
fn neutral(raw: &RawRow) -> bool {
    let none = Extras::default();
    kind::resolve(raw, &none).is_ok_and(|s| Row::check(s, raw, &none).is_ok())
}

#[test]
fn v63_every_fixture_moves_to_actlith_with_the_same_rows() {
    for text in FIXTURES {
        let ours = parse(text).map(|r| r.workflows).unwrap_or_default();
        let theirs = as_actlith(text);
        assert_eq!(ours, theirs);
        assert!(theirs.iter().all(neutral));
    }
}

/// A parse failure as an empty document, failing the row count above.
fn panic_free(_: &str) -> registry::Document {
    registry::Document::default()
}
