//! `completions` and `man` come from the argv table.

use clap_complete::Shell;

use super::{completions, man};

#[test]
fn completions_name_the_command_and_its_verbs() {
    let zsh = completions(Shell::Zsh);
    assert_eq!(zsh.code, 0);
    assert!(zsh.out.contains("#compdef lablith"));
    assert!(zsh.out.contains("detect"));
    let bash = completions(Shell::Bash);
    assert!(bash.out.contains("lablith"));
}

#[test]
fn man_is_roff_naming_every_verb() {
    let page = man();
    assert_eq!(page.code, 0);
    assert!(page.out.starts_with(".ie"));
    assert!(page.out.contains(".TH lablith"));
    assert!(page.out.contains("lablith\\-gen"));
}
