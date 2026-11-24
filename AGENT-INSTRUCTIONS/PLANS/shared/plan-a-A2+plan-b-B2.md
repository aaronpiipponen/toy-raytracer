# Shared design: plan-a-A2 + plan-b-B2 (materials × geometry)

Agreed by the plan-a and plan-b orchestrators. Both plans read this as the one copy. It settles
implementation detail only; the one product choice it cannot settle is listed as an open
user-owned decision.

## Target shape of the shared code

`src/hittable.rs` holds one `HitRecord`, not `Copy`:

```rust
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    /// Always faces against the incoming ray (requirement 7).
    pub normal: Vec3,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
    pub material: Rc<dyn Material>,
}
```

One constructor:

```rust
pub fn new(ray: &Ray, t: f64, point: Vec3, outward_normal: Vec3, u: f64, v: f64, material: Rc<dyn Material>) -> Self
```

It sets `front_face = ray.direction.dot(outward_normal) < 0.0`, then `normal = outward_normal` when
`front_face`, otherwise `-outward_normal`.

`HittableList::hit` reads the nearest distance through a borrow:
`let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);`.

Every shape holds an `Rc<dyn Material>` and passes it to `HitRecord::new`.

Spheres derive `(u, v)` identically on both branches, from the outward normal, so the sphere merge is
textual:

```
let outward_normal = (point - self.center) / self.radius;
let theta = (-outward_normal.y).acos();
let phi = (-outward_normal.z).atan2(outward_normal.x) + PI;
let u = phi / (2.0 * PI);
let v = theta / PI;
```

## Which plan changes what

**plan-a, A2 resolution, branch `plan-a`:**

- `src/hittable.rs` - add `front_face`, `u`, `v` and `HitRecord::new` as above; `normal` becomes
  ray-facing; the material stays.
- `src/sphere.rs` - compute `(u, v)` with the formula above and build the record through
  `HitRecord::new(ray, t, point, outward_normal, u, v, Rc::clone(&self.material))`;
  `Sphere::new` keeps `(center, radius, material)`.
- `src/material.rs` - the existing tests build records through `HitRecord::new`.
- `src/main.rs` - glass uses `hit.front_face` to tell entering from leaving and the ray-facing
  `hit.normal` for refraction; glass returns unit attenuation.
- `HittableList::hit` borrowing is already landed in A1.

After this, plan-a's branch holds the full combined record for spheres and materials.

**plan-b, B2, branch `plan-b`:**

- `Plane { point, normal }` and `Triangle { a, b, c }` keep their shape: no material placeholder, no
  type parameter, no comment about a future material.
- Both build records through the current six-argument `HitRecord::new`.
- B2 adds two test functions, the plane test and the triangle test.
- `HittableList::hit` stays Copy-based on this branch.
- B2 stays held until `RESOLVED`.

## Release merge

When both plans merge into the integration branch:

- Take the union `HitRecord` and the seven-argument `HitRecord::new` above.
- Keep plan-a's borrowing `HittableList::hit`.
- Add `material: Rc<dyn Material>` to `Plane` and `Triangle` (field plus `new` argument), pass it to
  `HitRecord::new`, and pass a material in B2's two tests.
- Give the scene triangle a material - see the open decision below.

## What each may rely on

- plan-b may rely on the six-argument `HitRecord::new` on its branch and on the `(u, v)` formula
  above.
- plan-a may rely on the combined record after A2.
- Neither plan adds a second `Material` trait, a placeholder material field, or a material-less
  construction path in the final product.

## Open user-owned decision (release)

The scene triangle's material changes the finished image, so the choice is the user's. Recommendation:
use the same diffuse material as plan-a's scene spheres, so the assembled scene reads consistently.
The release must ask the user, not pick silently.
