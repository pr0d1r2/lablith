use super::{ROOT, plan, render, row_path, tables};
use crate::registry::parse;
use crate::render::RenderError;

fn render_text(text: &str) -> Result<Vec<String>, RenderError> {
    let reg = parse(text).map_err(RenderError::from)?;
    render(&reg).map(|files| files.into_iter().map(|f| f.path).collect())
}

const ROWS: &str = "format 1\n## workflows\nid|kind|systems|cache|args\n\
    ci|gate|-|pr0d1r2|-\npins|update-pins|-|-|-\n\
    up|upstream|-|-|script=bump.sh\n\
    ## hand-written\npath|reason\n.gitlab/ci/release.yml|release flow\n";

#[test]
fn every_row_the_root_and_each_needed_script_once_sorted() {
    let want = [
        ".gitlab-ci.yml",
        ".gitlab/ci/ci.yml",
        ".gitlab/ci/pins.yml",
        ".gitlab/ci/up.yml",
        "scripts/lablith/cachix.sh",
        "scripts/lablith/open-mr.sh",
        "scripts/lablith/runner-system.sh",
        "scripts/lablith/update-pins.sh",
        "scripts/lablith/upstream.sh",
    ];
    assert_eq!(render_text(ROWS), Ok(want.map(str::to_owned).to_vec()));
}

#[test]
fn the_root_includes_rows_in_order_then_hand_written_files() {
    let reg = parse(ROWS).unwrap_or_default();
    let files = render(&reg).unwrap_or_default();
    let root = files
        .iter()
        .find(|f| f.path == ROOT)
        .map(|f| f.bytes.clone());
    let root = root.unwrap_or_default();
    let tail = "include:\n  - local: '.gitlab/ci/ci.yml'\n  - local: \
                '.gitlab/ci/pins.yml'\n  - local: '.gitlab/ci/up.yml'\n  - \
                local: '.gitlab/ci/release.yml'\n";
    assert!(root.ends_with(tail), "{root}");
}

#[test]
fn no_rows_still_render_the_root() {
    assert_eq!(render_text("format 1\n"), Ok(vec![ROOT.to_owned()]));
}

#[test]
fn an_unknown_kind_is_refused_naming_the_v1_kinds() {
    let text =
        "format 1\n## workflows\nid|kind|systems|cache|args\nx|lint|-|-|-\n";
    let e = render_text(text)
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default();
    assert_eq!(
        e,
        ".lablith:4: unknown kind `lint`\n  fix: use one of `gate`, `build`, \
         `update-pins`, `upstream`"
    );
}

#[test]
fn a_row_its_kind_refuses_is_reported_in_lablith_shape() {
    let text = "format 1\n## workflows\nid|kind|systems|cache|args\nb|build|-|-|package=x\n";
    let e = render_text(text)
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default();
    assert!(e.starts_with(".lablith:4: "), "{e}");
}

#[test]
fn a_hand_written_path_a_row_renders_is_refused() {
    let text = "format 1\n## workflows\nid|kind|systems|cache|args\nci|gate|-|-|-\n\
                ## hand-written\npath|reason\nx|y\n";
    assert!(render_text(text).is_ok());
    let reg = parse(&text.replace("x|y", ".gitlab/ci/ci.yml|why"))
        .unwrap_or_default();
    let files = render(&reg).unwrap_or_default();
    assert_eq!(files.iter().filter(|f| f.path == row_path("ci")).count(), 1);
}

#[test]
fn an_unknown_runner_system_is_refused() {
    let text = "format 1\n## runners\nsystem|tags\nriscv|x\n";
    let e = render_text(text).err();
    assert!(matches!(e, Some(RenderError::System(_))), "{e:?}");
}

#[test]
fn overrides_reach_the_tables_and_the_plan() {
    let text = "format 1\n## workflows\nid|kind|systems|cache|args\nci|gate|x86_64-linux|-|-\n\
                ## runners\nsystem|tags\nx86_64-linux|big\n";
    let reg = parse(text).unwrap_or_default();
    assert!(tables(&reg).is_ok());
    let plans = plan(&reg).unwrap_or_default();
    let tags = plans
        .first()
        .and_then(|p| p.jobs.first())
        .map(|j| j.tags.clone());
    assert_eq!(tags, Some(vec!["big".to_owned()]));
}

