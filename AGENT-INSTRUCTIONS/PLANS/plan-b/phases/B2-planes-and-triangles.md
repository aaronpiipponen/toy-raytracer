## Phase B2 - Planes and triangles

**Status:** active.
**Depends on:** `done/B1-face-side-and-texture-coordinates.md`, which froze `HitRecord` and
`HitRecord::new`.
**Scope:** add `src/plane.rs` (an infinite plane from a point and a normal, with texture
coordinates from the hit point along two in-plane axes, modulo 1) and `src/triangle.rs` (a
triangle from three vertices, hit with Möller-Trumbore, with barycentric texture coordinates), wire
both modules into `src/main.rs`, and add one triangle to the fixed scene. Nothing else changes: no
bounding boxes, no materials, no scattering, no output and no command line.
**Gate level:** standard - two new shape types behind the existing `Hittable` trait, one new scene
object, no new dependency, no configuration, no durable state. The rendered image is the phase's
own move-on gate, so the rendered-image conditional gate is required.
**Simplicity statement:** the plane stores the normal it is given and treats it as unit length, the
same way `Sphere` trusts its radius. The only degenerate cases handled are the two that would
otherwise return a meaningless record: a ray parallel to the plane (`denominator == 0.0`) and a ray
parallel to the triangle's plane (`determinant.abs() < 1e-8`). Both faces of every shape are hit
(no culling). Nothing normalizes a user value, recovers lost numerical precision, or accepts a
non-unit normal. No second construction of a `HitRecord` is added, no plane is put in the scene,
and the phase deletes the stale plan-naming comments listed under carry-overs.

**Current state (verified):**
- `src/hittable.rs:8-43` - `pub struct HitRecord { t, point, normal, front_face, u, v }` and
  `impl HitRecord { pub fn new(t: f64, point: Vec3, u: f64, v: f64, ray: &Ray, outward_normal: Vec3) -> Self }`,
  which records the face side and points `normal` against the ray.
- `src/hittable.rs:45-48` - `pub trait Hittable { fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>; }`.
- `src/hittable.rs:50-73` - `HittableList` holds `pub objects: Vec<Box<dyn Hittable>>`, `add` pushes,
  and `hit` keeps the nearest record by comparing `hit.t`. It needs no change.
- `src/sphere.rs:19-43` - the one existing `impl Hittable`, which shows the shape of a hit
  implementation and calls `HitRecord::new`.
- `src/main.rs:3-10` - the module list, `mod rng;` carrying `#[allow(dead_code)]`.
- `src/main.rs:32-37` - `fn scene() -> HittableList` adds two spheres.
- `src/main.rs:39-52` - `fn render(width: usize, height: usize) -> Vec<Colour>` builds the scene
  once and maps `column / (width - 1)` and `row / (height - 1)` to camera rays.
- `src/vec3.rs:21-29` - `pub fn cross(self, other: Vec3) -> Vec3`, carrying
  `#[allow(dead_code)]` and a plan-naming comment, because nothing calls it yet.
- `src/vec3.rs:75-80` - `impl Div<f64> for Vec3` exists, so normalization is `v / v.length()`.
- `src/ray.rs:11-18` - `Ray::new(origin, direction)` and `Ray::at(t)`.
- `src/main.rs:86-97` - two tests pin the centre pixel to the sphere and the top row to the sky.
- `docs/requirements.md:23-28` - items 7 and 8 (face side, texture coordinates for spheres, planes
  and triangles) and item 9 (the scene can hold planes and triangles).
- `src/main.rs:4-8` and `src/hittable.rs:11` - the plan-naming comments listed under carry-overs.

**Read first (do not invent):**
- `src/hittable.rs`, whole file - the record, `HitRecord::new` and the trait this phase implements.
- `src/sphere.rs`, whole file - the shape of an `impl Hittable` and its test module.
- `src/main.rs:1-52` and `src/main.rs:82-98` - the module list, the scene, the render loop and the
  tests that pin pixel colours.
