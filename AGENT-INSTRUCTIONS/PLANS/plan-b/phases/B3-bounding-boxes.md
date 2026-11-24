## Phase B3 - Bounding boxes

**Status:** active.
**Depends on:** B2 (`done/B2-planes-and-triangles.md`): `Plane` and `Triangle` build their records
through the six-argument `HitRecord::new`, `HittableList::hit` copies the nearest `HitRecord`, which
is `Copy`, and the scene holds one material-less triangle. `HitRecord` and `Plane` stay unchanged in
this phase, the `hit` bodies of `Sphere`, `Triangle` and `HittableList` keep their behaviour and
their hit results, and `Sphere` and `Triangle` gain a `bounding_box` override.
**Scope:** add `src/aabb.rs` with `Aabb { min, max }`, its `new` and a slab `hit` test; give
`Hittable` a `bounding_box(&self) -> Option<Aabb>` method with a default body returning `None`;
override it in `Sphere` (center plus and minus the radius) and `Triangle` (the corner-wise min and
max of the three vertices); make `HittableList::hit` test each object's box before calling the
object; declare `mod aabb;` in `src/main.rs`; and add `aabb.rs` to the repository map in
`AGENT-INSTRUCTIONS/LOCAL/project.md`. `Plane` and `HittableList` take the default `None`, so a
plane is always tested. No plane in the scene, no material, no output or command-line change.
**Gate level:** standard - one small geometry type, a default trait method and one dispatch change
inside the frozen `Hittable` seam, bounded runtime behaviour, no durable state, config, security or
data-access surface. Targeted tests, the fast gate, the rendered-image conditional gate and the
repository-map update are enough.
**Simplicity statement:** the phase does not store or cache a box on any object, does not compute or
test a box for `HittableList` or the whole scene in `ray_colour`, does not refit a box when geometry
changes, does not repair swapped corners, and does not handle NaN coordinates or a negative radius.
A box is derived from the object's fields each time it is asked for. No legacy or defensive code is
added or deleted.

**Current state (verified):**
- `src/hittable.rs:1-4` - `//! Anything a ray can hit, and the record of where it hit.` then
  `use crate::ray::Ray;` and `use crate::vec3::Vec3;`.
