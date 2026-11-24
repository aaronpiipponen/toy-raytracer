## Phase B2 - Planes and triangles

**Status:** active.
**Depends on:** B1r (`done/B1r-shared-hit-record-shape.md`): the six-argument
`HitRecord::new(ray, t, point, outward_normal, u, v)` that sets the face side and turns the normal
against the ray, and the rule that a shape records (u, v) without clamping.
**Scope:** add an infinite plane and a triangle as `Hittable` shapes, each with texture coordinates,
and put both in the scene. The scene's second sphere, the large ground sphere at
`(0.0, -100.5, -1.0)`, becomes the ground plane at `y = -0.5` with normal `(0.0, 1.0, 0.0)`, whose
top surface is the sphere's visible surface. A triangle stands behind the centre sphere. Every hit
goes through `HitRecord::new`. No bounding box, no material, no render-loop, camera, output or
command-line change.
**Gate level:** standard - two new shape modules on the existing `Hittable` seam plus a scene change
that alters the rendered image, with no durable state, trust boundary or migration.
**Simplicity statement:** a plane and a triangle are two-sided surfaces: the constructor records the
face side and flips the normal, and no shape carries a one-sided switch. The triangle rejects a ray
parallel to its plane (determinant near zero) and a ray outside its barycentric bounds. A degenerate
triangle therefore never hits. The plane rejects a ray parallel to it. Nothing clamps or wraps (u, v)
beyond the plane's `rem_euclid(1.0)`, nothing checks a zero-length plane normal or normalises a
degenerate triangle normal, and no bounding box, material, jitter or second hit path is added. The
phase deletes the `#[allow(dead_code)]` on `Vec3::cross`, which the triangle makes reachable.

**Current state (verified):**
- `src/hittable.rs:9-21` - `#[derive(Clone, Copy, Debug)] pub struct HitRecord { pub t: f64, pub point: Vec3, pub normal: Vec3, pub front_face: bool, pub u: f64, pub v: f64 }`, with `point`, `front_face`, `u` and `v` under `#[allow(dead_code)]`.
- `src/hittable.rs:23-42` - `impl HitRecord { pub fn new(ray: &Ray, t: f64, point: Vec3, outward_normal: Vec3, u: f64, v: f64) -> Self }` sets `front_face = ray.direction.dot(outward_normal) < 0.0`, flips the normal, and is the only construction path.
- `src/hittable.rs:44-47` - `pub trait Hittable { fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>; }`; it has no `bounding_box` method (B3 adds it).
- `src/hittable.rs:49-72` - `HittableList` with `pub objects: Vec<Box<dyn Hittable>>`, `add(&mut self, object: Box<dyn Hittable>)`, and `hit` that keeps the nearest record; it needs no edit.
- `src/vec3.rs:21-29` - `Vec3::cross(self, other: Vec3) -> Vec3` carries `// For the triangle normals.` and `#[allow(dead_code)]`; nothing calls it yet.
- `src/vec3.rs:31-37` - `length(self) -> f64` and `unit(self) -> Vec3`; `src/vec3.rs:40-83` - `Sub`, `Neg`, `Mul<f64>`, `mul` by vector and `Div<f64>`.
- `src/vec3.rs:5` - `pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }` with `PartialEq`.
- `src/ray.rs:5-19` - `pub struct Ray { pub origin: Vec3, pub direction: Vec3 }`, `new` and `at(self, t: f64) -> Vec3`; the direction is not normalised.
- `src/sphere.rs:20-47` - `impl Hittable for Sphere::hit` uses `HitRecord::new`; it is the model for the new shapes.
- `src/main.rs:3-10` - module list `camera, hittable, ray, rng, sphere, vec3`; `src/main.rs:14-18` - `use camera::Camera; use hittable::{Hittable, HittableList}; use ray::Ray; use sphere::Sphere; use vec3::{Colour, Vec3};`.
- `src/main.rs:32-37` - `fn scene() -> HittableList` adds `Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)` and `Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)`; the second sphere's top is `y = -0.5`.
- `src/main.rs:23-30` - `ray_colour` reads `hit.normal` and needs no edit.
- `src/main.rs:82-98` - tests `the_center_pixel_shows_the_sphere_facing_the_camera` and `the_top_row_is_sky` both call `render(200, 100)`; the centre ray still hits the centre sphere first and the top row still misses every object.
- `docs/requirements.md:23-30` - items 7 to 10; item 9 states the scene holds infinite planes and triangles, and nothing becomes wrong when this phase makes it true.
- Cargo and its tools are not on `PATH`; call `~/.cargo/bin/cargo`.

