# Shared design: pair `plan-a-A1+plan-b-B2`

Agreed between the plan-a and plan-b orchestrators. plan-b leads because phase B1 is already
committed on its branch. Both plans read this one file; it is the only copy of the agreement.

## Target shape of the shared code

One record, one constructor, for every shape:

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

**plan-b** (this plan's resolution phase, `B2r`):

1. `src/hittable.rs`, `HittableList::hit`: the `as_ref()` reference form above. Behaviour-neutral and
   correct with or without `Copy`.
2. Nothing else. Its plane and triangle shapes are written against the constructor as it exists on
   the branch (no material argument yet) and do not touch `src/main.rs`, `src/sphere.rs` or the
   sphere/main tests.

**plan-a** (its resolution phase for A1):

1. Owns the `Material` trait, `Rc<dyn Material>`, the `material` field, and the seventh constructor
   parameter.
2. Updates **every** `HitRecord::new` call site on its rebased tree in one change, including
   `src/plane.rs` and `src/triangle.rs` when the geometry work is on that tree. It keeps no second
   constructor and no parallel construction path.
3. Owns `src/main.rs` `ray_colour` (scattering) and the material side of the sphere tests. It keeps
   plan-b's existing sphere assertions passing (`front_face`, `u == 0.75`, `v == 0.5`, the
   near-surface normal), adapted only for how the sphere now receives its material.
4. Keeps a `#[allow(dead_code)]` attribute only on fields no non-test build reads yet, and removes
   one as soon as a real build reads the field.

## What each side may rely on

- plan-b may rely on: `HitRecord` keeps `t`, `point`, `normal`, `front_face`, `u`, `v` with the same
  meanings; `HitRecord::new` keeps its first six parameters in that order; the material arrives last.
- plan-a may rely on: `HitRecord::new` exists with the six-parameter signature and owns the
  face-side/normal logic; `Plane` and `Triangle` both construct records through it; no shape builds a
  record itself.

## Merge rule

Whichever branch lands second adapts the constructor call sites that exist on the merged tree. The
record shape, derive list and constructor signature above are the target on both sides. Neither
branch keeps a parallel construction path or a second record type.

## Precondition

plan-a's tree has no `HitRecord::new`: phase B1 is committed only on plan-b's branch. plan-a cannot
implement the target shape without duplicating that work. The coordinator must bring plan-b's B1 (or
its B1 + B2 tip) onto plan-a's branch, or authorize plan-a to merge or rebase onto it, before plan-a's
resolution phase can be implemented.
