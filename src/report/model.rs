//! What `report` says about a `.lablith`, before any format prints it.
//! Shape only; every list is sorted by its first field.

/// The whole report.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Report {
    /// One per `## workflows` row, by id.
    pub(crate) rows: Vec<RowReport>,
    /// One per `## hand-written` row, by path.
    pub(crate) hand_written: Vec<Manual>,
    /// What the merge request token must be, when a row needs one
    /// (`src/report:V64`).
    pub(crate) token: Option<&'static str>,
}

/// A generated row file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RowReport {
    /// File stem of `.gitlab/ci/<id>.yml`.
    pub(crate) id: String,
    /// The kind name.
    pub(crate) kind: &'static str,
    /// Each system with its runner tags, in row order.
    pub(crate) systems: Vec<Tagged>,
    /// The cachix cache, if any.
    pub(crate) cache: Option<String>,
    /// Every image its jobs run, by name: `name@digest` and version.
    pub(crate) images: Vec<(String, String)>,
    /// Every CI/CD variable its jobs read (`src/kind:V32`), by name.
    pub(crate) secrets: Vec<Need>,
    /// The pipeline schedule to create, for a scheduled kind
    /// (`src/bump:V17`).
    pub(crate) schedule: Option<Schedule>,
}

/// A nix system and the runner tags its jobs carry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Tagged {
    pub(crate) system: &'static str,
    pub(crate) tags: Vec<String>,
    /// `## runners` skips it on this forge (`src/system:V72`).
    pub(crate) skipped: bool,
}

/// A secret and what the row reads it for.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Need {
    pub(crate) name: String,
    pub(crate) purpose: &'static str,
}

/// A pipeline schedule to create in GitLab (`src/bump:V17`): cron, the
/// branch it runs on, and the variable that selects the row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Schedule {
    pub(crate) cron: String,
    pub(crate) variable: String,
}

/// A CI file lablith does not generate, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Manual {
    pub(crate) path: String,
    pub(crate) reason: String,
}
