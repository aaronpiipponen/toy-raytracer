# Orchestrator handoff

## Next action

AWAITING COORDINATOR. A `COORDINATOR HOLD` is armed for pair `plan-a-A2+plan-b-B2`; A2 may not start.
When the coordinator or partner messages, settle the overlap as leader (write
`AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A2+plan-b-B2.md`), spawn a fresh `phase-advancer` in DETAIL
RESOLUTION for A2, run the acceptance check on both resolution phase files, `channel_close`, then
implement A2.

## Current state

A1 is done and verified on branch `plan-a` (base `e180476`, tested head `de1cf31`); frozen contract in
`done/A1-diffuse-and-metal-materials.md`. A2 is `blocked`/held on the coordinator settlement of the
shared face-side shape with plan-b B2. A3 (samples per pixel) is detailed in
`phases/A3-samples-per-pixel.md`, passed the orchestrator acceptance check (line refs verified against
`de1cf31`; 2 new + 1 adapted test, within §3a cap), and is queued to start after the hold lifts.
`docs-pass.md` holds one item for the release.

## Learned rules

- A coordinator hold (`COORDINATOR HOLD`) stops every phase start, including unrelated ones. Make the
  held phase's `phase_subagent` call once so the stop is recorded, then start nothing until the
  coordinator or partner messages.

## Learned rules
