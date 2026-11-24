# Orchestrator handoff

## Next action

Attempt the A1 implementer. A1 is held against plan-c's C3 (plan-c leads), so the
`phase_subagent` call is expected to be refused with `COORDINATOR HOLD`; make it so the stop
is recorded, then start nothing else and wait for plan-c's orchestrator.

## Current state

A1r is done and compressed; A1 is active with its existing detail (header now depends on
A1r), re-verified current against `e180476` by the advancer. The advancer fixed two
bookkeeping gaps: the settlement and carry-overs now obligate the release merge to update
`HitRecord`'s doc comment to the normal facing the ray, and A1's header dependency. The
A1+B1 pair stays RESOLVED. The new hold covers A1 against plan-c's C3: plan-c's committed
C1 already put per-pixel averaging in `render` and changed `ray_colour`'s signature, so the
settlement must fix where sampling and bounces nest, where the single `Rng` lives and how
gamma applies to averaged samples, and whether A3 still has work. The user's ground-plane
and triangle materials are settled. Autonomous mode covers routine loop choices only; every
product decision goes to the user.

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