**Read first (do not invent):**
- `src/hittable.rs` - the whole file: the record, `HitRecord::new`, the `Hittable` trait and `HittableList`.
- `src/sphere.rs` - the whole file: the shape implementation this phase copies in structure and the existing test style.
- `src/vec3.rs` - the whole file: `cross`, `dot`, `unit`, `length` and the operators the formulas use.
- `src/ray.rs` - `Ray::at` and the fact that `direction` is not normalised.
- `src/main.rs` - the module list, imports, `scene` and both render tests.
- `docs/requirements.md` items 7 to 10 - the behaviour this phase makes real; item 10 (bounding boxes) is B3.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image command.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B1.md` - the merge note that the ground plane and the triangle get materials at the release merge; no material exists on this branch.

**Build plan:**
1. `src/plane.rs` - create the module with exactly this content (the tests come from the `Tests`
   section and sit in a `#[cfg(test)] mod tests` below the implementation):

   ```rust
   //! An infinite plane.

   use crate::hittable::{HitRecord, Hittable};
   use crate::ray::Ray;
   use crate::vec3::Vec3;

   /// An infinite plane through `point` with the unit `normal`.
   pub struct Plane {
       pub point: Vec3,
       pub normal: Vec3,
   }

   impl Plane {
       /// A plane through `point` with `normal` scaled to unit length.
       pub fn new(point: Vec3, normal: Vec3) -> Self {
           Plane {
               point,
               normal: normal.unit(),
           }
       }
   }

   impl Hittable for Plane {
       fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
           let denominator = self.normal.dot(ray.direction);
           if denominator.abs() < 1e-9 {
               return None;
           }
           let t = (self.point - ray.origin).dot(self.normal) / denominator;
           if t <= t_min || t >= t_max {
               return None;
           }
           let point = ray.at(t);
           let (u, v) = if self.normal.x.abs() >= self.normal.y.abs()
               && self.normal.x.abs() >= self.normal.z.abs()
           {
               (point.y, point.z)
           } else if self.normal.y.abs() >= self.normal.z.abs() {
               (point.x, point.z)
           } else {
               (point.x, point.y)
           };
           Some(HitRecord::new(
               ray,
               t,
               point,
               self.normal,
               u.rem_euclid(1.0),
               v.rem_euclid(1.0),
           ))
       }
   }
   ```

   The two in-plane axes are the pair the dominant normal component does not use, so a ground plane
   maps the hit position's `x` and `z` and a vertical plane maps another valid pair. `rem_euclid`
   gives (u, v) in `0..1` for negative as well as positive coordinates.

