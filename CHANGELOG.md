# Changelog

All notable changes to lablith are recorded here, in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The
project follows [Semantic Versioning](https://semver.org/): a change to
the bytes lablith emits for the same inputs is at least a minor version
(SPEC.md V25), and its entry names the files consumers will see change.

## [Unreleased]

### Added

- `ConfigError`: the one shape a refused `.lablith` is reported in,
  `.lablith:N: what` and an indented `fix:` line; pipelith's registry and
  kind errors convert into it.
- `.lablith` parsing: pipelith's neutral `## workflows` plus GitLab's
  `## images` (`image|version|digest`, digest `sha256:` + 64 hex),
  `## runners` (`system|tags`, at least one tag) and `## hand-written`.
- GitLab expression dialect for pipelith's YAML writer: an expression is
  one CI/CD variable, `$NAME`; literal text reaches `script:` unchanged.
- Image pin table: `nixos/nix` 2.35.3 pinned by its multi-arch index
  digest, overridden per repository by `## images`.
- Runner tag table: `nix` plus the nix system name per system, overridden
  by `## runners`; a system with no tags is refused naming the fix.
- GitLab pipeline model: jobs written in full (no `extends:`, no anchors)
  with a pinned image, runner tags, timeout and rules; a root
  `.gitlab-ci.yml` of `workflow: rules`, four fixed stages and
  `include: local:` only.
- Seeded scripts under `scripts/lablith/`: `cachix.sh` (push only with a
  token and not from a fork), `open-mr.sh` (GitLab API through `curl`,
  token never printed, idempotent), `update-pins.sh`, `upstream.sh`,
  `verify-cache.sh`.
- Shared job parts: every job runs the pinned `nixos/nix` image with
  flakes on, the row's cache as substituter, runner tags, timeout and a
  per-kind `GIT_DEPTH`; bump jobs take a resource group and run only on
  their own schedule or web run on a protected ref.
- Kind `gate`: one job `<id>:gate:<system>` per system running the row's
  command, through `cachix.sh` when the row names a cache.
- `image:` carries its `<name> <version>` in a comment above the key, so
  a pinned line stays within 120 columns.
- Kind `build`: `<id>:build:<system>` per system through `cachix.sh`,
  `<id>:check` (`nix flake check --all-systems`, eval of systems without
  runner tags) and `<id>:verify-cache` on the default branch.
- Kinds `update-pins` and `upstream`: one `<id>:<kind>` bump job run only
  by its own schedule or web run (`LABLITH_ROW`), on a protected ref, in
  the row's resource group; the seeded script runs in the dev shell and
  `open-mr.sh` reads the token named by `LABLITH_TOKEN_NAME`.
- Render: every `## workflows` row to `.gitlab/ci/<id>.yml`, the root
  `.gitlab-ci.yml` including each row and hand-written file, and each
  seeded script a rendered kind needs once, sorted by path; an unknown
  kind is refused naming the v1 kinds.
- `gen` and `gen --check` over the rendered set: pipelith's write and
  drift check, refusing any path that leaves the root.
- `check` totality over `.gitlab-ci.yml` and every CI file under
  `.gitlab/ci/`: each is generated or listed hand-written with a reason,
  every listed path exists, and none is both; portability findings for
  anything actlith could not render (none in v1).
- `detect` and `seed --init`: pipelith's proposed rows, with
  `update-pins` written commented out under `# off by default: uncomment
  to get daily nixpkgs-lock bump MRs`; `seed --init` never overwrites an
  existing `.lablith`.
- `report` in text, JSON (`schema_version` 1) and Markdown: per row its
  systems and runner tags, images, CI/CD variables, the pipeline schedule
  to create (cron, `LABLITH_ROW=<id>`), hand-written files, and what the
  merge request token must be.
