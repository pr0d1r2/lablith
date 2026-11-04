//! A secret a job reads (`src/kind:V32`): every kind declares each one,
//! so `report` can list them per row. Secrets are masked, protected
//! CI/CD variables (`src/kind:V15`); a job names one, never its value.

/// Why a job reads a secret: pipelith's purposes, and the cache read,
/// https push and read netrc `## caches` adds (`src/cache:V85`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Purpose {
    /// A cachix auth token, to push to a cachix cache.
    CachePush,
    /// A `## caches` url or key held by a secret.
    CacheRead,
    /// The netrc file of an https `## caches` cache, to push to it.
    NetrcPush,
    /// The netrc file a private https `## caches` cache is read with
    /// (`src/cache/read:V86`).
    NetrcRead,
    /// A token that pushes a branch and opens a merge request.
    OpenChange,
}

/// A CI/CD variable and what the job reads it for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Secret {
    /// The variable name.
    pub(crate) name: String,
    /// Why the job reads it.
    pub(crate) purpose: Purpose,
}

impl Secret {
    /// `CACHIX_AUTH_TOKEN`, read to push to the row's cache.
    pub(crate) fn cachix() -> Self {
        Self {
            name: "CACHIX_AUTH_TOKEN".to_owned(),
            purpose: Purpose::CachePush,
        }
    }

    /// `name`, read for `purpose`.
    pub(crate) fn new(name: &str, purpose: Purpose) -> Self {
        Self {
            name: name.to_owned(),
            purpose,
        }
    }

    /// The token that opens merge requests: the row's `token` arg, else
    /// `LABLITH_TOKEN` (`src/kind:V54`).
    pub(crate) fn open_change(token: Option<&str>) -> Self {
        Self {
            name: token.unwrap_or(DEFAULT_TOKEN).to_owned(),
            purpose: Purpose::OpenChange,
        }
    }

    /// What the purpose is, in words, for `report`.
    pub(crate) fn why(&self) -> &'static str {
        match self.purpose {
            Purpose::CachePush => "cachix push",
            Purpose::CacheRead => "cache read",
            Purpose::NetrcPush => "https cache push (netrc file)",
            Purpose::NetrcRead => "https cache read (netrc file)",
            Purpose::OpenChange => "push a branch and open a merge request",
        }
    }
}

/// The `token` default on GitLab: the job token cannot open a merge
/// request, so a project access token stands in (`src/kind:V54`).
pub(crate) const DEFAULT_TOKEN: &str = "LABLITH_TOKEN";

#[cfg(test)]
mod tests;
