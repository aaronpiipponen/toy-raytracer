## Phase B3 - Bounding boxes (done)

**Status:** done.
**Gate level:** standard.

**Frozen contract:**
- `src/aabb.rs` holds `pub struct Aabb { pub min: Vec3, pub max: Vec3 }` with the derive list
  `Clone, Copy, Debug`, and `Aabb::new(min: Vec3, max: Vec3) -> Aabb`. `min` is the lower corner on
  every axis. It has no other field, method or constructor.
- `Aabb::hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool` narrows the interval `t_near..t_far`
  on the three axes and returns `false` as soon as the interval is empty. A zero direction component
  gives infinite bounds, and a NaN bound produced by an origin exactly on that face is ignored by
  `f64::max` and `f64::min`, so that slab is skipped. The test has no epsilon, no parallel-ray
  branch, no NaN check and no infinity check.
- `Hittable` in `src/hittable.rs` declares exactly two methods: `hit` and
  `fn bounding_box(&self) -> Option<Aabb>` after it.
- `Sphere::bounding_box` returns the box from `center - radius` to `center + radius` on each axis.
  `Triangle::bounding_box` returns the componentwise minimum and maximum of `a`, `b` and `c`.
  `Plane::bounding_box` and `HittableList::bounding_box` return `None`, so a plane and the scene list
  are always tested.
- `HittableList::hit` keeps `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);` and, before
  `object.hit(ray, t_min, limit)`, skips the object when its box does not meet the ray across
  `t_min..limit`. The nearest-record expression and the returned `Option<HitRecord>` are unchanged.
- `src/main.rs` declares `mod aabb;` as the first module line. Every other line of the file and every
  shape's `hit` method are unchanged.
- `AGENT-INSTRUCTIONS/LOCAL/project.md:5` names `aabb.rs` in the repository map.
- Two test functions added, none changed or deleted:
  `a_ray_through_the_box_hits_it_and_a_ray_beside_it_misses` in `src/aabb.rs` and
  `the_list_skips_an_object_when_the_ray_misses_its_box` in `src/hittable.rs`. The repository's total
  is 11 test functions.

**Evidence:** `cargo test aabb` 1 passed, `cargo test hittable` 1 passed, `cargo test` 11 passed,
`cargo build`, `cargo clippy -- -D warnings`, `cargo fmt --check` and `git diff --check` passed at
`2b4e2b7`. The 200 by 100 PPM render is byte-identical before and after the box test, with the
header `P3`, `200 100`, `255` and line 7441 `127 127 255`. The agent-generation gate does not apply.
