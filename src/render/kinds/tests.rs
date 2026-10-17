use super::{entries, lookup, names};
use crate::core::kind::CATALOGUE;

#[test]
fn the_v1_kinds_are_pipelith_catalogue_in_order() {
    let ours: Vec<&str> = entries().iter().map(|k| k.spec().name).collect();
    let theirs: Vec<&str> = CATALOGUE.iter().map(|k| k.name).collect();
    assert_eq!(ours, theirs);
}

#[test]
fn lookup_finds_a_kind_by_name_only() {
    assert_eq!(lookup("build").map(|k| k.spec().name), Some("build"));
    assert!(lookup("lint").is_none());
    assert_eq!(names(), "`gate`, `build`, `update-pins`, `upstream`");
}
