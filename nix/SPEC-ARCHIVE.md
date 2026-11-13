# SPEC ARCHIVE

Task rows moved out of `SPEC.md` by `mth archive`. An id is never reused
(`V12`), so a citation to an archived row still resolves -- here.

This is a SINK, not a spec. The citations inside these rows point into
`SPEC.md`, so `mth check` on this file reports every one of them as dangling,
correctly and uselessly. The verb that reads it is `mth tasks`.

## §T TASKS

T1|x|AFTER T3: vendor `FORMAT.md` (actlith copy = cavekit rev in `.format-upstream`), `FORMAT-EXTENSIONS.md` via pinned `mth extensions` (actlith copy stale: lacks `§D`), `.format-upstream`, through live hooks|V40
T2|x|FIRST: flake dev shell on `nixpkgs-lock` + `nix-hk` (overlay ∴ `pkgs.hk` = nix-hk's hk; ∀ hk-using input `follows` nix-hk), cachix `nixConfig`, fleet guardrail tools pinned (= actlith pins), Rust toolchain, pklith `lib.devShell`; `.envrc` `use flake` (direnv). ASK user reload|R5,V42
T3|x|hk commit, right after T2: `.pklith`, `pklith gen` → `hk.pklith.pkl`, seeded `hk.pkl` (hk 2.7.0, `stage = true` on `fix`), vendored `pkl/Config.pkl` + `pkl/VENDORED.md`, tracked `.githooks/`; shell entry (direnv) runs hk hook install once `hk.pkl` exists ∴ hooks live from this commit. ASK user reload|R5,V40
T4|x|seed crate by hand through live hooks: LICENSE (MIT), `.gitignore`, `Cargo.toml` (edition 2024, MSRV 1.95, lints, `unsafe_code = "forbid"`, `pipelith = "0.1"`), `clippy.toml`, `rustfmt.toml`, `src/main.rs`, `src/lib.rs`, README stub, CHANGELOG|`.:V23`
T5|x|guardrails, 1 commit each, planted violation each: commit-msg (≤72), changelog-touched, hygiene, ripsecrets, typos, nixfmt, taplo, rustfmt, clippy (all targets), shellcheck, markdownlint (MD024 `siblings_only`), `links` (lychee `--offline`)|R5
T6|x|spec gates: `mth fmt`/`mth check`, `itok` + `.context-limits`, `sherd check`/`budget`; `ctrm`, `rekall`, `xenolith` (`rust-mirror`, `tdd-order`)|-
T7|x|cargo steps: `build` after clippy, `cargo run` steps depend on it; doctest, rustdoc, `cargo deny`; push-only `package`, `deny-advisories`, `schema-pin`; flake guard via `git+file://<root>`. nextest `test` (waits ∀ `cargo run`) & coverage floor + ratchet land w/ 1st real test (`src/core:T11`) ∵ nextest refuses empty suite; `package-nix` w/ T33 ∵ needs working verbs|R5
T8|x|private-name guard: committed files ⊥ instance names (extends paradigm name guard D40)|`.:V20`
T9|x|hand-written GitHub `ci.yml` via actlith (`.actlith`, `actlith gen`); `no-commit-to-branch main`; guardrails done → ASK user reload|`.:R3`
T32|x|dogfood: own `.lablith` & rendered GitLab files; `lablith gen --check` step in hk|`src/gen:V5`
T33|x|nix package `packages.<system>.default` built & run on fixture; `release.toml` + pre-release `hk check --all`|`.:V25`
T36|x|branch protection on first push: PR required, 0 approvals, admins included, `gate (...)` checks required & strict, ⊥ force push, ⊥ deletion|R5
T41|x|AFTER T7: guards `tdd-order.exempt` format, `core.worktree` refusal, pre-commit `stash = "none"`, hk parity step (pklr)|V44,V46,V47
T54|x|planted violation for `xnl-lint`: found — 9 `inputs.<name>.follows` lines in one flake input → statix (via `xnl lint`) "The key `inputs` is first assigned here" (writeShellScript `cd $1`, runCommand `ls *.txt` gave 0)|V41
T55|x|planted violation for `hk-parity`: resolved. Under hk 2.7.0, pklr and `pkl eval` agree on `List(...).join(" ")`, `.replaceAll`, and pklith B12's `for (…) { when (…) { … } }` filter (pklr fixed it); `toMap().remove()` fails in `pkl eval`; pklr also rejects `toMap().remove()`; the result is recorded in B2|V41,V47
T56|x|AFTER runner live: repo-only mirror wiring posts one GitHub commit status per GitLab gate system under the neutral context `gitlab gate (<system>)`: `pending` while running, `success` on pass, `failure` on gate failure, `error` on publisher/API error; then those contexts are required on GitHub `main`; no lablith feature, seeded script or arg|R15,R19
T60|x|GitHub repo settings: squash off, delete branch on merge, auto-merge on; topics on lablith, pipelith, actlith|V73,R15
T61|x|`glab` in dev shell|R16,V42
