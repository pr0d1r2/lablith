//! The report as Markdown tables, for a merge request or an issue.

use super::model::{Manual, Report, RowReport};

/// `report` as Markdown: a table of rows, of secrets, of schedules, of
/// hand-written files, then the token note.
pub(crate) fn md(report: &Report) -> String {
    let rows: Vec<_> = report.rows.iter().map(row).collect();
    let secrets: Vec<_> = report.rows.iter().flat_map(secrets).collect();
    let schedules: Vec<_> = report.rows.iter().filter_map(schedule).collect();
    let manual: Vec<_> = report.hand_written.iter().map(hand).collect();
    let mut out = [
        table("Rows", &["id", "kind", "systems", "cache", "images"], &rows),
        table("Secrets", &["row", "variable", "purpose"], &secrets),
        table("Schedules", &["row", "cron", "variable"], &schedules),
        table("Hand-written", &["path", "reason"], &manual),
    ]
    .concat();
    if let Some(token) = report.token {
        out = out + "## Token\n\n" + token + "\n";
    }
    out
}

/// `raw` as inline code, its pipes escaped for a table cell.
fn code(raw: &str) -> String {
    format!("`{}`", raw.replace('|', "\\|"))
}

fn table(heading: &str, header: &[&str], rows: &[Vec<String>]) -> String {
    let mut out = format!("## {heading}\n\n");
    if rows.is_empty() {
        out.push_str("None.\n\n");
        return out;
    }
    out.push_str(&line(header));
    out.push_str(&line(&vec!["---"; header.len()]));
    for r in rows {
        out.push_str(&line(r));
    }
    out.push('\n');
    out
}

fn line<S: AsRef<str>>(cells: &[S]) -> String {
    let cells: Vec<&str> = cells.iter().map(AsRef::as_ref).collect();
    format!("| {} |\n", cells.join(" | "))
}

fn row(r: &RowReport) -> Vec<String> {
    let systems: Vec<&str> = r.systems.iter().map(|t| t.system).collect();
    let images: Vec<String> = r
        .images
        .iter()
        .map(|(i, v)| format!("{} {v}", code(i)))
        .collect();
    vec![
        code(&r.id),
        code(r.kind),
        systems.join(", "),
        r.cache.as_deref().map_or_else(|| "-".to_owned(), code),
        images.join(", "),
    ]
}

fn secrets(r: &RowReport) -> Vec<Vec<String>> {
    let one = |n: &super::model::Need| {
        vec![code(&r.id), code(&n.name), n.purpose.to_owned()]
    };
    r.secrets.iter().map(one).collect()
}

fn schedule(r: &RowReport) -> Option<Vec<String>> {
    let s = r.schedule.as_ref()?;
    Some(vec![code(&r.id), code(&s.cron), code(&s.variable)])
}

fn hand(m: &Manual) -> Vec<String> {
    vec![code(&m.path), m.reason.replace('|', "\\|")]
}

#[cfg(test)]
mod tests;
