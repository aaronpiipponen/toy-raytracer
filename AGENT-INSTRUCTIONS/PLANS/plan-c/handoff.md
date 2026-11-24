# Orchestrator handoff

## Next action

The C1 implementer is running. When it returns, check that its review loop finished, extract the
commit SHA, the `VERIFICATION RECEIPT` and its comments, then spawn a fresh `phase-advancer` in
`REVIEW AND ADVANCE` mode for C1.

## Current state

- `full-plan.md` is scoped and revised 2026-11-24: C1 (PNG writer and command line) is active and
  detailed at `phases/C1-png-writer-and-command-line.md`; C2 is a `not applicable` row merged into
  C1; C3 (progress line) and Z1 (closing survey) are planned.
- C1's phase document passed the spec auditor twice, including the correction that removed the
  `gates.md` edit. The orchestrator acceptance check passed.
- `docs-pass.md` holds the one release item: delete the stale rendered-image fallback sentence in
  `AGENT-INSTRUCTIONS/LOCAL/gates.md` after all three plans merge. C1 must leave that file unchanged.
- `scope.md` is filled. Implementation has started with the C1 implementer.
- User decisions settled: C1 and the former C2 are one phase; `--samples` is accepted and validated
  in C1 and passed into `render`, which averages that many rays without jitter until the materials
  plan adds it; `gates.md` stays untouched here; C1 adds at most 3 test functions.
- Autonomous mode is on for the routine loop only; every product decision still goes to the user.
  Settled decisions under it are in `autonomous-decisions.md`.
- Linked to the coordinator; no hold has been announced.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information, not a blocker.
- Never write plan text until the user says to write; restate intent in plain words first.
- Ask plain questions in the chat, never with a question tool, and give a recommendation with each.
- A permanent file shared by every worktree (gates, workspace `LOCAL/`) must stay true in worktrees
  that lack this plan's code; defer its cleanup to the release through `docs-pass.md`.
- Autonomous mode never covers behaviour a user can see, deployment, security boundaries, spending
  or plan closure.
- Every child prompt must forbid the question tool and require `USER DECISION REQUIRED` or `BLOCKED`.
