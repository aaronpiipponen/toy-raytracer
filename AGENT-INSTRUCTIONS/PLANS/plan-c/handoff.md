# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` mode for Z1 (no implementer ran: Z1 is the
orchestrator-run closing survey). Evidence to pass: the note
`benchmarks/closing-survey-2026-11-24.md` (surveyed at `0ad0f2b`, its six findings fixed by C1's
remediation, the intentionally-unfixed list standing), the user's recorded acceptance "accept all",
the acceptance render at `a0c585b` (400x200 PNG opens, stdout empty, progress ends `100%` plus one
newline), and the closed pair plan-a-A3+plan-c-Z1 with `Z1r` done. Ask it to record the acceptance,
compress Z1 into `done/Z1-closing-survey.md`, flip the Z1 row to `done`, and leave the plan ready
for the release without closing or archiving it.

## Current state

All implementation and the pair resolution are done. C1 (with survey remediation), C2 not
applicable, C3, Z1r done; Z1 `active` with the survey complete and accepted by the user. Product
head `a0c585b`, clean; fast gate green; acceptance render inspected. The shared design is
`AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md`; both resolution files (ours and plan-a's
A3r) were written by this leader per the user's instruction, and plan-a owns running A3r and then
implementing A3. `docs-pass.md` holds the three release items (`gates.md` fallback sentence,
`project.md` and `repo-rules-domain.md` maps). No unresolved user decision.

## Learned rules

- Plan files live in the workspace root and are committed by the workflow plugin: never commit them
  from the worktree.
- Cargo's own "Finished"/"Running" lines share standard error with the program: capture program
  streams from the built binary under `target/release/`, not from `cargo run`.
- A plan survey's findings go to the user; only small, checked fixes are remediated before asking.
- When the leader is told to write both resolution files, it does so; each plan still changes only
  its own branch.
