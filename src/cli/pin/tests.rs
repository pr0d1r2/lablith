use std::fs;

use super::pin;
use crate::cli::scratch::{DIGEST, Fake, put, scratch};
use crate::fetch::{FetchError, Resolver};

const TEXT: &str = "format 1\n";

#[test]
fn pin_prints_and_writes_the_fresh_rows() {
    let root = scratch("pin-write");
    put(&root, ".lablith", TEXT);
    let out = pin(&root, false, &Fake).map(|o| o.out);
    let text = fs::read_to_string(root.join(".lablith")).unwrap_or_default();
    let _ = fs::remove_dir_all(&root);
    let rows = format!("image|version|digest\nnixos/nix|2.35.3|{DIGEST}\n");
    assert_eq!(out, Ok(rows.clone()));
    assert!(text.ends_with(&rows), "{text}");
}

/// A resolver whose every lookup fails.
struct Down;

impl Resolver for Down {
    fn digest(&self, image: &str, tag: &str) -> Result<String, FetchError> {
        let why = "down".to_owned();
        Err(FetchError::Resolve {
            image: image.into(),
            tag: tag.into(),
            why,
        })
    }
}

#[test]
fn a_failed_lookup_is_exit_one_naming_the_image() {
    let root = scratch("pin-down");
    put(&root, ".lablith", TEXT);
    let failed = pin(&root, false, &Down).err();
    let _ = fs::remove_dir_all(&root);
    assert_eq!(failed.as_ref().map(|o| o.code), Some(1));
    let err = failed.map(|o| o.err).unwrap_or_default();
    assert!(
        err.starts_with("image `nixos/nix:2.35.3` unresolved: down"),
        "{err}"
    );
}
