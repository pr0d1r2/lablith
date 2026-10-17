## Rows

| id | kind | systems | cache | images |
| --- | --- | --- | --- | --- |
| `ci` | `gate` | x86_64-linux, aarch64-linux | `pr0d1r2` | `nixos/nix@sha256:ff480dbb23511aa1c10710aa068ceea5b93fed9bfce5f5379f3a6b07282cbc7b` 2.35.3 |
| `lint` | `gate` | x86_64-linux, aarch64-linux, aarch64-darwin | - | `nixos/nix@sha256:ff480dbb23511aa1c10710aa068ceea5b93fed9bfce5f5379f3a6b07282cbc7b` 2.35.3 |
| `pins` | `update-pins` | x86_64-linux | `pr0d1r2` | `nixos/nix@sha256:ff480dbb23511aa1c10710aa068ceea5b93fed9bfce5f5379f3a6b07282cbc7b` 2.35.3 |

## Secrets

| row | variable | purpose |
| --- | --- | --- |
| `ci` | `CACHIX_AUTH_TOKEN` | cachix push |
| `pins` | `BUMP_TOKEN` | push a branch and open a merge request |

## Schedules

| row | cron | variable |
| --- | --- | --- |
| `pins` | `50 6 * * *` | `LABLITH_ROW=pins` |

## Hand-written

| path | reason |
| --- | --- |
| `.gitlab/ci/release.yml` | release flow \| not a kind yet |

## Token

a project access token, role Developer, scope `api`, expiring within a year; store it as a masked, protected CI/CD variable and rotate it before it expires
