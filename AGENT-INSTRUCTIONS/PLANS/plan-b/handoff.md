# Orchestrator handoff

## Next action

Plan-b is HELD: the B2 implementer start was refused with `COORDINATOR HOLD` (overlap with plan-a
A1 on `HitRecord`/`Sphere::hit`). Start nothing, retry nothing. Wait for the coordinator's
`CHANNEL OPEN`; then settle through `channel_send` and write
`AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md`,
spawn `DETAIL RESOLUTION: Pair plan-a-A1+plan-b-B2. Plans plan-b (held phase B2) and plan-a (held
phase A1). Shared design: PLANS/shared/plan-a-A1+plan-b-B2.md.`, run the orchestrator acceptance
check on both resolution phase files, `channel_close`, then continue the loop with plan-b's
resolution phase. After the resolution, detail B2 fresh before its implementer.

## Current state

- B1: `done` at `6eb43e09`; frozen contract `done/B1-face-side-and-texture-coordinates.md`.
- B2: `active`, detailed and accepted by the orchestrator at
  `phases/B2-planes-and-triangles.md`; no implementation; blocked by the coordinator hold.
- B3 (bounding boxes) and Z1 (closing survey and user acceptance): `planned`.
- `scope.md` is final. Autonomous mode is on; standing authorization for phase commits exists.
- No open user decision. Nothing deferred to the release, so no `docs-pass.md` yet.

**Coordinator hold reason (corrected by the user):** both plans rewrite `HitRecord` in
`src/hittable.rs` and `Sphere::hit` in `src/sphere.rs`. plan-b B1 (committed) adds
`front_face`/`u`/`v`, keeps `#[derive(Clone, Copy, Debug)]`, and makes `HitRecord::new` the only
construction; plan-a A1 adds `material: Rc<dyn Material>`, drops `Copy`, and builds the record with a
struct literal in `Sphere::hit`. `src/main.rs:ray_colour` and the sphere tests change under both.
plan-b leads because B1 is committed.

**Orchestrator's settlement intent (implementation detail, to agree in the channel):** keep exactly
one constructor. The material rides through it as one more argument supplied by the shape, which
holds its own material: `HitRecord::new(t, point, u, v, ray, outward_normal, material)`. The derive
becomes `Clone, Debug` (a shared material handle is not copyable). `HittableList::hit` must stop
relying on `Copy`: with a non-copy record, `nearest.map_or(t_max, |hit| hit.t)` moves the nearest
record out of the loop, so it must read the distance through a reference (`nearest.as_ref()`).
plan-a owns the material type, the field and the scattering; plan-b owns the record shape, the
constructor and the plane/triangle shapes that call it; the sphere is the shared file and ends up
holding its material and passing it through.

## Learned rules

- Detail the rendered-image gate as a before/after render comparison (`cmp`), never a remembered
  pixel value, when the claim is that the image did not change. A specific pixel probe is acceptable
  only when a diff cannot prove the new behaviour (for example, that a newly added object renders).
- A coordinator hold stops only the held phase's implementer start; a running advancer may still
  detail it, and unrelated phases stay blocked too once the hold is armed.
- Deleting comments that name a plan or a later stage (REPO-RULES §4.13) is legitimate work inside a
  phase that already touches the file; keep it to the comment lines only.
