# Shared design: plan-a phase A1 and plan-b phase B1

Settled between the plan-a and plan-b orchestrators for the pair `plan-a+plan-b`. Both copies of
this file, in `PLANS/plan-a/` and `PLANS/plan-b/`, are identical.

Already fixed elsewhere, so not ours to choose: requirement 7 gives `normal` its meaning (always
against the ray) and adds the front-face flag; requirement 8 adds `(u, v)` in 0..=1; the materials
plan binds the hit record to carry the hit object's material as `Rc<dyn Material>` and binds the
render loop to bounce along scattered rays. What follows settles the implementation detail and the
merge shape only.

## Final shape of the shared code

`HitRecord` in `src/hittable.rs`:

- fields `t`, `point`, `normal`, `front_face`, `u`, `v`, `material: Rc<dyn Material>`;
- derives `Clone` only. No `Copy` (an `Rc` can never be `Copy`) and no `Debug` (nothing formats a
  record, and a `Debug` supertrait on `Material` would exist only for the derive);
- `normal` always points against the ray, `front_face` records which side was hit, and the shape
  passes the outward normal in.

`Sphere` in `src/sphere.rs` holds its centre, its radius and its `Rc<dyn Material>`, fills `(u, v)`
from latitude and longitude, and puts its material on the record.

`src/main.rs` has one render behaviour after A1: `ray_colour` follows the material's scattered ray up
to 50 bounces and returns the material's attenuation times the colour the scattered ray sees, black
at the limit; output applies gamma 2. The normal-colouring loop is gone.

## Which plan owns what

B1 owns the face side and the texture coordinates: the `front_face`, `u` and `v` fields, the one
face-normal helper every shape uses, `Sphere`'s latitude/longitude, and the sphere test's adaptation
to them. B1 changes nothing in `src/main.rs` and adds no material.

A1 owns materials: the `material` field and everything the loss of `Copy` causes (`HittableList`'s
`nearest.map_or(...)` becomes `nearest.as_ref().map_or(...)`), `Sphere`'s material field and its
placement on the record, the scatter loop and gamma 2 in `src/main.rs`, and the scene's materials.
A1 chooses the `HitRecord` derive list as `Clone` alone.

## Face-normal helper

B1 adds one helper, `HitRecord::new(t, point, outward_normal, ray, u, v)`, which sets
`front_face = outward_normal.dot(ray.direction) < 0.0` and
`normal = if front_face { outward_normal } else { -outward_normal }`. In the merged tree the same
function gains `material: Rc<dyn Material>` as its last parameter and assigns it. The signature
changes exactly once, at merge; no branch ever names the other branch's new symbols.

## Keeping each branch green on its own

- A1's record change is purely additive: it adds the `material` field, drops `Copy`, and keeps
  today's literal construction with `material: Rc::clone(&self.material)` added. It does not add a
  constructor helper and does not touch `normal`, `front_face`, `u` or `v`.
- B1's record change is also additive: it adds its three fields and the helper at the same
  construction site and keeps `#[derive(Clone, Copy, Debug)]`, because its record is still all-`Copy`
  fields. B1 adds no material and no `Debug` impl to satisfy the merge.
- After B1 nothing reads `front_face`, `u` or `v` (`src/main.rs` colours by `normal`), so B1 marks
  each new field `#[allow(dead_code)]` with a one-line comment in the style `point` already uses.
  The merged tree keeps those allowances until a real reader appears.

## Shapes and materials (B2 and A1r)

Every shape owns its material and no shape owns a renderer decision. `Sphere`, `Plane` and
`Triangle` each hold `pub material: Rc<dyn Material>` as their last field and build their record
through the merged `HitRecord::new(t, point, outward_normal, ray, u, v, material)`, passing
`Rc::clone(&self.material)` as the last argument. `main`'s scene shares one material across the
shapes that need it.

Branch forms:

- B2 creates `Plane::new(point, normal)` and `Triangle::new(vertex0, vertex1, vertex2)` with
  geometry only and no `Rc`; both call the six-argument `HitRecord::new` that B1r landed.
