## Phase B3 - Bounding boxes

**Status:** active.
**Depends on:** `done/B2-planes-and-triangles.md` (the three shape types and the scene's triangle),
`done/B1-face-side-and-texture-coordinates.md` (`HitRecord`) and
`done/B2r-shared-hit-record-resolution.md` (`HittableList::hit` reads the nearest distance through
`nearest.as_ref()`).
**Scope:** add `src/aabb.rs` holding an axis-aligned box with a slab test, add
`Hittable::bounding_box() -> Option<Aabb>` with one implementation for each of the three shapes and
for `HittableList`, and make `HittableList::hit` test an object's box before the object. The rendered
image stays byte-identical. Nothing else changes: no materials, no output, no command line, no
scene or camera edit.
**Gate level:** standard - one new type, one required trait method across four existing
implementations, one loop guard, no dependency, no configuration and no durable state. The rendered
image is the phase's own move-on gate, so the rendered-image conditional gate is required.
**Simplicity statement:** the box test can only lose a hit by rejecting a ray that would have met
the object, so it is allowed to be conservative. A ray whose direction component is zero while its
origin lies exactly on that slab face produces a NaN bound, which `f64::max` and `f64::min` ignore,
so the slab is skipped and the ray is reported as meeting the box. The ray lies on the box boundary
there, so reporting a hit is correct, and the only cost is one object test. No epsilon, no
special-cased parallel ray and no clamping is added. A scene list reports no box, so a list is
always tested: nothing queries a list's extent and the scene is not nested. The phase adds no
`Aabb::union`, no box growing by a point, no empty-box sentinel and no box for the plane. It deletes
nothing.

**Current state (verified):**
- `src/hittable.rs:44-47` - `pub trait Hittable { fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>; }`,
  the only method the trait declares.
- `src/hittable.rs:61-72` - `impl Hittable for HittableList`, whose loop reads
  `nearest.as_ref().map_or(t_max, |hit| hit.t)` into `limit` at line 65 and calls
  `object.hit(ray, t_min, limit)`. This phase keeps that limit expression and adds the box guard
  before the call.
- `src/hittable.rs:3-4` - `use crate::ray::Ray;` and `use crate::vec3::Vec3;`, the imports a
  `crate::aabb::Aabb` import joins.
- `src/hittable.rs` has no test module.
- `src/sphere.rs:8-17` - `pub struct Sphere { pub center: Vec3, pub radius: f64 }` and
  `Sphere::new(center, radius) -> Sphere`.
- `src/sphere.rs:19-43` - the `impl Hittable for Sphere` that gains only `bounding_box`.
- `src/plane.rs:7-19` - `pub struct Plane { pub point: Vec3, pub normal: Vec3 }` and
  `Plane::new(point, normal) -> Plane`.
- `src/plane.rs:21-37` - the `impl Hittable for Plane` that gains only `bounding_box`.
- `src/triangle.rs:7-20` - `pub struct Triangle { pub a: Vec3, pub b: Vec3, pub c: Vec3 }` and
  `Triangle::new(a, b, c) -> Triangle`.
- `src/triangle.rs:22-51` - the `impl Hittable for Triangle` that gains only `bounding_box`.
- `src/ray.rs:5-9` - `pub struct Ray { pub origin: Vec3, pub direction: Vec3 }`, read by the slab
  test.
- `src/vec3.rs:5-10` - `pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }`, with no
  component accessor and no componentwise min/max, so the slab test and the triangle's box work on
  the fields directly.
- `src/main.rs:3-12` - the module list, alphabetical from `mod camera;` to `mod vec3;`.
- `docs/requirements.md:29-30` - item 10: every object reports an axis-aligned bounding box, the
  scene list tests a box before the object, and an infinite plane has no box and is always tested.
- A repository-wide search finds exactly four `impl Hittable` blocks: `Sphere`, `Plane`,
  `Triangle` and `HittableList`. No other implementor exists.

**Read first (do not invent):**
- `src/hittable.rs`, whole file - the trait, `HittableList` and its hit loop.
- `src/sphere.rs:1-43`, `src/plane.rs:1-37`, `src/triangle.rs:1-51` - the three existing
  implementations whose method list this phase extends.
- `src/ray.rs`, `src/vec3.rs` - the fields the slab test reads.
- `src/main.rs:1-60` - the module list, the scene and the render loop that must not change.
- `docs/requirements.md:29-30` - item 10.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image conditional gate.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/B2r-shared-hit-record-resolution.md` - the frozen
  nearest-distance loop.

**Build plan:**

1. `src/aabb.rs`, new file - an axis-aligned box and its slab test, with this content shape:

   ```rust
   //! Axis-aligned bounding boxes.

   use crate::ray::Ray;
   use crate::vec3::Vec3;

   /// An axis-aligned box whose sides lie along the coordinate axes.
   #[derive(Clone, Copy, Debug)]
   pub struct Aabb {
       pub min: Vec3,
       pub max: Vec3,
   }

   impl Aabb {
       /// Create the box whose corners are `min` and `max`.
       /// `min` is the lower corner in every axis.
       pub fn new(min: Vec3, max: Vec3) -> Self {
           Aabb { min, max }
       }

       /// Return whether the ray meets the box with `t` in `t_min..t_max`.
       pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
           let mut t_near = t_min;
           let mut t_far = t_max;
           let slabs = [
               (self.min.x, self.max.x, ray.origin.x, ray.direction.x),
               (self.min.y, self.max.y, ray.origin.y, ray.direction.y),
               (self.min.z, self.max.z, ray.origin.z, ray.direction.z),
           ];
           for (min, max, origin, direction) in slabs {
               let inverse = 1.0 / direction;
               // A zero `direction` gives infinite bounds, and a bound that is NaN because the
               // origin lies exactly on that face is ignored by `max` and `min`.
               let mut t0 = (min - origin) * inverse;
               let mut t1 = (max - origin) * inverse;
               if t0 > t1 {
                   std::mem::swap(&mut t0, &mut t1);
               }
               t_near = t_near.max(t0);
               t_far = t_far.min(t1);
               if t_near > t_far {
                   return false;
               }
           }
           true
       }
   }
   ```

   The slab test keeps the interval in `t_near..t_far`, narrows it on each axis and rejects when
   the interval is empty. Do not add epsilon, do not special-case a zero direction component, and
   do not add any other method, field or constructor.

2. `src/hittable.rs` - add the import, the trait method and the list behaviour:

   - Add `use crate::aabb::Aabb;` above `use crate::ray::Ray;`.
   - `pub trait Hittable` gains one method after `hit`, before the closing brace:
     ```rust
     /// The box used to skip objects the ray misses, or `None` when every ray must test the object.
     fn bounding_box(&self) -> Option<Aabb>;
     ```
   - `impl Hittable for HittableList::hit` gains the guard inside its loop, after `limit` is read
     and before `object.hit`:
     ```rust
     if let Some(bounding_box) = object.bounding_box() {
         if !bounding_box.hit(ray, t_min, limit) {
             continue;
         }
     }
     ```
     The guard passes `limit`, not `t_max`, so a farther object whose box begins past the nearest
     hit is skipped.
   - The same `impl` gains:
     ```rust
     fn bounding_box(&self) -> Option<Aabb> {
         None
     }
     ```
     Place it after `hit`, inside the existing `impl Hittable for HittableList` block.

3. `src/sphere.rs` - `impl Hittable for Sphere` gains after `hit`:
   ```rust
   fn bounding_box(&self) -> Option<Aabb> {
       let radius = Vec3::new(self.radius, self.radius, self.radius);
       Some(Aabb::new(self.center - radius, self.center + radius))
   }
   ```
   Add `use crate::aabb::Aabb;` above the existing `use crate::hittable::{HitRecord, Hittable};`.
   Touch nothing in `hit`.

4. `src/plane.rs` - `impl Hittable for Plane` gains after `hit`:
   ```rust
   fn bounding_box(&self) -> Option<Aabb> {
       None
   }
   ```
   Add `use crate::aabb::Aabb;` above the existing `use crate::hittable::{HitRecord, Hittable};`.
   Do not add a box that depends on `point` or `normal`, and do not touch `plane_axes` or `hit`.

5. `src/triangle.rs` - `impl Hittable for Triangle` gains after `hit`:
   ```rust
   fn bounding_box(&self) -> Option<Aabb> {
       Some(Aabb::new(
           Vec3::new(
               self.a.x.min(self.b.x).min(self.c.x),
               self.a.y.min(self.b.y).min(self.c.y),
               self.a.z.min(self.b.z).min(self.c.z),
           ),
           Vec3::new(
               self.a.x.max(self.b.x).max(self.c.x),
               self.a.y.max(self.b.y).max(self.c.y),
               self.a.z.max(self.b.z).max(self.c.z),
           ),
       ))
   }
   ```
   Add `use crate::aabb::Aabb;` above the existing `use crate::hittable::{HitRecord, Hittable};`.
   Touch nothing in `hit`.

6. `src/main.rs:3` - add `mod aabb;` as the first module line, above `mod camera;`, matching the
   alphabetical order. No other line of the file changes: `scene()`, the camera, the render loop,
   the output and the tests stay as they are.

7. `src/aabb.rs` and `src/hittable.rs` - add the two test modules named in the `Tests` section
   below. `src/aabb.rs` gets its module after the `impl Aabb` block; `src/hittable.rs` gets one at
   the end of the file. Do not add or change a test anywhere else.

8. `AGENT-INSTRUCTIONS/LOCAL/project.md:5` - the repository map names every module of `src/`, so
   add `aabb.rs`. Change only this line. Before:
   `` - **Repository map:** `src/` - `main.rs` (scene, render loop, output, command line), `png.rs` (PNG encoding), `vec3.rs`, `ray.rs`, `camera.rs`, `hittable.rs` (hit record, the `Hittable` trait, the scene list), `sphere.rs`, `plane.rs`, `triangle.rs`, `rng.rs` (seeded randomness).``
   After:
   `` - **Repository map:** `src/` - `main.rs` (scene, render loop, output, command line), `png.rs` (PNG encoding), `vec3.rs`, `ray.rs`, `camera.rs`, `hittable.rs` (hit record, the `Hittable` trait, the scene list), `aabb.rs`, `sphere.rs`, `plane.rs`, `triangle.rs`, `rng.rs` (seeded randomness).``
   Match the current line exactly before editing; if it differs, keep every entry it names and add
   only `aabb.rs`. This file is agent material under `AGENT-INSTRUCTIONS/`, so load the
   `agent-instructions` skill before editing it, as `AGENTS.md` requires.

9. Docs - no page change. `docs/requirements.md` items 10 already states the box report and the
   scene list's pre-test, and `README.md` already says the program renders one fixed scene. Read
   both and leave them as written.

**Callers / wiring to update:**
- `src/hittable.rs:44-47` - `Hittable` gains `bounding_box`; every implementor must provide it.
- `src/hittable.rs:61-72` - `HittableList::hit` gains the box guard, and the same impl gains the
  `None` box.
- `src/sphere.rs:19-43` - `Sphere` gains its box.
- `src/plane.rs:21-37` - `Plane` gains `None`, so the scene list always tests it.
- `src/triangle.rs:22-51` - `Triangle` gains its box.
- `src/main.rs:3-12` - the module list gains `mod aabb;`.
- `src/main.rs:27-33` - `ray_colour` calls `world.hit` unchanged.
- No other file implements `Hittable` or constructs an `Aabb`.
- `AGENT-INSTRUCTIONS/LOCAL/project.md:5` - the repository map names `aabb.rs`.

**Config / schema / migrations:**
- None. No configuration key, no schema, no migration and no generated file.
- Retained records and their named consumer/owner: none added. `Aabb` is built on demand inside
  `bounding_box()` and read by `HittableList::hit` in the same call, so nothing is stored, logged
  or receipted.

**Touched files:**
<!-- touched-files:start -->
- `src/aabb.rs` - created: `Aabb` (two corners), its slab test and its test module.
- `src/hittable.rs` - changed: imports `Aabb`, the trait gains `bounding_box`, `HittableList::hit` tests an object's box before the object, `HittableList` reports no box, and a test module is added.
- `src/sphere.rs` - changed: `Sphere::bounding_box` returns the box around the centre.
- `src/plane.rs` - changed: `Plane::bounding_box` returns `None`.
- `src/triangle.rs` - changed: `Triangle::bounding_box` returns the box around the three vertices.
- `src/main.rs` - changed: declares `mod aabb;`.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` - changed: the repository map names `aabb.rs`.
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add a dependency or touch `Cargo.toml` (hard invariant 1).
- Do not change a shape's `hit` method, its normal, its texture coordinates or the point it
  returns. Only `bounding_box` is added to the three shape impls.
