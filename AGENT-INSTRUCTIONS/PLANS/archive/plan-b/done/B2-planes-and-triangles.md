# B2 - Planes and triangles (frozen contract)

## Guarantees later phases build on

- `src/plane.rs` holds `Plane { pub point: Vec3, pub normal: Vec3 }` with
  `Plane::new(point: Vec3, normal: Vec3) -> Self`, which normalises the normal with `normal.unit()`.
  `Plane::hit` rejects a ray whose `abs` denominator against the normal is below `1e-9`, then
  returns a record for `t_min < t < t_max`. It maps the hit position to (u, v) on the two axes the
  dominant normal component does not use, each taken `rem_euclid(1.0)`.
- `src/triangle.rs` holds `Triangle { pub a: Vec3, pub b: Vec3, pub c: Vec3 }` with
  `Triangle::new(a: Vec3, b: Vec3, c: Vec3) -> Self` and a Möller-Trumbore `hit`. `u` weights the
  edge to `b` and `v` the edge to `c`, recorded unchanged as barycentric texture coordinates. A
  ray with an `abs` determinant below `1e-8` or a hit outside the barycentric bounds misses, so a
  degenerate triangle never hits.
- Both shapes are two-sided: every record is built by the shared
  `HitRecord::new(ray, t, point, outward_normal, u, v)`, which sets `front_face` and turns the
  normal against the ray. Neither shape carries a material, an `Rc` or a one-sided switch.
- `scene()` in `src/main.rs` holds, in order, the centre sphere
  `Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)`, the ground plane
  `Plane::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0))` and the triangle with vertices
  `(-2.0, -0.5, -2.0)`, `(2.0, -0.5, -2.0)` and `(0.0, 1.5, -2.0)`. The triangle's normal is
  `(0.0, 0.0, 1.0)`.
- `Vec3::cross` in `src/vec3.rs` is reachable by the triangle and therefore carries neither an
  `#[allow(dead_code)]` attribute nor its old purpose comment.
- `HitRecord` is unchanged: still `Clone, Copy, Debug`, six arguments, and the only construction
  path. `bounding_box` does not exist yet; B3 adds it, and every shape added here grows that method
  without a material on this branch.
- The release merge gives `Plane::new` and `Triangle::new` the object's material as the constructor's
  last argument and passes `Rc::clone(&self.material)` last to `HitRecord::new`
  (`PLANS/shared/plan-a-A1+plan-b-B2.md`, `done/B2r-shared-scene-settlement.md`). Nothing else in
  this contract changes at the merge.

## Documentation state

- No `docs/` page changed. `docs/requirements.md` item 9 ("the scene can hold infinite planes and
  triangles") is now true and accurate; item 10 (bounding boxes) is still B3's work. No page refers
  to a plan and no page lists the modules, so nothing in a page became stale.

## Test surface

- New, three functions: `src/plane.rs`
  `a_ray_hits_the_plane_and_its_texture_coordinates_wrap` (a front-face hit with `u = 0.25`,
  `v = 0.75` after `rem_euclid`), `src/triangle.rs`
  `a_ray_through_the_triangle_hits_it_with_barycentric_coordinates` (`t`, front face, normal
  `(0.0, 0.0, 1.0)`, `u = v = 0.25`) and `src/triangle.rs` `a_ray_beside_the_triangle_misses`.
- `src/main.rs` keeps `the_center_pixel_shows_the_sphere_facing_the_camera` and `the_top_row_is_sky`
  unchanged; both still pass because the centre ray hits the centre sphere first and the top row
  misses every object.
- The rendered image gate confirmed the sky gradient, the normal-coloured centre sphere, the ground
  plane as one flat green bottom half and the triangle showing beside and above the sphere.
