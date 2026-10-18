//! `.lablith` rows into the sorted [`Report`]: each row checked and built
//! by its kind (`src/render`'s plan), so the images listed are the ones
//! its jobs run and the secrets the ones its kind declares
//! (`src/kind:V32`).

use std::collections::BTreeSet;

use super::model::{Manual, Need, Report, RowReport, Schedule, Tagged};
use crate::core::kind::Purpose;
use crate::kind::{ROW, Tables};
use crate::registry::Registry;
use crate::render::{RenderError, RowPlan, plan, tables};

/// What a merge request token must be (`src/report:V64`).
pub(crate) const TOKEN: &str = "a project access token, role Developer, \
    scope `api`, expiring within a year; store it as a masked, protected \
    CI/CD variable and rotate it before it expires";

/// The report of `registry`.
///
/// # Errors
///
/// As `src/render`'s plan: a row naming no v1 kind or one its kind
/// refuses.
pub(crate) fn collect(registry: &Registry) -> Result<Report, RenderError> {
    let tables = tables(registry)?;
    let mut rows: Vec<RowReport> =
        plan(registry)?.iter().map(|p| row(p, &tables)).collect();
    rows.sort_by(|a, b| a.id.cmp(&b.id));
    let opens = rows
        .iter()
        .flat_map(|r| &r.secrets)
        .any(|n| n.purpose == why(Purpose::OpenChange));
    Ok(Report {
        rows,
        hand_written: manual(registry),
        token: opens.then_some(TOKEN),
    })
}

/// One planned row.
fn row(plan: &RowPlan, tables: &Tables) -> RowReport {
    RowReport {
        id: plan.row.id.clone(),
        kind: plan.kind.spec().name,
        systems: plan
            .row
            .systems
            .iter()
            .map(|&s| tagged(s, tables))
            .collect(),
        cache: plan.row.cache.as_ref().map(|c| c.name().to_owned()),
        images: images(plan),
        secrets: secrets(plan),
        schedule: schedule(plan),
    }
}

/// A system with the tags its jobs carry.
fn tagged(system: crate::system::System, tables: &Tables) -> Tagged {
    let tags = tables.runners.tags(system).map(<[String]>::to_vec);
    Tagged {
        system: system.name(),
        tags: tags.unwrap_or_default(),
        skipped: false,
    }
}

/// Every image the row's jobs run, once each, by reference.
fn images(plan: &RowPlan) -> Vec<(String, String)> {
    let all = plan
        .jobs
        .iter()
        .map(|j| (j.image.reference(), j.image.version.clone()));
    all.collect::<BTreeSet<_>>().into_iter().collect()
}

/// The secrets the row's kind declares, once each, by name.
fn secrets(plan: &RowPlan) -> Vec<Need> {
    let needs = plan.kind.secrets(&plan.row).into_iter().map(|s| Need {
        purpose: s.why(),
        name: s.name,
    });
    needs.collect::<BTreeSet<_>>().into_iter().collect()
}

/// The schedule a scheduled kind needs: its `cron` and `LABLITH_ROW`.
fn schedule(plan: &RowPlan) -> Option<Schedule> {
    let cron = plan.row.args.get("cron")?;
    Some(Schedule {
        cron: cron.to_owned(),
        variable: format!("{ROW}={}", plan.row.id),
    })
}

/// The words for `purpose`, as `Secret::why` gives them.
fn why(purpose: Purpose) -> &'static str {
    crate::kind::Secret {
        name: String::new(),
        purpose,
    }
    .why()
}

/// `## hand-written` rows, by path.
fn manual(registry: &Registry) -> Vec<Manual> {
    let mut rows: Vec<Manual> = registry
        .hand_written
        .iter()
        .map(|h| Manual {
            path: h.path.clone(),
            reason: h.reason.clone(),
        })
        .collect();
    rows.sort_by(|a, b| a.path.cmp(&b.path));
    rows
}

#[cfg(test)]
mod tests;
