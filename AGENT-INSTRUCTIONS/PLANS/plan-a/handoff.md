# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` for A1r3 with the implementer's
receipt, then start the A1 implementer once A1 is the active row.

## Current state

A1r3's implementer returned a documentation-only receipt at `3e1c8ab` (workspace-root
round commit; the plan-a worktree has no product commit and no product code moved) after
the user stopped its shell-check loop with a narrow follow-up; its reviewer pass was
skipped for that reason, so the advancer's own re-read and compression are the check.
A1r3 fixed four slips in the phase text only. A1 is planned behind A1r3 with its detail
current at `e180476`; its header still says `Status: active / Depends on: A1r` while the
table is authoritative. All three shared designs are settled and their pairs closed.
Autonomous mode covers routine loop choices only; every product decision goes to the user.

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
