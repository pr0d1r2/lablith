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
