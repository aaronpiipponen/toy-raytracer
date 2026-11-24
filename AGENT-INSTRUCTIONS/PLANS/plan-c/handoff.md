# Orchestrator handoff

## Next action

A coordinator hold covers phase C3 (pair plan-a-A1+plan-c-C3, plan-c leads). The C3
`phase_subagent` call was made and refused to record the stop; start nothing else until the
coordinator or the partner orchestrator resumes this session. On `CHANNEL OPEN`, settle the shared
design as leader through `channel_send`, write `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md`,
spawn a fresh advancer with `DETAIL RESOLUTION`, acceptance-check both resolution phase files, call
`channel_close`, then run the C3 implementer.

## Current state

- C1 is done, frozen contract at `done/C1-png-writer-and-command-line.md`, commit `910a9c5`.
- `phases/C3-progress-line.md` is detailed (gate level standard, 0 new tests, 1 adapted binary test,
  byte comparison against a pre-change render) and passed the orchestrator acceptance check. C3's
  status row is `active`; its implementer has not started.
- Hold reason from the coordinator: C1's committed `render(width, height, samples)` and the
  per-pixel averaging loop, `pub(crate) to_byte` and the changed module list/tests overlap plan-a's
  A1, which replaces the colour computation with a 50-bounce material loop threaded by one seeded
  `Rng` and rewrites the same hunks. The settlement must fix the combined render signature and loop
  nesting, where the single `Rng` lives, and how gamma applies to averaged samples. C3's progress
  line lands in that same loop.
- `docs-pass.md` holds the release item for the `gates.md` fallback sentence.
- Autonomous mode is on for the routine loop only; every product decision still goes to the user.
- Z1 (closing survey) stays planned behind C3.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information, not a blocker.
- Never write plan text until the user says to write; restate intent in plain words first.
- Ask plain questions in the chat, never with a question tool, and give a recommendation with each.
- A permanent file shared by every worktree (gates, workspace `LOCAL/`) must stay true in worktrees
  that lack this plan's code; defer its cleanup to the release through `docs-pass.md`.
- A held phase still gets its `phase_subagent` call: the refusal is the recorded stop, and nothing
  else starts afterwards.
- Anything that changes how the product behaves is the user's decision, including a shared-design
  choice; settle only implementation detail with the partner, and ask when in doubt.
- Autonomous mode never covers behaviour a user can see, deployment, security boundaries, spending
  or plan closure.
- Check a receipt for its explicit `REVIEW:` line before treating the review loop as finished.
- Every child prompt must forbid the question tool and require `USER DECISION REQUIRED` or `BLOCKED`.
