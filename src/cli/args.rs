//! The argv table: verbs, flags and `--help` text with examples (`.:I`,
//! `src/cli:T31`). One table feeds parsing, completions and the man page.

use std::path::PathBuf;

use clap::{Args, CommandFactory as _, Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

use crate::report::Format;

/// Every verb with an example, after the top-level help.
const EXAMPLES: &str = "\
Examples:
  lablith gen                    write the GitLab CI files the rows render
  lablith gen --check            exit 1 naming each stale or missing file
  lablith check                  exit 1 naming each undeclared CI file
  lablith portable               exit 1 naming each row actlith cannot render
  lablith detect                 print rows proposed from the tracked files
  lablith seed --init            write a first .lablith from detect
  lablith report --format json   rows x systems x images, secrets, schedules
  lablith pin --dry-run          print fresh `## images` rows (network)
  lablith completions zsh        print a shell completion script
  lablith man                    print the man page (roff)

Exit status: 0 ok, 1 finding, 2 usage, unreadable input or refused row.";

/// `lablith`: materialize GitLab CI pipelines from `.lablith`.
#[derive(Debug, Parser)]
#[command(name = "lablith", version, after_help = EXAMPLES)]
pub(crate) struct Cli {
    /// The verb.
    #[command(subcommand)]
    pub(crate) verb: Verb,
}

/// The repository root, shared by the verbs that read it.
#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub(crate) struct Root {
    /// The repository root holding `.lablith`.
    #[arg(long, value_name = "DIR", default_value = ".")]
    pub(crate) root: PathBuf,
}

/// The verbs (`.:I` cmd).
#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub(crate) enum Verb {
    /// Write `.gitlab-ci.yml`, `.gitlab/ci/<id>.yml` for each row and the
    /// scripts the kinds need.
    #[command(after_help = "Example:\n  lablith gen --check --root ../repo")]
    Gen {
        #[command(flatten)]
        root: Root,
        /// Write nothing; exit 1 naming each stale or missing file.
        #[arg(long)]
        check: bool,
    },
    /// Every CI file is generated or hand-written, and every row moves to
    /// actlith.
    #[command(after_help = "Example:\n  lablith check --root ../repo")]
    Check(Root),
    /// Exit 1 naming each row, kind or arg key actlith cannot render.
    #[command(after_help = "Example:\n  lablith portable")]
    Portable(Root),
    /// Print `## workflows` rows proposed from the tracked files.
    #[command(after_help = "Example:\n  lablith detect")]
    Detect(Root),
    /// Write the first `.lablith` from `detect`; never over one.
    #[command(after_help = "Example:\n  lablith seed --init")]
    Seed {
        #[command(flatten)]
        root: Root,
        /// Write `.lablith` (required: `seed` does nothing without it).
        #[arg(long)]
        init: bool,
    },
    /// Report rows x systems x image pins, secrets, schedules and token.
    #[command(after_help = "Example:\n  lablith report --format md")]
    Report {
        #[command(flatten)]
        root: Root,
        /// How to print the report.
        #[arg(long, value_enum, default_value_t = ReportFormat::Text)]
        format: ReportFormat,
    },
    /// Resolve every image tag to its index digest and write
    /// `## images` (the only networked verb).
    #[command(after_help = "Example:\n  lablith pin --dry-run")]
    Pin {
        #[command(flatten)]
        root: Root,
        /// Print the rows; write nothing.
        #[arg(long)]
        dry_run: bool,
    },
    /// Print a shell completion script.
    #[command(after_help = "Example:\n  lablith completions zsh > _lablith")]
    Completions {
        /// The shell.
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Print the man page (roff).
    #[command(after_help = "Example:\n  lablith man > lablith.1")]
    Man,
}

/// `report --format`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum ReportFormat {
    /// Indented plain text.
    Text,
    /// JSON with `schema_version`.
    Json,
    /// Markdown tables.
    Md,
}

impl From<ReportFormat> for Format {
    fn from(format: ReportFormat) -> Self {
        match format {
            ReportFormat::Text => Self::Text,
            ReportFormat::Json => Self::Json,
            ReportFormat::Md => Self::Md,
        }
    }
}

/// The clap command the argv table builds.
pub(crate) fn command() -> clap::Command {
    Cli::command()
}

#[cfg(test)]
mod tests;
