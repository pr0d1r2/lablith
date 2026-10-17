# Vendored: GitLab CI configuration schema

`ci.json` is copied verbatim from GitLab's own repository, so the
rendered pipelines are validated offline (tests:V19, tests:V61).

- source: <https://gitlab.com/gitlab-org/gitlab/-/raw/v19.5.0-ee/app/assets/javascripts/editor/schema/ci.json>
- GitLab tag: v19.5.0-ee
- `ci.json` sha256: 98e54051f78236ae22281857ff38c7fc634b88b5008d1326a8d1a5b0b22d8da5
- license: MIT, (c) GitLab B.V. (the CE/EE source this file belongs to)

The self-hosted instance's version is open question 4; until it is
known, the newest release is vendored, and a key it lacks would show up
as a failing `glab ci lint` run by hand. Do not edit it. To move it,
replace it from another tag in one commit, updating this file;
`ci-schema-pin` fails until the tag line and the sha256 match the copy.
