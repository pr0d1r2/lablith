use super::{bump_rules, change_rules, default_branch_rules};
use crate::pipeline::{Rule, When};
use crate::yaml::GITLAB;

fn shown(rules: &[Rule]) -> Vec<(String, Option<When>)> {
    rules
        .iter()
        .map(|r| {
            let cond = r.cond.as_ref().map(|c| c.render(&GITLAB));
            (cond.unwrap_or_default(), r.when)
        })
        .collect()
}

const SCHEDULE: &str = "$CI_PIPELINE_SOURCE == \"schedule\"";

#[test]
fn change_jobs_never_run_on_a_schedule() {
    let want = [
        (SCHEDULE.to_owned(), Some(When::Never)),
        (String::new(), Some(When::OnSuccess)),
    ];
    assert_eq!(shown(&change_rules()), want);
}

#[test]
fn default_branch_jobs_run_on_the_default_branch_only() {
    let main = "$CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH".to_owned();
    let want = [(SCHEDULE.to_owned(), Some(When::Never)), (main, None)];
    assert_eq!(shown(&default_branch_rules()), want);
}

#[test]
fn bump_jobs_run_on_their_own_schedule_or_web_run_on_protected_refs() {
    let one = |source: &str| {
        format!(
            "$CI_PIPELINE_SOURCE == \"{source}\" && $LABLITH_ROW == \
             \"pins\" && $CI_COMMIT_REF_PROTECTED == \"true\""
        )
    };
    let want = [(one("schedule"), None), (one("web"), None)];
    assert_eq!(shown(&bump_rules("pins")), want);
}
