# Orchestrator handoff

## Next action

Start the A3 implementer (`phase_subagent` with phase id A3). On its receipt, spawn a fresh
`phase-advancer` in `REVIEW AND ADVANCE` for A3.

## Current state

A1 and A2 are done and frozen; the branch is at `80d833c`. A3 is active and detailed at
`phases/A3-samples-per-pixel.md`. I corrected two things in the detail before starting:
the jitter now centres each draw in its own pixel (`+ next_f64() - 0.5`) so samples are
inside the pixel and the scene keeps A2's geometry instead of shifting half a pixel; and
the temporary source-edit small render was removed from the image gate, leaving the default
renders, the byte comparison and the A2-worktree comparison as the evidence. A3 adds two
tests (same settings reproduce, sample count changes colours), adapts the sky test, and
follows `PLANS/shared/plan-a-A1+plan-c-C3.md`. The three shared designs are settled and
their pairs closed. `/workflow-reload` remains optional for the user. Autonomous mode
covers routine loop choices only; every product decision goes to the user.

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
- Never accept a jitter formula that samples outside its own pixel: the offset is centred
  (`+ next_f64() - 0.5`), so the image keeps the previous revision's geometry.
