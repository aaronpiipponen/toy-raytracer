# Orchestrator handoff

## Next action

The C3r resolution advancer is running for pair plan-a-A1+plan-c-C3. When it returns: acceptance-check
`phases/C3r-*.md` and confirm C3's detail matches `PLANS/shared/plan-a-A1+plan-c-C3.md`, ask plan-a's
orchestrator (through `channel_send`) to record its `A1r2` file and confirm, acceptance-check that file
too, then `channel_close` and run the C3 implementer.

## Current state

- Shared design for pair plan-a-A1+plan-c-C3 is written and agreed by both orchestrators at
  `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md`: frozen `render(width, height, samples)`,
  row/column/sample/bounce nesting, one `Rng::new(SEED)` at the top of `render`, linear averaging with
  a single gamma in `to_byte`, C3's two progress calls inside the row loop, A3 jitter drawing two
  values per sample before the trace, `samples >= 1` guaranteed by the CLI, and no test asserting
  sample-count invariance (verified: none exists).
- C1 is done, frozen contract at `done/C1-png-writer-and-command-line.md`, commit `910a9c5`.
- `phases/C3-progress-line.md` is detailed, gate level standard, 0 new tests, 1 adapted binary test,
  byte comparison against a pre-change render; it passed the orchestrator acceptance check before the
  hold. C3 is held until the resolution phase is recorded and the channel closes.
- plan-a's side records `A1r2` ("Shared render settlement"), documentation-only; plan-a's A1 detail is
  unchanged on its branch.
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
- As leader, check a partner's factual claim before writing it into the shared design; plan-a's
  "delete the sample-count invariance test" claim was wrong — no such test exists.
- Anything that changes how the product behaves is the user's decision; a shared design settles
  implementation shape only.
- Autonomous mode never covers behaviour a user can see, deployment, security boundaries, spending
  or plan closure.
- Check a receipt for its explicit `REVIEW:` line before treating the review loop as finished.
- Every child prompt must forbid the question tool and require `USER DECISION REQUIRED` or `BLOCKED`.
