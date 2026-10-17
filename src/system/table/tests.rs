use super::{RunnerTable, System, SystemError};
use crate::registry::RunnerRow;

fn row(system: &str, tags: &[&str]) -> RunnerRow {
    RunnerRow {
        line: 4,
        system: system.into(),
        tags: tags.iter().map(|&t| t.to_owned()).collect(),
    }
}

#[test]
fn builtin_tags_are_nix_and_the_system_name() {
    let t = RunnerTable::builtin();
    for s in [
        System::X86_64Linux,
        System::Aarch64Linux,
        System::Aarch64Darwin,
    ] {
        let tags = t.tags(s).map(<[String]>::to_vec);
        assert_eq!(tags, Ok(vec!["nix".to_owned(), s.name().to_owned()]));
    }
}

#[test]
fn x86_64_darwin_has_no_tags_and_the_error_names_the_fix() {
    let t = RunnerTable::builtin();
    let e = t.tags(System::X86_64Darwin).err();
    assert_eq!(e, Some(SystemError::NoTags(System::X86_64Darwin)));
    assert_eq!(
        e.map(|e| e.to_string()).unwrap_or_default(),
        "system `x86_64-darwin` has no GitLab runner tags\n  fix: add a \
         `## runners` row `x86_64-darwin|<tags>`"
    );
}

#[test]
fn an_override_replaces_the_tags_of_its_system() {
    let t = RunnerTable::builtin()
        .with_overrides(&[row("aarch64-linux", &["arm", "docker"])]);
    let tags = t.map(|t| t.tags(System::Aarch64Linux).map(<[String]>::to_vec));
    assert_eq!(tags, Ok(Ok(vec!["arm".to_owned(), "docker".to_owned()])));
}

#[test]
fn an_override_may_add_a_system_the_builtin_lacks() {
    let t = RunnerTable::builtin()
        .with_overrides(&[row("x86_64-darwin", &["mac"])]);
    let tags = t.map(|t| t.tags(System::X86_64Darwin).map(<[String]>::to_vec));
    assert_eq!(tags, Ok(Ok(vec!["mac".to_owned()])));
}

#[test]
fn an_unknown_system_in_runners_is_refused_with_its_line() {
    let e = RunnerTable::builtin().with_overrides(&[row("riscv", &["x"])]);
    let e = e.err().map(|e| e.to_string()).unwrap_or_default();
    assert!(
        e.starts_with(".lablith:4: unknown system `riscv`\n  fix: use one of"),
        "{e}"
    );
    assert!(e.contains("aarch64-darwin"), "{e}");
}
