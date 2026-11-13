# SPEC ARCHIVE

Task rows moved out of `SPEC.md` by `mth archive`. An id is never reused
(`V12`), so a citation to an archived row still resolves -- here.

This is a SINK, not a spec. The citations inside these rows point into
`SPEC.md`, so `mth check` on this file reports every one of them as dangling,
correctly and uselessly. The verb that reads it is `mth tasks`.

## §T TASKS

T10|x|federate `SPEC.md` w/ sherd: nodes `src/core`, `src/*`, `tests`, `dev`, `nix`; `.context-limits` ~12% over measured. `src/<node>/mod.rs` + build-order edges land w/ `src:T42`|-
T23|x|`docs/KINDS-GITLAB.md` + parity test vs code & kind nodes (port actlith `nix:T33`)|V24
T34|x|human docs: README (GitLab CI first line) → INTEGRATION, REGISTRY, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT; badges after pages exist|I.docs
T46|x|pipelith issues: `token` default wording per forge → pr0d1r2/pipelith#9 (docs only). id charset ⊥ needed: pipelith already refuses non-file-stem ids|`src/registry:V55`,`src/kind:V54`
