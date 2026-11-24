# Orchestrator handoff

## Next action

The C3 review-and-advance advancer is running. When it returns, run the orchestrator acceptance check
on the Z1 closing-survey phase document it drafts (if it drafts it) before any Z1 implementer, and
check that C3's compressed contract does not weaken the settled render shape.

## Current state

- C1 done (`done/C1-png-writer-and-command-line.md`, commit `910a9c5`). C3 implemented at commit
  `e194181`: progress line on standard error, image bytes unchanged, 0 new tests, 1 adapted binary
  test, one review pass with no finding. C3 is awaiting the advancer's completion and compression.
- Shared render settlement closed with plan-a (`PLANS/shared/plan-a-A1+plan-c-C3.md`,
  `done/C3r-shared-render-settlement.md`). plan-a is finishing `A1r2` and then running its A1
  implementer.
- Z1 (closing survey and user acceptance) is the last phase, planned behind C3. Nothing in the plan
  is currently blocked; no coordinator hold is active.
- `docs-pass.md` holds the release item for the `gates.md` fallback sentence.
- Autonomous mode is on for the routine loop only; every product decision still goes to the user.

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
