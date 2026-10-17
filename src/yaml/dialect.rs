//! GitLab's variable syntax for pipelith's writer (`src/yaml:V18`).
//!
//! An expression is a CI/CD variable, written `$NAME`; GitLab expands it
//! in `rules: if:`, `variables:`, `image:` and `tags:`. Literal text needs
//! no escape: lablith writes caller text only into `script:` lines, which
//! GitLab hands to the shell unexpanded (as GitHub hands `run:` to it),
//! and builds every `rules:` and `variables:` value from constants and
//! row ids, whose charset holds no `$`.

use crate::core::yaml::Dialect;

/// GitLab CI/CD variables.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Gitlab;

/// The dialect every pipeline file lablith writes is in.
pub(crate) const GITLAB: Gitlab = Gitlab;

impl Dialect for Gitlab {
    /// `$inner` when `inner` is a variable name (`[A-Za-z_][A-Za-z0-9_]*`);
    /// `None` otherwise, since anything else would read as more than one
    /// variable or as text (an injected expression).
    fn expr(&self, inner: &str) -> Option<String> {
        name(inner).then(|| format!("${inner}"))
    }

    /// Literal text, unchanged (see the module docs).
    fn escape(&self, lit: &str) -> String {
        lit.to_owned()
    }
}

/// Whether `s` is a CI/CD variable name.
fn name(s: &str) -> bool {
    let mut chars = s.chars();
    let first = chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_');
    first && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests;
