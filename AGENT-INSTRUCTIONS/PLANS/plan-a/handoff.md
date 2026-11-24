# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` for A3 with the implementer's
receipt. Then the closing survey phase Z1 becomes the next unit.

## Current state

A1, A2 and A3 are implemented; A3 is verified at `46a5c02` (base `80d833c`): `render` takes
`samples`, the row/column/sample/bounce loop with two centred jitter draws per sample from
the one seeded generator, 16 passed from `main`, the sky test adapted plus two new tests
(12 pass), fast gate green, two renders byte-identical, the image smoother than A2's, one
review round with no finding. Z1 (closing survey and user acceptance) is the last row,
currently planned. The three shared designs are settled and their pairs closed.
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