/// Each `<job>|<tags>` of `.lablith` text's first row, or why it failed.
fn gate_jobs(rows: &str, runners: &str) -> Result<Vec<String>, String> {
    let text = format!(
        "format 1\n## workflows\nid|kind|systems|cache|args\n{rows}\n\
         ## runners\nsystem|tags\n{runners}"
    );
    let reg = parse(&text).map_err(|e| e.to_string())?;
    let plans = plan(&reg).map_err(|e| e.to_string())?;
    let jobs = plans.into_iter().flat_map(|p| p.jobs);
    Ok(jobs
        .map(|j| format!("{}|{}", j.name, j.tags.join(", ")))
        .collect())
}

const DARWIN_RUNNER: &str = "x86_64-darwin|nix, x86_64-darwin\n";

#[test]
fn x86_64_darwin_with_a_runner_row_renders_on_that_runner() {
    let got = gate_jobs("ci|gate|x86_64-darwin|-|-", DARWIN_RUNNER);
    let want = "ci:gate:x86_64-darwin|nix, x86_64-darwin".to_owned();
    assert_eq!(got, Ok(vec![want]));
}

#[test]
fn x86_64_darwin_without_a_runner_row_is_refused_naming_the_fix() {
    let got = gate_jobs("ci|gate|x86_64-darwin|-|-", "").err();
    let got = got.unwrap_or_default();
    assert!(got.contains("`x86_64-darwin`"), "{got}");
    assert!(
        got.contains("fix: add a `## runners` row `x86_64-darwin|<tags>`"),
        "{got}"
    );
}

#[test]
fn a_default_row_without_a_darwin_runner_row_keeps_three_systems() {
    let got = gate_jobs("ci|gate|-|-|-", "").unwrap_or_default();
    let want = [
        "ci:gate:x86_64-linux|nix, x86_64-linux",
        "ci:gate:aarch64-linux|nix, aarch64-linux",
        "ci:gate:aarch64-darwin|nix, aarch64-darwin",
    ];
    assert_eq!(got, want.map(str::to_owned).to_vec());
}

#[test]
fn a_default_row_with_a_darwin_runner_row_gates_it_too() {
    let got = gate_jobs("ci|gate|-|-|-", DARWIN_RUNNER).unwrap_or_default();
    let want = [
        "ci:gate:x86_64-linux|nix, x86_64-linux",
        "ci:gate:aarch64-linux|nix, aarch64-linux",
        "ci:gate:aarch64-darwin|nix, aarch64-darwin",
        "ci:gate:x86_64-darwin|nix, x86_64-darwin",
    ];
    assert_eq!(got, want.map(str::to_owned).to_vec());
}

/// The first rule of each job of `.lablith` text, as `if|when`.
fn first_rules(text: &str) -> Vec<String> {
    let reg = parse(text).unwrap_or_default();
    let jobs = plan(&reg)
        .unwrap_or_default()
        .into_iter()
        .flat_map(|p| p.jobs);
    let first = |j: crate::pipeline::Job| {
        let r = j.rules.into_iter().next()?;
        let cond = r.cond.map(|c| c.render(&crate::yaml::GITLAB));
        Some(format!("{}|{:?}", cond.unwrap_or_default(), r.when))
    };
    jobs.filter_map(first).collect()
}

const ONE_GATE: &str = "format 1\n## workflows\nid|kind|systems|cache|args\nci|gate|x86_64-linux|-|-\n";

/// `src/kind:V76`.
#[test]
fn a_job_on_an_own_runner_never_runs_for_a_fork_merge_request() {
    let text = format!("{ONE_GATE}## runners\nsystem|tags\nx86_64-linux|big\n");
    let guard = "$CI_MERGE_REQUEST_SOURCE_PROJECT_ID != \
                 $CI_MERGE_REQUEST_PROJECT_ID|Some(Never)";
    assert_eq!(first_rules(&text), [guard.to_owned()]);
}

/// `src/kind:V76`: built-in tags carry no guard.
#[test]
fn a_job_on_built_in_tags_keeps_its_rules() {
    let want = "$CI_PIPELINE_SOURCE == \"schedule\"|Some(Never)".to_owned();
    assert_eq!(first_rules(ONE_GATE), [want]);
}
