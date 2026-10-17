//! The report as a JSON document (`src/report:V36`): `schema_version`
//! first, then the sections. A field renamed or removed bumps the
//! version.

use super::json::Json;
use super::model::{Manual, Need, Report, RowReport, Schedule, Tagged};

/// The JSON layout version; bump on a field rename or removal.
pub(crate) const SCHEMA_VERSION: u32 = 1;

/// The whole report.
pub(crate) fn document(report: &Report) -> Json {
    let token = report.token.map_or(Json::Null, s);
    Json::Object(vec![
        ("schema_version", Json::Number(SCHEMA_VERSION)),
        ("rows", array(&report.rows, row)),
        ("hand_written", array(&report.hand_written, manual)),
        ("token", token),
    ])
}

fn array<T>(items: &[T], each: fn(&T) -> Json) -> Json {
    Json::Array(items.iter().map(each).collect())
}

fn s(text: &str) -> Json {
    Json::String(text.to_owned())
}

fn row(r: &RowReport) -> Json {
    let image = |(i, v): &(String, String)| {
        Json::Object(vec![("image", s(i)), ("version", s(v))])
    };
    Json::Object(vec![
        ("id", s(&r.id)),
        ("kind", s(r.kind)),
        ("systems", array(&r.systems, tagged)),
        ("cache", r.cache.as_deref().map_or(Json::Null, s)),
        ("images", Json::Array(r.images.iter().map(image).collect())),
        ("secrets", array(&r.secrets, need)),
        ("schedule", r.schedule.as_ref().map_or(Json::Null, schedule)),
    ])
}

fn tagged(t: &Tagged) -> Json {
    let tags = t.tags.iter().map(|t| s(t)).collect();
    Json::Object(vec![("system", s(t.system)), ("tags", Json::Array(tags))])
}

fn need(n: &Need) -> Json {
    Json::Object(vec![("name", s(&n.name)), ("purpose", s(n.purpose))])
}

fn schedule(c: &Schedule) -> Json {
    Json::Object(vec![("cron", s(&c.cron)), ("variable", s(&c.variable))])
}

fn manual(m: &Manual) -> Json {
    Json::Object(vec![("path", s(&m.path)), ("reason", s(&m.reason))])
}

#[cfg(test)]
mod tests;
