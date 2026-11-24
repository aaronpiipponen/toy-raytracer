# Orchestrator handoff

## Next action

Start the A1 implementer (`phase_subagent` with phase id A1). On its receipt, spawn a fresh
`phase-advancer` in `REVIEW AND ADVANCE` for A1.

## Current state

A1r3 is done and compressed; A1 is the sole active row with its existing detail at
`e180476`, header now depending on A1r3, and the advancer's spec audit fixed two items: the
"no comment mentions a plan" gate now scopes to comments A1 writes (the `Vec3::cross`
comment belongs to the geometry plan's branch), and the metal acceptance wording says
blurred reflection and no large black patches. All three shared designs are settled and
their pairs closed; no coordinator hold is active. Autonomous mode covers routine loop
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
