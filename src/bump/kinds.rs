//! Kinds `update-pins` (`src/bump:T21`) and `upstream` (`src/bump:T22`):
//! one job `<id>:<kind>` on the row's one system, run only by its own
//! schedule or web run (`LABLITH_ROW`, `src/bump:V17`), never
//! interruptible, in the row's resource group (`src/kind:V12`). The
//! seeded script does the work inside the project's dev shell and opens
//! the merge request through `open-mr.sh` with the row's token.

use crate::core::yaml::Text;
use crate::kind::{
    Depth, Kind, KindError, KindSpec, NixJob, Row, Secret, Tables, UPDATE_PINS,
    UPSTREAM, bump_rules, nix_job, quote,
};
use crate::pipeline::{Job, Stage};
use crate::script::Script;
use crate::system::System;

/// The kind `update-pins`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct UpdatePins;

/// The kind `upstream`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Upstream;

const PINS_WHY: &str = "Follow nixpkgs-lock and propose the bump as a \
    merge request. Started\nonly by this row's schedule or a web run with \
    LABLITH_ROW set to its id.";

const UPSTREAM_WHY: &str = "Poll upstream for a new release and propose \
    the bump as a merge request.\nThe bump script prints \
    `NEW_VERSION=<version>`; nothing printed, no merge\nrequest.";

impl Kind for UpdatePins {
    fn spec(&self) -> &'static KindSpec {
        &UPDATE_PINS
    }

    fn secrets(&self, row: &Row) -> Vec<Secret> {
        vec![Secret::open_change(row.args.get("token"))]
    }

    fn scripts(&self, _: &Row) -> Vec<Script> {
        vec![Script::UpdatePins, Script::OpenMr]
    }

    fn jobs(&self, row: &Row, tables: &Tables) -> Result<Vec<Job>, KindError> {
        let build = row.args.get("build").unwrap_or(".#default");
        let args = [quote(build)];
        bump(row, tables, (Script::UpdatePins, &args), PINS_WHY)
    }
}

impl Kind for Upstream {
    fn spec(&self) -> &'static KindSpec {
        &UPSTREAM
    }

    fn secrets(&self, row: &Row) -> Vec<Secret> {
        vec![Secret::open_change(row.args.get("token"))]
    }

    fn scripts(&self, _: &Row) -> Vec<Script> {
        vec![Script::Upstream, Script::OpenMr]
    }

    fn jobs(&self, row: &Row, tables: &Tables) -> Result<Vec<Job>, KindError> {
        let script = row.args.get("script").unwrap_or_default();
        let prefix = row.args.get("branch-prefix").unwrap_or("upstream/");
        let args = [quote(script), quote(prefix)];
        bump(row, tables, (Script::Upstream, &args), UPSTREAM_WHY)
    }
}

/// The one bump job: `nix develop --command bash <script> <args>`.
fn bump(
    row: &Row,
    tables: &Tables,
    (script, args): (Script, &[String]),
    why: &'static str,
) -> Result<Vec<Job>, KindError> {
    // pipelith gives a bump row exactly one system (src/bump:V47).
    let system = row.systems.first().copied().unwrap_or(System::X86_64Linux);
    let line = Text::lit(&command(script, args));
    let spec = NixJob {
        system,
        depth: Depth::Full,
        rules: bump_rules(&row.id),
        why: Some(why),
        ..NixJob::on(kind_name(script), Stage::Bump, vec![line])
    };
    let mut job = nix_job(row, tables, spec)?;
    let token = Secret::open_change(row.args.get("token")).name;
    job.variables
        .push(("LABLITH_TOKEN_NAME", Text::lit(&token)));
    Ok(vec![job])
}

/// The script line, inside the project's dev shell.
fn command(script: Script, args: &[String]) -> String {
    let path = script.path();
    format!("nix develop --command bash {path} {}", args.join(" "))
}

/// The job name after `<id>:`: the kind's.
fn kind_name(script: Script) -> String {
    match script {
        Script::Upstream => "upstream",
        _ => "update-pins",
    }
    .to_owned()
}

#[cfg(test)]
mod tests;
