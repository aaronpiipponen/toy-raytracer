# Materials Plan

> **Status:** scoped forward plan on branch `plan-a`.
> **Scope:** materials and how rays scatter: diffuse, metal and glass, the bounce limit, samples per
> pixel and gamma correction (`docs/requirements.md` items 1 to 6).
> **Out of scope:** new shapes, face sides and texture coordinates, bounding boxes (the geometry
> plan); the PNG writer, the command line and progress output (the output plan).
> **Target:** a correct, small hobby raytracer.
>
> **How to use this plan:** expand exactly one phase into an implementation-grade phase from
> current code before coding.

## Phase status

| Phase | Title | Status | Gate level | Depends on | Phase file | Exit state | Move-on gate |
|---|---|---|---|---|---|---|---|
| A1r | Materials on the shared hit record | active | standard | - | `phases/A1r-materials-on-the-shared-hit-record.md` | plan-a's half of the shared design is live | gates green, rendered image shows matte and metal spheres |
| A1 | Diffuse and metal materials | planned | standard | - | - | every object has a material and rays bounce by it | gates green, rendered image shows matte and metal spheres |
| A2 | Glass | planned | standard | A1 | - | a glass sphere refracts | gates green, rendered image shows a glass sphere |
| A3 | Samples per pixel | planned | standard | A1 | - | each pixel averages jittered samples | gates green, edges are smooth in the rendered image |
| Z1 | Closing survey and user acceptance | planned | minimal | A1 + A2 + A3 | - | the user accepts the result | the user accepts |

## How to implement this plan

Read `docs/requirements.md` and the code a phase touches before detailing it. Never invent a name.
Commit whenever useful. Run the fast gate in `AGENT-INSTRUCTIONS/LOCAL/gates.md`.

## Binding design rules

- A material is a trait, `Material`, in a new `src/material.rs`: given the incoming ray and the
  hit, it returns the attenuation and the scattered ray, or nothing when the ray is absorbed.
  Objects share materials as `Rc<dyn Material>`.
- The hit record carries the material of the object that was hit, so the render loop can scatter
  without knowing the object.
- All randomness uses `src/rng.rs` with a fixed seed.

## Starting state

`src/main.rs` colours a hit by its normal; there is no material and no bounce. `HitRecord` in
`src/hittable.rs` holds `t`, `point` and `normal`; `Sphere` in `src/sphere.rs` holds a centre and a
radius. `src/rng.rs` exists and is unused.

## Scoped phases

### A1 - Diffuse and metal materials

`src/material.rs` with the `Material` trait and two materials, diffuse (`Lambertian`, an albedo)
and metal (`Metal`, an albedo and a fuzz factor). `HitRecord` gains the hit object's material;
`Sphere` gains a material and puts it on the record. The render loop in `src/main.rs` follows
scattered rays up to 50 bounces and applies gamma 2 on output. The scene gives the centre sphere a
diffuse material, the ground a diffuse material, and adds a metal sphere.

### A2 - Glass

A `Dielectric` material with a refractive index: Snell's law, total internal reflection, Schlick's
approximation. Refraction needs to know whether the ray enters or leaves the surface; the scene
gains a glass sphere.

### A3 - Samples per pixel

`render` takes a sample count; each pixel averages that many rays jittered inside the pixel. The
default is 16.

### Z1 - Closing survey and user acceptance

Render the scene, compare it with the requirements, and ask the user to accept.

## Carry-overs

None.
