use std::fs;
use std::path::PathBuf;

use super::pin;
use crate::fetch::{FetchError, Resolver};

const NEW: &str =
    "sha256:2222222222222222222222222222222222222222222222222222222222222222";

/// Resolves every image to `NEW`, or refuses `bad`.
struct Fake(&'static str);

impl Resolver for Fake {
    fn digest(&self, image: &str, tag: &str) -> Result<String, FetchError> {
        if image == self.0 {
            return Err(FetchError::Resolve {
                image: image.into(),
                tag: tag.into(),
                why: "gone".into(),
            });
        }
        Ok(NEW.to_owned())
    }
}

fn scratch(name: &str, text: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("lablith-pin-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let made = fs::create_dir_all(&dir)
        .and_then(|()| fs::write(dir.join(".lablith"), text));
    assert!(made.is_ok());
    dir
}

const TEXT: &str =
    include_str!("../../../tests/fixtures/fetch/no-images.lablith");

#[test]
fn pin_writes_every_pin_with_its_fresh_digest() {
    let dir = scratch("write", TEXT);
    let rows = pin(&dir, false, &Fake("-"));
    let want = format!("image|version|digest\nnixos/nix|2.35.3|{NEW}\n");
    assert_eq!(rows, Ok(want.clone()));
    let text = fs::read_to_string(dir.join(".lablith")).unwrap_or_default();
    let _ = fs::remove_dir_all(&dir);
    assert!(text.ends_with(&format!("\n## images\n{want}")), "{text}");
}

#[test]
fn a_dry_run_writes_nothing() {
    let dir = scratch("dry", TEXT);
    assert!(pin(&dir, true, &Fake("-")).is_ok());
    let text = fs::read_to_string(dir.join(".lablith")).unwrap_or_default();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(text, TEXT);
}

#[test]
fn a_failure_leaves_the_file_as_it_was_and_exits_one() {
    let dir = scratch("fail", TEXT);
    let e = pin(&dir, false, &Fake("nixos/nix")).err();
    let text = fs::read_to_string(dir.join(".lablith")).unwrap_or_default();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(text, TEXT);
    assert_eq!(e.map(|e| e.code()), Some(1));
}

#[test]
fn a_missing_or_broken_file_exits_two() {
    let dir = scratch("broken", "not a registry\n");
    let broken = pin(&dir, false, &Fake("-")).err().map(|e| e.code());
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(broken, Some(2));
    let gone = pin(&dir, false, &Fake("-")).err().map(|e| e.code());
    assert_eq!(gone, Some(2));
}
