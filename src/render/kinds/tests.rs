//! The closed kind list, and parity with `docs/KINDS-GITLAB.md` and
//! each kind's node spec (`.:V24`, `.:T23`).

use super::{entries, lookup, names};
use crate::core::kind::ArgDefault;
use crate::core::kind::CATALOGUE;
use crate::kind::DEFAULT_TOKEN;

const KINDS_MD: &str = include_str!("../../../docs/KINDS-GITLAB.md");

/// The node whose spec owns each kind (`.:V24`).
const NODES: [(&str, &str); 4] = [
    ("gate", include_str!("../../gate/SPEC.md")),
    ("build", include_str!("../../build/SPEC.md")),
    ("update-pins", include_str!("../../bump/SPEC.md")),
    ("upstream", include_str!("../../bump/SPEC.md")),
];

/// One arg as written: key, whether required, the default it names.
type Arg<'a> = (&'a str, bool, Option<&'a str>);

#[test]
fn the_v1_kinds_are_pipelith_catalogue_in_order() {
    let ours: Vec<&str> = entries().iter().map(|k| k.spec().name).collect();
    let theirs: Vec<&str> = CATALOGUE.iter().map(|k| k.name).collect();
    assert_eq!(ours, theirs);
}

#[test]
fn lookup_finds_a_kind_by_name_only() {
    assert_eq!(lookup("build").map(|k| k.spec().name), Some("build"));
    for name in ["", "Gate", "lint", "update_pins"] {
        assert!(lookup(name).is_none(), "{name}");
    }
    assert_eq!(names(), "`gate`, `build`, `update-pins`, `upstream`");
}

/// The ``## `kind` `` sections of `docs/KINDS.md`, in order.
fn doc_kinds() -> Vec<(&'static str, &'static str)> {
    KINDS_MD
        .split("\n## ")
        .skip(1)
        .filter_map(|section| {
            let (head, body) = section.split_once('\n')?;
            let name = head.strip_prefix('`')?.strip_suffix('`')?;
            Some((name, body))
        })
        .collect()
}

/// The `- args:` bullet of a kind section, continuation lines included.
fn args_bullet(body: &str) -> &str {
    let rest = body.split_once("\n- args:").map_or("", |(_, r)| r);
    let bullet = rest.split("\n- ").next().unwrap_or(rest);
    bullet.split("\n\n").next().unwrap_or(bullet)
}

/// Parenthesis depth and backtick state after one more char.
fn scan((depth, tick): (u8, bool), c: char) -> (u8, bool) {
    match c {
        '`' => (depth, !tick),
        '(' if !tick => (depth.saturating_add(1), tick),
        ')' if !tick => (depth.saturating_sub(1), tick),
        _ => (depth, tick),
    }
}

/// The bullet split on commas outside parentheses and backticks.
fn items(bullet: &str) -> Vec<&str> {
    let (mut out, mut state, mut start) = (Vec::new(), (0, false), 0);
    for (i, c) in bullet.char_indices() {
        state = scan(state, c);
        if c == ',' && state == (0, false) {
            out.push(bullet.get(start..i).unwrap_or(""));
            start = i.saturating_add(1);
        }
    }
    out.push(bullet.get(start..).unwrap_or(""));
    out
}

/// One item: `` `key` (required | default `value` | ...) ``.
fn doc_arg(item: &str) -> Option<Arg<'_>> {
    let (_, rest) = item.split_once('`')?;
    let (key, rest) = rest.split_once('`')?;
    let paren = rest.split_once('(').map_or("", |(_, p)| p);
    let default = paren
        .split_once("default `")
        .and_then(|(_, d)| d.split_once('`'))
        .map(|(d, _)| d);
    Some((key, paren.starts_with("required"), default))
}

/// A spec's `## <head>` section, up to the next one.
fn section<'a>(spec: &'a str, head: &str) -> &'a str {
    let rest = spec.split_once(head).map_or("", |(_, r)| r);
    rest.split("\n## ").next().unwrap_or(rest)
}

#[test]
fn the_doc_lists_every_kind_in_code_order() {
    let doc: Vec<&str> = doc_kinds().into_iter().map(|(n, _)| n).collect();
    let code: Vec<&str> = entries().iter().map(|k| k.spec().name).collect();
    assert_eq!(doc, code);
}

/// The args `docs/KINDS-GITLAB.md` gives the kind `name`.
fn doc_args(name: &str) -> Vec<Arg<'static>> {
    let docs = doc_kinds();
    let body = docs.iter().find(|(n, _)| *n == name).map_or("", |d| d.1);
    items(args_bullet(body))
        .into_iter()
        .filter_map(doc_arg)
        .collect()
}

/// The default `docs/KINDS-GITLAB.md` names: the forge's is
/// `LABLITH_TOKEN` (`src/kind:V54`).
fn default(given: ArgDefault) -> Option<&'static str> {
    match given {
        ArgDefault::None => None,
        ArgDefault::Value(value) => Some(value),
        ArgDefault::Forge => Some(DEFAULT_TOKEN),
    }
}

#[test]
fn the_doc_args_are_each_schema_in_order() {
    for kind in entries() {
        let name = kind.spec().name;
        let code: Vec<Arg<'_>> = kind
            .spec()
            .schema
            .iter()
            .map(|a| (a.key, a.required, default(a.default)))
            .collect();
        assert_eq!(doc_args(name), code, "docs/KINDS-GITLAB.md `{name}` args");
    }
}

#[test]
fn every_kind_has_its_node_rules() {
    for (name, spec) in NODES {
        assert!(lookup(name).is_some(), "{name}");
        let named = format!("`{name}`");
        assert!(
            section(spec, "## \u{a7}I").contains(&named),
            "{name} interfaces"
        );
        let rules = section(spec, "## \u{a7}V").lines();
        assert!(
            rules.filter(|l| l.starts_with('V')).count() > 0,
            "{name} invariants"
        );
        let tasks = section(spec, "## \u{a7}T");
        assert!(tasks.contains(&format!("kind {named}")), "{name} tasks");
    }
}