- `src/vec3.rs` - `dot`, `cross`, `length`, `unit`, `Mul<f64>`, `Div<f64>`, `Sub`, `Neg`.
- `src/ray.rs` - `Ray` fields, `at`.
- `docs/requirements.md:21-30` - items 7, 8 and 9.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image conditional gate.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/B1-face-side-and-texture-coordinates.md` - the frozen
  record contract.

**Build plan:**

1. `src/plane.rs`, new file - an infinite plane, with this content shape:

   ```rust
   //! Planes.

   use crate::hittable::{HitRecord, Hittable};
   use crate::ray::Ray;
   use crate::vec3::Vec3;

   pub struct Plane {
       pub point: Vec3,
       pub normal: Vec3,
   }

   impl Plane {
       /// A plane through `point` with `normal`; `normal` is unit length.
       pub fn new(point: Vec3, normal: Vec3) -> Self {
           Plane { point, normal }
       }
   }

   impl Hittable for Plane {
       fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
           let denominator = ray.direction.dot(self.normal);
           if denominator == 0.0 {
               return None;
           }
           let t = (self.point - ray.origin).dot(self.normal) / denominator;
           if t <= t_min || t >= t_max {
               return None;
           }
           let point = ray.at(t);
           let (u_axis, v_axis) = plane_axes(self.normal);
           let u = point.dot(u_axis).rem_euclid(1.0);
           let v = point.dot(v_axis).rem_euclid(1.0);
           Some(HitRecord::new(t, point, u, v, ray, self.normal))
       }
   }

   /// Two unit axes in the plane, perpendicular to each other and to `normal`.
   fn plane_axes(normal: Vec3) -> (Vec3, Vec3) {
       // The helper is the world axis least aligned with the normal, so the two are never
       // parallel and the cross product is never zero.
       let helper = if normal.x.abs() <= normal.y.abs() && normal.x.abs() <= normal.z.abs() {
           Vec3::new(1.0, 0.0, 0.0)
       } else if normal.y.abs() <= normal.z.abs() {
           Vec3::new(0.0, 1.0, 0.0)
       } else {
           Vec3::new(0.0, 0.0, 1.0)
       };
       let u_axis = normal.cross(helper).unit();
       (u_axis, normal.cross(u_axis))
   }
   ```

   `denominator` is `normal . direction`; a zero value means the ray runs parallel to the plane.
   `rem_euclid(1.0)` gives the 0..=1 texture coordinate for a point on either side of the origin.
   Do not normalize `normal`, and do not divide by `normal.dot(normal)`: the constructor documents
   the unit precondition and every caller in this repository passes a unit normal.

2. `src/triangle.rs`, new file - a triangle, with this content shape:

   ```rust
   //! Triangles.

   use crate::hittable::{HitRecord, Hittable};
   use crate::ray::Ray;
   use crate::vec3::Vec3;

   pub struct Triangle {
       pub a: Vec3,
       pub b: Vec3,
       pub c: Vec3,
   }

   impl Triangle {
       pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
           Triangle { a, b, c }
       }
   }

   impl Hittable for Triangle {
       fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
           let edge1 = self.b - self.a;
           let edge2 = self.c - self.a;
           let pvec = ray.direction.cross(edge2);
           let determinant = edge1.dot(pvec);
           // A determinant near zero means the ray runs parallel to the triangle's plane.
           if determinant.abs() < 1e-8 {
               return None;
           }
           let inverse = 1.0 / determinant;
           let tvec = ray.origin - self.a;
           let u = tvec.dot(pvec) * inverse;
           if u < 0.0 || u > 1.0 {
               return None;
           }
           let qvec = tvec.cross(edge1);
           let v = ray.direction.dot(qvec) * inverse;
           if v < 0.0 || u + v > 1.0 {
               return None;
           }
           let t = edge2.dot(qvec) * inverse;
           if t <= t_min || t >= t_max {
               return None;
           }
           let point = ray.at(t);
           let outward_normal = edge1.cross(edge2).unit();
           Some(HitRecord::new(t, point, u, v, ray, outward_normal))
       }
   }
   ```

   This is Möller-Trumbore without culling: `u` is the weight of vertex `b` and `v` the weight of
   vertex `c`, so both are barycentric coordinates in 0..=1 with `u + v <= 1`. Pass them straight
   to `HitRecord::new`. The outward normal follows the vertex winding, and `HitRecord::new` turns
   it against the ray, so a triangle hit from either side works.

3. `src/main.rs` - declare the modules and add the triangle:

   - `mod plane;` with `#[allow(dead_code)]` on the line above it, directly after `mod hittable;`,
     because no scene object is a plane and only the plane tests build one:
     ```rust
     #[allow(dead_code)]
     mod plane;
     ```
   - `mod triangle;` directly after `mod sphere;`.
   - `use triangle::Triangle;` in the `use` block, after `use sphere::Sphere;`.
   - `scene()` gains one object after the two spheres:
     ```rust
     world.add(Box::new(Triangle::new(
         Vec3::new(-3.5, 0.0, -2.0),
         Vec3::new(-1.5, 0.0, -2.0),
         Vec3::new(-2.5, 1.5, -2.0),
     )));
     ```
     These vertices sit in the upper-left sky of the fixed 200 by 100 image, clear of both spheres
     and of the ground sphere's horizon, so the triangle is plainly visible and the centre-pixel
     test keeps finding the sphere. At `z = -2` the visible window is `x` in -4..4 and `y` in -2..2,
     and this triangle covers `x` in -3.5..-1.5 and `y` in 0..1.5.

