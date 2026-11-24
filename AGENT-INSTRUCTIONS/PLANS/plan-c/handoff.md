# Orchestrator handoff

## Next action

Phase Z1's document is being re-detailed to the binding survey method (orchestrator-dispatched
`ops-scout` per area, one independent cross-check `ops-scout`, orchestrator writes the note, no
implementer). When it returns, acceptance-check it, then execute Z1 myself: four scouts in parallel,
one cross-check scout, write `benchmarks/`, run the acceptance render, present to the user.

## Current state

- C1 and C3 are done (`done/C1-png-writer-and-command-line.md` at `910a9c5`,
  `done/C3-progress-line.md` at `e194181`). C3's progress line is verified: stderr rows, image bytes
  unchanged, 0 new tests, 1 adapted test.
- Z1 (closing survey and user acceptance) is active and temporarily mis-detailed: it gave the survey
  to a phase-implementer with a reviewer cross-check, which contradicts the binding survey method.
  A fresh advancer is correcting it.
- The plan's product work is complete apart from the closing survey and the user's acceptance; the
  plan is then ready for the release, which closes it.
- `docs-pass.md` holds the release item for the `gates.md` fallback sentence.
- Shared settlement with plan-a is closed (`PLANS/shared/plan-a-A1+plan-c-C3.md`).
- Autonomous mode is on for the routine loop only; the closing acceptance is the user's.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information, not a blocker.
- Never write plan text until the user says to write; restate intent in plain words first.
- Ask plain questions in the chat, never with a question tool, and give a recommendation with each.
- A permanent file shared by every worktree (gates, workspace `LOCAL/`) must stay true in worktrees
  that lack this plan's code; defer its cleanup to the release through `docs-pass.md`.
- A held phase still gets its `phase_subagent` call: the refusal is the recorded stop, and nothing
  else starts afterwards.
- As leader, verify a partner's factual claim before writing it into a shared design.
- The closing survey is orchestrator work: one independent `ops-scout` per area in parallel, one
  independent `ops-scout` cross-checking all findings, and the orchestrator writes the note. Never a
  phase-implementer, never a per-area checker.
- Autonomous mode never covers behaviour a user can see, deployment, security boundaries, spending
  or plan closure.
- Check a receipt for its explicit `REVIEW:` line before treating the review loop as finished.
- Every child prompt must forbid the question tool and require `USER DECISION REQUIRED` or `BLOCKED`.
