use std::path::PathBuf;

use super::FetchError;
use crate::core::ConfigError;

fn read() -> FetchError {
    FetchError::Read {
        path: PathBuf::from("a"),
        why: "w".into(),
    }
}

fn write() -> FetchError {
    FetchError::Write {
        path: PathBuf::from("a"),
        why: "w".into(),
    }
}

#[test]
fn file_errors_say_what_and_exit_two() {
    assert_eq!(
        (read().to_string(), read().code()),
        ("a: cannot read: w".to_owned(), 2)
    );
    assert_eq!(
        (write().to_string(), write().code()),
        ("a: cannot write: w".to_owned(), 2)
    );
    let config = FetchError::Config(ConfigError::new(1, "w", "f"));
    assert_eq!(
        (config.to_string(), config.code()),
        (".lablith:1: w\n  fix: f".to_owned(), 2)
    );
}

#[test]
fn a_name_failure_names_the_image_and_exits_one() {
    let name = FetchError::Name {
        image: "A".into(),
        tag: "t".into(),
    };
    assert!(
        name.to_string()
            .starts_with("image `A` tag `t` is not a registry name")
    );
    assert_eq!(name.code(), 1);
}

#[test]
fn a_resolve_failure_says_the_file_is_unchanged_and_exits_one() {
    let gone = FetchError::Resolve {
        image: "a".into(),
        tag: "1".into(),
        why: "404".into(),
    };
    let shown = gone.to_string();
    assert!(shown.starts_with("image `a:1` unresolved: 404"), "{shown}");
    assert!(shown.ends_with(".lablith was not changed"), "{shown}");
    assert_eq!(gone.code(), 1);
}
