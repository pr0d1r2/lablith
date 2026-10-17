use super::{eq, eq_var, var};
use crate::yaml::GITLAB;

#[test]
fn a_var_is_dollar_name() {
    assert_eq!(var("CI_COMMIT_TAG").render(&GITLAB), "$CI_COMMIT_TAG");
}

#[test]
fn an_eq_compares_a_var_with_a_quoted_value() {
    let t = eq("CI_PIPELINE_SOURCE", "schedule");
    assert_eq!(t.render(&GITLAB), "$CI_PIPELINE_SOURCE == \"schedule\"");
}

#[test]
fn a_name_the_dialect_refuses_stays_literal() {
    assert_eq!(var("not a name").render(&GITLAB), "$not a name");
}

#[test]
fn an_eq_var_compares_two_vars() {
    let t = eq_var("CI_COMMIT_BRANCH", "CI_DEFAULT_BRANCH");
    assert_eq!(t.render(&GITLAB), "$CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH");
    let t = eq_var("A", "not a name");
    assert_eq!(t.render(&GITLAB), "$A == $not a name");
}
