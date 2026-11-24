# Orchestrator handoff

## Next action

`phase-advancer` is detailing the resolution phase B1r in the background (`DETAIL RESOLUTION`, pair
`plan-a-A1+plan-b-B1`). When it returns `DETAIL READY`: run the orchestrator acceptance check on the
new resolution phase file, then call `channel_close` naming plan-a's resolution phase file, then
continue the loop with B1r's implementer.

## Current state

- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B1.md` - agreed shared `HitRecord` design; both
  orchestrators accepted it.
- `phases/B1-face-side-and-texture-coordinates.md` - detailed and accepted, but B1 is held; it
  becomes `planned` again after the resolution and is detailed fresh before its implementer.
- The only pending user decision is the material and colour for the ground plane and the triangle
  at the merge, settled before B2 is detailed and not blocking B1's resolution. It is recorded in
  the shared design; plan-a's orchestrator is asking the user.
- Autonomous mode is enabled for small, easily undoable, behaviour-neutral choices only;
  `autonomous-decisions.md` is created when the first such decision is recorded. Product decisions
  still go to the user.

## Learned rules
