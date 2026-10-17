//! Kind `gate` (`src/gate:T19`): one job `<id>:gate:<system>` per system,
//! full history, the row's command. With a cache, the command runs
//! through `cachix.sh`, which pushes only where the pipeline may
//! (`src/kind:V14`).

use crate::core::yaml::Text;
use crate::kind::{
    Depth, GATE, Kind, KindError, KindSpec, NixJob, Row, Secret, Tables,
    change_rules, nix_job, quote,
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
        row.cache.iter().map(|_| Secret::cachix()).collect()
    }

    fn scripts(&self, row: &Row) -> Vec<Script> {
        row.cache.iter().map(|_| Script::Cachix).collect()
    }

    fn jobs(&self, row: &Row, tables: &Tables) -> Result<Vec<Job>, KindError> {
        let script = vec![command(row)];
        let job = |&system: &System| {
            let spec = NixJob {
                name: format!("gate:{}", system.name()),
                stage: Stage::Gate,
                system,
                depth: Depth::Full,
                rules: change_rules(),
                script: script.clone(),
                why: Some(WHY),
            };
            nix_job(row, tables, spec)
        };
        row.systems.iter().take(1).map(job).collect()
    }
}

/// The row's command, through `cachix.sh` when the row names a cache.
fn command(row: &Row) -> Text {
    let command = row.args.get("command").unwrap_or(COMMAND);
    match &row.cache {
        Some(cache) => Text::lit(&format!(
            "bash {} {} {}",
            Script::Cachix.path(),
            cache.name(),
            {
                let _ = quote;
                command
            }
        )),
        None => Text::lit(command),
    }
}

#[cfg(test)]
mod tests;
