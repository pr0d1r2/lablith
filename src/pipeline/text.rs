//! Conditions built from CI/CD variables and constants only
//! (`src/yaml:V18`): `$NAME`, and `$NAME == "value"`.

use crate::core::yaml::{Expr, Part, Text};
use crate::yaml::GITLAB;

/// `$name` as a text. `name` is one of lablith's constant variable
/// names, so the dialect never refuses it; were it to, the text would be
/// the literal `$name`, which GitLab reads the same way.
pub(crate) fn var(name: &str) -> Text {
    Expr::new(&GITLAB, name)
        .map_or_else(|| Text::lit(&format!("${name}")), Text::expr)
}

/// `$name == "value"`; `value` is a constant or a row id, whose charset
/// holds no `"` or `$` (`src/registry:V55`).
pub(crate) fn eq(name: &str, value: &str) -> Text {
    var(name).then_lit(&format!(" == \"{value}\""))
}

/// `$a == $b`.
pub(crate) fn eq_var(a: &str, b: &str) -> Text {
    let t = var(a).then_lit(" == ");
    match Expr::new(&GITLAB, b) {
        Some(e) => t.then_expr(e),
        None => t.then_lit(&format!("${b}")),
    }
}

/// `a` followed by every part of `b`.
pub(crate) fn concat(a: Text, b: &Text) -> Text {
    b.parts().iter().fold(a, |t, part| match part {
        Part::Lit(s) => t.then_lit(s),
        Part::Expr(e) => t.then_expr(e.clone()),
    })
}

#[cfg(test)]
mod tests;
