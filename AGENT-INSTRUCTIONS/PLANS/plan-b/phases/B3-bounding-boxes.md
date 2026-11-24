## Phase B3 - Bounding boxes

**Status:** active.
**Depends on:** B2 (`done/B2-planes-and-triangles.md`): `Sphere`, `Plane` and `Triangle` all implement
`Hittable`, and `scene()` holds the centre sphere, the ground plane and the triangle. Also B1r
(`done/B1r-shared-hit-record-shape.md`) for the `HitRecord` shape this phase leaves alone.
**Scope:** add an `Aabb` type and a `bounding_box` method on `Hittable`, and make `HittableList::hit`
test an object's box before the object. `Plane` reports no box and is always tested. Every other
shape reports a box that holds it. The chosen hit and the rendered image do not change. No material,
no render-loop, camera, output, command-line or `docs/` change.
**Gate level:** standard - a new geometry type and one change on the live hit path, with no durable
state, trust boundary or migration, but the rendered image must stay byte-identical.
**Simplicity statement:** the box test is a plain slab rejection: when the ray misses an object's
box within `t_min..t_max` the object is skipped, and the object still decides every hit inside its
box, so the chosen record is unchanged. A ray exactly parallel to a slab axis is rejected only when
its origin lies outside that slab. Nothing shrinks a box, pads it, merges boxes of a scene list,
orders the scene objects, builds a hierarchy, or handles a degenerate box beyond the plain slab
comparison. `Aabb::hit` returns `bool` because no caller needs the entry distance. No existing code
or test is deleted.

**Current state (verified):**
- `src/hittable.rs:44-47` - `pub trait Hittable { fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>; }`; it has one method.
- `src/hittable.rs:61-72` - `impl Hittable for HittableList`: `let limit = nearest.map_or(t_max, |hit| hit.t);` then `object.hit(ray, t_min, limit)`. `HitRecord` is `Copy`, so `nearest` is by value.
- `src/hittable.rs:8-21` - `#[derive(Clone, Copy, Debug)] pub struct HitRecord` with `t`, `point`, `normal`, `front_face`, `u`, `v`; this phase does not change it.
- `src/sphere.rs:8-11` - `pub struct Sphere { pub center: Vec3, pub radius: f64 }`; `src/sphere.rs:19-45` - `impl Hittable for Sphere` with `hit` only.
- `src/plane.rs:8-11` - `pub struct Plane { pub point: Vec3, pub normal: Vec3 }`; `src/plane.rs:23-52` - `impl Hittable for Plane` with `hit` only.
- `src/triangle.rs:8-12` - `pub struct Triangle { pub a: Vec3, pub b: Vec3, pub c: Vec3 }`; `src/triangle.rs:21-48` - `impl Hittable for Triangle` with `hit` only.
- `src/vec3.rs:5-10` - `#[derive(Clone, Copy, Debug, PartialEq)] pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }`; `src/vec3.rs:13-15` - `Vec3::new`; `src/vec3.rs:40-78` - the `Add`, `Sub`, `Neg`, `Mul<f64>`, `Mul<Vec3>` and `Div<f64>` impls. It has no componentwise `min` or `max`.
- `src/ray.rs:5-19` - `pub struct Ray { pub origin: Vec3, pub direction: Vec3 }`, `new`, and `at`; the direction is not normalised.
- `src/main.rs:3-12` - the module list `camera, hittable, plane, ray, rng, sphere, triangle, vec3`; `src/main.rs:16-22` - the imports.
- `src/main.rs:28-34` - `ray_colour` reads `hit.normal` and needs no edit.
- `src/main.rs:36-49` - `scene()` adds the centre sphere, the ground plane and the triangle; this phase does not change it.
- `src/main.rs` has no unit test that builds a `HittableList` by hand; the two render tests at
  `src/main.rs:95-112` pin the centre pixel and the top row.
- `docs/requirements.md:27-29` - item 10 states the box behavior; nothing becomes wrong when this
  phase makes it true.
- `Cargo.toml` has no dependency. Cargo and its tools are not on `PATH`; call `~/.cargo/bin/cargo`.

