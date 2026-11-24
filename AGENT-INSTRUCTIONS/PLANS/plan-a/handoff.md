# Orchestrator handoff

## Next action

Spawn `phase-advancer` DETAIL RESOLUTION for pair `plan-a-A3+plan-b-Z1`: amend plan-a's A3 in place to
record the geometry-union merge contract from `PLANS/shared/plan-a-A3+plan-b-Z1.md` (A3's sampling work
unchanged). Then accept plan-a's A3 resolution and plan-b's Z1 resolution file, `channel_close`, and
implement A3.

## Current state

A1 done (`de1cf31`); A2 done (`a1d640a`); A3r done (plan-c render seam recorded); A3 active (samples per
pixel, amended for the plan-c seam). Pair `plan-a-A3+plan-b-Z1` is settled in
`PLANS/shared/plan-a-A3+plan-b-Z1.md` (the union: non-`Copy` record
with material + front_face/u/v, seven-argument constructor, borrow-plus-box-skip list loop, Sphere with
material and bounding box, Plane/Triangle with material, exact triangle vertices, open user-owned
triangle material). A3's resolution records that contract; plan-b's Z1 records it and runs its own
read-only survey. Z1 (plan-a closing survey) follows. `docs-pass.md` holds one item for the release.

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
- A resolved partner may receive a resolution-bookkeeping phase (documentation-only) that amends the held
  phase document in place; run it, then implement the held phase.

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
