//! The root `.gitlab-ci.yml` (`.:I` file): `workflow: rules`, the fixed
//! `stages:`, and one `include: local:` per row and per hand-written
//! file. No `default:` (`src/pipeline:V66`) and no remote include
//! (`src/pipeline:V7`).

use super::job::{Rule, Stage, rules, text};
use super::text::{eq, eq_var, var};
use crate::core::files;
use crate::core::yaml::{self, Comment, Entry, Node, Text};
use crate::yaml::GITLAB;

/// What the root file lists.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Root {
    /// Every included file, repository-relative, in the order given:
    /// row files first, then hand-written ones.
    pub(crate) includes: Vec<String>,
}

const RULES_WHY: &str = "One pipeline per change (src/pipeline:V10): \
    merge requests, the default\nbranch, tags, schedules and manual runs. \
    A push to a merge request's\nbranch runs its merge request pipeline \
    only.";

/// The root file: header, then the three keys.
pub(crate) fn root_document(root: &Root) -> String {
    let mut entries = vec![
        Entry::fixed("workflow", workflow()).above(Comment::new(RULES_WHY)),
        Entry::fixed("stages", stages()),
    ];
    if !root.includes.is_empty() {
        entries.push(Entry::fixed("include", includes(&root.includes)));
    }
    let body = yaml::document(&GITLAB, &[], &entries);
    format!("{}{body}", files::header("lablith", ".lablith"))
}

/// `workflow: rules:` (`src/pipeline:V10`, `src/pipeline:V51`).
fn workflow() -> Node {
    let source = |s| eq("CI_PIPELINE_SOURCE", s);
    let conds: [Text; 5] = [
        source("merge_request_event"),
        eq_var("CI_COMMIT_BRANCH", "CI_DEFAULT_BRANCH"),
        var("CI_COMMIT_TAG"),
        source("schedule"),
        source("web"),
    ];
    let rule = |c: Text| Rule {
        cond: Some(c),
        when: None,
    };
    Node::Map(vec![Entry::fixed("rules", rules(&conds.map(rule)))])
}

/// `stages:`, every stage in order (`src/pipeline:V52`).
fn stages() -> Node {
    Node::List(Stage::ALL.iter().map(|s| Text::lit(s.name())).collect())
}

/// `include:`, one `local:` per file.
fn includes(paths: &[String]) -> Node {
    let local = |p: &String| vec![Entry::fixed("local", text(p))];
    Node::Seq(paths.iter().map(local).collect())
}

#[cfg(test)]
mod tests;
