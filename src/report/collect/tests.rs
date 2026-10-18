use super::{TOKEN, collect};
use crate::registry::parse;

fn reg(rows: &str) -> crate::registry::Registry {
    let text =
        format!("format 1\n## workflows\nid|kind|systems|cache|args\n{rows}\n");
    parse(&text).unwrap_or_default()
}

#[test]
fn rows_are_sorted_by_id_with_their_tags_and_one_image() {
    let r = collect(&reg("z|gate|x86_64-linux|-|-\na|gate|aarch64-linux|-|-"))
        .unwrap_or_default();
    let ids: Vec<&str> = r.rows.iter().map(|r| r.id.as_str()).collect();
    assert_eq!(ids, ["a", "z"]);
    let first = r.rows.first();
    let tags = first
        .and_then(|r| r.systems.first())
        .map(|t| t.tags.clone());
    assert_eq!(
        tags,
        Some(vec!["nix".to_owned(), "aarch64-linux".to_owned()])
    );
    assert_eq!(first.map(|r| r.images.len()), Some(1));
}

#[test]
fn a_bump_row_needs_a_schedule_and_the_token_note() {
    let r = collect(&reg("pins|update-pins|-|-|cron=0 5 * * *"))
        .unwrap_or_default();
    let s = r.rows.first().and_then(|r| r.schedule.clone());
    let s = s.map(|s| (s.cron, s.variable));
    assert_eq!(
        s,
        Some(("0 5 * * *".to_owned(), "LABLITH_ROW=pins".to_owned()))
    );
    assert_eq!(r.token, Some(TOKEN));
}

#[test]
fn rows_that_open_no_merge_request_carry_no_token_note() {
    let r = collect(&reg("ci|gate|-|pr0d1r2|-")).unwrap_or_default();
    assert_eq!(r.token, None);
    let secrets: Vec<&str> = r
        .rows
        .iter()
        .flat_map(|r| &r.secrets)
        .map(|n| n.name.as_str())
        .collect();
    assert_eq!(secrets, ["CACHIX_AUTH_TOKEN"]);
}

/// `src/system:V72`: the report marks a skipped system.
#[test]
fn a_skipped_system_is_marked_in_the_report() {
    let text = "format 1\n## workflows\nid|kind|systems|cache|args\n\
                ci|gate|x86_64-linux, aarch64-linux|-|-\n## runners\n\
                system|tags\naarch64-linux|-\n";
    let r = collect(&parse(text).unwrap_or_default()).unwrap_or_default();
    let got: Vec<(&str, bool)> = r
        .rows
        .iter()
        .flat_map(|r| &r.systems)
        .map(|t| (t.system, t.skipped))
        .collect();
    assert_eq!(got, [("x86_64-linux", false), ("aarch64-linux", true)]);
}
