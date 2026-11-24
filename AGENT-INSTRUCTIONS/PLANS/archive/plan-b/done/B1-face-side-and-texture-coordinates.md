## B1 - Face side and texture coordinates

**Status:** done.
**Depends on:** none; the starting tree at `e180476`.
**Gate level:** standard.

**Frozen contract:**
- `HitRecord` in `src/hittable.rs` carries `front_face`, `u` and `v` beside `t`, `point` and
  `normal`, and `normal` always points against the ray.
- `HitRecord::new` is the only constructor of a hit record. It takes the ray, `t`, the point, the
  outward normal and `(u, v)`, sets `front_face` from the sign of `ray.direction.dot(outward_normal)`,
  and stores the outward normal or its negation so `normal` faces the ray.
- `Sphere::hit` derives `(u, v)` from the outward normal's latitude and longitude and builds the
  record through `HitRecord::new`. Texture coordinates are not clamped or wrapped in the record; a
  shape supplies them in `0..=1`.
- `HittableList`, `ray_colour`, `render` and the scene are unchanged, and every hit in the current
  scene is a front-face hit.
- `docs/` is unchanged: `docs/requirements.md` items 7 and 8 already state this behaviour.

**Proof in the tree:** `src/sphere.rs` tests `a_ray_through_the_center_hits_the_near_surface`,
`a_ray_from_inside_the_sphere_hits_the_back_face` and
`the_near_surface_has_texture_coordinates_from_latitude_and_longitude`.
