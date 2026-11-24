# Orchestrator handoff

## Next action

The plan's phase work is complete and accepted by the user. Nothing to spawn. The release
(`/release`, `AGENT-INSTRUCTIONS/BUILDING/RELEASE-WORKFLOW.md`) is the next step and it closes the
plan; it reads `docs-pass.md` as input. Do not archive or close the plan from here.

## Current state

- Every phase row in `full-plan.md` is `done` or `not applicable`: C1 (PNG writer and command line,
  `910a9c5`), C2 merged into C1, C3r (shared render settlement), C3 (progress line, `e194181`), Z1
  (closing survey and user acceptance).
- The closing survey is written (`benchmarks/2026-11-24-closing-survey.md`), with the four area
  returns in `survey-returns-2026-11-24.md`. The user accepted the survey, the result and the
  acceptance render on 2026-11-24 and accepted five coverage gaps as sufficient for a hobby project.
- Product tree is unchanged since `e194181`; the branch `plan-c` holds two commits of product work.
- `docs-pass.md` holds six release items, each with its file and what is wrong: the rendered-image
  gate fallback sentence plus the missing PPM command, the `project.md` map omitting `src/png.rs`,
  the three plan-naming comments (each removable only once its adjacent dead-code allowance is gone),
  and the README's bare `cargo`.
- Shared settlement `PLANS/shared/plan-a-A1+plan-c-C3.md` stands for the release merge; plan-a
  continues its own material phases independently.
- No coordinator hold is active. Autonomous mode is on for the routine loop only.

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
- A phase's MUST NOT that bans editing record files conflicts with a survey's own required
  corrections and with a later user instruction; the user's instruction and the survey step win, and
  the conflict is reported rather than silently resolved.
- Autonomous mode never covers behaviour a user can see, deployment, security boundaries, spending
  or plan closure.
- Check a receipt for its explicit `REVIEW:` line before treating the review loop as finished.
- Every child prompt must forbid the question tool and require `USER DECISION REQUIRED` or `BLOCKED`.
