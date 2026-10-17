//! The closed list of v1 kinds: `gate`, `build`, `update-pins`,
//! `upstream`, in `docs/KINDS.md` order (`.:V24`).

use crate::build::Build;
use crate::bump::{UpdatePins, Upstream};
use crate::gate::Gate;
use crate::kind::Kind;

/// Every v1 kind.
pub(crate) fn entries() -> [&'static dyn Kind; 4] {
    [&Gate, &Build, &UpdatePins, &Upstream]
}

/// The kind a row's `kind` cell names, if it is a v1 kind.
pub(crate) fn lookup(name: &str) -> Option<&'static dyn Kind> {
    entries().into_iter().find(|k| k.spec().name == name)
}

/// The v1 kind names, as a fix hint lists them.
pub(crate) fn names() -> String {
    let quoted: Vec<String> =
        entries().iter().map(|k| k.spec().name.to_owned()).collect();
    quoted.join(", ")
}

#[cfg(test)]
mod tests;