- `src/hittable.rs:9-21` - `#[derive(Clone, Copy, Debug)] pub struct HitRecord { pub t, pub point, pub normal, pub front_face, pub u, pub v }`; unchanged this phase.
- `src/hittable.rs:23-41` - `impl HitRecord { pub fn new(ray: &Ray, t: f64, point: Vec3, outward_normal: Vec3, u: f64, v: f64) -> Self }`; unchanged.
- `src/hittable.rs:43-46` - `pub trait Hittable { fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>; }`, with the doc line `/// The nearest hit with \`t\` in \`t_min..t_max\`, if any.` This phase adds `bounding_box` here.
- `src/hittable.rs:48-71` - `#[derive(Default)] pub struct HittableList { pub objects: Vec<Box<dyn Hittable>> }`, `impl HittableList { pub fn add(&mut self, object: Box<dyn Hittable>) }`, and `impl Hittable for HittableList` whose `hit` walks `for object in &self.objects`, computes `let limit = nearest.map_or(t_max, |hit| hit.t);`, and keeps the nearest record. This phase adds the box test inside that loop.
- `src/sphere.rs:1-18` - `use std::f64::consts::PI;`, the `crate::hittable::{HitRecord, Hittable}`, `crate::ray::Ray` and `crate::vec3::Vec3` imports, and `pub struct Sphere { pub center: Vec3, pub radius: f64 }` with `pub fn new(center: Vec3, radius: f64) -> Self`. This phase adds a `bounding_box` override and the `Aabb` import.
- `src/sphere.rs:20-46` - `impl Hittable for Sphere` with `fn hit`; unchanged.
- `src/triangle.rs:1-19` - the same import shape and `pub struct Triangle { pub a: Vec3, pub b: Vec3, pub c: Vec3 }` with `pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self`; the struct doc comment reads `/// A triangle: its three vertices, counter-clockwise seen from its front face.` This phase adds a `bounding_box` override and the `Aabb` import.
- `src/triangle.rs:21-44` - `impl Hittable for Triangle` with `fn hit`; unchanged.
- `src/plane.rs:20-42` - `impl Hittable for Plane` with `fn hit`; this phase does not edit the file, because the plane takes the trait's default `None` box.
- `src/ray.rs:6-18` - `pub struct Ray { pub origin: Vec3, pub direction: Vec3 }`, `Ray::new`, `pub fn at(&self, t: f64) -> Vec3`; unchanged.
- `src/vec3.rs:5-36` - `#[derive(Clone, Copy, Debug, PartialEq)] pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }`, `Vec3::new`, `dot`, `cross`, `length`, `unit`; `src/vec3.rs:38-78` - `Add`, `Sub`, `Neg`, `Mul<f64>`, `Mul<Vec3>`, `Div<f64>`. There is no indexing or component-wise `min`/`max` helper, so the box code reads `.x`, `.y` and `.z` directly.
- `src/main.rs:3-12` - the module list `mod camera; mod hittable; #[allow(dead_code)] mod plane; mod ray; #[allow(dead_code)] mod rng; mod sphere; mod triangle; mod vec3;`. This phase adds `mod aabb;` as the first entry.
- `src/main.rs:23-24` - `const WIDTH: usize = 200;` and `const HEIGHT: usize = 100;`.
- `src/main.rs:27-33` - `fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Colour` calls `world.hit(ray, 0.001, f64::INFINITY)`; unchanged, and it tests no box.
- `src/main.rs:35-45` - `fn scene() -> HittableList` holds two spheres and one triangle; unchanged.
- `src/main.rs:47-60` - `fn render(width: usize, height: usize) -> Vec<Colour>`; `src/main.rs:90-106` - the `#[cfg(test)] mod tests` with `the_center_pixel_shows_the_sphere_facing_the_camera` and `the_top_row_is_sky`; unchanged.
- `src/hittable.rs` has no test module, so this phase creates one at the end of the file; `src/aabb.rs` and `src/triangle.rs` gain their test the same way `src/sphere.rs:48-93` does.
- `docs/requirements.md:29-30` - item 10: `Every object reports an axis-aligned bounding box, and the scene list tests a ray against an object's box before the object. An infinite plane has no box and is always tested.` It already states this phase's behaviour.
- Only four types implement `Hittable`: `HittableList`, `Sphere`, `Plane`, `Triangle` (grep `impl Hittable for` over `src/`). No other file reads or builds a box.

**Read first (do not invent):**
- `src/hittable.rs` - whole file; the trait, the list loop and the import block this phase edits.
- `src/sphere.rs` and `src/triangle.rs` - whole files; the impl blocks that gain the override and the test style the new tests copy.
- `src/plane.rs` - the impl that stays untouched and relies on the default.
- `src/vec3.rs` - the fields and operators the box code uses; there is no indexing API.
- `src/ray.rs` - `origin`, `direction` and `at`.
- `src/main.rs` - the module list, `ray_colour` and `render`; neither changes beyond the module line.
- `docs/requirements.md` item 10 - the behaviour this phase makes real.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image conditional gate.
- `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md` section "Code documentation" - the header, doc-comment and comment rules the new prose follows.

**Build plan:**
1. `src/aabb.rs` (create) - header `//! Axis-aligned bounding boxes.` first, then exactly:

   ```rust
   use crate::ray::Ray;
   use crate::vec3::Vec3;

   /// The box between two opposite corners.
   pub struct Aabb {
       pub min: Vec3,
       pub max: Vec3,
   }

   impl Aabb {
       /// Build a box from opposite corners, `min` low on every axis.
       pub fn new(min: Vec3, max: Vec3) -> Self {
           Aabb { min, max }
       }

       /// Report whether the ray crosses the box with some `t` in `t_min..t_max`.
       pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
           let origin = [ray.origin.x, ray.origin.y, ray.origin.z];
           let direction = [ray.direction.x, ray.direction.y, ray.direction.z];
           let min = [self.min.x, self.min.y, self.min.z];
           let max = [self.max.x, self.max.y, self.max.z];
           let mut t_enter = t_min;
           let mut t_exit = t_max;
           for axis in 0..3 {
               if direction[axis] == 0.0 {
                   if origin[axis] < min[axis] || origin[axis] > max[axis] {
                       return false;
                   }
                   continue;
               }
               let inverse = 1.0 / direction[axis];
               let mut near = (min[axis] - origin[axis]) * inverse;
               let mut far = (max[axis] - origin[axis]) * inverse;
               if near > far {
                   (near, far) = (far, near);
               }
               t_enter = t_enter.max(near);
               t_exit = t_exit.min(far);
               if t_enter > t_exit {
                   return false;
               }
           }
           true
       }
   }
   ```

   Each axis is a slab between `min[axis]` and `max[axis]`. `near` and `far` are the distances where
   the ray enters and leaves that slab, sorted because a negative direction swaps them. The ray
   crosses the box when every slab overlaps inside `t_min..t_max`. A ray component of exactly zero
   is parallel to its slab, so the ray misses only when its origin lies outside that slab. This
   structure is clippy-clean under `cargo clippy -- -D warnings`. Do not add a cache, an
   `Aabb::from_points` constructor, a union or an `Index` impl for `Vec3`.

