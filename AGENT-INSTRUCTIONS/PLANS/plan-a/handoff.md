# Orchestrator handoff

## Next action

Start A2's implementer with `phase_subagent` (`A2`). When it returns, take its receipt into REVIEW AND
ADVANCE for A2, then implement A3.

## Current state

A1 done (head `de1cf31`). Pair `plan-a-A2+plan-b-B2` closed; both resolution phase files accepted
(plan-a `phases/A2-glass.md`, plan-b `phases/B2-planes-and-triangles.md`) against
`PLANS/shared/plan-a-A2+plan-b-B2.md`. A2's resolution awaits implementation; A3 is `next` (detailed
and accepted). `docs-pass.md` holds one item for the release.

## Learned rules

- A coordinator hold (`COORDINATOR HOLD`) stops every phase start, including unrelated ones. Make the
  held phase's `phase_subagent` call once so the stop is recorded, then start nothing until the
  coordinator or partner messages.
- As pair leader, use `channel_send` to agree implementation detail only; a product-behavior choice
  (such as the scene triangle's material) is the user's and goes in the shared design as an open
  decision, not a settled one.
- Keep exactly one `active` row. When a resolution makes an earlier phase active, move the previously
  active queued phase to `next`.
- When the user directs one advancer to write both plans' resolution files, it may write plan-b's
  phase document (not its code) per the shared design.
- A coordinator hold lifts only on `channel_close`; close the pair after accepting both resolution
  phase files, before starting the resolution implementer.

## Learned rules
