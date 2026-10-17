//! The hardened nix job every kind builds on (`src/kind:V11` to
//! `src/kind:V14`, `src/kind:V57`): the pinned nix image with flakes on,
//! the row's cache as a substituter, the system's runner tags, the row's
//! timeout and a per-kind clone depth.

use super::contract::Tables;
use super::error::KindError;
use crate::core::kind::{Cache, Row};
use crate::core::yaml::{Comment, Text};
use crate::pipeline::{Job, Rule, Stage};
use crate::system::System;

/// The image every job runs in.
pub(crate) const NIX_IMAGE: &str = "nixos/nix";

/// The nix settings every job needs: flakes, which the image leaves off.
const FLAKES: &str = "experimental-features = nix-command flakes\n";

/// `GIT_DEPTH` per kind (`src/kind:V53`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Depth {
    /// Full history: tdd-order reads the push range, a semver step the
    /// last tag, and a bump branch is pushed from it.
    Full,
    /// One commit: a build needs only the tree.
    Shallow,
}

/// What a kind decides about one job; the rest comes from the row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NixJob<'a> {
    /// The name after `<id>:`, e.g. `gate:x86_64-linux`.
    pub(crate) name: String,
    /// The stage.
    pub(crate) stage: Stage,
    /// The system whose runner tags the job carries.
    pub(crate) system: System,
    /// The clone depth.
    pub(crate) depth: Depth,
    /// When the job runs.
    pub(crate) rules: Vec<Rule>,
    /// The commands, one per line.
    pub(crate) script: Vec<Text>,
    /// The comment above the job.
    pub(crate) why: Option<&'a str>,
}

impl NixJob<'_> {
    /// A job named `name` on `stage` running `script`: on x86_64-linux,
    /// shallow, never on a schedule, with no comment. Kinds override the
    /// rest with struct update syntax.
    pub(crate) fn on(name: String, stage: Stage, script: Vec<Text>) -> Self {
        Self {
            name,
            stage,
            system: System::X86_64Linux,
            depth: Depth::Shallow,
            rules: super::rules::change_rules(),
            script,
            why: None,
        }
    }
}

/// The job: `<id>:<name>`, interruptible unless it is a bump
/// (`src/kind:V12`), which instead takes the row's resource group.
pub(crate) fn nix_job(
    row: &Row,
    tables: &Tables,
    spec: NixJob<'_>,
) -> Result<Job, KindError> {
    let bump = spec.stage == Stage::Bump;
    Ok(Job {
        name: format!("{}:{}", row.id, spec.name),
        comment: spec.why.map(Comment::new),
        stage: spec.stage,
        image: tables.pins.image(NIX_IMAGE)?,
        tags: tables.runners.tags(spec.system)?.to_vec(),
        timeout: row.args.timeout()?,
        interruptible: !bump,
        resource_group: bump.then(|| format!("lablith-{}", row.id)),
        needs: Vec::new(),
        rules: spec.rules,
        variables: variables(spec.depth, row.cache.as_ref()),
        script: spec.script,
    })
}

/// `GIT_DEPTH` and `NIX_CONFIG`.
fn variables(depth: Depth, cache: Option<&Cache>) -> Vec<(&'static str, Text)> {
    let depth = match depth {
        Depth::Full => "0",
        Depth::Shallow => "1",
    };
    let config = cache.map_or_else(String::new, Cache::nix_config);
    vec![
        ("GIT_DEPTH", Text::lit(depth)),
        ("NIX_CONFIG", Text::lit(&format!("{FLAKES}{config}"))),
    ]
}

#[cfg(test)]
mod tests;
