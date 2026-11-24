# Orchestrator handoff

## Next action

Run Z1 (closing survey): dispatch four read-only `ops-scout` area scouts in parallel, save each return
under `benchmarks/returns/`, dispatch one cross-check `ops-scout`, write
`benchmarks/closing-survey-<date>.md` myself, run the branch acceptance render, then ask the user for
acceptance of the note and image.

## Current state

A1 done (`de1cf31`), A2 done (`a1d640a`), A3 done (`566b11c`); A3r done. All three coordinator pairs are
closed; designs in `PLANS/shared/`. Z1 (the plan's closing survey and the only user gate) is active and
detailed at `phases/Z1-closing-survey.md`. `docs-pass.md` holds four documentation items plus the
answered release-merge carry-over (triangle material: `Lambertian` `(0.7, 0.3, 0.3)`).

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
