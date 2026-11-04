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

/// `src/kind/rules:V76`.
#[test]
fn a_job_on_an_own_runner_never_runs_for_a_fork_merge_request() {
    let text = format!("{ONE_GATE}## runners\nsystem|tags\nx86_64-linux|big\n");
    let guard = "$CI_MERGE_REQUEST_SOURCE_PROJECT_ID != \
                 $CI_MERGE_REQUEST_PROJECT_ID|Some(Never)";
    assert_eq!(first_rules(&text), [guard.to_owned()]);
}

/// `src/kind/rules:V76`: built-in tags land on the owner's runners on a
/// self-hosted GitLab too, so their jobs carry the same guard.
#[test]
fn a_job_on_built_in_tags_never_runs_for_a_fork_merge_request() {
    let guard = "$CI_MERGE_REQUEST_SOURCE_PROJECT_ID != \
                 $CI_MERGE_REQUEST_PROJECT_ID|Some(Never)";
    assert_eq!(first_rules(ONE_GATE), [guard.to_owned()]);
}

/// A valid ed25519 public key for an example cache.
const KEY: &str =
    "example-cache-1:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

/// `.lablith` text with `caches` as its `## caches` rows and `row` as
/// its one workflow.
fn with_caches(caches: &str, row: &str) -> String {
    format!(
        "format 1\n## workflows\nid|kind|systems|cache|args\n{row}\n\
         ## caches\nname|kind|url|key|push\n{caches}"
    )
}

/// Each job of `text` as `name`, its `NIX_CONFIG` and its script lines.
fn cache_jobs(
    text: &str,
) -> Result<Vec<(String, String, Vec<String>)>, String> {
    let reg = parse(text).map_err(|e| e.to_string())?;
    let plans = plan(&reg).map_err(|e| e.to_string())?;
    let gitlab = &crate::yaml::GITLAB;
    let one = |j: crate::pipeline::Job| {
        let config = j.variables.iter().find(|(k, _)| *k == "NIX_CONFIG");
        let config = config.map(|(_, v)| v.render(gitlab)).unwrap_or_default();
        let script = j.script.iter().map(|t| t.render(gitlab)).collect();
        (j.name, config, script)
    };
    Ok(plans.into_iter().flat_map(|p| p.jobs).map(one).collect())
}

/// The seeded scripts `text` renders.
fn seeded_scripts(text: &str) -> Vec<String> {
    let paths = render_text(text).unwrap_or_default();
    paths
        .into_iter()
        .filter(|p| p.starts_with("scripts/"))
        .collect()
}

const GATE: &str = "nix develop --command hk check --all --check";

/// The `## caches` row of an https cache with literal url and key,
/// pushed with a netrc file.
fn https_row() -> String {
    let push = "secret:EXAMPLE_NETRC";
    format!("example-cache|https|https://cache.example.org|{KEY}|{push}\n")
}

/// `src/cache:V80`: an https cache with literal url and key is trusted
/// in `NIX_CONFIG`.
#[test]
fn an_https_cache_with_literals_is_read_from_nix_config() {
    let text =
        with_caches(&https_row(), "ci|gate|x86_64-linux|example-cache|-");
    let jobs = cache_jobs(&text).unwrap_or_default();
    let (name, config, _) = jobs.first().cloned().unwrap_or_default();
    assert_eq!(name, "ci:gate:x86_64-linux");
    let want = format!(
        "experimental-features = nix-command flakes\n\
         extra-substituters = https://cache.example.org\n\
         extra-trusted-public-keys = {KEY}\n"
    );
    assert!(config.contains(&want), "{config}");
}

/// `src/cache:V82`: its push goes through `cache-push.sh`, seeded in
/// place of `cachix.sh`.
#[test]
fn an_https_cache_is_pushed_by_cache_push() {
    let text =
        with_caches(&https_row(), "ci|gate|x86_64-linux|example-cache|-");
    let jobs = cache_jobs(&text).unwrap_or_default();
    let script = jobs.first().map(|j| j.2.clone()).unwrap_or_default();
    let push = format!(
        "bash scripts/lablith/cache-push.sh https 'https://cache.example.org' \
         EXAMPLE_NETRC any '{GATE}' \
         .#devShells.x86_64-linux.default.inputDerivation"
    );
    assert_eq!(script.last(), Some(&push), "{script:?}");
    let scripts = seeded_scripts(&text);
    assert!(scripts.contains(&"scripts/lablith/cache-push.sh".to_owned()));
    assert!(!scripts.contains(&"scripts/lablith/cachix.sh".to_owned()));
}

/// `src/cache/read:V81`: secret url and key stay out of `NIX_CONFIG`; the job
/// reads them at its start, and a read-only cache gets no push step.
#[test]
fn a_secret_cache_is_read_at_job_start_and_a_read_only_one_not_pushed() {
    let row = "hidden-cache|https|secret:HIDDEN_URL|secret:HIDDEN_KEY|-\n";
    let text = with_caches(row, "ci|gate|x86_64-linux|hidden-cache|-");
    let jobs = cache_jobs(&text).unwrap_or_default();
    let (_, config, script) = jobs.first().cloned().unwrap_or_default();
    assert!(!config.contains("extra-substituters"), "{config}");
    let read = "NIX_CONFIG=\"$NIX_CONFIG$(bash scripts/lablith/cache-read.sh \
                'secret:HIDDEN_URL' 'secret:HIDDEN_KEY')\"";
    assert_eq!(script.get(1).map(String::as_str), Some(read), "{script:?}");
    assert_eq!(script.last().map(String::as_str), Some(GATE), "{script:?}");
    let scripts = seeded_scripts(&text);
    assert!(scripts.contains(&"scripts/lablith/cache-read.sh".to_owned()));
    assert!(!scripts.contains(&"scripts/lablith/cache-push.sh".to_owned()));
}

