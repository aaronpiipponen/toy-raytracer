# Orchestrator handoff

## Next action

`phase-advancer` is reviewing and advancing B2 in the background. When it returns, read its
next-phase handoff, then run the orchestrator acceptance check on B3's detail before starting B3's
implementer.

## Current state

- B1r done at `f0f11e2`; B2r (scene settlement) done; B2 implemented and reviewed clean at `9afa2db`
  (build, 10 tests, clippy, fmt, `git diff --check`, rendered image shows the flat ground and a
  visible triangle; 3 tests added).
- B2 stays material-free, matching the frozen settlements; the release merge adds materials, the
  borrowed nearest record and the box test.
- B3 (bounding boxes) and Z1 (closing survey plus the user's acceptance) follow.
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
