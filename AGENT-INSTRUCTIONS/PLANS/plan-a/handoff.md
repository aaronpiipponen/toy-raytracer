# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` for A1r with the implementer's
receipt. Then make A1 active and attempt its implementer; A1 is held against plan-c's C3,
so expect `COORDINATOR HOLD` and stop there.

## Current state

A1r's implementer returned a passing documentation-only receipt at unchanged head
`e180476`; it corrected three citation ranges in the phase file and handed two items to the
advancer (the merge's `HitRecord` doc-comment obligation, and A1's stale `Status`/`Depends
on` header). The A1+B1 pair is RESOLVED; plan-a's branch recorded the shared design without
product code. A new coordinator hold covers A1 against plan-c's C3 (plan-c leads): plan-c's
C1 already put per-pixel averaging in `render` and changed `ray_colour`'s signature, so the
settlement must fix where sampling and bounces nest, where the single `Rng` lives and how
gamma applies to averaged samples; it also decides whether A3 still has work. The user's
ground-plane and triangle materials are settled (matte; mid-grey ground, dull green
triangle). Autonomous mode covers routine loop choices only; every product decision goes to
the user.

## Learned rules

- The user's word overrides `docs/requirements.md`; report a conflict as information.
- Autonomous mode covers only small, easily undoable, behaviour-neutral choices; record
  each in `autonomous-decisions.md`. Every product decision goes to the user.
- Tell advancers and implementers the work kind (product here) and the gate level from the
  status table.
- The shared design is authoritative for a resolution; when the leading plan writes a
  file, stop the local advancer at once rather than let two copies exist.
- A child's claimed correction must be checked against the file.
- A1 must not duplicate the geometry plan's face side, texture coordinates or constructor.
