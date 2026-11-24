# Orchestrator handoff

## Next action

Start C1's remediation implementer with `phase_subagent` and phase id `C1`. The remediation phase
document `phases/C1-survey-remediation.md` passed the orchestrator acceptance check: five small
fixes, no new test function, no behaviour change, standard gate.

## Current state

C1 and C3 are done (`done/` contracts; heads `fb4d00c`, `0ad0f2b`). Z1's closing survey ran on
`0ad0f2b`: four area scouts plus one cross-check; the note is
`benchmarks/closing-survey-2026-11-24.md`. It confirms six small defects (four in test proof, one
`parse_args` duplication, one plan-record wording) and no material code defect. `docs-pass.md` was
corrected and now also defers the `project.md` and `repo-rules-domain.md` map updates to the release.
Acceptance render at `0ad0f2b`: 400x200 PNG opens correctly, stdout empty, progress ends `100%` plus
one newline. No unresolved user decision. Linked to the coordinator; no shared resource.

## Learned rules

- Plan files live in the workspace root and are committed by the workflow plugin: never commit them
  from the worktree.
- Cargo's own "Finished"/"Running" lines share standard error with the program: capture program
  streams from the built binary under `target/release/`, not from `cargo run`.
- A plan survey's findings go to the user; only small, checked fixes are remediated before asking.
