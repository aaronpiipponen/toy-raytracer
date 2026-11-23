# Shared design: pair `plan-a-A1+plan-b-B2`

Agreed between the plan-a and plan-b orchestrators. plan-b leads because phase B1 is already
committed on its branch. Both plans read this one file; it is the only copy of the agreement.

## Target shape of the shared code

This is the shape after both plans' releases merge into `main`; each branch lands only its own part
of it (below). One record, one constructor, for every shape:

```rust
use std::rc::Rc;
use crate::material::Material;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,      // always points against the ray
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        t: f64,
        point: Vec3,
        u: f64,
        v: f64,
        ray: &Ray,
        outward_normal: Vec3,
        material: Rc<dyn Material>,
    ) -> Self
}
```

- The material is the **last** parameter of the constructor and is supplied by the shape that was
  hit; each shape holds its own `Rc<dyn Material>` and passes a clone.
- `HitRecord::new` keeps sole ownership of `front_face` and of pointing `normal` against the ray. No
  shape re-implements that, and no code builds a record with a struct literal.
- Derive is `Clone` only. `Copy` is gone because a shared material handle is not copyable. `Debug` is
  gone because `dyn Material` has no `Debug`; nothing prints a hit record, and adding `Debug` as a
  `Material` supertrait would impose a bound on every material for no reader.
- `HittableList::hit` must not rely on `Copy`. It reads the nearest distance through a reference:

  ```rust
  let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);
  ```

  The value form would move the nearest record out of the loop once the record is not `Copy`.

## Which plan changes what

Each plan writes its own side of the target shape on its own branch. **Commits never move between
plan branches, and no plan branch is merged, rebased or cherry-picked onto another.** The releases
merge both branches into `main` later and resolve the textual conflicts by this design. Each branch
therefore passes its own gates alone, with its own code, and neither branch waits for the other's
code to exist.

**plan-b** (this plan, resolution phase `B2r`):

1. `src/hittable.rs`, `HittableList::hit`: the `as_ref()` reference form above. Behaviour-neutral and
   correct with or without `Copy`.
2. Nothing else. Its record and constructor already carry `t`, `point`, `normal`, `front_face`, `u`,
   `v` and no material; its plane and triangle shapes call the constructor as it exists on its
   branch, and it does not touch `src/main.rs`, `src/sphere.rs` or the sphere/main tests.

**plan-a** (its resolution phase for A1, on plan-a's branch):

1. Writes its **own** `HitRecord::new` constructor with the fields plan-a's branch has, plus
   `material: Rc<dyn Material>` as the **last** parameter, and derives `Clone` only. One constructor,
   no struct literal anywhere on its branch.
2. It does **not** add `front_face`, `u`, `v`, the ray argument or the outward-normal argument; those
   arrive with plan-b at the merge.
3. Adds the `as_ref()` form to `HittableList::hit` on its branch, for the same reason.
4. Owns `src/main.rs` `ray_colour` (scattering) and the material side of its sphere tests.
5. Keeps a `#[allow(dead_code)]` attribute only on fields no non-test build reads yet, and removes
   one as soon as a real build reads the field.

## What each side may rely on

- plan-b may rely on: `HitRecord` keeps `t`, `point`, `normal`, `front_face`, `u`, `v` with the same
  meanings; `HitRecord::new` exists and is the only construction; the material, when it arrives,
  comes last in the parameter list.
- plan-a may rely on: it needs no plan-b code. At the merge, `HitRecord` is the union of both sides'
  fields, the derive is `Clone` only, and the single constructor takes plan-b's face-side arguments
  followed by the material.

## Shared hunks the merge must reconcile

Both resolution phases are checked against this list:

1. **`HitRecord` shape.** Merged: `t`, `point`, `normal`, `front_face`, `u`, `v`, `material`;
   `#[derive(Clone)]` only. plan-b brings `front_face`/`u`/`v`, plan-a brings `material`.
2. **`HitRecord::new`.** Merged: `(t, point, u, v, ray, outward_normal, material)`, the only
   construction, deriving the ray-facing normal internally. plan-a's third argument (its `normal`)
   occupies the `outward_normal` slot, so its call sites gain the ray argument and drop their own
   normal argument; plan-b's call sites keep theirs and gain the material.
3. **`HittableList::hit`.** Both branches write the same `nearest.as_ref().map_or(t_max, |hit|
   hit.t)`, so the hunk reconciles to identical content.
4. **`#[allow(dead_code)]` markers on `HitRecord::point` and on `mod rng;`.** plan-a removes them
   because its code reads both; plan-b's B2 removes only their comments and keeps the attributes.
   Merged result: both attributes gone, both comments gone.
5. **`Sphere::hit`.** plan-a passes the material; plan-b computes `(u, v)` from the outward normal.
   Merged: one call passing both.
6. **`src/main.rs` tests.** plan-a adapts them for materials and gamma; plan-b does not touch them.
   Merged: plan-a's version, plus whatever plan-b's scene triangle needs.
7. **`Vec3::cross`.** plan-b removes its `#[allow(dead_code)]` and plan-naming comment because the
   triangle calls it; plan-a leaves the file alone. Merged: plan-b's version.

## Merge rule

Neither branch keeps a parallel construction path or a second record type. At the merge into `main`
the release resolves the conflicting hunks by this design: one `HitRecord` holding both sides'
fields, `Clone` only, and one `HitRecord::new` whose parameters are plan-b's face-side arguments with
the material last. The material stays the last parameter in the merged constructor. Both branches
carry the same `as_ref()` line in `HittableList::hit`, so that hunk reconciles to identical content.

