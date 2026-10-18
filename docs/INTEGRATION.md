# Integration

From no pipeline to a generated one, without a red pipeline in between.
Each step leaves the project working.

## What you need

- `lablith` on your `PATH` (see [Install](../README.md#install)).
- `git`: lablith reads the list of tracked files from it.
- For the `gate` kind: a nix flake whose dev shell provides
  [hk](https://hk.jdx.dev) and an `hk.pkl`, because the generated job runs
  `nix develop --command hk check --all --check`. A different command goes
  in the row's `command` arg. [pklith](https://github.com/pr0d1r2/pklith)
  builds such a gate from your files.
- GitLab runners with nix, and carrying the tags your jobs ask for: a
  Docker executor runs the pinned `nixos/nix` image, a shell executor on
  a NixOS host uses the host's nix and ignores `image:`. The built-in tags are `nix` plus the nix system name
  (`nix`, `x86_64-linux`); if your runners are tagged differently, say so
  in `## runners` ([REGISTRY.md](REGISTRY.md)).
- Optional: a [cachix](https://www.cachix.org) cache, and its auth token
  as the masked, protected CI/CD variable `CACHIX_AUTH_TOKEN`.

## A project with no pipeline yet

```sh
lablith detect        # print proposed rows; nothing is written
lablith seed --init   # write .lablith from them
lablith gen           # write .gitlab-ci.yml, .gitlab/ci/*.yml and scripts/lablith/*.sh
lablith check         # exit 0: every CI file is accounted for
```

`detect` proposes a `gate` row when it finds `flake.nix`. When
`flake.lock` has a `nixpkgs-lock` input it also proposes `update-pins`,
commented out: that job pushes branches and opens merge requests on a
schedule, so it stays off until you uncomment it:

```text
# off by default: uncomment to get daily nixpkgs-lock bump MRs
# update-pins|update-pins|-|-|-
```

Edit `.lablith` before `gen` if you want different systems, a cache or
other args; [REGISTRY.md](REGISTRY.md) describes every column. Review the
generated files, commit them together with `.lablith`, and push.

The scripts under `scripts/lablith/` are generated too, with the same
header. Commit them, and do not edit them.

## A project that already has a pipeline

Keep it, and declare it: list each file under `## hand-written` with the
reason, and lablith includes it from the root file next to the generated
ones. Moving a job to a kind is a reviewed change of its own: add the
row, run `lablith gen`, delete the hand-written job, and check the diff.
lablith has no `import`; the hand-written pipelines it was built for did
not match any kind's shape.

## Scheduled kinds

GitLab keeps pipeline schedules outside the YAML. For each `update-pins`
or `upstream` row, create a schedule on the default branch with the cron
and the variable `lablith report` names:

```sh
lablith report
#   pins: kind update-pins, cache -
#     schedule 50 6 * * * on the default branch, variable LABLITH_ROW=pins
```

Every bump job runs only when `LABLITH_ROW` names its row, so several
schedules on one project each start their own row. A manual run from the
pipeline page with the same variable set starts it too.

The bump kinds push a branch and open a merge request with a project
access token, role Developer, scope `api`, stored as the masked,
protected CI/CD variable `LABLITH_TOKEN` (or the name the row's `token`
arg gives). GitLab's job token cannot open a merge request.

## Keeping it true

Add both checks to the gate that runs on every commit, so drift fails
where it starts:

```sh
lablith gen --check   # a generated file was edited by hand, or is stale
lablith check         # a CI file nobody declared
```

When you upgrade lablith, run `lablith gen` and review the diff. A change
to the bytes lablith writes for the same `.lablith` is at least a minor
version, and [CHANGELOG.md](../CHANGELOG.md) names the files that change.

## Pinning images

The built-in pin moves with lablith releases. To move it yourself:

```sh
lablith pin --dry-run   # print fresh `## images` rows
lablith pin             # write them into .lablith
lablith gen
```

`pin` is the only command that reaches the network: it asks the image's
registry for the multi-arch index digest of the recorded tag, with
`curl`. A failure leaves `.lablith` as it was.

## What lablith will not do to your project

- Write anywhere but `.gitlab-ci.yml`, `.gitlab/ci/`, `scripts/lablith/`
  and `.lablith` (only `seed --init`, and `pin` for its `## images`
  section).
- Overwrite an existing `.lablith` (`seed --init` exits 1 instead).
- Rewrite a file whose bytes already match.
- Touch a file listed under `## hand-written`.
- Register runners, create schedules, set CI/CD variables or change
  project settings: those stay your actions. `lablith report` lists what
  your rows assume.
- Include remote, project or template configuration, or reach the
  network from any command but `pin`.

## Moving to GitHub

The `## workflows` rows are pipelith's, the same on both forges, and
`lablith portable` confirms none uses anything actlith lacks. The move is
one reviewed commit: rename `.lablith` to `.actlith`, drop its
`## images` and `## runners` sections, point `## hand-written` paths at
`.github/workflows/`, run `actlith gen`, and delete `.gitlab-ci.yml`,
`.gitlab/ci/` and `scripts/lablith/`.
