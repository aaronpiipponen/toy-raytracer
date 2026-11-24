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
| A1 | Diffuse and metal materials | done | standard | - | done/A1-diffuse-and-metal-materials.md | every object has a material and rays bounce by it | gates green, rendered image shows matte and metal spheres |
| A2 | Glass | active | standard | A1 | phases/A2-glass.md | a glass sphere refracts | gates green, the rendered image shows a refracting glass sphere |
| A3 | Samples per pixel | active | standard | A1 | phases/A3-samples-per-pixel.md | each pixel averages jittered samples | gates green, edges are smooth in the rendered image |
| Z1 | Closing survey and user acceptance | planned | minimal | A1 + A2 + A3 | - | the plan's changes surveyed; the user accepts the image | the user accepts the survey findings and the rendered image |

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
- Glass tells entering from leaving from the hit record's `front_face` flag and the ray-facing
  `normal`; it adds no face flag of its own. The shared shape with the geometry plan is settled in
  `PLANS/shared/plan-a-A2+plan-b-B2.md`.

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
approximation. It tells entering from leaving from the hit record's `front_face` flag and the
ray-facing `normal`, and works with the normal in the glass's frame. The scene gains a glass sphere.
The face-side shape shared with the geometry plan is settled in `PLANS/shared/plan-a-A2+plan-b-B2.md`.

### A3 - Samples per pixel

`render` takes a sample count; each pixel averages that many rays jittered inside the pixel, sharing
one seeded generator. The default is 16. This is the seam the output plan's command line uses for
`--samples`.

### Z1 - Closing survey and user acceptance

**Status:** planned.
**Depends on:** A1, A2, A3.
**Scope:** a read-only survey of everything this plan added or changed, then the user's acceptance of
the rendered image.
**Expected gate level:** minimal.

**Starting state / reason:**
- The plan is finished when requirements 1 to 6 hold in the rendered image and the user accepts the
  result.
- This is the plan's only closing survey. Its findings go to the user; nothing is fixed here.

**Areas to survey (one read-only scout each, run in parallel):**
- `src/material.rs` and the material trait: diffuse, metal and glass behaviour against requirements
  1 to 4; unnecessary complexity, duplication and dead code.
- `src/hittable.rs` and `src/sphere.rs`: the material seam on the hit record and how objects get
  their material.
- `src/main.rs` and `src/rng.rs`: the bounce loop, gamma, sampling loop and the scene, and that every
  random value comes from the seeded generator.
- Tests and docs: `docs/requirements.md` and `README.md` still accurate; test bloat against
  REPO-RULES §3a; drift from the user's decisions.

**TODOs:**
1. Dispatch one independent read-only `ops-scout` per area above, in parallel; each reads whatever
   context outside its area it needs, and reports out-of-scope observations with the evidence it has.
2. Dispatch one independent read-only `ops-scout` to cross-check every finding claim by claim
   against evidence.
3. The orchestrator writes `benchmarks/closing-survey-<date>.md` from the saved returns: confirmed
   findings, out-of-scope observations, unnecessary complexity, duplication, dead code, redundant
   checks or records, test bloat, stale docs and drift from the user's decisions.
4. Report the note to the user. Fix nothing in this phase; clear defects and drift from the user's
   decisions go to remediation rounds, and a change needing a phase of its own or a user-owned
   decision goes to the note's user-review list.
5. Render the final scene and walk the user through it.

**Must not:**
- Change product code, tests or documentation in this phase.
- Narrow the survey to a diff-only review; each scout covers the whole area the plan changed.
- Close or archive the plan.

**Verification shape:**
- Documentation-only gate for the note: changed-file reread plus `git diff --check`.
- Rendered-image gate on the final revision, as survey evidence and for the user's acceptance.

**Exit:** the survey note exists and the user has accepted the rendered result.
**Move-on gate:** the user accepts the survey findings and the rendered image.

## Carry-overs

- A2 reads the `front_face` flag and the ray-facing `normal` the settled record carries; the shared
  shape is in `PLANS/shared/plan-a-A2+plan-b-B2.md`. The glass must keep working when the geometry
  plan's shapes join the same record.
