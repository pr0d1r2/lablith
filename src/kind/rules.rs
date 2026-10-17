//! The `rules:` presets every job takes one of (`src/kind:V50`).

use crate::core::yaml::Text;
use crate::pipeline::{Rule, When, concat, eq, eq_var};

/// The variable naming why a pipeline runs.
const SOURCE: &str = "CI_PIPELINE_SOURCE";

/// The schedule variable that selects one bump row (`src/bump:V17`).
pub(crate) const ROW: &str = "LABLITH_ROW";

/// Push and merge request kinds: every pipeline the workflow admits
/// except a schedule, which runs bump rows only.
pub(crate) fn change_rules() -> Vec<Rule> {
    vec![never_on_schedule(), always()]
}

/// Jobs that push to the cache or check it: the default branch only,
/// never a schedule.
pub(crate) fn default_branch_rules() -> Vec<Rule> {
    let main = eq_var("CI_COMMIT_BRANCH", "CI_DEFAULT_BRANCH");
    vec![never_on_schedule(), rule(main)]
}

/// Bump kinds: a schedule or a web run whose `LABLITH_ROW` names this
/// row, on a protected ref, where the token exists (`src/kind:V15`,
/// `src/bump:V17`).
pub(crate) fn bump_rules(id: &str) -> Vec<Rule> {
    ["schedule", "web"]
        .into_iter()
        .map(|source| rule(selected(source, id)))
        .collect()
}

/// `$CI_PIPELINE_SOURCE == "<source>" && $LABLITH_ROW == "<id>" &&
/// $CI_COMMIT_REF_PROTECTED == "true"`.
fn selected(source: &str, id: &str) -> Text {
    let and = |t: Text, b: Text| concat(t.then_lit(" && "), &b);
    let row = and(eq(SOURCE, source), eq(ROW, id));
    and(row, eq("CI_COMMIT_REF_PROTECTED", "true"))
}

/// `if: $CI_PIPELINE_SOURCE == "schedule"`, `when: never`.
fn never_on_schedule() -> Rule {
    Rule {
        cond: Some(eq(SOURCE, "schedule")),
        when: Some(When::Never),
    }
}

/// `when: on_success`, matching every pipeline.
fn always() -> Rule {
    Rule {
        cond: None,
        when: Some(When::OnSuccess),
    }
}

/// A rule that runs when `cond` holds.
fn rule(cond: Text) -> Rule {
    Rule {
        cond: Some(cond),
        when: None,
    }
}

#[cfg(test)]
mod tests;
