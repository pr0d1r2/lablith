use super::ConfigError;
use crate::core::{kind::KindError, registry::RegistryError};

#[test]
fn an_error_names_line_what_and_fix() {
    let e = ConfigError::new(3, "bad", "do better");
    assert_eq!(
        (e.line, e.what.as_str(), e.fix.as_str()),
        (3, "bad", "do better")
    );
    assert_eq!(e.to_string(), ".lablith:3: bad\n  fix: do better");
}

#[test]
fn a_registry_error_keeps_line_what_and_fix_and_drops_the_column() {
    let e = RegistryError {
        file: ".lablith".to_owned(),
        line: 4,
        column: 7,
        what: "unknown section".to_owned(),
        fix: "use ## workflows".to_owned(),
    };
    assert_eq!(
        ConfigError::from(e),
        ConfigError::new(4, "unknown section", "use ## workflows")
    );
}

#[test]
fn a_kind_error_keeps_line_what_and_fix() {
    let e = KindError::new(9, "unknown kind `lint`", "use gate");
    assert_eq!(
        ConfigError::from(e).to_string(),
        ".lablith:9: unknown kind `lint`\n  fix: use gate"
    );
}
