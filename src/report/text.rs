//! The report as indented plain text, for a terminal: a heading per
//! section, an entry per row two spaces in, its details four spaces in.

use super::model::{Manual, Report, RowReport};

/// `report` as text: `rows`, `hand-written`, then `token` when a row
/// opens merge requests; `-` for an empty section.
pub(crate) fn text(report: &Report) -> String {
    let mut out = String::new();
    section(&mut out, "rows", report.rows.iter().map(row));
    section(
        &mut out,
        "hand-written",
        report.hand_written.iter().map(manual),
    );
    if let Some(token) = report.token {
        out = out + "token\n  " + token + "\n";
    }
    out
}

fn section(
    out: &mut String,
    heading: &str,
    entries: impl Iterator<Item = String>,
) {
    out.push_str(heading);
    out.push('\n');
    let before = out.len();
    entries.for_each(|e| out.push_str(&e));
    if out.len() == before {
        out.push_str("  -\n");
    }
}

fn row(r: &RowReport) -> String {
    let cache = r.cache.as_deref().unwrap_or("-");
    let head = format!("  {}: kind {}, cache {cache}\n", r.id, r.kind);
    head + &details(r)
}

/// The row's systems, images, secrets and schedule, a line each.
fn details(r: &RowReport) -> String {
    [systems(r), images(r), secrets(r), schedule(r)]
        .concat()
        .concat()
}

fn systems(r: &RowReport) -> Vec<String> {
    let one = |t: &super::model::Tagged| {
        let what = if t.skipped {
            format!("{} skipped: no runner on this forge", t.system)
        } else {
            format!("{} tags {}", t.system, t.tags.join(", "))
        };
        line("system", &what)
    };
    r.systems.iter().map(one).collect()
}

fn images(r: &RowReport) -> Vec<String> {
    let one = |(i, v): &(String, String)| line("image", &format!("{i} {v}"));
    r.images.iter().map(one).collect()
}

fn secrets(r: &RowReport) -> Vec<String> {
    let one = |n: &super::model::Need| {
        line("secret", &format!("{}: {}", n.name, n.purpose))
    };
    r.secrets.iter().map(one).collect()
}

fn schedule(r: &RowReport) -> Vec<String> {
    let one = |s: &super::model::Schedule| {
        let on = format!("{} on the default branch", s.cron);
        line("schedule", &format!("{on}, variable {}", s.variable))
    };
    r.schedule.iter().map(one).collect()
}

/// One detail line: four spaces, the label, the value.
fn line(label: &str, value: &str) -> String {
    format!("    {label} {value}\n")
}

fn manual(m: &Manual) -> String {
    format!("  {}: {}\n", m.path, m.reason)
}

#[cfg(test)]
mod tests;
