//! Tests for the lablith-specific detection wrapper.

use super::detect;

#[test]
fn keeps_neutral_detection_when_no_flake_is_available() {
    let rows = detect(&["flake.nix".to_owned()], None, None);
    assert!(rows.first().is_some_and(|row| row.systems.is_empty()));
}