4. `src/vec3.rs:21-23` - `cross` gains its first caller, so delete the comment
   `// For the triangle normals the geometry plan adds.` and the `#[allow(dead_code)]` above
   `pub fn cross`. Change nothing else in the file.

5. Carry-overs from `done/B1-face-side-and-texture-coordinates.md` - delete the comment
   `// The origin of the bounce rays the materials plan adds.` at `src/hittable.rs:11` and the
   comment `// Seeded randomness for the scattering and sampling the plans add.` at
   `src/main.rs:6`. Keep both `#[allow(dead_code)]` attributes. Delete the comment lines only.

6. Tests - one new test module in each new file, with the exact functions listed in the `Tests`
   section below. Do not add a test to `src/main.rs`, to `src/sphere.rs` or to any other module.

7. `AGENT-INSTRUCTIONS/LOCAL/project.md:5` - the repository map names every module of `src/`, so
   add the two new ones. Before:
   `` - **Repository map:** `src/` - `main.rs` (scene, render loop, output, command line), `vec3.rs`, `ray.rs`, `camera.rs`, `hittable.rs` (hit record, the `Hittable` trait, the scene list), `sphere.rs`, `rng.rs` (seeded randomness).``
   After:
   `` - **Repository map:** `src/` - `main.rs` (scene, render loop, output, command line), `vec3.rs`, `ray.rs`, `camera.rs`, `hittable.rs` (hit record, the `Hittable` trait, the scene list), `sphere.rs`, `plane.rs`, `triangle.rs`, `rng.rs` (seeded randomness).``
   This file is agent material under `AGENT-INSTRUCTIONS/`, so load the `agent-instructions` skill
   before editing it, as `AGENTS.md` requires. Change only the one line.

8. Docs - no page changes. `docs/requirements.md` items 8 and 9 already state plane and triangle
   texture coordinates and that the scene holds both shapes, and `README.md` already says the
   program renders one fixed scene. Read both and leave them as written.

**Callers / wiring to update:**
- `src/main.rs:3-10` - module list gains `#[allow(dead_code)] mod plane;` and `mod triangle;`.
- `src/main.rs:14-18` - `use` block gains `use triangle::Triangle;`.
- `src/main.rs:32-37` - `scene()` gains the one `Triangle` object from step 3.
- `src/vec3.rs:21-23` - `cross` keeps its signature and loses its dead-code attribute and comment.
- `src/hittable.rs:11` and `src/main.rs:6` - the two plan-naming comments are deleted.
- `AGENT-INSTRUCTIONS/LOCAL/project.md:5` - the repository map names the two new modules.
- `HittableList::hit` needs no change: it already keeps the nearest record from any `Hittable`.
- No other file constructs a `HitRecord` or names a shape: a repository-wide search finds
  `HitRecord` only in `src/hittable.rs`, `src/sphere.rs` and `src/main.rs` as `hit.normal`.

