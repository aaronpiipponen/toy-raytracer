# Shared design: plan-a-A3 + plan-b-Z1 (materials × geometry union)

Agreed by the plan-a and plan-b orchestrators. Both plans read this as the one copy. The
render/progress seam is settled separately in `PLANS/shared/plan-a-A3+plan-c-Z1.md`; this file adds the
hit record, the shapes, the bounding-box skip and the scene. Implementation detail only; the one
product choice it leaves open is the scene triangle's material.

## Target shape of the shared code

`src/hittable.rs` holds one `HitRecord`, not `Copy`:

```rust
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    /// Faces against the incoming ray.
    pub normal: Vec3,
    pub front_face: bool,
    #[allow(dead_code)]
    pub u: f64,
    #[allow(dead_code)]
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

`point` carries **no** `#[allow(dead_code)]` and no bounce-origin comment: the scatter path reads it.
`u` and `v` keep the allow because no render path reads them. `front_face` is read by glass and
`material` by the render loop, so neither carries an allow.

`Hittable` declares `hit(...)` plus the default `fn bounding_box(&self) -> Option<Aabb> { None }`.
`Sphere` and `Triangle` override it; `Plane` and `HittableList` take the default.

`HittableList::hit` - the recorded design choice, plan-a's borrow with plan-b's box skip:

```rust
let mut nearest: Option<HitRecord> = None;
for object in &self.objects {
    let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);
    if let Some(bounds) = object.bounding_box() {
        if !bounds.hit(ray, t_min, limit) {
            continue;
        }
    }
    if let Some(hit) = object.hit(ray, t_min, limit) {
        nearest = Some(hit);
    }
}
nearest
```

## Shapes

- `Sphere { center, radius, material }`, `Sphere::new(center, radius, material)`. Its `hit` keeps
  plan-b's exact `(u, v)` formula and builds through the seven-argument `HitRecord::new` with
  `Rc::clone(&self.material)`. `bounding_box` returns `center - radius` to `center + radius`.
- `Plane { point, normal, material }`, `Plane::new(point, normal, material)`. Its `hit` builds through
  the seven-argument `HitRecord::new`; it takes the default `bounding_box` (`None`).
- `Triangle { a, b, c, material }`, `Triangle::new(a, b, c, material)`. Its `hit` builds through the
  seven-argument `HitRecord::new`; `bounding_box` is the corner-wise minimum and maximum.
- `src/aabb.rs` stays exactly as plan-b landed it.

## Scene

`src/main.rs` takes plan-a's material render and scene and adds plan-b's one triangle at the exact
landed vertices `(0.6, 0.15, -1.0)`, `(1.95, 0.15, -1.0)`, `(1.275, 0.92, -1.0)`. Do not move or
"improve" the placement during the merge; its measured margins are valid. The triangle's material is
the open user-owned decision below.

## Which plan changes what

**plan-a, A3, branch `plan-a`:** changes only `render`, its tests and `main` (the jittered sampling);
it does not touch `hittable.rs`, `sphere.rs`, the shapes or `main`'s scene beyond the sampling. It
records this design and needs no product change for the union, which is a release-merge concern.

**plan-b, Z1, branch `plan-b`:** changes no product code. It records this design, confirms its landed
branch, and runs its read-only closing survey and acceptance. Its branch image is **not** evidence for
the merged scene's appearance, because the triangle's material changes its colour.

## Release merge

- Take the union `HitRecord` and the seven-argument `HitRecord::new`, with the `point` allow removed.
- Take the `Hittable::bounding_box` default and the combined `HittableList::hit` above.
- `Sphere` takes material plus `bounding_box`; `Plane` and `Triangle` take material; `Aabb` is plan-b's.
- `main` takes plan-a's material render and scene plus the triangle at its exact landed vertices.
- Tests: keep one test per behavior. Drop plan-b's
  `the_center_pixel_shows_the_sphere_facing_the_camera` - its premise disappears when material
  scattering replaces normal colouring, and rewriting it as a colour assertion would be tape; the
  release's own rendered-image gate is the proof that the fixed scene still shows the sphere, and no
  replacement test is added (the test budget governs). `the_top_row_is_sky` stays. Drop plan-b's
  `the_near_surface_has_texture_coordinates_from_latitude_and_longitude`, which duplicates the `(u, v)`
  assertions plan-a's `a_ray_through_the_center_hits_the_near_surface` already makes. Plan-b's shape and
  box tests pass a material after the merge.
- Delete plan-a's `SAMPLES`/`WIDTH`/`HEIGHT` per the render seam design; `DEFAULT_SAMPLES` is the one
  default.

## What each may rely on

- plan-a may rely on the record and shape shapes above after the merge.
- plan-b may rely on its branch staying read-only until the release.
- Neither plan adds a second record path, a second list loop, a second box test, a second material-less
  construction path, or a second generator.

## Open user-owned decision (release)

The scene triangle's material changes the finished image, so the choice is the user's. Recommendation:
a Lambertian in the scene spheres' family, so the triangle reads as part of the scene. The release asks
the user, not pick silently.
