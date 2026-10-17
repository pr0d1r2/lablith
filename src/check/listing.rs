//! The files `check` looks at, read from disk under `--root`.

use std::fs;
use std::io;
use std::path::Path;

/// The root CI file GitLab reads.
pub(crate) const ROOT: &str = ".gitlab-ci.yml";

/// Where row files live, relative to the root.
pub(crate) const CI_DIR: &str = ".gitlab/ci/";

/// `.gitlab-ci.yml` when present, then every file under `.gitlab/ci/` at
/// any depth, as sorted repository-relative paths. A missing directory
/// lists nothing.
///
/// # Errors
///
/// The directory or one of its entries cannot be read.
pub(crate) fn listing(root: &Path) -> io::Result<Vec<String>> {
    let mut paths = Vec::new();
    if root.join(ROOT).is_file() {
        paths.push(ROOT.to_owned());
    }
    walk(root, CI_DIR.trim_end_matches('/'), &mut paths)?;
    paths.sort();
    Ok(paths)
}

/// Every file under `root/rel`, appended to `out`.
fn walk(root: &Path, rel: &str, out: &mut Vec<String>) -> io::Result<()> {
    let entries = match fs::read_dir(root.join(rel)) {
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(()),
        other => other?,
    };
    for entry in entries {
        let entry = entry?;
        let path = format!("{rel}/{}", entry.file_name().to_string_lossy());
        if entry.file_type()?.is_dir() && rel.is_empty() {
            walk(root, &path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
