# Orchestrator handoff

## Next action

Start the A2 implementer (`phase_subagent` with phase id A2). On its receipt, spawn a fresh
`phase-advancer` in `REVIEW AND ADVANCE` for A2.

## Current state

A1 is done and frozen at `4869d5f`. A2 (glass) is active and detailed at
`phases/A2-glass.md`: a `Dielectric` material with Snell refraction, TIR and Schlick, the
face side derived locally from the outward normal in two lines (the release merge replaces
that), white attenuation, and one glass sphere at (-0.85, -0.1, -1.3) radius 0.4 index 1.5
in the scene. Three new tests, one per requirement-4 branch, within the §3a cap. A1's
five small doc-comment gaps were recorded as carry-overs in `done/A1` and folded into A2's
steps. The three shared designs remain settled and their pairs closed. The user still owes
the optional `/workflow-reload` after the `project.md` map edit. Autonomous mode covers
routine loop choices only; every product decision goes to the user.

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
