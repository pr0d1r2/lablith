use super::{BUILTIN, PinTable};
use crate::pin::PinError;
use crate::registry::ImagePin;

const OTHER: &str =
    "sha256:1111111111111111111111111111111111111111111111111111111111111111";

fn row(image: &str, version: &str) -> ImagePin {
    ImagePin {
        line: 3,
        image: image.into(),
        version: version.into(),
        digest: OTHER.into(),
    }
}

#[test]
fn every_builtin_digest_is_sha256_and_64_hex() {
    let hex = |c: char| c.is_ascii_digit() || ('a'..='f').contains(&c);
    for (name, _, digest) in BUILTIN {
        let h = digest.strip_prefix("sha256:").unwrap_or_default();
        assert!(h.len() == 64 && h.chars().all(hex), "{name}: {digest}");
    }
}

#[test]
fn the_builtin_table_pins_nix() {
    let nix = PinTable::builtin().image("nixos/nix");
    let nix = nix.map(|i| (i.version.clone(), i.reference()));
    assert_eq!(
        nix,
        Ok((
            "2.35.3".to_owned(),
            "nixos/nix@sha256:ff480dbb23511aa1c10710aa068ceea5b93fed9bfce5f5379f3a6b07282cbc7b".to_owned()
        ))
    );
}

#[test]
fn an_override_replaces_a_pin_and_adds_new_ones() {
    let t = PinTable::builtin()
        .with_overrides(&[row("nixos/nix", "2.30.0"), row("alpine", "3")]);
    let nix = t.image("nixos/nix").map(|i| i.version);
    assert_eq!(nix, Ok("2.30.0".to_owned()));
    let names: Vec<&str> = t.all().map(|i| i.name.as_str()).collect();
    assert_eq!(names, ["alpine", "nixos/nix"]);
}

#[test]
fn an_unpinned_image_is_refused_by_name() {
    assert_eq!(
        PinTable::builtin().image("alpine"),
        Err(PinError {
            image: "alpine".into()
        })
    );
}