2. `src/main.rs`, module list (`src/main.rs:3-12`) - add `mod aabb;` before `mod camera;`. The module
   is used by `hittable`, `sphere` and `triangle`, so it needs no `#[allow(dead_code)]`. Change
   nothing else in the file.

3. `src/hittable.rs`, imports (`src/hittable.rs:3-4`) - add `use crate::aabb::Aabb;` above
   `use crate::ray::Ray;`, keeping the block alphabetical.

4. `src/hittable.rs`, `Hittable` trait (`src/hittable.rs:43-46`) - after `fn hit(...)`, add exactly:

   ```rust
       /// The axis-aligned box around the whole object, or `None` when the object has no finite
       /// extent.
       fn bounding_box(&self) -> Option<Aabb> {
           None
       }
   ```

   The default states that an object of infinite extent has no box. `Plane` and `HittableList` take
   it, so `Plane` is always tested by the list. Do not add an override that returns `None`, and do
   not give the method a different name.

5. `src/hittable.rs`, `HittableList::hit` (`src/hittable.rs:60-71`) - change the loop body from:

   ```rust
           for object in &self.objects {
               let limit = nearest.map_or(t_max, |hit| hit.t);
               if let Some(hit) = object.hit(ray, t_min, limit) {
                   nearest = Some(hit);
               }
           }
   ```

   to:

   ```rust
           for object in &self.objects {
               let limit = nearest.map_or(t_max, |hit| hit.t);
               if let Some(bounds) = object.bounding_box() {
                   if !bounds.hit(ray, t_min, limit) {
                       continue;
                   }
               }
               if let Some(hit) = object.hit(ray, t_min, limit) {
                   nearest = Some(hit);
               }
           }
   ```

   The box test uses the same `limit` as the object test, so a box beyond the nearest hit is
   rejected as well.

6. `src/sphere.rs`, imports and impl - add `use crate::aabb::Aabb;` above `use crate::hittable::{HitRecord, Hittable};`. Inside `impl Hittable for Sphere`, after `hit`, add exactly:

   ```rust
       fn bounding_box(&self) -> Option<Aabb> {
           let extent = Vec3::new(self.radius, self.radius, self.radius);
           Some(Aabb::new(self.center - extent, self.center + extent))
       }
   ```

7. `src/triangle.rs`, imports and impl - add `use crate::aabb::Aabb;` above `use crate::hittable::{HitRecord, Hittable};`. Inside `impl Hittable for Triangle`, after `hit`, add exactly:

   ```rust
       fn bounding_box(&self) -> Option<Aabb> {
           let min = Vec3::new(
               self.a.x.min(self.b.x).min(self.c.x),
               self.a.y.min(self.b.y).min(self.c.y),
               self.a.z.min(self.b.z).min(self.c.z),
           );
           let max = Vec3::new(
               self.a.x.max(self.b.x).max(self.c.x),
               self.a.y.max(self.b.y).max(self.c.y),
               self.a.z.max(self.b.z).max(self.c.z),
           );
           Some(Aabb::new(min, max))
       }
   ```

8. `src/plane.rs` - no edit. It takes the trait's default `None` box, which keeps an infinite plane
    always tested.

9. Tests - add exactly the three test functions named under `Tests`, one in `src/aabb.rs`, one in a
    new `#[cfg(test)] mod tests` at the end of `src/hittable.rs`, and one in the existing
    `src/triangle.rs` test module, in the style of `src/sphere.rs`.

10. `docs/` - no page changes. `docs/requirements.md` item 10 already states the box report and the
    list test, and no page states that boxes are absent.

