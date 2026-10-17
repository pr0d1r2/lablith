//! `## images` rows and the one-section rewrite (`src/fetch:V34`).

use crate::fetch::section::{HEADER, replace, rows};
use crate::pin::Image;
use crate::registry::parse;

const FULL: &str = include_str!("../../../tests/fixtures/fetch/full.lablith");
const NONE: &str =
    include_str!("../../../tests/fixtures/fetch/no-images.lablith");

const DIGEST: &str =
    "sha256:1111111111111111111111111111111111111111111111111111111111111111";

fn pins() -> Vec<Image> {
    let image = |name: &str, version: &str| Image {
        name: name.to_owned(),
        version: version.to_owned(),
        digest: DIGEST.to_owned(),
    };
    vec![image("alpine", "3.21"), image("nixos/nix", "2.35.3")]
}

fn new_rows() -> String {
    format!("{HEADER}\nalpine|3.21|{DIGEST}\nnixos/nix|2.35.3|{DIGEST}\n")
}

#[test]
fn rows_are_the_header_and_one_row_per_pin() {
    assert_eq!(rows(&pins()), new_rows());
    assert_eq!(rows(&[]), format!("{HEADER}\n"));
}

#[test]
fn replace_keeps_the_section_comments_and_everything_after_the_rows() {
    let got = replace(FULL, &new_rows());
    let lead = format!(
        "## images\n# kept: a comment inside the section\n{}",
        new_rows()
    );
    assert!(got.contains(&lead), "{got}");
    let tail = "# kept: a trailing comment\n\n## hand-written\n";
    assert!(got.contains(tail), "{got}");
    assert_eq!(parse(&got).map(|r| r.images.len()), Ok(2));
}

#[test]
fn a_file_without_images_gets_the_section_appended() {
    let got = replace(NONE, &new_rows());
    assert_eq!(
        got,
        format!("{}\n\n## images\n{}", NONE.trim_end(), new_rows())
    );
    let versions: Vec<String> = parse(&got)
        .unwrap_or_default()
        .images
        .into_iter()
        .map(|i| i.version)
        .collect();
    assert_eq!(versions, ["3.21", "2.35.3"]);
}
