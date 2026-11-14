# Moving from GitLab to GitHub

This procedure is blocked until pipelith resolves open question 1 (Q3),
which decides the config file name used by the release migration. Do not
perform this migration until that decision is recorded and the commands
below match the released `actlith` version.

## Reviewed migration commit

Complete the following changes in one reviewed commit. Run
`lablith portable` first and resolve every finding; the workflow rows must
be usable by actlith before they move.

1. Rename `.lablith` to `.actlith`.
2. In `hk.pkl`, replace the `lablith-gen-check` and `lablith-check` rows with
   the matching `actlith-gen-check` and `actlith-check` rows, then regenerate
   `hk.pklith.pkl`; update `hk.pkl` in this same reviewed commit.
3. Remove the `## images` and `## runners` sections from `.actlith`.
4. Translate each hand-written GitLab workflow to GitHub Actions syntax,
   review the translation, then change its `## hand-written` path to the
   corresponding `.github/workflows/` path.
5. Run `actlith gen` to render the GitHub workflow files.
6. Delete `.gitlab-ci.yml`, `.gitlab/ci/`, and `scripts/lablith/`.
7. Review the complete diff, including that every workflow row and every
   hand-written workflow is present on GitHub, then commit the migration.

The migration does not rewrite workflow rows. Rows remain the shared
pipelith declaration. Hand-written workflows must be translated to GitHub
Actions syntax before they move to their `.github/workflows/` paths.

## GitLab settings and secrets

After the reviewed commit is merged, and outside the repository:

- Delete the GitLab pipeline schedules for every scheduled row.
- Delete the GitLab CI/CD variable `LABLITH_TOKEN` (or the variable name
  selected by a row's `token` argument).
- Run `actlith report` and create the GitHub secrets it reports, using the
  names and scope required by each row.
- Confirm the GitHub workflow runs successfully before disabling the old
  GitLab project or mirror.

Do not commit credentials, runner names, host names, or other private
GitLab instance details while carrying out this procedure.