11. `AGENT-INSTRUCTIONS/LOCAL/project.md` (repository map, line 5) - insert `` `aabb.rs` (an
    axis-aligned bounding box and its ray test), `` directly before `` `sphere.rs` `` in the `src/`
    list, keeping every existing entry and its wording. This edit sits under `AGENT-INSTRUCTIONS/`
    outside `PLANS/`, so load the `agent-instructions` skill before making it.

**Callers / wiring to update:**
- `src/main.rs:3-12` - declares `mod aabb;`.
- `src/hittable.rs:3-4` - imports `Aabb` for the trait signature.
- `src/hittable.rs:43-46` - the trait gains `bounding_box` with its default `None`.
- `src/hittable.rs:60-71` - the list consults `object.bounding_box()` before `object.hit`.
- `src/sphere.rs:1-6` and `:20-46` - imports `Aabb` and overrides `bounding_box`.
- `src/triangle.rs:1-6` and `:21-44` - imports `Aabb` and overrides `bounding_box`.
- `src/plane.rs` - no change, takes the default.
- `src/main.rs:27-33` and `:47-60` - `ray_colour` and `render` need no edit; the scene is unchanged.
- No other type implements `Hittable`, so no other override is needed (verified by grep over `src/`).

**Config / schema / migrations:**
- None: no configuration key, migration, generated file or persisted record.
- Retained records and their named consumer/owner: none; a box is an in-memory value derived from
  the object's fields on each call.

**Touched files:**
<!-- touched-files:start -->
- `src/aabb.rs` - created: `Aabb` with two corners, `new`, the slab `hit` test, and one test
- `src/main.rs` - changed: declares `mod aabb;`
- `src/hittable.rs` - changed: imports `Aabb`, adds the default `bounding_box` to `Hittable`, tests each object's box in `HittableList::hit`, and gains one test
- `src/sphere.rs` - changed: imports `Aabb` and overrides `bounding_box` with the sphere's extent
- `src/triangle.rs` - changed: imports `Aabb`, overrides `bounding_box` with the vertex extents, and gains one test
- `AGENT-INSTRUCTIONS/LOCAL/project.md` - changed: the repository map lists `aabb.rs`
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add a dependency to `Cargo.toml` and do not use `unsafe` (domain invariants 1 and 3).
- Do not change `HitRecord`, `HitRecord::new`, `Plane`, the scene, the camera, `ray_colour`,
  `render`, `write_ppm` or the existing tests; the sphere and triangle `hit` bodies stay as they are.
- Do not give `HittableList` or `Plane` a `bounding_box` override, do not compute a list box, and do
  not test the world's box in `ray_colour`.
- Do not store a box in a shape, cache one, or add a union, `from_points`, `Default`, `Copy`, or
  component-index helper.
- Do not repair swapped corners, handle a NaN coordinate, a zero-length normal or a negative radius.
- Do not add a test function beyond the three named here (REPO-RULES §3a).
- Do not touch `docs/`, `README.md`, `Cargo.toml`, another plan's files, or any other
  `AGENT-INSTRUCTIONS/` file; the repository map line is the only instruction-tree edit.
- Do not write plan or phase references in code comments or doc comments.

**Tests:**
- New (3 functions):
  - `src/aabb.rs` - `a_ray_hits_the_box_it_crosses_and_misses_the_box_beside_it`, in a new
    `#[cfg(test)] mod tests { use super::*; }`: box
    `Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0))`; the ray
    `Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0))` gives
    `bounds.hit(&ray, 0.001, f64::INFINITY)` true, and the ray
    `Ray::new(Vec3::new(2.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0))` gives false.
  - `src/hittable.rs` - `the_list_tests_an_object_without_a_box_and_skips_one_whose_box_the_ray_misses`,
    in a new `#[cfg(test)] mod tests { use super::*; use crate::aabb::Aabb; use crate::plane::Plane; }`:
    a `struct Unreachable` implements `Hittable` with `fn hit` that panics with
    `"the list tests an object whose box the ray misses"` and `fn bounding_box` returning
    `Some(Aabb::new(Vec3::new(10.0, 10.0, -6.0), Vec3::new(11.0, 11.0, -5.0)))`. A `HittableList`
    holds `Unreachable` then `Plane::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0))`. With
    the ray `Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0))`,
    `world.hit(&ray, 0.001, f64::INFINITY)` is `Some`, and its `t` is about `1.0` within `1e-9`. The
    stub observes that `hit` is not called and the plane shows that an object with no box is still
    tested.
  - `src/triangle.rs` - `the_triangle_reports_a_box_around_its_vertices`, added to the existing
    `#[cfg(test)] mod tests`: triangle
    `Triangle::new(Vec3::new(2.0, -1.0, -3.0), Vec3::new(-1.0, 4.0, -2.0), Vec3::new(0.5, 0.0, -6.0))`;
    `triangle.bounding_box().expect("a triangle has a box")` has
    `min == Vec3::new(-1.0, -1.0, -6.0)` and `max == Vec3::new(2.0, 4.0, -2.0)`.
