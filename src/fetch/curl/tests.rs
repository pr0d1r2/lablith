use super::{Curl, Http, parse};
use std::ffi::OsString;

#[test]
fn the_last_response_wins_and_names_are_lower_cased() {
    let text = "HTTP/1.1 100 Continue\r\n\r\nHTTP/2 200\r\nContent-Type: a/b\r\nDocker-Content-Digest: sha256:x\r\n";
    let r = parse(text);
    assert_eq!(r.status, 200);
    assert_eq!(r.header("content-type"), Some("a/b"));
    assert_eq!(r.header("docker-content-digest"), Some("sha256:x"));
    assert_eq!(r.header("missing"), None);
}

#[test]
fn an_unreadable_status_is_zero() {
    assert_eq!(parse("HTTP/2 abc\r\n").status, 0);
    assert_eq!(parse("").status, 0);
}

/// `bash -c SCRIPT` standing in for curl: `$@` are curl's arguments.
fn fake(script: &str) -> Curl {
    let prefix = ["-c", script, "curl"].map(OsString::from).to_vec();
    Curl::with("bash", prefix)
}

#[test]
fn head_passes_its_headers_and_reads_the_answer() {
    let curl = fake("printf 'HTTP/2 200\\nx-args: %s\\n' \"$*\"");
    let r = curl
        .head("https://h/v2", &["Accept: a".to_owned()])
        .unwrap_or_default();
    assert_eq!(r.status, 200);
    assert_eq!(r.header("x-args"), Some("-sS -I -H Accept: a https://h/v2"));
}

#[test]
fn get_returns_the_body_or_why_it_failed() {
    let ok = fake("printf '%s' \"$*\"").get("https://h/t");
    assert_eq!(ok, Ok("-fsS https://h/t".to_owned()));
    let bad = fake("echo 'curl: (22) 404' >&2; exit 22").get("https://h/t");
    assert_eq!(bad, Err("curl: (22) 404".to_owned()));
    let missing = Curl::with("no-such-program-lablith", Vec::new()).get("x");
    assert!(missing.is_err_and(|e| e.starts_with("cannot run")));
    assert_eq!(Curl::new(), Curl::with("curl", Vec::new()));
}