2. `src/triangle.rs` - create the module with exactly this content (tests from the `Tests`
   section):

   ```rust
   //! A triangle.

   use crate::hittable::{HitRecord, Hittable};
   use crate::ray::Ray;
   use crate::vec3::Vec3;

   /// A triangle with the vertices `a`, `b` and `c` in counter-clockwise order seen from outside.
   pub struct Triangle {
       pub a: Vec3,
       pub b: Vec3,
       pub c: Vec3,
   }

   impl Triangle {
       /// A triangle through the three vertices.
       pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
           Triangle { a, b, c }
       }
   }

   impl Hittable for Triangle {
       fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
           let edge1 = self.b - self.a;
           let edge2 = self.c - self.a;
           let h = ray.direction.cross(edge2);
           let determinant = edge1.dot(h);
           if determinant.abs() < 1e-8 {
               return None;
           }
           let inverse_determinant = 1.0 / determinant;
           let s = ray.origin - self.a;
           let u = inverse_determinant * s.dot(h);
           if !(0.0..=1.0).contains(&u) {
               return None;
           }
           let q = s.cross(edge1);
           let v = inverse_determinant * ray.direction.dot(q);
           if v < 0.0 || u + v > 1.0 {
               return None;
           }
           let t = inverse_determinant * edge2.dot(q);
           if t <= t_min || t >= t_max {
               return None;
           }
           let outward_normal = edge1.cross(edge2).unit();
           Some(HitRecord::new(ray, t, ray.at(t), outward_normal, u, v))
       }
   }
   ```

   This is Möller-Trumbore. `u` weights the edge to `b` and `v` the edge to `c`, and both are
   recorded unchanged as the barycentric texture coordinates. A back-face hit gets the same `u`, `v`
   and the flipped normal from `HitRecord::new`, so the triangle is two-sided.

3. `src/vec3.rs`, `Vec3::cross` - delete the `// For the triangle normals.` comment line and the
   `#[allow(dead_code)]` attribute directly above `pub fn cross`; the triangle calls it, so it is
   used. Change nothing else in the file.

4. `src/main.rs`, module list - add `mod plane;` after `mod hittable;` and `mod triangle;` after
   `mod sphere;`, keeping the list alphabetical.

5. `src/main.rs`, imports - add `use plane::Plane;` after the `hittable` import and
   `use triangle::Triangle;` after the `sphere` import.

6. `src/main.rs`, `scene` - keep the first sphere line and replace the ground sphere line, then add
   the triangle, so the function ends as:

   ```rust
   fn scene() -> HittableList {
       let mut world = HittableList::default();
       world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
       world.add(Box::new(Plane::new(
           Vec3::new(0.0, -0.5, 0.0),
           Vec3::new(0.0, 1.0, 0.0),
       )));
       world.add(Box::new(Triangle::new(
           Vec3::new(-2.0, -0.5, -2.0),
           Vec3::new(2.0, -0.5, -2.0),
           Vec3::new(0.0, 1.5, -2.0),
       )));
       world
   }
   ```

   The triangle's base lies on the ground plane and its apex projects above the centre sphere, so
   both visible sides and the apex show around the sphere rather than behind it. Its normal is
   `(0.0, 0.0, 1.0)`, which the normal colouring renders in a colour distinct from the plane's
   `(0.5, 1.0, 0.5)` and the sky.

7. `docs/` - no page changes. `docs/requirements.md` item 9 already states that the scene holds
   planes and triangles and nothing it says becomes wrong. `README.md` states only that the program
   renders one fixed scene. No module list appears in a page.

**Callers / wiring to update:**
- `src/main.rs:3-10` and `src/main.rs:14-18` - the two new modules and their imports, from steps 4 and 5.
- `src/main.rs:32-37` - `scene` gains the plane and the triangle, from step 6.
- `src/main.rs:23-30` - `ray_colour` reads only `hit.normal` and needs no edit; it now also colours the plane and the triangle.
- `src/hittable.rs:49-72` - `HittableList` stores `Box<dyn Hittable>` and copies records; no edit.
- No other file builds a `HitRecord` or stores a `Hittable` (grep over `src/` finds `HitRecord::new` in `src/sphere.rs` only).

**Config / schema / migrations:**
- None: no configuration key, migration, generated file or persisted record.
- Retained records and their named consumer/owner: none; `HitRecord` stays an in-memory value.

