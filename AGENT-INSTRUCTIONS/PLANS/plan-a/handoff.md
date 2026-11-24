# Orchestrator handoff

## Next action

Run plan-a's A1r3 unit: submit A1r3 to a fresh `phase-implementer`, then a fresh
`phase-advancer` in `REVIEW AND ADVANCE`. A1 resumes after A1r3 is done.

## Current state

The A1+B2 pair is resolved (plan-a led) and the channel is closed. `PLANS/shared/`
holds all three settled designs: the hit record, the render shape and the scene
composition. Plan-a's `A1r3` is the sole active row, gate `minimal`, documentation-only,
recording plan-a's half of the scene settlement; `A1` is planned behind it with its
existing detail current at `e180476`; `A3` depends on `A1 + A1r2`. Plan-b's `B2r` was
written by this session and accepted by plan-b, which owns its row and compression. The
`docs-pass.md` file was removed because plan-b's B2 deletes the stale `Vec3::cross`
comment. Autonomous mode covers routine loop choices only; every product decision goes to
the user.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information.
- Autonomous mode covers only small, easily undoable, behaviour-neutral choices; record
  each in `autonomous-decisions.md`. Every product decision goes to the user.
- Tell advancers and implementers the work kind (product here) and the gate level from the
  status table.
- The shared design is authoritative for a resolution; a partner never starts the held
  phase before `RESOLVED`; a closed pair is not reopened for wording alone.
- The leader of a pair writes both resolution phase files; the partner checks them.
- A child's claimed correction must be checked against the file.
- A1 must not duplicate the geometry plan's face side, texture coordinates, constructor or
  bounding boxes.
- Resolution ids never reuse an existing id: a future resolution of A1 is `A1r4`.
- A documentation-only phase must not be left in a shell-check loop: send one narrow
  follow-up to re-read the settlement against the shared file and return the receipt, and
  never let a paper phase block the next product phase.
