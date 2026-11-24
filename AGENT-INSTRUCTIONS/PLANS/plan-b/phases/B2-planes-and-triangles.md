## Phase B2 - Planes and triangles

**Status:** active.
**Depends on:** B1 (`done/B1-face-side-and-texture-coordinates.md`): `HitRecord` carries `front_face`,
`u` and `v`, and `HitRecord::new` is the only constructor.
**Scope:** add `src/plane.rs` (a point and a unit normal, hit by ray, (u, v) from the hit point along
two in-plane axes modulo 1) and `src/triangle.rs` (three vertices, Möller-Trumbore, barycentric (u, v)),
declare both modules in `src/main.rs`, put one triangle in the scene, and drop the two plan references
that sit in code comments in the files this phase edits. No material, no bounding box, no new output,
no command line, no plane in the scene.
**Gate level:** standard - two new shape modules built on the frozen `HitRecord` contract, bounded
runtime behaviour, no durable state, config, security or data-access seam; targeted shape tests plus
the fast gate, the rendered-image conditional gate and the repository-map update are enough.
**Simplicity statement:** a plane's normal is a unit vector supplied by the caller, as documented, and
a degenerate triangle (zero-area, or a ray parallel to it within `1e-9`) returns no hit rather than a
repair. The phase does not clamp or wrap texture coordinates beyond `rem_euclid(1.0)`, does not build a
UV basis cache, does not guarantee watertight shared edges between triangles, does not handle
zero-length normals or NaN inputs, and adds no material, bounding-box, output or command-line code.
No legacy or defensive code is added or deleted.

**Current state (verified):**
- `src/hittable.rs:6-41` - `pub struct HitRecord { pub t: f64, #[allow(dead_code)] pub point: Vec3, pub normal: Vec3, #[allow(dead_code)] pub front_face: bool, #[allow(dead_code)] pub u: f64, #[allow(dead_code)] pub v: f64 }`, and `impl HitRecord { pub fn new(ray: &Ray, t: f64, point: Vec3, outward_normal: Vec3, u: f64, v: f64) -> Self }`, which sets `front_face` from `ray.direction.dot(outward_normal) < 0.0` and stores the outward normal or its negation so `normal` faces the ray.
- `src/hittable.rs:43-46` - `pub trait Hittable { fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>; }`; this phase adds no method.
- `src/hittable.rs:48-71` - `HittableList { pub objects: Vec<Box<dyn Hittable>> }`, `add`, and `hit` taking the nearest record with `let limit = nearest.map_or(t_max, |hit| hit.t);`.
- `src/sphere.rs:9-46` - the shape module pattern to copy: `pub struct Sphere` with public fields, `impl Sphere { pub fn new(center: Vec3, radius: f64) -> Self }`, `impl Hittable for Sphere`, and a final `Some(HitRecord::new(ray, t, point, outward_normal, u, v))`.
- `src/sphere.rs:48-93` - the `#[cfg(test)] mod tests { use super::*; ... }` pattern: `Sphere::new`, `Ray::new`, `.hit(&ray, 0.001, f64::INFINITY).expect("the ray hits")`, tolerance `1e-9`.
- `src/vec3.rs:23-29` - `pub fn cross(self, other: Vec3) -> Vec3` with `#[allow(dead_code)]` at line 22 and the comment `// For the triangle normals the geometry plan adds.` at line 21; nothing calls `cross` yet.
- `src/vec3.rs:31-38` - `pub fn length(self) -> f64` and `pub fn unit(self) -> Vec3`; `src/vec3.rs:40-80` - `Add`, `Sub`, `Neg`, `Mul<f64>`, `Mul<Vec3>`, `Div<f64>`, and `pub type Colour = Vec3`.
- `src/ray.rs:5-18` - `pub struct Ray { pub origin: Vec3, pub direction: Vec3 }`, `Ray::new`, `pub fn at(&self, t: f64) -> Vec3`.
- `src/main.rs:3-10` - `mod camera; mod hittable; mod ray;` then the comment `// Seeded randomness for the scattering and sampling the plans add.` at line 6, `#[allow(dead_code)] mod rng;`, `mod sphere; mod vec3;`.
- `src/main.rs:14-18` - `use camera::Camera; use hittable::{Hittable, HittableList}; use ray::Ray; use sphere::Sphere; use vec3::{Colour, Vec3};`.
- `src/main.rs:23-30` - `fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Colour` colours a hit by `(hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5`; it reads only `normal`, so it works for any new shape unchanged.
- `src/main.rs:32-37` - `fn scene() -> HittableList` adds `Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)` and `Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)`.
- `src/main.rs:82-98` - tests `the_center_pixel_shows_the_sphere_facing_the_camera` (centre pixel `center.z > 0.9`) and `the_top_row_is_sky` (`pixels[0].z > pixels[0].x`); the scene addition must leave both true.
- `src/camera.rs:14-28` - the viewport at `z = -1` spans `x` in `-2..2` and `y` in `-1..1` for the 2:1 image (`src/main.rs:20-21`, `WIDTH = 200`, `HEIGHT = 100`), so a point `(x, y, -1)` lands at pixel column `(x + 2) / 4 * 199` and row from the top `(1 - y) / 2 * 99`. The camera is not changed this phase.
- `docs/requirements.md:21-30` - items 7, 8, 9 and 10: the face side, texture coordinates (planes by position along two in-plane axes modulo 1, triangles barycentric), planes and triangles in the scene, and bounding boxes. Items 7 and 8 are already the target behaviour; item 10 belongs to B3.

