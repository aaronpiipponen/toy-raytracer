# Orchestrator handoff

## Next action

Start the A1r2 implementer (documentation-only settlement). On its receipt, spawn a fresh
`phase-advancer` in `REVIEW AND ADVANCE` for A1r2, then attempt the A1 implementer. A1 is
held against plan-b's B2 (I lead that settlement), so expect `COORDINATOR HOLD`, record the
stop, and wait for plan-b's orchestrator.

## Current state

The plan-a/plan-c pair is RESOLVED and plan-c closed the channel. Plan-a's A1r2
(`phases/A1r2-shared-render-settlement.md`) records the shared render design and passed my
acceptance check; gate `minimal`, documentation-only, A1's detail byte-identical. A1r2's
status row is active and A1/A3 wait behind it. A new hold covers A1 against plan-b's B2
(both rewrite `scene()`; B3 builds on the hit record and scene list). The geometry and
render-merge designs are in `PLANS/shared/`. The user's ground-plane and triangle materials
are settled (matte; mid-grey ground, dull green triangle). Autonomous mode covers routine
loop choices only; every product decision goes to the user.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information.
- Autonomous mode covers only small, easily undoable, behaviour-neutral choices; record
  each in `autonomous-decisions.md`. Every product decision goes to the user.
- Tell advancers and implementers the work kind (product here) and the gate level from the
  status table.
- The shared design is authoritative for a resolution; a partner never starts the held
  phase before `RESOLVED`.
- A child's claimed correction must be checked against the file.
- A1 must not duplicate the geometry plan's face side, texture coordinates or constructor.
- Resolution ids never reuse an existing id: the third resolution of A1 is `A1r3`.