- Do not change `scene()`, the camera, the render loop's pixel mapping, `to_byte`, `write_ppm` or
  `main`.
- Do not add `Aabb::union`, a componentwise min/max helper on `Vec3`, a box for `HittableList`, a
  box for the plane, or a box grown by a point: nothing consumes any of them.
- Do not add epsilon, a zero-direction branch, a NaN check, or an infinity check to the slab test.
- Do not reorder the list's objects, sort by distance, or cache a box between calls.
- Do not add a `#[allow(dead_code)]` attribute to `Aabb`, its methods or the trait method. Leave
  the existing `#[allow(dead_code)]` on `mod plane;` in `src/main.rs` and on the `HitRecord` fields
  as they are.
- Do not edit `src/ray.rs`, `src/vec3.rs`, `src/camera.rs`, `src/rng.rs`, `docs/requirements.md` or
  `README.md`.
- Do not add more than two new test functions and do not change, weaken or delete an existing test
  (REPO-RULES §3a).
- Do not write a comment that names a plan, a phase or a future reader; keep every comment a
  present-tense fact about the code.
- Do not add `unsafe`, threads, randomness or materials. No second `HitRecord` construction site.

**Tests:**
- New: `src/aabb.rs` - `a_ray_through_the_box_hits_it_and_a_ray_beside_it_misses`: the box from
  `(-1, -1, -1)` to `(1, 1, 1)` is met by `Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0))`
  and not met by `Ray::new(Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0))`, both with
  `t_min = 0.001` and `t_max = f64::INFINITY`. Both rays have a zero `x` and `y` direction, so the
  first assertion also covers the infinite-bound path.