**Read first (do not invent):**
- `src/hittable.rs` - the whole file: the trait this phase extends and the list whose `hit` gains the box test.
- `src/sphere.rs`, `src/plane.rs`, `src/triangle.rs` - each shape's `impl Hittable` block and the vertex and centre fields each `bounding_box` reads.
- `src/vec3.rs` - the whole file: `Vec3` is `Copy` and has `PartialEq`, and it has no componentwise min or max helper.
- `src/ray.rs` - `Ray::origin` and `Ray::direction` are plain fields, and `direction` is not normalised.
- `src/main.rs` - the two render tests whose output this phase must preserve.
- `docs/requirements.md` item 10 - the behavior this phase makes real.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image command.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md` and
  `AGENT-INSTRUCTIONS/PLANS/plan-b/done/B2r-shared-scene-settlement.md` - the release merge's
  obligations on this branch.

**Build plan:**
1. `src/aabb.rs` - create the module with exactly this content (the test comes from the `Tests`
   section and sits in a `#[cfg(test)] mod tests` block below the implementation, opened with
   `use super::*;`):

   ```rust
   //! An axis-aligned bounding box.

   use crate::ray::Ray;
   use crate::vec3::Vec3;

   /// An axis-aligned box between `min` and `max`.
   pub struct Aabb {
       pub min: Vec3,
       pub max: Vec3,
   }

   impl Aabb {
       /// Return the box between the two corners `a` and `b`.
       pub fn new(a: Vec3, b: Vec3) -> Self {
           Aabb {
               min: Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
               max: Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
           }
       }

       /// Whether the ray meets the box with a `t` in `t_min..t_max`.
       pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
           let mut t_min = t_min;
           let mut t_max = t_max;
           for (origin, direction, min, max) in [
               (ray.origin.x, ray.direction.x, self.min.x, self.max.x),
               (ray.origin.y, ray.direction.y, self.min.y, self.max.y),
               (ray.origin.z, ray.direction.z, self.min.z, self.max.z),
           ] {
               if direction == 0.0 {
                   if origin < min || origin > max {
                       return false;
                   }
                   continue;
               }
               let inverse = 1.0 / direction;
               let near = (min - origin) * inverse;
               let far = (max - origin) * inverse;
               let (near, far) = if near > far { (far, near) } else { (near, far) };
               t_min = t_min.max(near);
               t_max = t_max.min(far);
               if t_max < t_min {
                   return false;
               }
           }
           true
       }
   }
   ```

   `Aabb::new` orders the two corners per axis, so a caller may pass them in either order. The slab
   test is only a rejection test: a `false` means the ray meets no point of the box with `t` in
   `t_min..t_max`, so the caller may skip the object.

2. `src/hittable.rs`, `Hittable` trait - add a second required method after `hit`:

   ```rust
   /// The axis-aligned box that holds every point of the object, or `None` when it has none.
   fn bounding_box(&self) -> Option<Aabb>;
   ```

   Add `use crate::aabb::Aabb;` to the imports at the top of the file. A required method (not a
   default) makes every `Hittable` state its box, so a new shape cannot silently lose the test.

3. `src/hittable.rs`, `impl Hittable for HittableList` - add `bounding_box` returning `None`, and
   insert the box test before each `object.hit` call so `hit` becomes:

   ```rust
   fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
       let mut nearest: Option<HitRecord> = None;
       for object in &self.objects {
           if object
               .bounding_box()
               .is_some_and(|bounds| !bounds.hit(ray, t_min, t_max))
           {
               continue;
           }
           let limit = nearest.map_or(t_max, |hit| hit.t);
           if let Some(hit) = object.hit(ray, t_min, limit) {
               nearest = Some(hit);
           }
       }
       nearest
   }

   /// A scene list has no box of its own. Each object reports its own.
   fn bounding_box(&self) -> Option<Aabb> {
       None
   }
   ```

   The box is tested over the full `t_min..t_max` and the object is still tested with the running
   `limit`, so skipping a box the ray misses cannot change the chosen record. A list reports no box
   because it can hold an unbounded plane and nothing nests a list as an object.

4. `src/sphere.rs` - add `use crate::aabb::Aabb;` and, inside `impl Hittable for Sphere`, after `hit`:

   ```rust
   /// The box that holds the whole sphere.
   fn bounding_box(&self) -> Option<Aabb> {
       let radius = Vec3::new(self.radius, self.radius, self.radius);
       Some(Aabb::new(self.center - radius, self.center + radius))
   }
   ```

