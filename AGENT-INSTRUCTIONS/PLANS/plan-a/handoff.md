# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `DETAIL RESOLUTION` for pair plan-a-A1+plan-c-C3 and
plan-a's held phase A1, creating documentation-only phase `A1r2 - Shared render settlement`.
Then acceptance-check it, notify plan-c of the path, and wait for `RESOLVED` before starting
A1.

## Current state

The plan-a/plan-c shared design is written at
`PLANS/shared/plan-a-A1+plan-c-C3.md` and matches what I agreed, including the four notes I
sent. Plan-c's C1 is landed on its branch; its C3 is detailed and held. Plan-a's A1 is
still held against C3 by the coordinator until the pair resolves. A1r (the geometry-pair
settlement) is done; A1's detail is current and unchanged. The user's ground-plane and
triangle materials are settled. Autonomous mode covers routine loop choices only; every
product decision goes to the user.

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
- Resolution ids never reuse an existing id: the second resolution of A1 is `A1r2`.
