# Orchestrator handoff

## Next action

The Z1 completion advancer is running to compress Z1 and flip its status row now that the user has
accepted the survey. When it returns, verify the status table shows every phase done and report the
plan complete to the user; the plan then waits for the release, which closes it.

## Current state

- C1 and C3 are done (`910a9c5`, `e194181`). Z1's survey is complete and accepted by the user on
  2026-11-24: note at `benchmarks/2026-11-24-closing-survey.md`, raw returns at
  `survey-returns-2026-11-24.md`.
- The user accepted the five coverage gaps as sufficient for a hobby project (recorded in the note as
  accepted gaps) and asked the release to clean the three plan-naming comments plus the README's bare
  `cargo` line. Those are on `docs-pass.md`, each comment item saying its removal waits until the
  dead-code allowance beside it is gone.
- `docs-pass.md` also holds the `gates.md` fallback/PPM command item and the `project.md` `png.rs`
  item.
- No product code changed during the survey; the worktree is clean at `e194181`.
- Autonomous mode is on for the routine loop only. No coordinator hold is active.

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
- Test findings that would assert wording or a choice the requirements do not state are not defects;
  say so rather than adding tests.
- Autonomous mode never covers behaviour a user can see, deployment, security boundaries, spending
  or plan closure.
- Check a receipt for its explicit `REVIEW:` line before treating the review loop as finished.
- Every child prompt must forbid the question tool and require `USER DECISION REQUIRED` or `BLOCKED`.
