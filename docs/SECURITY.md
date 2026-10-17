# Security policy

## Reporting a vulnerability

Please report vulnerabilities privately, through GitHub's
[private vulnerability reporting](https://github.com/pr0d1r2/lablith/security/advisories/new),
not in a public issue. Include what you ran, what happened, and what you
expected.

You will get an acknowledgement within a week. A fix, or a reason there
will not be one, follows as soon as the issue is understood.

## Supported versions

lablith has no release yet. Fixes land on `main`; once releases exist,
the latest one is supported.

## What lablith trusts, and what it promises

lablith writes CI configuration, so what it generates runs with your
project's permissions. Treat `.lablith` as code, the same as the
pipeline it replaces:

- A `gate` row's `command` arg and an `upstream` row's `script` arg become
  commands the generated jobs run.
- A `token` arg names the CI/CD variable the bump kinds read to push a
  branch and open a merge request.

The merge request token is a project access token: role Developer, scope
`api`, expiring within a year. Store it as a masked, protected CI/CD
variable, so only protected refs see it, and rotate it before it expires.
Bump jobs run only on protected refs, from their own schedule or a web
run naming their row. The token travels in HTTP headers only, and no job
prints a variable.

In return, every pipeline lablith generates is hardened (see
[KINDS-GITLAB.md](KINDS-GITLAB.md)): every image pinned by digest, only
local includes, no inherited job settings, the cache pushed only with the
protected token and never from a fork's merge request, and every rendered
file valid under GitLab's CI schema in lablith's own gate.

Only `lablith pin` reaches the network, through `curl` to the image's
registry, and only when you run it. In scope: a generated job that runs
something its row did not name, a variable that reaches a log, a remote
include, an image left unpinned, or any other command that touches the
network.
