# Orchestrator handoff

## Next action

Run `phase-advancer` in DETAIL RESOLUTION for pair `plan-a-A2+plan-b-B2` (shared design at
`PLANS/shared/plan-a-A2+plan-b-B2.md`). Then run the acceptance check on BOTH resolution phase files,
`channel_close`, and continue with A2's resolution implementer.

## Current state

A1 done (head `de1cf31`). The A2/B2 overlap is settled: combined `HitRecord` (material + front_face +
u/v, ray-facing normal, seven-argument `HitRecord::new`); plan-a lands the full record in A2, plan-b's
B2 stays material-less, and the release merge completes Plane/Triangle. A2 is being re-detailed as a
resolution phase. A3 is detailed in `phases/A3-samples-per-pixel.md` and passed acceptance, queued.
`docs-pass.md` holds one item for the release.

## Learned rules

- A coordinator hold (`COORDINATOR HOLD`) stops every phase start, including unrelated ones. Make the
  held phase's `phase_subagent` call once so the stop is recorded, then start nothing until the
  coordinator or partner messages.
- As pair leader, use `channel_send` to agree implementation detail only; a product-behavior choice
  (such as the scene triangle's material) is the user's and goes in the shared design as an open
  decision, not a settled one.

## Learned rules
