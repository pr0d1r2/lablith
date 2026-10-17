//! One GitLab job and its row file (`src/pipeline:V8`,
//! `src/pipeline:V9`): every key written out, no `extends:` and no
//! anchors, in a fixed key order.

use crate::core::files;
use crate::core::yaml::{self, Comment, Entry, Node, Text, Value};
use crate::pin::Image;
use crate::yaml::GITLAB;

/// The fixed stages, in run order (`src/pipeline:V52`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Stage {
    /// The hk gate.
    Gate,
    /// Package builds.
    Build,
    /// Checks of what the build stage produced.
    Verify,
    /// Scheduled bump merge requests.
    Bump,
}

impl Stage {
    /// Every stage, in order.
    pub(crate) const ALL: [Self; 4] =
        [Self::Gate, Self::Build, Self::Verify, Self::Bump];

    /// The stage name as GitLab spells it.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Gate => "gate",
            Self::Build => "build",
            Self::Verify => "verify",
            Self::Bump => "bump",
        }
    }
}

/// A rule's `when:`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum When {
    /// Do not run.
    Never,
    /// Run when the earlier stages passed.
    OnSuccess,
}

/// One entry of `rules:`: an optional `if:` and an optional `when:`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Rule {
    /// The condition; `None` matches every pipeline.
    pub(crate) cond: Option<Text>,
    /// What to do when it matches; `None` is GitLab's default.
    pub(crate) when: Option<When>,
}

/// A job, every setting on the job itself (`src/pipeline:V66`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Job {
    /// `<id>:<job>`; unique across rows (`src/pipeline:V9`).
    pub(crate) name: String,
    /// Comment above the job.
    pub(crate) comment: Option<Comment>,
    /// The stage.
    pub(crate) stage: Stage,
    /// The pinned image (`src/pin:V2`).
    pub(crate) image: Image,
    /// Runner tags (`src/system:V56`).
    pub(crate) tags: Vec<String>,
    /// `timeout:` in minutes (`src/kind:V11`).
    pub(crate) timeout: u32,
    /// `interruptible:` (`src/kind:V12`).
    pub(crate) interruptible: bool,
    /// `resource_group:` (`src/kind:V12`).
    pub(crate) resource_group: Option<String>,
    /// Jobs of the same row this one waits for.
    pub(crate) needs: Vec<String>,
    /// `rules:` (`src/kind:V50`).
    pub(crate) rules: Vec<Rule>,
    /// `variables:`, in order.
    pub(crate) variables: Vec<(&'static str, Text)>,
    /// `script:`, one command per line (`src/script:V16`).
    pub(crate) script: Vec<Text>,
}

impl Job {
    /// The job as a YAML entry.
    pub(crate) fn entry(&self) -> Entry {
        let entry = Entry::user(&self.name, Node::Map(self.body()));
        match &self.comment {
            Some(c) => entry.above(c.clone()),
            None => entry,
        }
    }

    /// The keys of the job, in the fixed order.
    fn body(&self) -> Vec<Entry> {
        let mut out = vec![
            Entry::fixed("stage", text(self.stage.name())),
            self.image(),
            Entry::fixed("tags", list(&self.tags)),
            Entry::fixed("timeout", text(&format!("{} minutes", self.timeout))),
            Entry::fixed("interruptible", flag(self.interruptible)),
        ];
        out.extend(self.optional());
        out
    }

    /// `image: name@digest  # version`.
    fn image(&self) -> Entry {
        let image = text(&self.image.reference());
        Entry::fixed("image", image).after(Comment::new(&self.image.version))
    }

    /// The keys a job may leave out.
    fn optional(&self) -> Vec<Entry> {
        let group = self.resource_group.as_deref();
        let needs = (!self.needs.is_empty()).then(|| list(&self.needs));
        let rules = (!self.rules.is_empty()).then(|| rules(&self.rules));
        let vars = (!self.variables.is_empty()).then(|| map(&self.variables));
        [
            group.map(|g| Entry::fixed("resource_group", text(g))),
            needs.map(|n| Entry::fixed("needs", n)),
            rules.map(|r| Entry::fixed("rules", r)),
            vars.map(|v| Entry::fixed("variables", v)),
            Some(Entry::fixed("script", Node::List(self.script.clone()))),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

/// The row file `.gitlab/ci/<id>.yml`: header, then every job.
pub(crate) fn row_document(jobs: &[Job]) -> String {
    let entries: Vec<Entry> = jobs.iter().map(Job::entry).collect();
    let body = yaml::document(&GITLAB, &[], &entries);
    format!("{}{body}", files::header("lablith", ".lablith"))
}

/// `rules:` as a sequence of maps.
pub(crate) fn rules(rules: &[Rule]) -> Node {
    Node::Seq(rules.iter().map(rule).collect())
}

/// One rule's keys.
fn rule(rule: &Rule) -> Vec<Entry> {
    let when = rule.when.map(|w| match w {
        When::Never => "never",
        When::OnSuccess => "on_success",
    });
    [
        rule.cond.clone().map(|c| Entry::fixed("if", value(c))),
        when.map(|w| Entry::fixed("when", text(w))),
    ]
    .into_iter()
    .flatten()
    .collect()
}

/// A literal text value.
pub(crate) fn text(s: &str) -> Node {
    value(Text::lit(s))
}

/// A text value.
fn value(t: Text) -> Node {
    Node::Value(Value::Text(t))
}

/// A boolean value.
fn flag(b: bool) -> Node {
    Node::Value(Value::Bool(b))
}

/// A list of literal texts.
pub(crate) fn list(items: &[String]) -> Node {
    Node::List(items.iter().map(|s| Text::lit(s)).collect())
}

/// A map of constant keys to texts.
fn map(pairs: &[(&'static str, Text)]) -> Node {
    let entry =
        |(k, v): &(&'static str, Text)| Entry::fixed(k, value(v.clone()));
    Node::Map(pairs.iter().map(entry).collect())
}

#[cfg(test)]
mod tests;
