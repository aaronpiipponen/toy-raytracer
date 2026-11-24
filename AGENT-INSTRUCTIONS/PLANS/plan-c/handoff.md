# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `DETAIL ACTIVE` mode to detail initial active phase C1 of plan-c,
then run the orchestrator acceptance check on `phases/C1.md` before any implementer starts.

## Current state

- `full-plan.md` is scoped and revised 2026-11-24: C1 (PNG writer and command line) is active and
  not yet detailed; C2 is a `not applicable` row merged into C1; C3 (progress line) and Z1 (closing
  survey) are planned.
- `scope.md` is filled. No phase document exists. No implementation has started.
- User decisions already settled and written into the plan: C1 and the former C2 are one phase;
  `--samples` is accepted and validated in C1 and passed into `render`, which averages that many rays
  without jitter until the materials plan adds it; `gates.md`'s rendered-image fallback line is
  updated in C1; merged-phase test budget is at most 3 new test functions.
- Autonomous mode is on for the routine loop only; every product decision still goes to the user.
  Decisions settled under it are recorded in `autonomous-decisions.md`.
- Linked to the coordinator; no hold has been announced.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information, not a blocker.
- Never write plan text until the user says to write; restate intent in plain words first.
- Ask plain questions in the chat, never with a question tool, and give a recommendation with each.
- Autonomous mode never covers behaviour a user can see, deployment, security boundaries, spending
  or plan closure.
- Every child prompt must forbid the question tool and require `USER DECISION REQUIRED` or `BLOCKED`.
