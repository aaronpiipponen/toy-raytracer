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
| B1 | Face side and texture coordinates | done | standard | - | `done/B1-face-side-and-texture-coordinates.md` | every hit records its face side and (u, v) | gates green |
| B2 | Planes and triangles | planned | standard | B1 | - (re-detail after B2r) | the scene can hold planes and triangles | gates green, rendered image shows a triangle |
| B2r | Shared hit-record resolution | active | minimal | B1 | - | the nearest-hit loop no longer relies on a copyable record | gates green |
| B3 | Bounding boxes | planned | standard | B2 | - | the scene list skips objects whose box the ray misses | gates green, same image as before |
| Z1 | Closing survey and user acceptance | planned | minimal | B1 + B2 + B3 | - | the user accepts the result | the user accepts |

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

`HitRecord` gains `front_face` and `u`, `v`. A helper sets the normal against the ray from the
outward normal. `Sphere` fills (u, v) from latitude and longitude. The normal-colouring in
`src/main.rs` keeps working.

### B2 - Planes and triangles

`src/plane.rs` (a point and a normal; (u, v) from the hit position along two in-plane axes, modulo
1) and `src/triangle.rs` (three vertices, Möller-Trumbore; (u, v) barycentric). The scene adds a
triangle.

### B2r - Shared hit-record resolution

Settle the overlap with the materials plan: `PLANS/shared/plan-a-A1+plan-b-B2.md` is the agreed
design. This plan changes `HittableList::hit` in `src/hittable.rs` to read the nearest distance
through `nearest.as_ref()`, so the scene list works once the hit record stops being `Copy` and
carries its material. Nothing else. B2 is re-detailed from that state before its implementer.

### B3 - Bounding boxes

An `Aabb` type, a `bounding_box` method on `Hittable` returning `Option<Aabb>` (none for a plane),
and `HittableList` testing a box before its object.

### Z1 - Closing survey and user acceptance

Render the scene, compare it with the requirements, and ask the user to accept.

## Carry-overs

- B2: delete the plan-naming comments at `src/hittable.rs:11` and `src/main.rs:6` (frozen in
  `done/B1-face-side-and-texture-coordinates.md`).
- B2: its detailed phase file predates B2r and must be re-detailed from the post-B2r code before any
  implementer starts.
