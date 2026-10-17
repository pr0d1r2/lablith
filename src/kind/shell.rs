//! One shell word: a row's free text passed to a seeded script as one
//! argument, never spliced into the command around it.

/// `s` as a single-quoted shell word; an embedded `'` is closed, escaped
/// and reopened (`'\''`).
pub(crate) fn quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

#[cfg(test)]
mod tests;
