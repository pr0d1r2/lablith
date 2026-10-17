# Agents

Instructions for coding agents working in this repository.

## Read first

- `SPEC.md` is the law, federated by sherd: read the root, the hub and
  the one node you work in (`sherd lens <dir>`). `SPEC-PROMPT.md` is the
  historical input the spec was made from.
- lablith is the GitLab CI counterpart of actlith, built on pipelith:
  when a choice is open, do what actlith does.

## Working rules

- Work in TDD: a failing test first, then the code, then refactor.
- One topic per commit; commit through the hooks (never `--no-verify`).
- Commit subjects are `type(scope): claim` (at most 72 characters, scope
  in lower case) with a body that says why.
- Enter the dev shell (`direnv allow` or `nix develop`) before committing:
  the hooks refuse without `hk`.
- Spec edits go through `/spec`; `/build` only flips task status.
- A new check is claimed in `.pklith` and lands with its step in one
  commit, after proving it rejects a planted violation.
- Keep every spec within `.context-limits`; a node that outgrows its
  ceiling splits, it does not raise.
- When lablith needs something from pipelith, file a pipelith issue and
  add a `Depends-on:` trailer on the lablith side.
- Never name a private repository, runner tag, host or project path of
  the self-hosted GitLab instance in a committed file.

## The gate

Generated from `hk.pkl`; the `agents-table` step fails when it is stale.

<!-- BEGIN steps: generated from hk.pkl by `cargo run -p lablith-dev -- agents`; do not edit -->
55 steps run on every commit and 61 on push and `hk check`; the commit-msg hook checks the message.

| step | runs on | files | fixes |
| --- | --- | --- | --- |
| `trailing-whitespace` | commit | `**/*` | yes |
| `final-newline` | commit | `**/*` | yes |
| `line-endings` | commit | `**/*` | yes |
| `no-bom` | commit | `**/*` | - |
| `no-merge-conflict` | commit | `**/*` | - |
| `no-case-conflict` | commit | `**/*` | - |
| `no-broken-symlinks` | commit | whole tree | - |
| `no-private-key` | commit | `**/*` | - |
| `ripsecrets` | commit | `**/*` | - |
| `no-local-paths` | commit | `**/*` | - |
| `nixfmt` | commit | `**/*.nix` | yes |
| `shfmt` | commit | `**/*.sh` `**/*.bash` | yes |
| `taplo` | commit | `**/*.toml` | yes |
| `rustfmt` | commit | `**/*.rs` | yes |
| `typos` | commit | `**/*` | - |
| `shellcheck` | commit | `.githooks/*` `.envrc` `**/*.sh` | - |
| `clippy` | commit | `**/*.rs` `Cargo.toml` `clippy.toml` | - |
| `actionlint` | commit | `.github/workflows/*.yml` `.github/workflows/*.yaml` | - |
| `zizmor` | commit | `.github/workflows/*.yml` `.github/workflows/*.yaml` | - |
| `statix` | commit | `**/*.nix` | yes |
| `deadnix` | commit | `**/*.nix` | yes |
| `markdownlint` | commit | `**/*.md` | yes |
| `yamllint` | commit | `**/*.yml` `**/*.yaml` | - |
| `editorconfig-checker` | commit | `**/*` | - |
| `links` | commit | `**/*.md` | - |
| `ctrm` | commit | `**/*` | yes |
| `xnl-check` | commit | `**/*.nix` `**/*.pkl` `.envrc` `.githooks/*` | - |
| `xnl-lint` | commit | `**/*.nix` `**/*.pkl` `.envrc` `.githooks/*` | yes |
| `rekall` | commit | `AGENTS.md` `CLAUDE.md` `rekall.toml` | - |
| `rust-mirror` | commit | `**/*.rs` `nix/guard/rust-mirror.sh` | - |
| `rustdoc` | commit | `**/*.rs` `Cargo.toml` | - |
| `mth-fmt` | commit | `**/SPEC.md` | yes |
| `mth-check` | commit | `**/SPEC.md` | - |
| `sherd-check` | commit | `**/SPEC.md` `**/*.rs` | - |
| `sherd-nav` | commit | `**/SPEC.md` | yes |
| `sherd-budget` | commit | `**/SPEC.md` `.context-limits` | - |
| `itok` | commit | `**/*` `.context-limits` | - |
| `agents-table` | commit | `hk.pkl` `hk.pklith.pkl` `AGENTS.md` `dev/**` | yes |
| `readme-badges` | commit | `README.md` `Cargo.toml` `.coverage` `hk.pkl` `hk.pklith.pkl` `flake.lock` `**/SPEC.md` `docs/LLM-DISCLAIMER.md` `.github/workflows/ci.yml` `dev/**` | yes |
| `notices` | commit | `Cargo.toml` `Cargo.lock` `docs/THIRD-PARTY-NOTICES.md` `dev/**` | yes |
| `doctest` | commit | `**/*.rs` `Cargo.toml` | - |
| `deny` | commit | `Cargo.toml` `Cargo.lock` `deny.toml` | - |
| `gen-check` | commit | `.pklith` `hk.pklith.pkl` `nix/pklith.nix` | yes |
| `pklith-check` | commit | whole tree | - |
| `private-names` | commit | `**/*` | - |
| `no-core-worktree` | commit | `**/*` | - |
| `hk-parity` | commit | `hk.pkl` `hk.pklith.pkl` `pkl/**` `nix/guard/hk-parity.sh` | - |
| `ci-schema-pin` | commit | `tests/schema/**` `nix/guard/ci-schema-pin.sh` | - |
| `gitlab-schema` | commit | `tests/schema/**` `tests/fixtures/**` `src/**` `.gitlab-ci.yml` `.gitlab/ci/**` `nix/guard/gitlab-schema.sh` | - |
| `actlith-gen-check` | commit | `.actlith` `.github/workflows/**` | yes |
| `actlith-check` | commit | `.actlith` `.github/workflows/**` | - |
| `lablith-gen-check` | commit | `.lablith` `.gitlab-ci.yml` `.gitlab/ci/**` `scripts/lablith/**` `src/**` | yes |
| `lablith-check` | commit | `.lablith` `.gitlab-ci.yml` `.gitlab/ci/**` `src/**` | - |
| `build` | commit | `**/*.rs` `Cargo.toml` `Cargo.lock` | - |
| `test` | push | `**/*.rs` `Cargo.toml` `Cargo.lock` | - |
| `coverage` | push | `**/*.rs` `Cargo.toml` `.coverage` | - |
| `tdd-order` | push | `**/*.rs` `nix/guard/tdd-order.sh` `nix/guard/tdd-order.exempt` | - |
| `schema-pin` | push | `pkl/**` `flake.lock` `nix/guard/schema-pin.sh` | - |
| `package` | push | `Cargo.toml` `Cargo.lock` `src/**` | - |
| `deny-advisories` | push | `Cargo.toml` `Cargo.lock` `deny.toml` | - |
| `package-nix` | push | `flake.nix` `flake.lock` `Cargo.toml` `Cargo.lock` `src/**` `nix/guard/package-nix.sh` | - |
<!-- END steps -->
