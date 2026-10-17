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