**Read first (do not invent):**
- `src/hittable.rs` - whole file; `HitRecord::new` is the only allowed way to build a record in both new shapes.
- `src/sphere.rs` - whole file; the module layout, the `hit` shape and the test style the two new modules copy.
- `src/vec3.rs` - `dot`, `cross`, `unit`, `length`, and the operator impls used by the steps below.
- `src/ray.rs` - `Ray::at` and the unnormalised `direction`.
- `src/main.rs` - the module list, the use block, `scene` and the two render tests the triangle must not break.
- `src/camera.rs` - the viewport extents above, used to place the scene triangle on screen.
- `docs/requirements.md` items 7, 8 and 9 - the behaviour this phase makes real.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate, the rendered-image conditional gate and the commands.
- `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md` section "Code documentation" - the header, doc-comment and comment rules the new prose follows.

**Build plan:**
1. `src/plane.rs` (create) - header `//! Planes.` first, then:

   ```rust
   use crate::hittable::{HitRecord, Hittable};
   use crate::ray::Ray;
   use crate::vec3::Vec3;

   /// An infinite plane: a point on it and its unit normal.
   pub struct Plane {
       pub point: Vec3,
       pub normal: Vec3,
   }

   impl Plane {
       /// Build a plane through `point` with unit normal `normal`.
       pub fn new(point: Vec3, normal: Vec3) -> Self {
           Plane { point, normal }
       }
   }
   ```

   Then `impl Hittable for Plane` with exactly this body, so the tests below hold:

   ```rust
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
       let tangent = if self.normal.x.abs() > self.normal.y.abs() {
           Vec3::new(-self.normal.z, 0.0, self.normal.x).unit()
       } else {
           Vec3::new(0.0, -self.normal.z, self.normal.y).unit()
       };
       let bitangent = self.normal.cross(tangent);
       let u = point.dot(tangent).rem_euclid(1.0);
       let v = point.dot(bitangent).rem_euclid(1.0);
       Some(HitRecord::new(ray, t, point, self.normal, u, v))
   }
   ```

   `tangent` and `bitangent` are two unit axes in the plane (the formula uses the smaller component
   of the unit normal to stay clear of a zero cross product), and `rem_euclid(1.0)` maps each
   position into `0..1` and wraps `2.25` to `0.25`. `self.normal` is the outward normal given to
   `HitRecord::new`, which records the face side. Do not add a helper, a cache or a second hit path.

2. `src/triangle.rs` (create) - header `//! Triangles.` first, then:

   ```rust
   use crate::hittable::{HitRecord, Hittable};
   use crate::ray::Ray;
   use crate::vec3::Vec3;

   pub struct Triangle {
       pub a: Vec3,
       pub b: Vec3,
       pub c: Vec3,
   }

   impl Triangle {
       /// Build a triangle from its three vertices, counter-clockwise seen from its front face.
       pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
           Triangle { a, b, c }
       }
   }
   ```

   Then `impl Hittable for Triangle` with exactly this body:

   ```rust
   fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
       let edge1 = self.b - self.a;
       let edge2 = self.c - self.a;
       let h = ray.direction.cross(edge2);
       let determinant = edge1.dot(h);
       if determinant.abs() < 1e-9 {
           return None;
       }
       let inverse = 1.0 / determinant;
       let s = ray.origin - self.a;
       let u = inverse * s.dot(h);
       if u < 0.0 || u > 1.0 {
           return None;
       }
       let q = s.cross(edge1);
       let v = inverse * ray.direction.dot(q);
       if v < 0.0 || u + v > 1.0 {
           return None;
       }
       let t = inverse * edge2.dot(q);
       if t <= t_min || t >= t_max {
           return None;
       }
       let point = ray.at(t);
       let outward_normal = edge1.cross(edge2).unit();
       Some(HitRecord::new(ray, t, point, outward_normal, u, v))
   }
   ```

   Möller-Trumbore yields the barycentric weights `u` of `b` and `v` of `c` relative to `a`, which
   are the triangle's texture coordinates; pass them unchanged and do not renormalise them.

