//! The formats `report` prints (`.:I` `--format`).

/// How the report is printed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Format {
    /// Indented plain text, for a terminal.
    Text,
    /// JSON with `schema_version` (`src/report:V36`), for tools.
    Json,
    /// Markdown tables, for a merge request or an issue.
    Md,
}
