# Orchestrator handoff

## Next action

Close Z1: the survey ran, the user accepted the findings and the image in writing, and R1
fixed the two accepted defects. Spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` for
Z1 with the user's acceptance and R1's receipt as evidence, so it records the acceptance in
the note's user-decision section, compresses Z1 into `done/`, and flips its row. Then report
the plan complete and ready for the release.

## Current state

All product work is landed and frozen at `a7ce19d`: materials (A1), glass (A2), jittered
sampling (A3), and the two accepted survey fixes (R1: ray-parameter wording, small sky
test, corrected shared render-note line). The closing survey note
`benchmarks/closing-survey-2026-11-24.md` holds the findings, the user's dated acceptance
and the decision to leave `Vec3::cross` to the geometry plan and the release merge. R1
supersedes the old `the_top_row_is_sky` call recorded in `done/A3-samples-per-pixel.md`;
the A3 contract stays frozen and the release merge follows R1. Z1 is the only remaining
row, planned but its exit conditions are met. Autonomous mode covers routine loop choices
only; every product decision goes to the user.

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
- A later contract may supersede a frozen one; record the supersession in the new contract
  and leave the old file frozen and unedited.
