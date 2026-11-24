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
| B2 | Planes and triangles | done | standard | B1 | `done/B2-planes-and-triangles.md` | the scene can hold planes and triangles | gates green, rendered image shows a triangle |
| B3 | Bounding boxes | done | standard | B2 | `done/B3-bounding-boxes.md` | the scene list skips objects whose box the ray misses | gates green, same image as before |
| Z1 | Closing survey and user acceptance | done | minimal | B1 + B2 + B3 | `done/Z1-closing-survey-and-user-acceptance.md` | the user accepts the result | the user accepts |

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

### B3 - Bounding boxes

**Status:** done. The frozen contract is `done/B3-bounding-boxes.md`.

An `Aabb` type, a `bounding_box` method on `Hittable` returning `Option<Aabb>` (none for a plane),
and `HittableList` testing a box before its object.

### Z1 - Closing survey and user acceptance

**Status:** done. The frozen contract and the carry-overs are in
`done/Z1-closing-survey-and-user-acceptance.md`.

The binding closing survey of everything the plan added or changed: one read-only `ops-scout` per
area in parallel, then one independent `ops-scout` cross-checking every finding against the code, and
the orchestrator writes the dated note in `benchmarks/`. Findings go to the user; none are fixed in
this phase. Also render the scene, compare it with the requirements, and ask the user to accept.

## Carry-overs

The release-merge carry-overs are listed in `done/Z1-closing-survey-and-user-acceptance.md`.
