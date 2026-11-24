# Geometry Plan

> **Status:** scoped forward plan on branch `plan-b`.
> **Scope:** what a hit records and which shapes exist: the front-face flag, texture coordinates,
> planes, triangles and bounding boxes (`docs/requirements.md` items 7 to 10).
> **Out of scope:** materials and scattering (the materials plan); the PNG writer, the command line
> and progress output (the output plan).
> **Target:** a correct, small hobby raytracer.
>
> **How to use this plan:** expand exactly one phase into an implementation-grade phase from
> current code before coding.

## Phase status

| Phase | Title | Status | Gate level | Depends on | Phase file | Exit state | Move-on gate |
|---|---|---|---|---|---|---|---|
| B1r | Shared hit record shape | done | standard | - | `done/B1r-shared-hit-record-shape.md` | the branch matches the shared hit record design | gates green |
| B1 | Face side and texture coordinates | not applicable | standard | - | - | scope absorbed by B1r | - |
| B2r | Shared scene settlement | done | minimal | - | `done/B2r-shared-scene-settlement.md` | the shared scene design is recorded for this branch and the merge | plan documents re-read, `git diff --check` clean, no product code moved |
| B2 | Planes and triangles | done | standard | B1r + B2r | `done/B2-planes-and-triangles.md` | the scene can hold planes and triangles | gates green, rendered image shows a triangle |
| B3 | Bounding boxes | active | standard | B2 | `phases/B3-bounding-boxes.md` | the scene list skips objects whose box the ray misses | gates green, same image as before |
| Z1 | Closing survey and user acceptance | planned | minimal | B1r + B2 + B3 | - | the user accepts the result | the user accepts |

## How to implement this plan

Read `docs/requirements.md` and the code a phase touches before detailing it. Never invent a name.
Commit whenever useful. Run the fast gate in `AGENT-INSTRUCTIONS/LOCAL/gates.md`.

## Binding design rules

- `HitRecord` in `src/hittable.rs` stays the one record every shape returns. A helper on it sets
  the normal against the ray and records the face side, so every shape does it the same way.
- One module per shape in `src/`.

## Starting state

`HitRecord` in `src/hittable.rs` holds `t`, `point` and the outward `normal`; `Sphere` in
`src/sphere.rs` builds it. `HittableList` tests every object. `Vec3::cross` exists and is unused.

## Scoped phases

### B1 - Face side and texture coordinates

Its scope moved into B1r, the resolution phase of the shared hit record design with the materials
plan; B1 stays as the record of the original intent.

`HitRecord` gains `front_face` and `u`, `v`. A helper sets the normal against the ray from the
outward normal. `Sphere` fills (u, v) from latitude and longitude. The normal-colouring in
`src/main.rs` keeps working.

### B2r - Shared scene settlement

Records the scene composition settled with the materials plan in
`PLANS/shared/plan-a-A1+plan-b-B2.md`: B2's branch shape stays as detailed and stays material-free,
and the release merge gives `Plane` and `Triangle` a material argument, composes the four-material
scene, keeps the borrowed nearest record and adds B3's box test. No product code changes on this
branch. The frozen contract is `done/B2r-shared-scene-settlement.md`.

### B2 - Planes and triangles

`src/plane.rs` (a point and a normal; (u, v) from the hit position along two in-plane axes, modulo
1) and `src/triangle.rs` (three vertices, Möller-Trumbore; (u, v) barycentric). The large ground
sphere becomes an infinite ground plane at the same visible height and the scene gains a triangle.
Both new shapes are matte after the materials merge: the ground plane takes the mid-grey the current
ground sphere uses and the triangle a distinct dull green.

### B3 - Bounding boxes

An `Aabb` type, a `bounding_box` method on `Hittable` returning `Option<Aabb>` (none for a plane),
and `HittableList` testing a box before its object.

### Z1 - Closing survey and user acceptance

The binding closing survey of everything the plan added or changed: one read-only `ops-scout` per
area in parallel, then one independent `ops-scout` cross-checking every finding against the code, and
the orchestrator writes the dated note in `benchmarks/`. Findings go to the user; none are fixed in
this phase. Also render the scene, compare it with the requirements, and ask the user to accept.

## Carry-overs

- The release merge's obligations on this branch are in `done/B2r-shared-scene-settlement.md`:
  `Plane::new` and `Triangle::new` take the object's material as the last argument, the scene is the
  centre sphere, the ground plane, the triangle and the metal sphere with the four named materials,
  and `HittableList::hit` keeps the borrowed nearest record and adds B3's box test. B2, B3 and Z1
  change no material on this branch.