5. `src/plane.rs` - add `use crate::aabb::Aabb;` and, inside `impl Hittable for Plane`, after `hit`:

   ```rust
   /// A plane has no box, so it is always tested.
   fn bounding_box(&self) -> Option<Aabb> {
       None
   }
   ```

6. `src/triangle.rs` - add `use crate::aabb::Aabb;` and, inside `impl Hittable for Triangle`, after `hit`:

   ```rust
   /// The box that holds the three vertices.
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

   The box is built from the vertices with no padding and no edge-case branch, so a degenerate
   triangle keeps a flat box and is skipped or hit exactly as its own `hit` decides.

7. `src/main.rs`, module list - add `mod aabb;` before `mod camera;`, keeping the list alphabetical.
   No import is needed in `main.rs`: nothing there names `Aabb`.

8. `docs/` - no page changes. `docs/requirements.md` item 10 states the box test and the plane's
   missing box exactly as this phase implements them. `README.md` describes the program as rendering
   one fixed scene and names no module or shape.

**Callers / wiring to update:**
- `src/hittable.rs:44-47` - the trait gains `bounding_box`, so all four implementations below compile only once each has it.
- `src/hittable.rs:61-72` - `HittableList::hit` gains the box test and `HittableList` gains `bounding_box`.
- `src/sphere.rs:19-45`, `src/plane.rs:23-52`, `src/triangle.rs:21-48` - each `impl Hittable` gains `bounding_box`.
- `src/main.rs:3-12` - the module list gains `aabb`.
- `src/main.rs:28-34` - `ray_colour` and the two render tests need no edit; the box test must leave their output unchanged.
- The release merge, not this phase, converts `HittableList::hit` to the borrowed nearest record
  `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);` and gives `Plane` and `Triangle` their
  material argument. The box test and every `bounding_box` method stay as written here.

**Config / schema / migrations:**
- None: no configuration key, migration, generated file or persisted record.
- Retained records and their named consumer/owner: none; `Aabb` and every hit value stay in memory.

**Touched files:**
<!-- touched-files:start -->
- `src/aabb.rs` - created: `Aabb` with `new` ordering two corners and `hit` rejecting a missed box
- `src/hittable.rs` - changed: the `Hittable` trait gains `bounding_box`; `HittableList::hit` tests a box before its object and `HittableList` reports no box
- `src/sphere.rs` - changed: `Sphere::bounding_box` returns the box around the sphere
- `src/plane.rs` - changed: `Plane::bounding_box` returns none
- `src/triangle.rs` - changed: `Triangle::bounding_box` returns the box around the three vertices
- `src/main.rs` - changed: the module list gains `aabb`
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add a dependency to `Cargo.toml`, do not use `unsafe`, and keep rendering single-threaded (domain invariants 1 and 3).
- Do not change `HitRecord`, `HitRecord::new`, `ray_colour`, `render`, the camera, the output or `scene()`.
- Do not change `Sphere::hit`, `Plane::hit` or `Triangle::hit`, and do not add a second hit path.
- Do not give any box a material or an `Rc`; materials are the release merge's addition.
- Do not clamp, pad or shrink a box, do not merge the boxes of a scene list, do not order the scene objects, and do not add a hierarchy.
- Do not change the chosen hit: the box test runs over `t_min..t_max` and the object's `hit` keeps the running `limit`, so a hit inside a box is never dropped.
- Do not make `bounding_box` a default method and do not trim `Aabb` fields or methods that the listed code uses.
- Do not change, rename or delete an existing test. Add only the three test functions below.
- Do not touch `docs/`, `README.md`, `Cargo.toml`, `AGENT-INSTRUCTIONS/LOCAL/` or another plan's files.

**Tests:**
- Each test sits in its module's own `#[cfg(test)] mod tests { use super::*; ... }` block, the form
  `src/sphere.rs:47-49` uses.
