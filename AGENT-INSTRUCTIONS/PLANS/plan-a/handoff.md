# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `DETAIL ACTIVE` for the survey remediation unit `R1`
(two accepted defects from the closing survey). Then an implementer, then review, then
close Z1 and report the plan ready for the release.

## Current state

The plan's product work is done: A1, A2 and A3 are frozen at `46a5c02`; the closing survey
Z1 ran with four area scouts plus one cross-check; the note is
`benchmarks/closing-survey-2026-11-24.md` (returns beside it) and the user accepted the
findings and the image, so Z1's own gate is met. The user approved two remediations: R1 the
hit record's ray-parameter wording and the `0.001` epsilon wording, R2 the whole-image sky
test shrunk to a small image with a few pixels asserted plus one correcting line in
`PLANS/shared/plan-a-A1+plan-c-C3.md`. `Vec3::cross` stays untouched; the geometry plan's
plane phase and the release merge resolve it. Recording the acceptance and closing Z1
happen after R1 lands. Autonomous mode covers routine loop choices only; every product
decision goes to the user.

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
