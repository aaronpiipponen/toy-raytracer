# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` for A2 with the implementer's
receipt. Then A3 (samples per pixel) becomes the next unit; the shared render settlement
already fixes its merge shape.

## Current state

A1 is done and frozen at `4869d5f`. A2 (glass) is implemented at `80d833c`: `Dielectric`
with Snell, TIR and Schlick, face side derived locally, white attenuation, the glass sphere
in the scene, A1's doc-comment carry-overs, repository map updated. Fast gate green, 10
tests pass, two renders byte-identical, image inspected with the glass sphere visible, one
review round with no finding, scratch files cleaned up. The three shared designs are
settled and their pairs closed. The user still owes the optional `/workflow-reload` after
the `project.md` map edits. Autonomous mode covers routine loop choices only; every
product decision goes to the user.

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
