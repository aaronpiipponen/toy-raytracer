# Orchestrator handoff

## Next action

B2 is held by the coordinator as pair `plan-a-A1+plan-b-B2` (plan-a leads). The `phase_subagent`
start for B2 was attempted and refused, which records the stop. Next: answer plan-a's channel
messages as the partner, wait for `RESOLVED`, run the orchestrator acceptance check on B2's
resolution phase file, then continue the loop with it.

## Current state

- B1r done at `f0f11e2`; B2 detailed and accepted (`phases/B2-planes-and-triangles.md`) but held.
- B2 detail replaces the ground sphere with the ground plane at `y = -0.5` and adds a triangle at
  `z = -2`; the user's merge-material decision already names "the ground plane" and the triangle.
- Overlap with plan-a A1: `scene()` in `src/main.rs` (A1 gives materials and adds a metal sphere, B2
  swaps in the plane and adds the triangle) and `src/hittable.rs` (A1 changes the record and
  `HittableList::hit`, which B3's `bounding_box` builds on). The merged scene must combine both and
  give the new shapes materials; the merge must also keep the normal-colouring until materials land.
- Shared design for the earlier pair: `PLANS/shared/plan-a-A1+plan-b-B1.md` (final).
- Autonomous mode: small, easily undoable, behaviour-neutral choices only; `autonomous-decisions.md`
  when the first is recorded. Product decisions still go to the user.

## Learned rules

- The leader of a pair writes both resolution phase files, not only its own; correct the partner's
  draft in place rather than creating a second file.
- A dropped phase keeps its id as a `not applicable` row; a resolution phase that absorbs a held
  phase's whole scope takes that scope and later phases depend on it.
- The plan-b branch must never add the material field or `Rc`; that is the release merge's work.
