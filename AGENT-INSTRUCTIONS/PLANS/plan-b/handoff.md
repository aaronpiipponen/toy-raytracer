# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in REVIEW AND ADVANCE mode for phase B1, with the receipt below. On
its handoff, run the acceptance check on B2's detail before any implementer.

## Current state

B1 is implemented and pending review/advance; tested head `6eb43e09`. Receipt: base `e180476`,
head `6eb43e09`; `cargo build`, `cargo test` (6 passed), `cargo clippy -- -D warnings`,
`cargo fmt --check`, `git diff --check` all pass; rendered-image gate passed as a byte-identical
before/after `cmp`; test budget 1 added / 0 deleted. Implementer review loop: 1 round, no real
finding. User test: not required. No `USER DECISION REQUIRED`, no blocker. Next phases: B2 (planes
and triangles), B3 (bounding boxes), Z1 (closing survey and user acceptance). `scope.md` is final.
Autonomous mode is on; standing authorization for phase commits exists. The plan is linked to the
coordinator.

**Coordinator hold:** `HOLD ARMED (pair plan-a-A1+plan-b-B2)` - armed reason corrected by the user:
both plans rewrite `HitRecord` in `src/hittable.rs` and `Sphere::hit` in `src/sphere.rs`. plan-b B1
(committed at `6eb43e09`) adds `front_face`/`u`/`v`, keeps `#[derive(Clone, Copy, Debug)]`, and makes
`HitRecord::new` the only construction. plan-a A1 adds `material: Rc<dyn Material>`, drops `Copy`, and
builds the record with a struct literal in `Sphere::hit`; `src/main.rs:ray_colour` and the sphere
tests change under both. plan-b leads because B1 is already committed. B2 detailing may proceed; the
B2 implementer start must not be retried or routed around. Wait for `CHANNEL OPEN`, then settle
through `channel_send` and write `PLANS/shared/plan-a-A1+plan-b-B2.md`.

## Learned rules

- Detail the rendered-image gate as a before/after render comparison (`cmp` of two PPM files), never
  as a hardcoded expected pixel value or line number.
- A coordinator hold stops only the held phase's implementer start; a running advancer may still
  detail it, and unrelated phases stay blocked too once the hold is armed.
