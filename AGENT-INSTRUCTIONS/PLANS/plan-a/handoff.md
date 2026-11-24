# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `DETAIL ACTIVE` for Z1 to write
`phases/Z1-closing-survey.md` and point the row at it. Then run the survey itself: four
read-only `ops-scout`s in parallel (one per area), one cross-checking `ops-scout`, and the
dated note written by this session.

## Current state

A1, A2 and A3 are done and frozen; the branch is at `46a5c02` with the full materials
feature — matte, metal, glass, the 50-bounce loop, one seeded generator, gamma and 16
centred jittered samples per pixel — all fast gates green, 12 tests passing, renders
byte-identical, image inspected and smooth. Z1 is the last row, promoted to active, and
still only has its inline spec in `full-plan.md`. The survey is orchestrator-run: the
binding method has this session dispatch the scouts and write the note, so no implementer
is spawned for Z1. The three shared designs are settled and their pairs closed.
`/workflow-reload` remains optional for the user. Autonomous mode covers routine loop
choices only; every product decision goes to the user.

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
- The closing survey is orchestrator-run: no implementer writes the note and Z1 fixes
  nothing it finds.