**Touched files:**
<!-- touched-files:start -->
- `src/plane.rs` - created: `Plane` with `new` normalising the normal and `hit` recording the face side and position-based (u, v) modulo 1
- `src/triangle.rs` - created: `Triangle` with `new` and Möller-Trumbore `hit` recording barycentric (u, v)
- `src/main.rs` - changed: module list and imports gain `plane` and `triangle`; `scene` uses the ground plane and adds the triangle
- `src/vec3.rs` - changed: `Vec3::cross` loses its dead-code allowance and purpose comment because the triangle calls it
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add a dependency to `Cargo.toml`, do not use `unsafe`, and keep rendering single-threaded (domain invariants 1 and 3).
- Do not add a second way to build a `HitRecord`: both shapes call `HitRecord::new` and no struct literal or face-side helper appears outside it.
- Do not change `HitRecord`, its constructor, the `Hittable` trait, `HittableList`, `ray_colour`, `render`, the camera or the output. Bounding boxes are B3.
- Do not give either shape a material field or an `Rc`; materials are the release merge's addition.
- Do not change the centre sphere, and do not add a fourth scene object.
- Do not clamp, wrap or normalise (u, v) beyond the plane's `rem_euclid(1.0)`, do not add a one-sided-triangle switch, and do not handle a degenerate triangle or a zero-length plane normal beyond the two determinant checks above.
- Do not change, rename or delete an existing test. Add only the three test functions below.
- Do not touch `docs/`, `README.md`, `Cargo.toml`, `AGENT-INSTRUCTIONS/LOCAL/` or another plan's files.

**Tests:**
- New (3 functions, the phase maximum):
  - `src/plane.rs` - `a_ray_hits_the_plane_and_its_texture_coordinates_wrap`:

    ```rust
    #[test]
    fn a_ray_hits_the_plane_and_its_texture_coordinates_wrap() {
        let plane = Plane::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let ray = Ray::new(Vec3::new(1.25, 0.0, -0.25), Vec3::new(0.0, -1.0, 0.0));
        let hit = plane
            .hit(&ray, 0.001, f64::INFINITY)
            .expect("the ray hits");
        assert!((hit.t - 0.5).abs() < 1e-9);
        assert!(hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 1.0, 0.0));
        assert!((hit.u - 0.25).abs() < 1e-9);
        assert!((hit.v - 0.75).abs() < 1e-9);
    }
    ```

    The hit point is `(1.25, -0.5, -0.25)`, so the plane's `x` and `z` axes give `u = 1.25 mod 1 = 0.25` and `v = -0.25 mod 1 = 0.75`.
  - `src/triangle.rs` - `a_ray_through_the_triangle_hits_it_with_barycentric_coordinates`:

    ```rust
    #[test]
    fn a_ray_through_the_triangle_hits_it_with_barycentric_coordinates() {
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, -2.0),
            Vec3::new(1.0, 0.0, -2.0),
            Vec3::new(0.0, 1.0, -2.0),
        );
        let ray = Ray::new(Vec3::new(0.25, 0.25, -1.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = triangle
            .hit(&ray, 0.001, f64::INFINITY)
            .expect("the ray hits");
        assert!((hit.t - 1.0).abs() < 1e-9);
        assert!(hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
        assert!((hit.u - 0.25).abs() < 1e-9);
        assert!((hit.v - 0.25).abs() < 1e-9);
    }
    ```

  - `src/triangle.rs` - `a_ray_beside_the_triangle_misses`:

    ```rust
    #[test]
    fn a_ray_beside_the_triangle_misses() {
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, -2.0),
            Vec3::new(1.0, 0.0, -2.0),
            Vec3::new(0.0, 1.0, -2.0),
        );
        let ray = Ray::new(Vec3::new(2.0, 2.0, -1.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(triangle.hit(&ray, 0.001, f64::INFINITY).is_none());
    }
    ```
