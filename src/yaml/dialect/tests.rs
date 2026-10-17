//! GitLab's `$NAME` through pipelith's `Expr` and `Text` (`src/yaml:V18`).

use super::GITLAB;
use crate::core::yaml::{Expr, Text};

#[test]
fn expr_is_a_dollar_variable_trimmed() {
    let e = Expr::new(&GITLAB, "  CI_PIPELINE_SOURCE ");
    assert_eq!(e.as_ref().map(Expr::inner), Some("CI_PIPELINE_SOURCE"));
    assert_eq!(e.as_ref().map(Expr::text), Some("$CI_PIPELINE_SOURCE"));
    let e = Expr::new(&GITLAB, "_x9");
    assert_eq!(e.as_ref().map(Expr::text), Some("$_x9"));
}

#[test]
fn expr_that_is_not_one_variable_name_is_refused() {
    for bad in ["9X", "A B", "A-B", "$A", "A}", "${A}", "A == \"b\""] {
        assert_eq!(Expr::new(&GITLAB, bad), None, "{bad}");
    }
}

#[test]
fn a_literal_reaches_the_shell_unchanged() {
    let t = Text::lit("echo \"$HOME\" $$ ${X}");
    assert_eq!(t.render(&GITLAB), "echo \"$HOME\" $$ ${X}");
}

#[test]
fn a_text_mixes_literals_and_variables() {
    let e = Expr::new(&GITLAB, "LABLITH_ROW");
    let t = e.map(|e| Text::lit("row ").then_expr(e).then_lit("!"));
    let t = t.map(|t| t.render(&GITLAB));
    assert_eq!(t.as_deref(), Some("row $LABLITH_ROW!"));
}
