# Orchestrator handoff

## Next action

NONE for this plan. Every phase row is `done` and the closing survey is accepted; the plan awaits the
release (which also closes and archives it) and the user's explicit closure authorization. Do not
archive it here.

## Current state

Plan-a (materials) is complete on branch `plan-a` at head `b22d647`, five commits `de1cf31`, `a1d640a`,
`566b11c`, `7853f29`, `b22d647`, worktree clean. A1, A2, A3, A3r, R1 and Z1 are all `done`. The closing
survey (`benchmarks/closing-survey-2026-11-24.md` with five returns) and the rendered image were accepted
by the user; the accepted cleanups are landed (R1). Test-coverage gaps were accepted as-is by the user.
`docs-pass.md` carries four documentation items plus the answered triangle-material carry-over
(`Lambertian` `(0.7, 0.3, 0.3)`) for the release. Three coordinator pairs are closed with designs under
`PLANS/shared/`.

## Learned rules

- A coordinator hold (`COORDINATOR HOLD`) stops every phase start, including unrelated ones. Make the
  held phase's `phase_subagent` call once so the stop is recorded, then start nothing until the
  coordinator or partner messages.
- A hold stops only the held phase's implementer; advancers and the review-and-advance of a different,
  unheld phase may still run before reaching the held phase's start.
- As pair leader, use `channel_send` to agree implementation detail only; a product-behavior choice
  (such as the scene triangle's material) is the user's and goes in the shared design as an open
  decision, not a settled one. As partner, answer channel messages from known plan facts and start
  nothing until `RESOLVED`.
- The leader may ask the partner to write its own resolution record via `channel_send` and wait for a
  `channel_notify`; it need not write the partner's file.
- Keep exactly one `active` row. When a resolution makes an earlier phase active, move the previously
  active queued phase to `next`.
- When the user directs one advancer to write both plans' resolution files, it may write plan-b's
  phase document (not its code) per the shared design.
- A coordinator hold lifts only on `channel_close`; close the pair after accepting both resolution
  phase files, before starting the resolution implementer.
- A resolved partner may receive a resolution-bookkeeping phase (documentation-only) that amends the held
  phase document in place; run it, then implement the held phase.
- The branch's `render` doc sentence describes the merged product's progress line (frozen by the shared
  designs); it is deliberately not branch-accurate and must not be edited again at the merge.
- A survey phase runs no implementer: I dispatch the scouts myself, save their returns, write the note,
  and stop for the user's subjective image/survey acceptance.
- After the survey, the user's accepted cleanups run as a small remediation phase; the accepted test
  gaps add no test, and the survey acceptance stands because the fixes change no behaviour.
- Plan-folder edits are committed by the workflow plugin at the workspace root, never from the worktree.
- Plan completion is not closure: the release closes and archives the plan, and archiving needs the
  user's explicit authorization.

## Learned rules

- A coordinator hold (`COORDINATOR HOLD`) stops every phase start, including unrelated ones. Make the
  held phase's `phase_subagent` call once so the stop is recorded, then start nothing until the
  coordinator or partner messages.
- A hold stops only the held phase's implementer; advancers and the review-and-advance of a different,
  unheld phase may still run before reaching the held phase's start.
- As pair leader, use `channel_send` to agree implementation detail only; a product-behavior choice
  (such as the scene triangle's material) is the user's and goes in the shared design as an open
  decision, not a settled one. As partner, answer channel messages from known plan facts and start
  nothing until `RESOLVED`.
- Keep exactly one `active` row. When a resolution makes an earlier phase active, move the previously
  active queued phase to `next`.
- When the user directs one advancer to write both plans' resolution files, it may write plan-b's
  phase document (not its code) per the shared design.
- A coordinator hold lifts only on `channel_close`; close the pair after accepting both resolution
  phase files, before starting the resolution implementer.

## Learned rules