- Changed: none. `src/main.rs`'s two render tests stay as they are and pass unchanged.
- Deleted: none. Functions added 3, changed 0, deleted 0.
- Fast gate, from the worktree root: `~/.cargo/bin/cargo test`, `~/.cargo/bin/cargo build`, `~/.cargo/bin/cargo clippy -- -D warnings`, `~/.cargo/bin/cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image, required because the scene changes and the triangle must be visible. Render `~/.cargo/bin/cargo run --release > /tmp/opencode/b2.ppm` and inspect it with the numbered acceptance steps below. Agent generation: not affected. Bounding-box and material gates: not applicable.
- Slow gate (>2 minutes): none; the 200 by 100 single-ray render finishes in well under a second in release mode.

**User test (acceptance):**
- **Executor:** agent.
1. From the worktree root run `~/.cargo/bin/cargo run --release > /tmp/opencode/b2.ppm` and expect exit status 0.
2. Write this scratch converter to `/tmp/opencode/ppm2png.py`. It is verification tooling, not product code:

   ```python
   import struct, sys, zlib

   def chunk(kind, data):
       body = kind + data
       return struct.pack('>I', len(data)) + body + struct.pack('>I', zlib.crc32(body))

   src = open(sys.argv[1]).read().split()
   assert src[0] == 'P3'
   w, h = int(src[1]), int(src[2])
   values = list(map(int, src[4:]))
   rows = b''
   for y in range(h):
       rows += b'\x00' + bytes(values[y * w * 3:(y + 1) * w * 3])
   png = (b'\x89PNG\r\n\x1a\n'
          + chunk(b'IHDR', struct.pack('>IIBBBBB', w, h, 8, 2, 0, 0, 0))
          + chunk(b'IDAT', zlib.compress(rows))
          + chunk(b'IEND', b''))
   open(sys.argv[2], 'wb').write(png)
   ```

3. Run `python3 /tmp/opencode/ppm2png.py /tmp/opencode/b2.ppm /tmp/opencode/b2.png` and open
   `/tmp/opencode/b2.png` with the image reader.
4. Expect a 200 by 100 image with the sky gradient at the top, the normal-coloured centre sphere, the
   ground plane filling the bottom half in a single flat green, and a triangle standing behind the
   sphere: its left and right slopes show beside the sphere and its apex shows above it. The picture
   has the same shape as `render`'s current output apart from the ground plane replacing the ground
   sphere and the triangle appearing.
5. If the triangle is absent or a shape is wrong, stop and report instead of ticking the checklist.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `Plane` and `Triangle` implement `Hittable`, each builds its record only through `HitRecord::new`, and neither carries a material.
- [ ] The plane maps the hit position to (u, v) modulo 1 on the two axes its normal does not use, and the triangle records Möller-Trumbore barycentric (u, v).
- [ ] A back-face hit still records the face side and a normal that faces the ray, through the shared constructor.
- [ ] `scene` holds the centre sphere, the ground plane and the triangle, and `Vec3::cross` has no dead-code allowance or purpose comment.
- [ ] The two existing `src/main.rs` render tests pass without an edit.
- [ ] The rendered PPM shows the sky, the centre sphere, the flat ground plane and a visible triangle, checked with the numbered acceptance steps.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a); receipt reports 3 functions added, 0 changed, 0 deleted. Any line counts are observations, not gates.
- [ ] Fast phase gate and the rendered-image conditional gate from `AGENT-INSTRUCTIONS/LOCAL/gates.md` pass; no slow gate applies.
- [ ] No `docs/` page needed a change; `docs/requirements.md` item 9 is accurate and no page refers to a plan.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the agent-run acceptance above supplies its evidence.
- [ ] On passing evidence: compress this file into `done/B2-planes-and-triangles.md` (frozen contract only) and flip the `full-plan.md` status row to `done`.

**Exit:** the scene holds an infinite ground plane and a triangle beside the centre sphere, every hit
records the face side and (u, v) through `HitRecord::new`, the rendered image shows the triangle, and
all gates are green.
