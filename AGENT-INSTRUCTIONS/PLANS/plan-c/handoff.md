# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `DETAIL ACTIVE` mode with the settled correction: C1 must not edit
`AGENT-INSTRUCTIONS/LOCAL/gates.md`; that removal moves to the release (`docs-pass.md`). Then run the
orchestrator acceptance check on `phases/C1-*.md` and start the C1 implementer.

## Current state

- `full-plan.md` is scoped and revised 2026-11-24: C1 (PNG writer and command line) is active;
  C2 is a `not applicable` row merged into C1; C3 (progress line) and Z1 (closing survey) are planned.
- `phases/C1-png-writer-and-command-line.md` exists and passed the spec auditor, but still contains
  the now-rejected `gates.md` edit and must be corrected before implementation.
- `scope.md` is filled. No implementation has started. `docs-pass.md` holds the one deferred item.
- User decisions settled and written into the plan: C1 and the former C2 are one phase; `--samples` is
  accepted and validated in C1 and passed into `render`, which averages that many rays without jitter
  until the materials plan adds it; `gates.md`'s fallback line stays in place and its removal is
  deferred to the release; C1's test budget is at most 3 new test functions.
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
