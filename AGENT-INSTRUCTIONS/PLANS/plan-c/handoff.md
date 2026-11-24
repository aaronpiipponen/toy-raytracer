# Orchestrator handoff

## Next action

The C1 review-and-advance advancer is running. When it returns, read its next-phase handoff, run the
orchestrator acceptance check on the C3 phase document it produces (or on the compressed `done/C1`
contract first), and only then start the C3 implementer.

## Current state

- `full-plan.md` is scoped and revised 2026-11-24: C1 (PNG writer and command line) is active and
  implemented at commit `910a9c5`; C2 is a `not applicable` row merged into C1; C3 (progress line)
  and Z1 (closing survey) are planned.
- C1 implementation: `src/png.rs` added, `src/main.rs` rewritten around one resolved `Settings`,
  `tests/cli.rs` with 3 binary-level tests, README run example updated. Fast gate and rendered-image
  gate pass; the independent implementation reviewer returned no findings; head unchanged since.
- `docs-pass.md` holds the one release item: delete the stale rendered-image fallback sentence in
  `AGENT-INSTRUCTIONS/LOCAL/gates.md` after all three plans merge. C1 left that file unchanged.
- User decisions settled: one output phase; `--samples` accepted and validated here and passed into
  `render` without jitter; `gates.md` untouched; C1 add at most 3 test functions (used 3/0/0).
- Autonomous mode is on for the routine loop only; every product decision still goes to the user.
- Linked to the coordinator; no hold has been announced.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information, not a blocker.
- Never write plan text until the user says to write; restate intent in plain words first.
- Ask plain questions in the chat, never with a question tool, and give a recommendation with each.
- A permanent file shared by every worktree (gates, workspace `LOCAL/`) must stay true in worktrees
  that lack this plan's code; defer its cleanup to the release through `docs-pass.md`.
- Autonomous mode never covers behaviour a user can see, deployment, security boundaries, spending
  or plan closure.
- Check a receipt for its explicit `REVIEW:` line before treating the review loop as finished.
- Every child prompt must forbid the question tool and require `USER DECISION REQUIRED` or `BLOCKED`.
