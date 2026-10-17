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