- New: `src/hittable.rs` - `the_list_skips_an_object_when_the_ray_misses_its_box`: a probe object
  whose `hit` counts its calls in an `Rc<Cell<usize>>` and returns `None`, with a box from
  `(5, 5, -3)` to `(6, 6, -2)`. A ray from `(0.0, 0.0, 0.0)` along `(0.0, 0.0, -1.0)` leaves the
  count at 0. A ray from `(5.5, 5.5, 0.0)` along `(0.0, 0.0, -1.0)` and a second `list.hit` call
  bring the count to 1. The probe and its counter live inside the test module.
- Changed/deleted: none. `src/main.rs`, `src/sphere.rs`, `src/plane.rs` and `src/triangle.rs`
  tests stay as written.
- Fast gate, run in this order:
  - `cargo test aabb`
  - `cargo test hittable`
  - `cargo test`
  - `cargo build`
  - `cargo clippy -- -D warnings`
  - `cargo fmt --check`
  - `git diff --check`
- Conditional gates:
  - Rendered image: required. The box test can change output only by discarding a hit, so a
    byte comparison of the render before and after the change is the cheapest complete proof that
    no hit was lost anywhere in the image, and it is the phase's move-on gate. No other
    rendered-image check is needed.
    1. Before editing any file, render the tree as it stands:
       `cargo run --release > /tmp/opencode/render-before-aabb.ppm`.
    2. After the change, render again:
       `cargo run --release > /tmp/opencode/render-after-aabb.ppm`.
    3. `cmp /tmp/opencode/render-before-aabb.ppm /tmp/opencode/render-after-aabb.ppm` must print
       nothing and exit 0. The header stays `P3`, `200 100`, `255`, and line 7441 stays
       `127 127 255`.
  - Agent generation: not run; no file under `AGENT-INSTRUCTIONS/SUBAGENTS/` changes.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - automated phase gates prove acceptance: the two named tests cover the slab test
  and the list's skip, and the byte-identical render proves every shape in the fixed scene still
  produces the same pixels as before the box test existed.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `src/aabb.rs` exists with `Aabb { min, max }`, `Aabb::new` and `Aabb::hit` as written in step
      1, with no additional method or field.
