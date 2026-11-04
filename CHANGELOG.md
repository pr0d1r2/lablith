# Changelog

All notable changes to lablith are recorded here, in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The
project follows [Semantic Versioning](https://semver.org/): a change to
the bytes lablith emits for the same inputs is at least a minor version
(SPEC.md V25), and its entry names the files consumers will see change.

## [Unreleased]

### Changed

- pipelith 0.2.0: `x86_64-darwin` is a CI system. A row may list it when
  a `## runners` row gives it tags, for example
  `x86_64-darwin|nix, x86_64-darwin`, and its jobs run on that runner;
  without the row it is refused naming that fix. A `-` row runs on the
  systems the runner table covers, so `.gitlab/ci/<id>.yml` of a `-` row
  gains `x86_64-darwin` jobs only in a repository with such a row, and a
  `.lablith` without one renders the same bytes as before.
- No generated job runs for a merge request from a fork: its first rule
  is `if: $CI_MERGE_REQUEST_SOURCE_PROJECT_ID != $CI_MERGE_REQUEST_PROJECT_ID`,
  `when: never`, whatever its tags, since on a self-hosted GitLab the
  built-in tags also reach the owner's runners. Every job in
  `.gitlab/ci/<id>.yml` gains that rule; jobs in `## hand-written`
  files need their own.

### Added

- `## caches` (pipelith 0.2.1), columns `name|kind|url|key|push`: a row
  names a `cachix` or `https` nix binary cache. An https url and key are
  required, each a literal or `secret:NAME`; a cachix url is `-` or its
  own, never a secret; a push is `-` or `secret:NAME`, never a literal
  (docs/REGISTRY.md has the full table). A `.lablith` without the
  section renders the same bytes as before. With it, each row naming
  such a cache changes its `.gitlab/ci/<id>.yml` jobs as below:
  - literal url and key join `NIX_CONFIG`; a secret one is added at the
    job's start by the new seeded `scripts/lablith/cache-read.sh`, only
    when both values are there, so a pipeline without protected
    variables runs without the cache;
  - a push secret goes through the new seeded
    `scripts/lablith/cache-push.sh`: cachix with that token, or https
    with `nix copy` and the netrc file, copying a gate's
    `.#devShells.<system>.default.inputDerivation` or a build's package
    before the command; never for a fork's merge request, and for
    `build` only on the default branch;
  - a `build` row on an https cache verifies it with the new seeded
    `scripts/lablith/https-verify.sh`;
  - `lablith report` lists push and read secrets (`cache read`,
    `cachix push`, `https cache push (netrc file)`);
  - an https push is best-effort, a failed copy a warning before the
    command still runs, and a url that is not `https://` is refused
    before any netrc is used.

- A private `https` cache (pipelith 0.2.2), read with its own netrc:
  the seeded `scripts/lablith/cache-read.sh` takes the netrc's File
  variable and the cache name after the url and key. It checks that the
  resolved url is `https://`, and that the netrc has no `default`
  entry (which would reach every host; read as curl does, in any case,
  with quoted tokens whole and `#` comments skipped) and can be read, before it changes the
  netrc or hands it to nix, and stops naming the cache and the
  variable, never the url or the credential, when a check fails;
  then it makes the File variable's own file readable only by the job
  and prints `netrc-file = <that file>` after both settings. It makes
  no copy, so no credential outlives the job: the runner removes the
  File variable's file when the job ends. Without the secret (an
  unprotected ref) it prints nothing and the job runs without the
  cache; a fork's merge request never gets that far, since the fork
  guard skips the job.
- A row naming such a cache (`read=secret:R` in the `push` cell)
  renders its `.gitlab/ci/<id>.yml` jobs with the read netrc: the line
  after the runner check passes `R` and the cache name to
  `cache-read.sh`, a build's `https-verify.sh` asks with `R`, and the
  push still uses its own `push=secret:P`. `lablith report` lists `R` as
  `https cache read (netrc file)`. Every `cache-read.sh` line now
  assigns `NIX_CONFIG="$NIX_CONFIG$(...)"` instead of exporting it, so
  a refused url stops the job rather than passing unseen; a job reading
  a secret url or key changes by that one word. `R` is a protected File
  variable, apart from the push one.

- Every nix job's `script:` starts with `bash
  scripts/lablith/runner-system.sh <system>`. It passes when the
  runner's nix `system` setting is `<system>` or its nix lists it in
  `extra-platforms` (binfmt or Rosetta emulation), and otherwise stops
  with `runner nix system is <native> (extra-platforms: <list or
  none>), this job needs <system>`. It needs nix 2.20 or later on the
  runner (a shell runner's host) and stops naming the version found on
  an older one. `lablith gen` seeds the new
  `scripts/lablith/runner-system.sh` in every repository with a row, and
  every job in `.gitlab/ci/<id>.yml` gains that first line, so a
  one-line `script: [..]` becomes a block list.
- `runner-system.sh` reads the version from the first line of
  `nix --version`, so a Lix runner, which prints more lines, passes.

- `## runners` skips a system with `<system>|-`: no gate or build job is
  rendered for it, the build `check` evaluates it, and `lablith report`
  marks it (text: `skipped: no runner on this forge`; JSON: a new
  per-system `skipped` field, so `.gitlab/ci/<id>.yml` loses that
  system's jobs and `report --format json` gains the field).

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
- `pin` core: each pinned image's tag resolved to its multi-arch index
  digest over the OCI distribution API with `curl` (anonymous bearer
  token on `401`), `## images` rewritten only after every image resolved.
- The `lablith` command: `gen [--check]`, `check`, `portable`, `detect`,
  `seed --init`, `report [--format text|json|md]`, `pin [--dry-run]`,
  `completions <shell>` and `man`, each taking `--root DIR`; exit 0 ok,
  1 finding, 2 usage or refused input.

### Fixed

- `scripts/lablith/update-pins.sh` reads the nixpkgs revision the root
  follows through `nixpkgs-lock`, resolving node names and follows paths,
  instead of the lock node literally named `nixpkgs`. When another input
  brought its own nixpkgs-lock, that node was the other input's and never
  moved, so the job always reported nothing to do. Consumers see the
  script's bytes change.
