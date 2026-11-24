# Orchestrator handoff

## Next action

Attempt the A1 implementer. A1 is held against plan-b's B2 and plan-a leads that
settlement, so the `phase_subagent` call is expected to be refused with `COORDINATOR HOLD`;
make it so the stop is recorded, then start nothing else and wait for plan-b's orchestrator.

## Current state

A1r2 is done, compressed and its row flipped; A1 is the sole active row with its existing
detail, which the advancer confirmed unchanged and current at `e180476`. All three shared
designs are settled: the hit record (with plan-b), the render shape (with plan-c), and the
pending scene/hit-record-list design with plan-b's B2 that I lead. The user's
ground-plane and triangle materials are settled (matte; mid-grey ground, dull green
triangle). Autonomous mode covers routine loop choices only; every product decision goes to
the user.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information.
- Autonomous mode covers only small, easily undoable, behaviour-neutral choices; record
  each in `autonomous-decisions.md`. Every product decision goes to the user.
- Tell advancers and implementers the work kind (product here) and the gate level from the
  status table.
- The shared design is authoritative for a resolution; a partner never starts the held
  phase before `RESOLVED`, and a closed pair is not reopened for wording that does not
  change the merged code.
- A child's claimed correction must be checked against the file.
- A1 must not duplicate the geometry plan's face side, texture coordinates or constructor.
- Resolution ids never reuse an existing id: the third resolution of A1 is `A1r3`.
- The shared file for the B2 pair is `PLANS/shared/plan-a-A1+plan-b-B2.md`; the earlier
  geometry-pair file is authoritative for the record shape and the new shapes' materials.
