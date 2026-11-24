# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` for A1 with the implementer's
receipt. Then detail/start A2 unless a coordinator hold covers it.

## Current state

A1 is implemented and verified at commit `4869d5f` (base `e180476`): materials module,
material on the hit record and spheres, 50-bounce loop with one seeded generator, gamma 2,
and the three-material scene. Fast gate green, 7 tests pass, two renders byte-identical,
image inspected. One review round, no real finding. The implementer edited the repository
map in `AGENT-INSTRUCTIONS/LOCAL/project.md`, so the session's injected project context is
slightly stale until the user runs `/workflow-reload`; it does not affect the loop. The
three shared designs are settled and their pairs closed. Autonomous mode covers routine
loop choices only; every product decision goes to the user.

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