- A1r leaves `Plane` and `Triangle` alone and gives only `Sphere` its material.
- Merge adds the material field and the last constructor parameter to `Plane` and `Triangle`,
  stores it, and adds the seventh helper argument at the three shape call sites.

Merged `scene()` in `src/main.rs`, the one scene both plans' exit states require:

- `let ground: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));` and
  `Plane::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), Rc::clone(&ground))`; this
  replaces the ground sphere.
- `Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::clone(&red))` and
  `Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.3, Rc::clone(&metal))`, exactly as A1r details them.
- `let triangle_material: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.3, 0.5, 0.8)));`
  and `Triangle::new(Vec3::new(1.6, -0.5, -1.2), Vec3::new(2.3, -0.5, -1.2), Vec3::new(1.95, 0.15, -1.2), Rc::clone(&triangle_material))`.

B2's own branch keeps its triangle at `x` 0.5..1.2; the merge shifts it right so the metal sphere,
which projects to `x` 0.7..1.3 at `z = -1`, does not occlude it. The shifted triangle projects to
1.333..1.917, inside the frame, and keeps the metal, centre and ground acceptance pixels unchanged.

The merged module list is alphabetical: `camera`, `hittable`, `material`, `plane`, `ray`, `rng`,
`sphere`, `triangle`, `vec3`, with the union of both branches' imports.

## Merge checklist

1. `HitRecord` ends as `Clone` only, with all seven fields and one construction helper carrying
   `material` as its last parameter.
2. `Sphere` ends holding centre, radius and material, and builds its record through the helper.
3. `src/main.rs` ends with the scatter loop, not the normal-colouring loop.
4. Every shape test builds through the material-carrying constructors: the four `Sphere::new` calls
   in `src/sphere.rs` (two from B1's original tests, two from B1r's), the two `Plane::new` calls in
   `src/plane.rs` and the one `Triangle::new` call in `src/triangle.rs`, each with a test material
   (`Rc::new(Lambertian::new(colour))`). The plane and triangle test modules gain
   `use std::rc::Rc;` and `use crate::material::Lambertian;`. Every sphere in the merged scene is
   hit on its front face, so the sphere and centre-pixel assertions stay valid.
5. Nobody reruns the other plan's move-on gate; each verifies its own on its own branch, and the
   merge follows this checklist.
6. Test helpers that build a record follow the merged shape: the materials test helper in
   `src/material.rs` (`hit_with_normal`) either builds its record through `HitRecord::new` or keeps a
   literal carrying `front_face`, `u` and `v`. Through the helper the ray must point into the
   surface, so `front_face` and the against-the-ray normal stay consistent with the normal the test
   asserts; as a literal the fields must be that same self-consistent triple (`front_face: true`,
   the test's normal, and any `(u, v)` in 0..=1). No test-only construction path keeps the
   four-field shape.
7. `Plane` and `Triangle` hold `pub material: Rc<dyn Material>` as their last field, their
   constructors take it as the last parameter, and every shape builds its record through the
   seven-argument `HitRecord::new`.
8. Merged `scene()` is exactly the one in `## Shapes and materials (B2 and A1r)`.
9. The merged module list is alphabetical (`camera`, `hittable`, `material`, `plane`, `ray`, `rng`,
   `sphere`, `triangle`, `vec3`) with the union of both branches' imports.
10. The repository-map line in `AGENT-INSTRUCTIONS/LOCAL/project.md` names every module: `main.rs`
    (scene, render loop, output, command line), `vec3.rs`, `ray.rs`, `camera.rs`, `hittable.rs` (hit
    record, the `Hittable` trait, the scene list), `material.rs` (the `Material` trait and its
    materials), `plane.rs` (infinite planes), `sphere.rs`, `triangle.rs` (triangles), `rng.rs`
    (seeded randomness). Each plan adds its own names when it lands; the merged line carries all
    three additions and nothing else changes.

## What each plan may rely on

A1 may rely on `normal` always pointing against the ray: Lambertian scatter around the normal then
stays in the ray's hemisphere, and reflection is orientation-independent. A1 may also rely on the
record being `Clone`, not `Copy`.

B1 may rely on every object carrying its material on the record and on the scatter loop replacing
normal colouring after A1.
