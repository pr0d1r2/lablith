//! Row typing and key rules, through `parse` (`src/registry:T12`).

use crate::core::ConfigError;
use crate::registry::{
    Arg, HandWritten, ImagePin, Registry, RunnerRow, WorkflowRow, parse,
};

const DIGEST: &str =
    "sha256:0d2fd8c2bf4bb5b1c3b2e9bd1d2c7a2f7a1b0f3c6c3f3b4e3f0d1e2c3b4a5968";

/// A section: its name and its header row.
type Table = (&'static str, &'static str);

const WORKFLOWS: Table = ("workflows", "id|kind|systems|cache|args");
const IMAGES: Table = ("images", "image|version|digest");
const RUNNERS: Table = ("runners", "system|tags");
const HAND_WRITTEN: Table = ("hand-written", "path|reason");

/// `row` alone in its section, on line 9.
fn one(table: Table, row: &str) -> Result<Registry, ConfigError> {
    let (name, header) = table;
    parse(&format!("format 1\n\n\n\n\n\n## {name}\n{header}\n{row}\n"))
}

fn refused(table: Table, row: &str) -> String {
    one(table, row).err().unwrap_or_default().what
}

#[test]
fn a_workflow_row_is_typed() {
    let reg = one(WORKFLOWS, "ci|gate|x86_64-linux|pr0d1r2|a=b");
    let want = WorkflowRow {
        line: 9,
        id: "ci".into(),
        kind: "gate".into(),
        systems: vec!["x86_64-linux".into()],
        cache: Some("pr0d1r2".into()),
        args: vec![Arg {
            key: "a".into(),
            value: "b".into(),
        }],
    };
    assert_eq!(reg.unwrap_or_default().workflows, [want]);
}

#[test]
fn a_workflow_needs_id_and_kind() {
    assert_eq!(refused(WORKFLOWS, "-|gate|-|-|-"), "`id` is empty");
    assert_eq!(refused(WORKFLOWS, "ci|-|-|-|-"), "`kind` is empty");
}

#[test]
fn a_workflow_id_keeps_pipelith_charset() {
    for bad in ["a/b", "a b", ".hidden", "ci:yml", "a$b"] {
        let row = format!("{bad}|gate|-|-|-");
        let what = refused(WORKFLOWS, &row);
        assert!(what.contains("is not a file name"), "{bad}: {what}");
    }
    assert!(one(WORKFLOWS, "Up_pins.2-x|gate|-|-|-").is_ok());
}

#[test]
fn an_image_row_is_typed() {
    let row = format!("nixos/nix|2.28.3|{DIGEST}");
    let want = ImagePin {
        line: 9,
        image: "nixos/nix".into(),
        version: "2.28.3".into(),
        digest: DIGEST.into(),
    };
    assert_eq!(one(IMAGES, &row).unwrap_or_default().images, [want]);
}

#[test]
fn an_image_digest_is_sha256_and_64_lowercase_hex() {
    let short = format!("{DIGEST}0");
    let upper = DIGEST.to_uppercase();
    let bare = DIGEST.trim_start_matches("sha256:");
    for bad in ["abc", short.as_str(), upper.as_str(), bare] {
        let what = refused(IMAGES, &format!("a/b|v1|{bad}"));
        assert!(what.contains("is not `sha256:` and 64 hex"), "{what}");
    }
    assert_eq!(refused(IMAGES, "a/b|v1|-"), "`digest` is empty");
    let row = format!("-|v1|{DIGEST}");
    assert_eq!(refused(IMAGES, &row), "`image` is empty");
    let row = format!("a/b|-|{DIGEST}");
    assert_eq!(refused(IMAGES, &row), "`version` is empty");
}

#[test]
fn a_runner_row_is_typed() {
    let reg = one(RUNNERS, "aarch64-linux|nix, arm64");
    let want = RunnerRow {
        line: 9,
        system: "aarch64-linux".into(),
        tags: vec!["nix".into(), "arm64".into()],
    };
    assert_eq!(reg.unwrap_or_default().runners, [want]);
}

#[test]
fn a_runner_row_needs_a_system_and_a_tag() {
    assert_eq!(refused(RUNNERS, "-|nix"), "`system` is empty");
    let e = one(RUNNERS, "x86_64-linux|-").err().unwrap_or_default();
    assert_eq!(e.line, 9);
    assert_eq!(e.what, "runner `x86_64-linux` has no tags");
    assert!(e.fix.contains("`nix`"), "{}", e.fix);
}

#[test]
fn a_hand_written_row_needs_a_reason() {
    let reg = one(HAND_WRITTEN, ".gitlab/ci/x.yml|why");
    let want = HandWritten {
        line: 9,
        path: ".gitlab/ci/x.yml".into(),
        reason: "why".into(),
    };
    assert_eq!(reg.unwrap_or_default().hand_written, [want]);
    let what = refused(HAND_WRITTEN, ".gitlab/ci/x.yml|-");
    assert_eq!(what, "`reason` is empty");
    assert_eq!(refused(HAND_WRITTEN, "-|why"), "`path` is empty");
}

#[test]
fn a_duplicate_key_names_both_lines() {
    let image = format!("a/b|v1|{DIGEST}");
    let cases = [
        (WORKFLOWS, "ci|gate|-|-|-", "id `ci`"),
        (IMAGES, image.as_str(), "image `a/b`"),
        (RUNNERS, "x86_64-linux|nix", "system `x86_64-linux`"),
        (HAND_WRITTEN, "a.yml|r", "path `a.yml`"),
    ];
    for ((name, header), row, key) in cases {
        let text = format!("format 1\n## {name}\n{header}\n{row}\n{row}\n");
        let e = parse(&text).err().unwrap_or_default();
        assert_eq!(e.line, 5);
        assert_eq!(e.what, format!("{key} given twice, first on line 4"));
        assert_eq!(e.fix, "keep one row per key");
    }
}