- Changed: none. Deleted: none. The four `src/sphere.rs` tests, the `src/plane.rs` and
  `src/triangle.rs` hit tests, the `src/rng.rs` test and the two `src/main.rs` tests stay as they are.
- Fast gate: `cargo test` (12 tests: the existing 9 plus the 3 above), `cargo build`,
  `cargo clippy -- -D warnings`, `cargo fmt --check`, `git diff --check`.
- Conditional gates: the rendered-image gate is required because `HittableList::hit` changes. Before
  the first edit, render at the base revision with
  `cargo run --release > /tmp/opencode/plan-b-b3-before.ppm`. After the change, render with
  `cargo run --release > /tmp/opencode/plan-b-b3-after.ppm` and require `cmp` to report no
  difference, because every object's box contains it and culling must not change a pixel. Then open
  the converted after image with the image reader and confirm the B2 scene: a flat light-blue
  triangle with straight edges and a horizontal base on the right, wholly in the sky above the
  ground, and the centred sphere with its shaded normal colours. Convert with the snippet below.
  Agent generation gate: none, `AGENT-INSTRUCTIONS/SUBAGENTS/**` and model assignment are untouched.
- Slow gate (>2 minutes): none.
- Conversion snippet for the rendered PPM (a scratch file under `/tmp/opencode`, never committed):

  ```python
  import struct, sys, zlib
  src, dst = sys.argv[1], sys.argv[2]
  tokens = open(src).read().split()
  assert tokens[0] == "P3", tokens[0]
  w, h, top = int(tokens[1]), int(tokens[2]), int(tokens[3])
  vals = [round(int(v) * 255 / top) for v in tokens[4:]]
  assert len(vals) == w * h * 3, (len(vals), w, h)
  raw = b"".join(b"\x00" + bytes(vals[y * w * 3:(y + 1) * w * 3]) for y in range(h))
  def chunk(k, d):
      return struct.pack(">I", len(d)) + k + d + struct.pack(">I", zlib.crc32(k + d) & 0xFFFFFFFF)
  png = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b"")
  open(dst, "wb").write(png)
  ```

**User test (acceptance):**
- **Executor:** agent.
- Render at the default 200 by 100 size and compare it with the base-revision render as described in
  the rendered-image gate. The two PPM files are byte-identical, and the image shows the same scene
  as B2: the flat light-blue triangle on the right, wholly in the sky above the green ground, and
  the centred sphere with its shaded normal colours.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `src/aabb.rs` holds `Aabb { min, max }`, `new`, and `hit` that reports whether the ray crosses
      the box inside `t_min..t_max`.
- [ ] `Hittable::bounding_box` exists with a default body returning `None`; only `Sphere` and
      `Triangle` override it, and `Plane` and `HittableList` take the default.
- [ ] `HittableList::hit` tests `object.bounding_box()` with the same limit before `object.hit`.
- [ ] `src/main.rs` declares `mod aabb;` and no shape stores or caches a box.
- [ ] The base-revision render and the current render are byte-identical and the image shows the B2
      scene.
- [ ] Named tests pass, and the test receipt reports 3 functions added, 0 deleted.
- [ ] Test budget respected (REPO-RULES §3a); any line counts are observations, not gates.
- [ ] Fast phase gate and the rendered-image conditional gate pass; no slow gate applies.
- [ ] `docs/` needed no change and `docs/requirements.md` item 10 is accurate; `README.md` is still
      accurate; the repository map lists `aabb.rs` and no page or comment refers to a plan.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the user test ran as the agent acceptance step
      above.
- [ ] On passing evidence: compress this file into `done/B3-bounding-boxes.md` (frozen contract
      only) and flip the `full-plan.md` status row to `done`.

**Exit:** every object reports a box, the scene list tests an object's box before the object, a
plane has none and is always tested, and the rendered image is byte-identical to the B2 image.