/// A gate row on an https cache read with `READ_NETRC` and pushed with
/// `PUSH_NETRC`.
fn private_gate() -> String {
    let push = "read=secret:READ_NETRC, push=secret:PUSH_NETRC";
    let row = format!("private|https|https://cache.example.org|{KEY}|{push}\n");
    with_caches(&row, "ci|gate|x86_64-linux|private|-")
}

/// `src/cache/read:V86`: a read netrc is written by `cache-read.sh`
/// right after the runner check, and the push keeps its own netrc.
#[test]
fn a_read_netrc_cache_is_read_at_job_start_and_pushed_apart() {
    let jobs = cache_jobs(&private_gate()).unwrap_or_default();
    let (_, config, script) = jobs.first().cloned().unwrap_or_default();
    assert!(!config.contains("extra-substituters"), "{config}");
    let read = format!(
        "NIX_CONFIG=\"$NIX_CONFIG$(bash scripts/lablith/cache-read.sh \
         'https://cache.example.org' '{KEY}' READ_NETRC private)\""
    );
    assert_eq!(script.get(1), Some(&read), "{script:?}");
    let push = script.last().cloned().unwrap_or_default();
    assert!(push.contains(" PUSH_NETRC any "), "{push}");
    assert!(!push.contains("READ_NETRC"), "{push}");
}

/// `src/kind/rules:V76`: the fork guard stays the first rule of a job
/// that reads with a netrc.
#[test]
fn a_read_netrc_job_keeps_the_fork_guard_first() {
    let guard = "$CI_MERGE_REQUEST_SOURCE_PROJECT_ID != \
                 $CI_MERGE_REQUEST_PROJECT_ID|Some(Never)";
    assert_eq!(first_rules(&private_gate()), [guard.to_owned()]);
}

/// `src/cache:V82`: a cachix row with its own token pushes through
/// `cache-push.sh` with that variable.
#[test]
fn a_cachix_row_with_its_own_token_pushes_with_it() {
    let row = format!("own-cachix|cachix|-|{KEY}|secret:OWN_CACHIX_TOKEN\n");
    let text = with_caches(&row, "ci|gate|x86_64-linux|own-cachix|-");
    let jobs = cache_jobs(&text).unwrap_or_default();
    let (_, config, script) = jobs.first().cloned().unwrap_or_default();
    assert!(config.contains("https://own-cachix.cachix.org"), "{config}");
    let push = format!(
        "bash scripts/lablith/cache-push.sh cachix own-cachix OWN_CACHIX_TOKEN \
         any '{GATE}'"
    );
    assert_eq!(script.last(), Some(&push));
}

/// The last script line of job `name` of an https-cached build row.
fn https_build_line(name: &str) -> String {
    let row = "pkg|build|x86_64-linux|example-cache|package=hk";
    let jobs = cache_jobs(&with_caches(&https_row(), row)).unwrap_or_default();
    let job = jobs.into_iter().find(|(n, _, _)| n == name);
    job.and_then(|(_, _, s)| s.last().cloned())
        .unwrap_or_default()
}

/// `src/cache:V83`: a build row pushes to an https cache from the
/// default branch only.
#[test]
fn a_build_row_pushes_https_on_the_default_branch() {
    let build = "bash scripts/lablith/cache-push.sh https 'https://cache.example.org' \
                 EXAMPLE_NETRC default 'nix build --print-build-logs \
                 .#packages.x86_64-linux.hk' .#packages.x86_64-linux.hk";
    assert_eq!(https_build_line("pkg:build:x86_64-linux"), build);
}

/// `src/cache:V84`: and verifies it over https, through a seeded script.
#[test]
fn a_build_row_verifies_an_https_cache_over_https() {
    let verify = "bash scripts/lablith/https-verify.sh 'https://cache.example.org' \
                  EXAMPLE_NETRC hk x86_64-linux";
    assert_eq!(https_build_line("pkg:verify-cache"), verify);
    let row = "pkg|build|x86_64-linux|example-cache|package=hk";
    let scripts = seeded_scripts(&with_caches(&https_row(), row));
    assert!(scripts.contains(&"scripts/lablith/https-verify.sh".to_owned()));
}

/// `src/cache:V80`: a `## caches` row naming the built-in cache replaces
/// it, while no section leaves the built-in rendering as it was.
#[test]
fn a_caches_row_shadows_the_built_in_cache() {
    let row = "pr0d1r2|cachix|-|-|-\n";
    let text = with_caches(row, "ci|gate|x86_64-linux|pr0d1r2|-");
    let jobs = cache_jobs(&text).unwrap_or_default();
    let (_, config, script) = jobs.first().cloned().unwrap_or_default();
    assert!(config.contains("https://pr0d1r2.cachix.org"), "{config}");
    assert_eq!(script.last().map(String::as_str), Some(GATE));
}
