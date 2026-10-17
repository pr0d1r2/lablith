use super::{Image, PinError};

#[test]
fn a_reference_is_name_at_digest() {
    let i = Image {
        name: "nixos/nix".into(),
        version: "2.35.3".into(),
        digest: "sha256:ab".into(),
    };
    assert_eq!(i.reference(), "nixos/nix@sha256:ab");
}

#[test]
fn a_missing_pin_names_the_image_and_the_fix() {
    let e = PinError {
        image: "alpine".into(),
    };
    assert_eq!(
        e.to_string(),
        "image `alpine` has no pin\n  fix: add an `## images` row \
         `alpine|<version>|<digest>` (`lablith pin` resolves it)"
    );
}
