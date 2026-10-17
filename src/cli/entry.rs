//! [`lablith`]: the entry point `main` calls (`src:V37`), and the dispatch
//! from a parsed verb to its node (`src/cli:V34`).

use std::ffi::OsString;
use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser as _;

use super::args::{Cli, Verb};
use super::exit::{OK, Outcome, USAGE};
use super::files::{check, generate, portable};
use super::meta::{completions, man};
use super::pin::pin;
use super::report::report;
use super::rows::{detect, seed};
use crate::fetch::{Curl, Oci, Resolver};

/// Run `lablith` on this process's argv and streams; `pin` reaches the
/// image registries through `curl`. Returns the exit code
/// (`src/cli:V34`).
#[must_use]
pub fn lablith() -> ExitCode {
    let (mut out, mut err) = (io::stdout().lock(), io::stderr().lock());
    let streams: [&mut dyn Write; 2] = [&mut out, &mut err];
    ExitCode::from(run(std::env::args_os(), streams, &Oci(Curl::new())))
}

/// Parse `args` (program name first), run the verb, print its results
/// to `streams[0]` and diagnostics to `streams[1]`; the exit code.
pub(crate) fn run(
    args: impl IntoIterator<Item = OsString>,
    streams: [&mut dyn Write; 2],
    resolver: &dyn Resolver,
) -> u8 {
    let outcome = match Cli::try_parse_from(args) {
        Ok(cli) => dispatch(cli.verb, resolver),
        Err(error) => parse_failed(&error),
    };
    let [out, err] = streams;
    let _ = out
        .write_all(outcome.out.as_bytes())
        .and_then(|()| out.flush());
    let _ = err.write_all(outcome.err.as_bytes());
    outcome.code
}

/// Run `verb` on its node.
fn dispatch(verb: Verb, resolver: &dyn Resolver) -> Outcome {
    let ran = match verb {
        Verb::Gen { root, check } => generate(&root.root, check),
        Verb::Check(root) => check(&root.root),
        Verb::Portable(root) => portable(&root.root),
        Verb::Detect(root) => detect(&root.root),
        Verb::Seed { root, init } => seed(&root.root, init),
        Verb::Report { root, format } => report(&root.root, format.into()),
        Verb::Pin { root, dry_run } => pin(&root.root, dry_run, resolver),
        Verb::Completions { shell } => Ok(completions(shell)),
        Verb::Man => Ok(man()),
    };
    ran.unwrap_or_else(|outcome| outcome)
}

/// clap's text: help and version on stdout with exit 0; a usage error
/// on stderr with exit 2 (it ends by naming `--help`).
fn parse_failed(error: &clap::Error) -> Outcome {
    let text = error.render().to_string();
    if error.use_stderr() {
        return Outcome::fail(USAGE, text);
    }
    Outcome {
        code: OK,
        out: text,
        err: String::new(),
    }
}

#[cfg(test)]
mod tests;