3. `src/main.rs`, module list (`src/main.rs:3-10`) - before -> after, keeping the alphabetical order:

   ```rust
   // before
   mod camera;
   mod hittable;
   mod ray;
   // Seeded randomness for the scattering and sampling the plans add.
   #[allow(dead_code)]
   mod rng;
   mod sphere;
   mod vec3;

   // after
   mod camera;
   mod hittable;
   #[allow(dead_code)]
   mod plane;
   mod ray;
   #[allow(dead_code)]
   mod rng;
   mod sphere;
   mod triangle;
   mod vec3;
   ```

   `plane` carries `#[allow(dead_code)]` because the scene holds no plane and only the tests build
   one, so `cargo build` would warn otherwise, the same handling `rng` already has. Delete the
   comment naming "the plans": no code comment refers to a plan. Add no comment in its place.

4. `src/main.rs`, use block (`src/main.rs:14-18`) - add `use triangle::Triangle;` after
   `use sphere::Sphere;`. No `use plane::Plane;` is added, because the scene holds no plane.

5. `src/main.rs`, `scene` (`src/main.rs:32-37`) - after the two `world.add` lines add exactly:

   ```rust
   world.add(Box::new(Triangle::new(
       Vec3::new(0.6, 0.15, -1.0),
       Vec3::new(1.95, 0.15, -1.0),
       Vec3::new(1.275, 0.92, -1.0),
   )));
   ```

   The triangle lies in the viewport plane at `z = -1`, to the right of the sphere. It covers
   pixels around columns 130 to 196 and rows 5 to 42, every one of them against sky: no part of it
   overlaps the ground sphere's silhouette or leaves the image. It faces the camera with normal
   `(0, 0, 1)`, so `ray_colour` paints it the flat colour `(0.5, 0.5, 1.0)`. The centre pixel and
   the top-left pixel stay on the sphere and the sky.

6. `src/vec3.rs`, `cross` (`src/vec3.rs:21-29`) - delete the comment `// For the triangle normals the
   geometry plan adds.` and the `#[allow(dead_code)]` attribute above `pub fn cross`. The two new
   modules call `cross`, so the allow is no longer needed and the comment names a plan. Change
   nothing else in the file.

7. `src/plane.rs` and `src/triangle.rs`, tests - add exactly the two test functions named under
   `Tests`, one in each module's `#[cfg(test)] mod tests`, in the style of `src/sphere.rs`.

8. `docs/` - no page changes. `docs/requirements.md` items 8 and 9 already state planes, triangles
   and their texture coordinates, and no page states that the scene holds only spheres. `README.md`
   stays accurate because the scene is still one fixed scene.

9. `AGENT-INSTRUCTIONS/LOCAL/project.md` (repository map, line 5) - append `plane.rs` and
   `triangle.rs` to the `src/` list, for example `..., `sphere.rs`, `plane.rs` (an infinite plane),
   `triangle.rs` (three vertices), `material.rs` (the `Material` trait and the diffuse and metal
   materials), `rng.rs` (seeded randomness).` Keep the existing entries and their wording. This edit
   sits under `AGENT-INSTRUCTIONS/` outside `PLANS/`, so load the `agent-instructions` skill before
   making it.

**Callers / wiring to update:**
- `src/main.rs:3-10` - declares `mod plane;` and `mod triangle;` and drops the plan comment.
- `src/main.rs:14-18` - imports `Triangle` for the scene.
- `src/main.rs:32-37` - adds the one scene triangle; `ray_colour` and `render` need no edit.
- `src/hittable.rs:48-71` - `HittableList` needs no edit; it stores `Box<dyn Hittable>` and copies records.
- `src/vec3.rs:21-29` - `cross` loses its allow and comment because both new modules call it.
- No other file reads or builds a triangle or a plane (verified by grep across `src/`).

**Config / schema / migrations:**
- None: no configuration key, migration, generated file or persisted record.
- Retained records and their named consumer/owner: none; `HitRecord` stays an in-memory value.

