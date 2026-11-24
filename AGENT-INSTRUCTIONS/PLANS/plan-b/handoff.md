# Orchestrator handoff

## Next action

Waiting on plan-a's resolution phase file A1r. When plan-a notifies that it exists: run the
orchestrator acceptance check on `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A1r*.md` against
`AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B1.md`, then `channel_close` naming A1r and B1r,
then start B1r's implementer.

## Current state

- `phases/B1r-shared-hit-record-shape.md` - detailed, accepted, `DETAIL READY`; B1r is the active
  phase and carries B1's whole scope.
- B1 is `not applicable` (absorbed by B1r); B2 now depends on B1r; Z1 depends on B1r + B2 + B3.
- B1r implements plan-b's side of the shared design: `front_face`/`u`/`v`, one six-argument
  constructor, sphere (u, v), 2 new tests + 1 adapted, byte-identical PPM. Material, `Rc` and the
  dropped `Copy`/`Debug` stay in the release merge.
- Merge materials are settled: both matte, ground mid-grey like the current ground, triangle a
  distinct dull green; recorded in the shared design and in B2's scope.
- Autonomous mode is enabled for small, easily undoable, behaviour-neutral choices only;
  `autonomous-decisions.md` is created when the first such decision is recorded. Product decisions
  still go to the user.

## Learned rules
