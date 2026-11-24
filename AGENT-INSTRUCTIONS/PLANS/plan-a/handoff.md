# Orchestrator handoff

## Next action

Start the R1 implementer (`phase_subagent` with phase id R1). On its receipt, spawn a fresh
`phase-advancer` in `REVIEW AND ADVANCE` for R1. Then close Z1 with the user's recorded
acceptance and report the plan ready for the release.

## Current state

A1, A2 and A3 are frozen at `46a5c02`. The closing survey note
`benchmarks/closing-survey-2026-11-24.md` holds the accepted findings and the user's dated
decision: fix the ray-parameter wording, shrink the sky test and correct the one shared
render-note line, leave `Vec3::cross` to the geometry plan and the release merge. R1 is
active and detailed at `phases/R1-closing-survey-fixes.md`; Z1 is planned depending on R1.
R1 changes doc comments, one code comment, one test's size and one shared-file line; the
rendered bytes must be identical. Autonomous mode covers routine loop choices only; every
product decision goes to the user.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information.
- Autonomous mode covers only small, easily undoable, behaviour-neutral choices; record
  each in `autonomous-decisions.md`. Every product decision goes to the user.
- Tell advancers and implementers the work kind (product here) and the gate level from the
  status table.
- The shared design is authoritative for a resolution; a partner never starts the held
  phase before `RESOLVED`; a closed pair is not reopened for wording alone - except a line
  the user explicitly approved correcting, which is changed together with the code it
  describes.
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
- The closing survey is orchestrator-run: no implementer writes the note and Z1 fixes
  nothing it finds; accepted findings become a separate remediation unit.