**Touched files:**
<!-- touched-files:start -->
- `src/plane.rs` - created: `Plane` with a point and a unit normal, its `Hittable` hit, in-plane (u, v) modulo 1, and one test
- `src/triangle.rs` - created: `Triangle` with three vertices, Möller-Trumbore hit, barycentric (u, v), and one test
- `src/main.rs` - changed: declares the plane and triangle modules, imports `Triangle`, adds the scene triangle, drops the plan comment above `mod rng`
- `src/vec3.rs` - changed: `cross` loses its `#[allow(dead_code)]` and its plan comment
- `AGENT-INSTRUCTIONS/LOCAL/project.md` - changed: the repository map lists `plane.rs` and `triangle.rs`
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add a dependency to `Cargo.toml` and do not use `unsafe` (domain invariants 1 and 3).
- Do not add a second way to build a `HitRecord`; both shapes call `HitRecord::new` with their outward normal.
- Do not change `HitRecord`, the `Hittable` trait, `HittableList`, `ray_colour`, `render`, `write_ppm`, the camera or the output. Do not add a plane to the scene.
- Do not change `the_center_pixel_shows_the_sphere_facing_the_camera` or `the_top_row_is_sky`; the triangle is placed so both stay true.
- Do not clamp, wrap or offset texture coordinates beyond the formulas above, and do not normalise a plane normal, handle a zero-length normal, a zero-area triangle or a NaN input.
- Do not add fields, helper functions, caches or builder types beyond the two modules above.
- Do not add a test function beyond the two named here (REPO-RULES §3a).
- Do not touch `docs/`, `README.md`, `Cargo.toml`, another plan's files, or any other `AGENT-INSTRUCTIONS/` file; the repository map line is the only instruction-tree edit.
- Do not write plan or phase references in code comments.

**Tests:**
- New (2 functions):
  - `src/plane.rs` - `a_ray_through_the_plane_records_the_hit_and_texture_coordinates_that_wrap`: plane `Plane::new(Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 1.0))`, ray `Ray::new(Vec3::new(2.25, -0.5, 0.0), Vec3::new(0.0, 0.0, -1.0))`; the hit is `Some`, `t` is about `2.0`, `front_face` is true, `normal == Vec3::new(0.0, 0.0, 1.0)`, `u` is about `0.5` and `v` is about `0.25` (the `x` position `2.25` wraps), all within `1e-9`.
  - `src/triangle.rs` - `a_ray_through_the_triangle_hits_its_front_face_with_barycentric_coordinates`: triangle `Triangle::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(1.0, 0.0, -1.0), Vec3::new(0.0, 1.0, -1.0))`, ray `Ray::new(Vec3::new(0.25, 0.25, 0.0), Vec3::new(0.0, 0.0, -1.0))`; the hit is `Some`, `t` is about `1.0`, `front_face` is true, `normal == Vec3::new(0.0, 0.0, 1.0)`, and `u` is about `0.25` and `v` is about `0.25`, each within `1e-9`.
- Changed: none. Deleted: none. The four existing `src/sphere.rs` tests, the `src/rng.rs` test and the two `src/main.rs` tests stay as they are.
- Fast gate: `cargo test` (9 tests), `cargo build`, `cargo clippy -- -D warnings`, `cargo fmt --check`, `git diff --check`.
- Conditional gates: the rendered-image gate is required because the scene changes. Render `cargo run --release > /tmp/opencode/plan-b-b2.ppm`, convert it to PNG with the snippet below, open the PNG with the image reader, and confirm a flat light-blue triangle with straight edges on the right of the image, wholly against the sky, clear of the sphere, the ground and the image edges. The image is expected to differ from the B1 render, so no `cmp` comparison applies. Agent-generation gate: none, `AGENT-INSTRUCTIONS/SUBAGENTS/**` and model assignment are untouched.
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
- Render at the default 200 by 100 size and inspect the image as described in the rendered-image gate: a flat light-blue triangle with straight edges and a horizontal base sits on the right of the image, wholly in the sky above the green ground, and the centred sphere keeps its shaded normal colours. The triangle replaces sky pixels along its two slanted edges and its base, and nothing else in the image changes.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `src/plane.rs` holds `Plane` with a point and a unit normal and a hit that records the face side and (u, v) wrapped into `0..1`.
- [ ] `src/triangle.rs` holds `Triangle` with three vertices and a Möller-Trumbore hit that records the face side and barycentric (u, v).
- [ ] Both shapes build their record through `HitRecord::new`; no other construction site exists.
- [ ] `src/main.rs` declares both modules without a plan comment and the scene holds the one triangle; `the_center_pixel_shows_the_sphere_facing_the_camera` and `the_top_row_is_sky` still pass unedited.
- [ ] `Vec3::cross` has no `#[allow(dead_code)]` or plan comment and both new modules use it.
- [ ] The rendered image shows the triangle and the sphere, inspected as described above.
- [ ] Named tests pass and the test receipt reports 2 functions added, 0 deleted.
- [ ] Test budget respected (REPO-RULES §3a); any line counts are observations, not gates.
- [ ] Fast phase gate and the rendered-image conditional gate pass; no slow gate applies.
- [ ] `docs/` needed no change and `docs/requirements.md` items 8 and 9 are accurate; `README.md` is still accurate; the repository map lists both new modules and no page or comment refers to a plan.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the user test ran as the agent acceptance step above.
- [ ] On passing evidence: compress this file into `done/B2-planes-and-triangles.md` (frozen contract only) and flip the `full-plan.md` status row to `done`.

**Exit:** the program still renders its fixed scene, which now shows one triangle, and the codebase holds a working plane and a working triangle that record the face side and texture coordinates.
