//! The real HTTP client: `curl` as a subprocess, so no HTTP crate joins
//! the build (`.:C` runtime deps). Repository variables are dropped, as
//! for every child process (`nix:V45`).

use std::ffi::OsString;
use std::process::Command;

/// Repository variables a subprocess must not inherit.
const GIT_REPO_VARS: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
];

/// What the OCI flow asks of HTTP: response headers of a `HEAD`, and a
/// body of a `GET`. `Err` is why the request could not be made at all.
pub(crate) trait Http {
    /// The status code and headers (lower-cased names) of `HEAD url`.
    fn head(&self, url: &str, headers: &[String]) -> Result<Response, String>;
    /// The body of `GET url`, which must answer 2xx.
    fn get(&self, url: &str) -> Result<String, String>;
}

/// A `HEAD` answer.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Response {
    /// The status code of the last response.
    pub(crate) status: u16,
    /// `(name, value)` of the last response, names lower-cased.
    pub(crate) headers: Vec<(String, String)>,
}

impl Response {
    /// The value of header `name` (lower case), if present.
    pub(crate) fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v.as_str())
    }
}

/// `curl` runner: `program prefix... ARGS`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Curl {
    program: OsString,
    prefix: Vec<OsString>,
}

impl Curl {
    /// `curl` from `PATH`.
    pub(crate) fn new() -> Self {
        Self::with("curl", Vec::new())
    }

    /// Another program in place of `curl`, with leading arguments.
    pub(crate) fn with(
        program: impl Into<OsString>,
        prefix: Vec<OsString>,
    ) -> Self {
        Self {
            program: program.into(),
            prefix,
        }
    }

    /// The command with `args`, repository vars dropped.
    fn command(&self, args: &[String]) -> Command {
        let mut command = Command::new(&self.program);
        command.args(&self.prefix).args(args);
        for var in GIT_REPO_VARS {
            command.env_remove(var);
        }
        command
    }

    /// stdout of the command, or why it failed.
    fn run(&self, args: &[String]) -> Result<String, String> {
        let out = self.command(args).output();
        let out = out.map_err(|e| {
            format!("cannot run `{}`: {e}", self.program.display())
        })?;
        let text = |b: &[u8]| String::from_utf8_lossy(b).trim().to_owned();
        if out.status.success() {
            return Ok(text(&out.stdout));
        }
        Err(text(&out.stderr))
    }
}

impl Http for Curl {
    fn head(&self, url: &str, headers: &[String]) -> Result<Response, String> {
        let mut args = vec!["-sS".to_owned(), "-I".to_owned()];
        for h in headers {
            args.extend(["-H".to_owned(), h.clone()]);
        }
        args.push(url.to_owned());
        self.run(&args).map(|text| parse(&text))
    }

    fn get(&self, url: &str) -> Result<String, String> {
        self.run(&["-fsS".to_owned(), url.to_owned()])
    }
}

/// The last response of `curl -I` output: a status line starts each
/// response (a redirect or a `100 Continue` comes first), headers follow.
pub(crate) fn parse(text: &str) -> Response {
    let mut out = Response::default();
    for line in text.lines() {
        if let Some(status) = line.strip_prefix("HTTP/").map(status) {
            out = Response {
                status,
                headers: Vec::new(),
            };
        } else if let Some(pair) = header(line) {
            out.headers.push(pair);
        }
    }
    out
}

/// The code of a status line after `HTTP/`, 0 when unreadable.
fn status(rest: &str) -> u16 {
    let code = rest.split_whitespace().nth(1);
    code.and_then(|c| c.parse().ok()).unwrap_or_default()
}

/// `(name, value)` of a header line, the name lower-cased.
fn header(line: &str) -> Option<(String, String)> {
    let (name, value) = line.split_once(':')?;
    Some((name.trim().to_ascii_lowercase(), value.trim().to_owned()))
}

#[cfg(test)]
mod tests;
