# Orchestrator handoff

## Next action

The C3 implementer is running (pair plan-a-A1+plan-c-C3 was settled and the channel closed). When it
returns: check for its `REVIEW:` line and finished review loop, extract the commit SHA, receipt and
comments, then spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` mode for C3, which also drafts
Z1 (the closing survey).

## Current state

- Shared design `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md` is agreed and closed. Both
  resolution files passed the orchestrator check: `done/C3r-shared-render-settlement.md` here and
  plan-a's `A1r2`.
- C1 is done (`done/C1-png-writer-and-command-line.md`, commit `910a9c5`). C3 is active and detailed
  at `phases/C3-progress-line.md`, unchanged by the settlement; gate level standard, 0 new tests,
  1 adapted binary test, byte comparison against a pre-change render.
- Z1 (closing survey and user acceptance) stays planned behind C3.
- `docs-pass.md` holds the release item for the `gates.md` fallback sentence.
- Autonomous mode is on for the routine loop only; every product decision still goes to the user.
- No coordinator hold is active.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information, not a blocker.
- Never write plan text until the user says to write; restate intent in plain words first.
- Ask plain questions in the chat, never with a question tool, and give a recommendation with each.
- A permanent file shared by every worktree (gates, workspace `LOCAL/`) must stay true in worktrees
  that lack this plan's code; defer its cleanup to the release through `docs-pass.md`.
- A held phase still gets its `phase_subagent` call: the refusal is the recorded stop, and nothing
  else starts afterwards.
- As leader, verify a partner's factual claim before writing it into a shared design.
- A settled shared design fixes implementation shape only; anything that changes how the product
  behaves is the user's decision.
- Autonomous mode never covers behaviour a user can see, deployment, security boundaries, spending
  or plan closure.
- Check a receipt for its explicit `REVIEW:` line before treating the review loop as finished.
- Every child prompt must forbid the question tool and require `USER DECISION REQUIRED` or `BLOCKED`.
