# Orchestrator handoff

## Next action

`phase-implementer` is implementing B2 (planes and triangles) in the background. When it returns a
complete receipt (fresh reviewer, fixes, fresh passes finished), spawn a fresh `phase-advancer` in
`REVIEW AND ADVANCE` for B2.

## Current state

- B1r done at `f0f11e2`; B2r (shared scene settlement) done, `done/B2r-shared-scene-settlement.md`.
- B2 active and accepted by my check: `phases/B2-planes-and-triangles.md` - plane at `y = -0.5`,
  triangle at `z = -2`, six-argument `HitRecord::new`, no material, no box, three new tests, one
  rendered-image acceptance by the agent, no `docs/` change. I corrected the plan's scoped B2 entry
  to say the ground sphere becomes the plane.
- Settlements that bind later phases: `PLANS/shared/plan-a-A1+plan-b-B1.md` and
  `PLANS/shared/plan-a-A1+plan-b-B2.md`; both branches stay material-free; the release merge composes
  the scene and adds materials, boxes, and the borrowed nearest record.
- B3 (bounding boxes) and Z1 follow; Z1 is the mandatory closing survey plus the user's acceptance.
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
