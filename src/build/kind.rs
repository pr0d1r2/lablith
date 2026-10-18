//! Kind `build` (`src/build:T24`): `<id>:build:<system>` per system
//! through `cachix.sh`, `<id>:check` (`nix flake check --all-systems`,
//! then an eval of the package on each system with no runner tags), and
//! `<id>:verify-cache` on the default branch after every build
//! (`src/build:V14`). pipelith refuses a `build` row without a cache.

use crate::core::kind::Cache;
use crate::core::yaml::Text;
use crate::kind::{
    BUILD, Kind, KindError, KindSpec, NixJob, Row, Secret, Tables,
    default_branch_rules, nix_job, quote, runnable,
};
use crate::pipeline::{Job, Stage};
use crate::script::Script;
use crate::system::System;

/// Every system pipelith knows; those without tags are evaluated only.
const KNOWN: [System; 4] = [
    System::X86_64Linux,
    System::Aarch64Linux,
    System::Aarch64Darwin,
    System::X86_64Darwin,
];

const BUILD_WHY: &str = "One native runner per system: a cross-built path \
    is a different store\npath, never the one consumers substitute.";

const CHECK_WHY: &str = "--all-systems is the point: a bare `nix flake \
    check` skips every system\nthe runner cannot build. Systems without \
    runner tags are evaluated, so\nthey cannot rot silently.";

const VERIFY_WHY: &str = "A failed cachix push does not fail its job, so \
    this asks the cache\nitself for every built path, after the builds.";

/// The kind `build`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Build;

impl Kind for Build {
    fn spec(&self) -> &'static KindSpec {
        &BUILD
    }

    fn secrets(&self, _: &Row) -> Vec<Secret> {
        vec![Secret::cachix()]
    }

    fn scripts(&self, _: &Row) -> Vec<Script> {
        vec![Script::Cachix, Script::VerifyCache]
    }

    fn jobs(&self, row: &Row, tables: &Tables) -> Result<Vec<Job>, KindError> {
        let plan = Plan::new(row, tables)?;
        let mut jobs = plan
            .systems
            .iter()
            .map(|&s| nix_job(row, tables, build(&plan, s)))
            .collect::<Result<Vec<_>, _>>()?;
        let needs = jobs.iter().map(|j| j.name.clone()).collect();
        jobs.push(nix_job(row, tables, check(&plan, tables))?);
        let mut verify = nix_job(row, tables, verify(&plan))?;
        verify.needs = needs;
        jobs.push(verify);
        Ok(jobs)
    }
}

/// What every job of a row needs: its package, cache and the systems
/// this forge builds it on.
struct Plan<'a> {
    package: &'a str,
    cache: &'a Cache,
    systems: Vec<System>,
}

impl<'a> Plan<'a> {
    /// The row's package and cache; pipelith already refused a row
    /// missing either, so this only names the row if one is gone.
    fn new(row: &'a Row, tables: &Tables) -> Result<Self, KindError> {
        let missing = || {
            KindError::row(
                row.line,
                "build needs package and cache",
                "write `package=<attr>` and a cache",
            )
        };
        let package = row.args.get("package").ok_or_else(missing)?;
        let cache = row.cache.as_ref().ok_or_else(missing)?;
        Ok(Self {
            package,
            cache,
            systems: runnable(row, tables)?,
        })
    }
}

/// `<id>:build:<system>`: build and push the system's package.
fn build(plan: &Plan<'_>, system: System) -> NixJob<'static> {
    let attr = format!(".#packages.{}.{}", system.name(), plan.package);
    let command = format!("nix build --print-build-logs {attr}");
    let cachix = (Script::Cachix.path(), plan.cache.name());
    let line = format!("bash {} {} {}", cachix.0, cachix.1, quote(&command));
    let name = format!("build:{}", system.name());
    NixJob {
        system,
        why: Some(BUILD_WHY),
        ..NixJob::on(name, Stage::Build, vec![Text::lit(&line)])
    }
}

/// `<id>:check`: the flake on every system, eval where nothing builds.
fn check(plan: &Plan<'_>, tables: &Tables) -> NixJob<'static> {
    let flake = "nix flake check --all-systems --print-build-logs";
    let untagged = KNOWN
        .into_iter()
        .filter(|&s| tables.runners.tags(s).is_err());
    let eval = |s: System| {
        let attr = format!(".#packages.{}.{}.drvPath", s.name(), plan.package);
        Text::lit(&format!("nix eval --raw {attr}"))
    };
    let script = std::iter::once(Text::lit(flake)).chain(untagged.map(eval));
    NixJob {
        why: Some(CHECK_WHY),
        ..NixJob::on("check".into(), Stage::Build, script.collect())
    }
}

/// `<id>:verify-cache`: the seeded script asks the cache for every
/// built system's path, on the default branch only.
fn verify(plan: &Plan<'_>) -> NixJob<'static> {
    let systems: Vec<&str> = plan.systems.iter().map(|s| s.name()).collect();
    let script = Script::VerifyCache.path();
    let (cache, package) = (plan.cache.name(), plan.package);
    let line = format!("bash {script} {cache} {package} {}", systems.join(" "));
    NixJob {
        rules: default_branch_rules(),
        why: Some(VERIFY_WHY),
        ..NixJob::on(
            "verify-cache".into(),
            Stage::Verify,
            vec![Text::lit(&line)],
        )
    }
}

#[cfg(test)]
mod tests;
