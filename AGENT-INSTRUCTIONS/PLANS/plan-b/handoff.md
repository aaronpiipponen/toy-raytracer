# Orchestrator handoff

## Next action

Spawn `phase-implementer` with `phase_subagent` for the active phase B1. Its phase document is
`AGENT-INSTRUCTIONS/PLANS/plan-b/phases/B1-face-side-and-texture-coordinates.md`, accepted by the
orchestrator with one correction already applied (the rendered-image gate compares a before/after
render with `cmp` instead of a remembered pixel value).

## Current state

B1 is detailed and accepted; no implementation exists. The phase adds `front_face`, `u`, `v` and
one `HitRecord::new` constructor in `src/hittable.rs`, and computes latitude/longitude `(u, v)` in
`src/sphere.rs`. Product work, gate level standard: fast gate plus the rendered-image conditional
gate as a before/after byte comparison. Expected receipt: one adapted test, one new test
(`a_ray_from_inside_the_sphere_flips_the_normal`). Next after B1: B2 (planes and triangles), then
B3 (bounding boxes), then Z1 (closing survey and user acceptance). `scope.md` is final. Autonomous
mode is on; standing authorization for phase commits exists.

## Learned rules

- Detail the rendered-image gate as a before/after render comparison (`cmp` of two PPM files), never
  as a hardcoded expected pixel value or line number.