- [ ] `Hittable::bounding_box` exists and all four implementors provide it: boxes for `Sphere` and
      `Triangle`, `None` for `Plane` and `HittableList`.
- [ ] `HittableList::hit` tests each object's box against `t_min` and `limit` before calling
      `object.hit`, and its nearest-record expression is unchanged.
- [ ] `scene()`, the camera, the render loop, the output and the command line are unchanged.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a): two new test functions, no changed or deleted test;
      the receipt reports functions added and deleted.
- [ ] Fast phase gate and the rendered-image conditional gate from
      `AGENT-INSTRUCTIONS/BUILDING/VERIFICATION-COMMITS-DEPLOY.md` §1 pass; the two renders are
      byte-identical; no slow gate.
- [ ] `docs/requirements.md` item 10 and `README.md` are checked and need no edit, and
      `AGENT-INSTRUCTIONS/LOCAL/project.md:5` names `aabb.rs`. No page refers to a plan.
- [ ] No new comment names a plan or a later phase.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists.
- [ ] On passing evidence: compress this file into `done/B3-bounding-boxes.md` (frozen contract
      only: the `Aabb` shape, the slab test, the trait method and the list's pre-test) and flip the
      `full-plan.md` status row to `done`.

**Exit:** every shape reports its box, the scene list tests a box before the object and always
tests a plane, the fixed scene renders byte-identically to the image before the change, and cargo
build, test, clippy and fmt pass.
