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
`front_face = outward_normal.dot(&ray.direction) < 0.0` and
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

## Merge checklist

1. `HitRecord` ends as `Clone` only, with all seven fields and one construction helper carrying
   `material` as its last parameter.
2. `Sphere` ends holding centre, radius and material, and builds its record through the helper.
3. `src/main.rs` ends with the scatter loop, not the normal-colouring loop.
4. B1's `Sphere::new(center, radius)` call in the sphere test becomes
   `Sphere::new(center, radius, material)`; the centre-pixel normal test stays valid because the
   fixed scene hits front faces only.
5. Nobody reruns the other plan's move-on gate; each verifies its own on its own branch, and the
   merge follows this checklist.

## What each plan may rely on

A1 may rely on `normal` always pointing against the ray: Lambertian scatter around the normal then
stays in the ray's hemisphere, and reflection is orientation-independent. A1 may also rely on the
record being `Clone`, not `Copy`.

B1 may rely on every object carrying its material on the record and on the scatter loop replacing
normal colouring after A1.
