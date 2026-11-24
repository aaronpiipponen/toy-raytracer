# Orchestrator handoff

## Next action

`phase-implementer` is implementing B3 (bounding boxes) in the background. When it returns a
complete receipt, spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` for B3.

## Current state

- B1r, B2r and B2 are done; B2 landed at `9afa2db`. My own commit `281dda8` removed the stale
  "the plans add" comment on `mod rng` in `src/main.rs`.
- B3 active and accepted by my check: `phases/B3-bounding-boxes.md` - `Aabb` type, required
  `bounding_box` on `Hittable`, list box test over `t_min..t_max` before each object's `hit`, plane
  and list report none, 3 new tests, byte-identical rendered image.
- Open small item for the user: `AGENT-INSTRUCTIONS/LOCAL/project.md`'s repository map still omits
  `plane.rs` and `triangle.rs` (and will omit `aabb.rs`); proposed exact text was sent to the user,
  and an edit there needs the user's reload decision.
- Z1 (closing survey plus the user's acceptance) follows after B3.
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
