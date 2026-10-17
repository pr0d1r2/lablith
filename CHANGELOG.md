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