**Config / schema / migrations:**
- None. No configuration key, no schema, no migration and no generated file.
- Retained records and their named consumer/owner: none added. `Plane` and `Triangle` are in-memory
  scene objects behind `Box<dyn Hittable>`, consumed by `HittableList::hit` and `ray_colour` for the
  pixel colour. No log, receipt or stored field is added.

**Touched files:**
<!-- touched-files:start -->
- `src/plane.rs` - created: `Plane` (point and unit normal), its `Hittable` impl with the two in-plane axes and modulo-1 texture coordinates, and its test module.
- `src/triangle.rs` - created: `Triangle` (three vertices), its Möller-Trumbore `Hittable` impl with barycentric texture coordinates, and its test module.
- `src/main.rs` - changed: declares both modules, imports `Triangle`, adds one triangle to `scene()`, deletes one plan-naming comment.
- `src/vec3.rs` - changed: `cross` keeps its signature and loses `#[allow(dead_code)]` and the plan-naming comment above it.
- `src/hittable.rs` - changed: one plan-naming comment deleted above `point`.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` - changed: the repository map names `plane.rs` and `triangle.rs`.
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add a dependency or touch `Cargo.toml` (hard invariant 1).
- Do not add bounding boxes, an `Aabb` type or a `bounding_box` method; that is phase B3.
- Do not add materials, scattering, a PNG writer, a command line or progress output; those belong
  to other plans.
- Do not add a plane to the scene, and do not change the two spheres, the camera or the render
  loop's pixel mapping.
- Do not add a second way to build a `HitRecord`; both shapes call `HitRecord::new`.
- Do not add back-face culling, normal flipping inside a shape, or a `front_face` field of your
  own. `HitRecord::new` owns that decision.
- Do not normalize the plane's normal in the hit path, divide by `normal.dot(normal)`, or guard a
  non-unit normal, a non-positive radius or a NaN value.
- Do not add epsilon guards beyond the two named ones: `denominator == 0.0` in the plane and
  `determinant.abs() < 1e-8` in the triangle.
- Do not clamp or wrap `(u, v)` further: planes use `rem_euclid(1.0)`, triangles pass their
  barycentric values, and both already land in 0..=1.
- Do not edit `src/sphere.rs`, `src/ray.rs`, `src/camera.rs` or `src/rng.rs`.
- Do not add more than three new test functions and do not change an existing test (REPO-RULES
  §3a). Do not delete or weaken any existing test.
- Do not write a comment that names a plan, a phase or a future reader; keep every comment a
  present-tense fact about the code.
- Do not add `unsafe`, threads or seeded randomness; this phase reads none.

**Tests:**
- New: `src/plane.rs` - `a_ray_hits_the_plane_with_wrapped_texture_coordinates`: a plane through
  `(0, 0, -2)` with normal `(0, 0, 1)` hit by `Ray::new(Vec3::new(0.25, 0.5, 0.0), Vec3::new(0.0, 0.0, -1.0))`
  returns `t == 2`, `point == (0.25, 0.5, -2)`, `front_face` true, `normal == (0, 0, 1)`,
  `u == 0.5` and `v == 0.75`. The `v` value proves the modulo: the point's coordinate along that
  axis is -0.25.
- New: `src/triangle.rs` - `a_ray_hits_the_triangle_with_barycentric_texture_coordinates`: the
  triangle `(0, 0, -2)`, `(1, 0, -2)`, `(0, 1, -2)` hit by
  `Ray::new(Vec3::new(0.25, 0.25, 0.0), Vec3::new(0.0, 0.0, -1.0))` returns `t == 2`,
  `point == (0.25, 0.25, -2)`, `front_face` true, `normal == (0, 0, 1)`, `u == 0.25` and
  `v == 0.25`.
- New: `src/triangle.rs` - `a_ray_beside_the_triangle_misses`: the same triangle and a ray from
  `(2.0, 2.0, 0.0)` along `(0, 0, -1)` return `None`.
- Changed: none. `src/main.rs` and `src/sphere.rs` tests stay as written; the added triangle covers
  neither the centre pixel nor the top row.
- Fast gate, run in this order:
  - `cargo test plane`
  - `cargo test triangle`
  - `cargo test`
  - `cargo build`
  - `cargo clippy -- -D warnings`
  - `cargo fmt --check`
  - `git diff --check`
- Conditional gates:
  - Rendered image: required, because the scene gains an object and the phase's move-on gate is
    "rendered image shows a triangle". The command line does not exist yet, so render to standard
    output:
    1. Before editing `scene()`, render the tree as it stands:
       `cargo run --release > /tmp/opencode/render-before-triangle.ppm`.
    2. After the change, render again: `cargo run --release > /tmp/opencode/render-after-triangle.ppm`.
    3. `cmp /tmp/opencode/render-before-triangle.ppm /tmp/opencode/render-after-triangle.ppm` must
       report a difference, because the image gains the triangle.
    4. The PPM is `P3` text with three header lines followed by one pixel per line. Line 7441 is
       column 37 of row 37 counted from the top, which lies inside the triangle. `sed -n '7441p'
       /tmp/opencode/render-after-triangle.ppm` must print `127 127 255`, the triangle's normal
       colour `(0.5, 0.5, 1.0)` from the `(normal + 1) * 0.5` shading scaled to bytes. That pixel
       is sky (`182 211 255`) before the change, so the two files differ there.
  - Agent generation: not run; no file under `AGENT-INSTRUCTIONS/SUBAGENTS/` changes.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - automated phase gates prove acceptance: the three unit tests cover the plane hit,
  the modulo texture coordinate, the barycentric texture coordinate and the triangle's miss path,
  and the rendered-image gate above proves the scene now holds a triangle at the expected pixels
  while the existing pixel tests keep the sphere and the sky in place.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `src/plane.rs` and `src/triangle.rs` exist with the exact shapes in steps 1 and 2, and both
  implement `Hittable` through `HitRecord::new`.
- [ ] `Plane::hit` returns `(u, v)` from the two in-plane axes wrapped with `rem_euclid(1.0)`, and a
  ray parallel to the plane returns `None`.
- [ ] `Triangle::hit` is Möller-Trumbore without culling, passes barycentric `u` and `v`, and
  rejects a ray outside the triangle and a ray in its plane.
- [ ] The scene holds exactly the two spheres and one triangle, and `Triangle::new` is called only
  from `scene()` and the tests.
- [ ] `Vec3::cross` has a caller and carries no `#[allow(dead_code)]`.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a): three new test functions, no changed or deleted test;
  the receipt reports functions added and deleted.
- [ ] Fast phase gate and the rendered-image conditional gate from
  `AGENT-INSTRUCTIONS/BUILDING/VERIFICATION-COMMITS-DEPLOY.md` §1 pass; no slow gate.
- [ ] `docs/requirements.md` items 8 and 9 and `README.md` are checked and need no edit, and
  `AGENT-INSTRUCTIONS/LOCAL/project.md:5` names the two new modules. No page refers to a plan.
- [ ] Both carry-over comments are deleted and no comment names a plan or a later phase.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists.
- [ ] On passing evidence: compress this file into `done/B2-planes-and-triangles.md` (frozen
  contract only: the two types' fields, the hit formulas and the scene's triangle) and flip the
  `full-plan.md` status row to `done`.

**Exit:** `Plane` and `Triangle` both hit correctly through `HitRecord`, the fixed scene holds one
triangle above the sphere and clear of the centre, cargo build, test, clippy and fmt pass, and the
rendered image shows the triangle with the same sky and sphere elsewhere.
