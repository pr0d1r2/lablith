//! One resolved pin: `name@sha256:<digest>` and its version
//! (`src/pin:V2`), and the refusal when a kind asks for an unpinned
//! image.

use std::fmt;

/// An image pinned by digest, with the version the digest was taken from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Image {
    /// The image name, e.g. `nixos/nix`.
    pub(crate) name: String,
    /// The tag the digest was resolved from, e.g. `2.35.3`.
    pub(crate) version: String,
    /// `sha256:` and 64 hex digits: the multi-arch index digest.
    pub(crate) digest: String,
}

impl Image {
    /// The `image:` value: `name@sha256:<digest>`.
    pub(crate) fn reference(&self) -> String {
        self.name.clone()
    }
}

/// An image no pin covers (`src/pin:V2`): the caller exits 2 with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PinError {
    /// The image name the kind asked for.
    pub(crate) image: String,
}

impl fmt::Display for PinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let image = &self.image;
        write!(
            f,
            "image `{image}` has no pin\n  fix: add an `## images` row \
             `{image}|<version>|<digest>` (`lablith pin` resolves it)"
        )
    }
}

impl std::error::Error for PinError {}

#[cfg(test)]
mod tests;
