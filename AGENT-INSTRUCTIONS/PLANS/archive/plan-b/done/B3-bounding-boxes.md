# B3 - Bounding boxes (frozen contract)

## Guarantees later phases build on

- `src/aabb.rs` holds `Aabb { pub min: Vec3, pub max: Vec3 }` with
  `Aabb::new(a: Vec3, b: Vec3) -> Self`, which orders the two corners per axis, and
  `hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool`, a plain slab rejection over the three
  axes. A `false` means the ray meets no point of the box with `t` in `t_min..t_max`, so the caller
  may skip the object. A ray exactly parallel to a slab axis is rejected only when its origin lies
  outside that slab. `hit` returns `bool` because no caller needs the entry distance.
- `Hittable` in `src/hittable.rs` has a required second method,
  `fn bounding_box(&self) -> Option<Aabb>`. It is not a default method, so every shape states its
  box and a new shape cannot silently lose the test. Returns `Some` only for the box that holds
  every point of the object.
- `Sphere::bounding_box` returns the box between `center - (r, r, r)` and `center + (r, r, r)`.
  `Triangle::bounding_box` returns the per-axis minimum and maximum of the three vertices with no
  padding, so a degenerate triangle keeps a flat box. `Plane::bounding_box` returns `None` because
  an infinite plane has no box. `HittableList::bounding_box` returns `None` because the list is
  always tested as a whole and nothing nests a list as an object.
- `HittableList::hit` tests a present box once, over the full `t_min..t_max`, and skips the object
  when the box test fails. It then tests the object's `hit` with the running
  `let limit = nearest.map_or(t_max, |hit| hit.t);`. The box test can only skip an object that
  cannot be hit, so the chosen record is unchanged and the rendered image is byte-identical.
- No box is clamped, padded, shrunk, merged across a scene list, ordered, or cached. There is no
  hierarchy, no second hit path, and no `HitRecord` change. `Aabb` carries no material or `Rc`.
  `scene()`, `ray_colour`, `render`, the camera and the output are untouched.
- `docs/requirements.md` item 10 states the box test and the plane's missing box exactly as the
  code performs them. No `docs/` page changed and no page refers to a plan.
- The release merge keeps this box test and every `bounding_box` method as written and composes them
  with the material-carrying `Plane` and `Triangle`. It converts the nearest record to the borrowed
  form `nearest.as_ref().map_or(t_max, |hit| hit.t)` when the record loses `Copy`; the by-value form
  on this branch does not return. `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md` and
  `done/B2r-shared-scene-settlement.md` hold the rest of the merge's obligations.

## Test surface

- Three new functions: `src/aabb.rs` `a_ray_hits_the_box_it_crosses_and_misses_one_beside_it`,
  `src/triangle.rs` `the_triangle_box_holds_its_three_vertices`, and `src/hittable.rs`
  `the_scene_list_still_hits_a_plane_that_has_no_box` (the ray misses the sphere's box and the
  boxless plane is still tested and hit).
- The existing tests are unchanged, including `src/main.rs`'s
  `the_center_pixel_shows_the_sphere_facing_the_camera` and `the_top_row_is_sky`; the suite holds
  13 functions.
- The rendered-image gate produced a PPM byte-identical to the B2 render (`cmp` clean) and showed
  the sky gradient, the normal-shaded centre sphere, the flat green ground and the triangle beside
  and above the sphere.

## Documentation state

- No `docs/` page changed. `docs/requirements.md` items 7 to 10 are accurate for this branch. The
  repository map in `AGENT-INSTRUCTIONS/LOCAL/project.md` omits `aabb.rs`, `plane.rs` and
  `triangle.rs`; the release owns that update and `PLANS/plan-b/docs-pass.md` records it.
