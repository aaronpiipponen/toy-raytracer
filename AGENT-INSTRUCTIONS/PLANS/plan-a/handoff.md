# Orchestrator handoff

## Next action

Start the A1r implementer (documentation-only settlement phase). On its receipt, spawn a
fresh `phase-advancer` in `REVIEW AND ADVANCE` for A1r, then the A1 implementer.

## Current state

The A1+B1 pair is RESOLVED. Plan-a's resolution phase is
`phases/A1r-shared-hit-record-settlement.md` (gate `minimal`, no product code, no receipt
commit invented); the status table has A1r `active` and A1 `planned`, still pointing at its
unchanged detail `phases/A1-diffuse-and-metal-materials.md`. I corrected that A1 row's
`Phase file`, which the leader's edit left empty. The shared design is
`PLANS/shared/plan-a-A1+plan-b-B1.md`: plan-a's branch changes no record shape; the face
side, (u, v), the seven-argument constructor, A2's face-side replacement and the new
shapes' materials are the release merge's. A1 remains held-behind-A1r only in the phase
order, not by the coordinator. The user's ground-plane and triangle materials are settled
(matte; mid-grey ground, dull green triangle). Autonomous mode covers routine loop choices
only; every product decision goes to the user.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information.
- Autonomous mode covers only small, easily undoable, behaviour-neutral choices; record
  each in `autonomous-decisions.md`. Every product decision goes to the user.
- Tell advancers and implementers the work kind (product here) and the gate level from the
  status table.
- The shared design is authoritative for the resolution; when the leading plan writes a
  file, stop the local advancer at once and hand the path over rather than let two copies
  exist.
- A child's claimed correction must be checked against the file; the leader reported fixing
  A1's row but the status table still had an empty Phase file, which I fixed myself.
- A1 must not duplicate the geometry plan's face side, texture coordinates or constructor.
