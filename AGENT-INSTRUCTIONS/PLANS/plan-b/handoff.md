# Orchestrator handoff

## Next action

`phase-advancer` is advancing B2r in the background. Pair `plan-a-A1+plan-b-B2` is settled:
`PLANS/shared/plan-a-A1+plan-b-B2.md`, plan-a's `A1r3` and plan-b's
`phases/B2r-shared-scene-settlement.md` both passed my check, and I sent plan-a `RESOLVED`. When the
advancer returns, run the orchestrator acceptance check on B2's (re-)detail before starting B2's
implementer.

## Current state

- B1r done at `f0f11e2`; B2 detailed and accepted but its implementation was held by the B2 overlap.
- B2's detail (`phases/B2-planes-and-triangles.md`) is unchanged: plane at `y = -0.5`, triangle at
  `z = -2`, six-argument `HitRecord::new`, no material, three new tests, no `docs/` change.
- The B2 settlement (`phases/B2r-shared-scene-settlement.md`, written by plan-a as leader, corrected
  by me) is documentation-only: plan-b's branch shape unchanged; the merge adds
  `Rc<dyn Material>` to `Plane`/`Triangle`, composes the four-material scene, uses the borrowed
  nearest record plus B3's box test, and keeps the plane's normalisation and `(u, v)` mapping.
- B2r has no status row yet; adding it, compressing it and promoting B2 are the advancer's work.
- Autonomous mode: small, easily undoable, behaviour-neutral choices only; `autonomous-decisions.md`
  when the first is recorded. Product decisions still go to the user.

## Learned rules

- The leader of a pair writes both resolution phase files, not only its own; correct the partner's
  draft in place rather than creating a second file.
- A dropped phase keeps its id as a `not applicable` row; a resolution phase that absorbs a held
  phase's whole scope takes that scope and later phases depend on it.
- The plan-b branch must never add the material field or `Rc`; that is the release merge's work.
- A resolution phase's own document is not a plan-state edit: status rows, `scope.md`, `done/` and
  compression belong to the advancer, never to the child that writes the settlement.
