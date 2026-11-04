//! Kind `gate` (`src/gate:T19`): one job `<id>:gate:<system>` per system,
//! full history, the row's command. With a cache, the command runs
//! through `cachix.sh`, which pushes only where the pipeline may
//! (`src/kind:V14`).

use crate::cache::{self, Branches};
use crate::core::yaml::Text;
use crate::kind::{
    Depth, GATE, Kind, KindError, KindSpec, NixJob, Row, Secret, Tables,
    change_rules, nix_job, runnable,
};
use crate::pipeline::{Job, Stage};
use crate::script::Script;
use crate::system::System;

/// The gate run when the row gives no `command` (pipelith's default).
const COMMAND: &str = "nix develop --command hk check --all --check";

const WHY: &str = "The hk gate on this system. Full history: tdd-order \
    reads the push\nrange, and a semver step compares with the last \
    release tag.";

/// The kind `gate`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Gate;

impl Kind for Gate {
    fn spec(&self) -> &'static KindSpec {
        &GATE
    }

    fn secrets(&self, row: &Row) -> Vec<Secret> {
        cache::secrets(row.cache.as_ref())
    }

    fn scripts(&self, row: &Row) -> Vec<Script> {
        cache::scripts(row.cache.as_ref(), false)
    }

    fn jobs(&self, row: &Row, tables: &Tables) -> Result<Vec<Job>, KindError> {
        let job = |&system: &System| {
            let script = vec![command(row, system)];
            let spec = NixJob {
                name: format!("gate:{}", system.name()),
                stage: Stage::Gate,
                system,
                depth: Depth::Full,
                rules: change_rules(),
                script,
                why: Some(WHY),
            };
            nix_job(row, tables, spec)
        };
        runnable(row, tables)?.iter().map(job).collect()
    }
}

/// The row's command, pushing to the row's cache on any ref when it may
/// (`src/cache:V82`); an https push copies the system's dev shell inputs,
/// everything `nix develop` needs, first.
fn command(row: &Row, system: System) -> Text {
    let command = row.args.get("command").unwrap_or(COMMAND);
    let shell =
        format!(".#devShells.{}.default.inputDerivation", system.name());
    Text::lit(&row.cache.as_ref().map_or_else(
        || command.to_owned(),
        |c| cache::push_line(c, Branches::Any, command, &shell),
    ))
}

#[cfg(test)]
mod tests;