- New (3 functions, the phase maximum):
  - `src/aabb.rs` - `a_ray_hits_the_box_it_crosses_and_misses_one_beside_it`:

    ```rust
    #[test]
    fn a_ray_hits_the_box_it_crosses_and_misses_one_beside_it() {
        let bounds = Aabb::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::new(0.5, 0.5, 0.5));
        let through = Ray::new(Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(bounds.hit(&through, 0.001, f64::INFINITY));
        let beside = Ray::new(Vec3::new(2.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(!bounds.hit(&beside, 0.001, f64::INFINITY));
    }
    ```

  - `src/triangle.rs` - `the_triangle_box_holds_its_three_vertices`:

    ```rust
    #[test]
    fn the_triangle_box_holds_its_three_vertices() {
        let triangle = Triangle::new(
            Vec3::new(-2.0, -0.5, -2.0),
            Vec3::new(2.0, -0.5, -2.0),
            Vec3::new(0.0, 1.5, -2.0),
        );
        let bounds = triangle.bounding_box().expect("the triangle has a box");
        assert_eq!(bounds.min, Vec3::new(-2.0, -0.5, -2.0));
        assert_eq!(bounds.max, Vec3::new(2.0, 1.5, -2.0));
    }
    ```

  - `src/hittable.rs` - `the_scene_list_still_hits_a_plane_that_has_no_box`:

    ```rust
    #[test]
    fn the_scene_list_still_hits_a_plane_that_has_no_box() {
        let mut world = HittableList::default();
        world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Box::new(Plane::new(
            Vec3::new(0.0, -0.5, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )));
        let ray = Ray::new(Vec3::new(5.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let hit = world
            .hit(&ray, 0.001, f64::INFINITY)
            .expect("the ray hits the plane");
        assert!((hit.t - 0.5).abs() < 1e-9);
        assert_eq!(hit.normal, Vec3::new(0.0, 1.0, 0.0));
    }
    ```

    The ray misses the sphere's box because its origin is at `x = 5`, and the boxless plane is
    tested and hit at `t = 0.5`. The test module needs `use crate::plane::Plane;` and
    `use crate::sphere::Sphere;` beside `use super::*;`.
- Changed: none. Deleted: none. `src/main.rs`'s two render tests stay as they are.
- Fast gate, from the worktree root: `~/.cargo/bin/cargo test`, `~/.cargo/bin/cargo build`,
  `~/.cargo/bin/cargo clippy -- -D warnings`, `~/.cargo/bin/cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image, required because the hit path changes and the output must stay
  byte-identical. Render `~/.cargo/bin/cargo run --release > /tmp/opencode/b3-after.ppm` and compare
  it with the pre-change render, with the numbered acceptance steps below. Agent generation: not
  affected.
- Slow gate (>2 minutes): none; the 200 by 100 single-ray render finishes in well under a second in
  release mode.

**User test (acceptance):**
- **Executor:** agent.
1. Before changing any file, from the worktree root run
   `~/.cargo/bin/cargo run --release > /tmp/opencode/b3-before.ppm` and expect exit status 0.
2. After implementing, run `~/.cargo/bin/cargo run --release > /tmp/opencode/b3-after.ppm` and
   expect exit status 0.
3. Run `cmp /tmp/opencode/b3-before.ppm /tmp/opencode/b3-after.ppm` and expect no output and exit
   status 0: the box test leaves the image byte-identical. As a cross-check, the file's
   `sha256sum` is `36fa51831310f0d4f627f86d20e89215793f8988bb99f74ae2660bfda67488ae`.
4. If the two files differ, stop and report instead of ticking the checklist.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `Aabb` exists with `new` and `hit`, and `Hittable` has the required `bounding_box` method.
- [ ] `Sphere` and `Triangle` report a box that holds their geometry, and `Plane` and `HittableList` report none.
- [ ] `HittableList::hit` tests an object's box before the object and still returns the nearest record.
- [ ] The rendered PPM is byte-identical to the pre-change render, checked with the numbered acceptance steps.
- [ ] The two existing `src/main.rs` render tests pass without an edit.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a); receipt reports 3 functions added, 0 changed, 0 deleted. Any line counts are observations, not gates.
- [ ] Fast phase gate and the rendered-image conditional gate from `AGENT-INSTRUCTIONS/LOCAL/gates.md` pass; no slow gate applies.
- [ ] No `docs/` page needed a change; `docs/requirements.md` item 10 is accurate and no page refers to a plan.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the agent-run acceptance above supplies its evidence.
- [ ] The release merge's obligations are unchanged by this phase: `HittableList::hit` keeps the borrowed nearest record and `Plane` and `Triangle` take a material argument.
- [ ] On passing evidence: compress this file into `done/B3-bounding-boxes.md` (frozen contract only) and flip the `full-plan.md` status row to `done`.

**Exit:** every shape reports or declines a box, the scene list skips an object whose box the ray
misses without changing the chosen hit, and the rendered image is identical to the B2 image with all
gates green.
