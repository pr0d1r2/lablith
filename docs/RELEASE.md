# Moving a project from GitLab to GitHub

This procedure is for a project whose CI is rendered by lablith and is
moving to GitHub. It is deliberately blocked until pipelith resolves Q3:
the name of the forge-neutral configuration file. Do not perform the
rename below until that decision is recorded and the corresponding
`actlith` version is available.

## Reviewed migration commit

Perform the file changes below together in one reviewed commit:

1. Rename `.lablith` to `.actlith`. Q3 must confirm that this is the
   config-file name selected by pipelith for `actlith` before this step is
   carried out.
2. Keep the `## workflows` section unchanged. Review every `## runners` row
   against the GitHub runner mapping and workflow matrix, and record explicit
   approval of equivalence (or of an intentional change) before removing the
   section. A runner row such as `system|-` may skip a system; do not remove
   it without this review. Then remove the GitLab-only `## images` and
   `## runners` sections.
3. Move every hand-written file to its corresponding
   `.github/workflows/` path, translating GitLab CI syntax to GitHub
   Actions when required. Update each `## hand-written` path and keep
   its reason.
4. Run `actlith gen` and review the generated workflow diff.
5. Run the complete gate, including `actlith check`.
6. Delete `.gitlab-ci.yml`, `.gitlab/ci/`, and `scripts/lablith/`.

The resulting workflow files must be generated or listed as hand-written
by the `actlith` configuration. Do not rewrite workflow rows to make the
move fit GitHub: portability is checked before the migration, and the
rows are shared with actlith by design.

Before merging, run the project's complete gate and confirm that the
generated GitHub workflows are current. The migration is not complete if
any GitLab-generated file remains.

## Outside the repository

After the reviewed commit is merged:

- delete the project's GitLab pipeline schedules;
- delete the `LABLITH_TOKEN` CI/CD variable;
- create the GitHub secrets required by `actlith report`.

These provider settings are intentionally not represented in the
repository. `lablith report` lists the schedules and secrets assumed by
the GitLab configuration before the move; use the corresponding
`actlith report` output for the GitHub setup.
