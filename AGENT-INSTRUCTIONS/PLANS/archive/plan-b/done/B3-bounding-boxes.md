## B3 - Bounding boxes

**Status:** done.
**Depends on:** B2 (`done/B2-planes-and-triangles.md`): `Plane` and `Triangle` build their records
through the six-argument `HitRecord::new`, `HittableList::hit` keeps the nearest `HitRecord`, and the
scene holds one triangle.
**Gate level:** standard.

**Frozen contract:**
- `src/aabb.rs` holds `Aabb { pub min: Vec3, pub max: Vec3 }`, `Aabb::new(min, max)` for two opposite
  corners, and `Aabb::hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool`, the slab test that
  reports whether the ray crosses the box with some `t` in `t_min..t_max`. A ray component of exactly
  zero is parallel to its slab, and the ray then misses only when its origin lies outside that slab.
  The box is a value derived on each call; no shape stores, caches or refits one.
- `Hittable` in `src/hittable.rs` declares `fn bounding_box(&self) -> Option<Aabb>` with a default
  body returning `None`, the box of an object with no finite extent. `Plane` and `HittableList` take
  the default, so a plane is always tested. Only `Sphere` and `Triangle` override it.
- `Sphere::bounding_box` returns the box from center minus the radius to center plus the radius on
  every axis. `Triangle::bounding_box` returns the corner-wise minimum and maximum of its three
  vertices.
- `HittableList::hit` asks `object.bounding_box()` first and skips the object with `continue` when the
  box's `hit` reports false, using the same `limit` (the nearest hit's `t`, or `t_max`) that the
  object test uses. An object without a box is still tested.
- `src/main.rs` declares `mod aabb;` as the first module. `ray_colour`, `render`, the scene, the
  camera and the writers are unchanged.
- The rendered 200 by 100 image is byte-identical to the B2 image: the flat light-blue triangle on the
  right in the sky, the centred sphere with shaded normal colours, and the green ground.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` lists `aabb.rs` in the repository map. `docs/` and
  `README.md` are unchanged, and `docs/requirements.md` item 10 describes the box report, the list
  test and the boxless plane.

**Proof in the tree:** `src/aabb.rs` test
`a_ray_hits_the_box_it_crosses_and_misses_the_box_beside_it`, `src/hittable.rs` test
`the_list_tests_an_object_without_a_box_and_skips_one_whose_box_the_ray_misses`, `src/triangle.rs`
test `the_triangle_reports_a_box_around_its_vertices`, and 12 passing tests in `cargo test`.
