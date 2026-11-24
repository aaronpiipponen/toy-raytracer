# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `DETAIL ACTIVE` for A3. Zero phases are active; the
advancer promotes A3 and writes `phases/A3-samples-per-pixel.md`.

## Current state

A1 and A2 are done and frozen (`done/A1-...`, `done/A2-...`); the branch is at `80d833c`
with the materials, bounce loop, gamma and glass sphere landed. A3 is the last product
phase before Z1: it adds the per-pixel sample loop with two jitter draws per sample from
the one seeded generator, row-major, linear averaging, gamma once in `to_byte`, with 16
passed from `main` on this branch, following `PLANS/shared/plan-a-A1+plan-c-C3.md`. At the
release merge plan-c's identical loop wins textually and A3's remaining contribution is
the jitter; the command line already passes `--samples`. The three shared designs are
settled and their pairs closed. `/workflow-reload` remains optional for the user after the
`project.md` map edits. Autonomous mode covers routine loop choices only; every product
decision goes to the user.

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
