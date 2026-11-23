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
| B1r | Face side and texture coordinates | done | standard | - | `done/B1r-face-side-and-texture-coordinates.md` | plan-b's half of the shared design is live | gates green |
| B1 | Face side and texture coordinates | done | standard | B1r | `done/B1-face-side-and-texture-coordinates.md` | every hit records its face side and (u, v), plan-free comments | gates green |
| B2 | Planes and triangles | active | standard | B1 | `phases/B2-planes-and-triangles.md` | the scene can hold planes and triangles | gates green, rendered image shows a triangle |
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

`HitRecord` in `src/hittable.rs` holds `t`, `point`, `normal`, `front_face`, `u` and `v`, and is built
only through `HitRecord::new(t, point, outward_normal, ray, u, v)`, which sets the normal against the
ray and records the face side. `point`, `front_face`, `u` and `v` carry `#[allow(dead_code)]` because
nothing reads them on this branch. `Sphere` in `src/sphere.rs` fills `(u, v)` from latitude and
longitude. `HittableList` tests every object in order. `Vec3::cross` exists and has no caller.

## Scoped phases

### B1 - Face side and texture coordinates (done)

`HitRecord` records `front_face` and `(u, v)`, and `HitRecord::new` sets the normal against the ray
from the outward normal. `Sphere` fills `(u, v)` from latitude and longitude. The normal colouring in
`src/main.rs` is unchanged, and no source comment names a plan.

### B2 - Planes and triangles

`src/plane.rs` (a point and a normal; `(u, v)` from the hit position along two in-plane axes, modulo
1) and `src/triangle.rs` (three vertices, Möller–Trumbore, `(u, v)` barycentric). Both build their
records through `HitRecord::new`. The scene's ground becomes a plane and the scene gains a triangle.

### B3 - Bounding boxes

An `Aabb` type, a `bounding_box` method on `Hittable` returning `Option<Aabb>` (none for a plane),
and `HittableList` testing a box before its object.

### Z1 - Closing survey and user acceptance

Render the scene, compare it with the requirements, and ask the user to accept.

## Carry-overs

- `README.md:3-5` describes the checkout as the test project for three parallel plans. The text is
  product history, it becomes wrong once the plans merge, and all three plans' worktrees share the
  file, so it is one post-merge edit. The orchestrator decides its owner (Z1's closing survey or the
  release docs pass); no plan-b phase edits it.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` repository map: B2 adds `plane.rs` and `triangle.rs`, and B3
  updates the line for whatever modules it adds. Each phase re-reads the shared line before editing
  it.
