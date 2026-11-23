## Phase B1 - Face side and texture coordinates

**Status:** active.
**Depends on:** none; this is the first phase of the geometry plan.
**Scope:** `HitRecord` in `src/hittable.rs` gains `front_face`, `u` and `v`, and one constructor on
`HitRecord` sets `normal` against the ray from a shape's outward normal and records the face side.
`Sphere` in `src/sphere.rs` uses that constructor and fills `(u, v)` from latitude and longitude.
Nothing else in `src/` changes; the normal-colouring in `src/main.rs` keeps producing the same image.
**Gate level:** standard - the phase changes the contents of the record every shape returns and the
meaning of `HitRecord::normal`, but the runtime path is bounded (`Sphere::hit` and the same render
loop), there is no new dependency, no configuration and no durable state.
**Simplicity statement:** no handling for a non-positive radius, a non-unit outward normal, a NaN
ray or a zero-length direction; such values are not produced by this repository and a wrong result
raises no special error. `(u, v)` is not clamped, because the two formulas below already land in
0..=1. No legacy path and no duplicate construction are added: `Sphere` stops building the record
field by field and calls the one constructor.

**Current state (verified):**
- `src/hittable.rs:6-14` - `pub struct HitRecord { pub t: f64, pub point: Vec3, pub normal: Vec3 }`
  with `#[derive(Clone, Copy, Debug)]`; `point` carries `#[allow(dead_code)]` and a comment above it.
  There are no other fields and no `impl` block on `HitRecord`.
- `src/hittable.rs:16-19` - `pub trait Hittable { fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>; }`.
- `src/hittable.rs:33-43` - `impl Hittable for HittableList` keeps the nearest hit by copying
  `HitRecord`; it reads only `hit.t` and never inspects the normal.
- `src/sphere.rs:19-42` - `fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>`
  solves the quadratic, then builds `HitRecord { t, point, normal: (point - self.center) / self.radius }`
  directly at `src/sphere.rs:37-41`.
- `src/sphere.rs:45-66` - a `#[cfg(test)] mod tests` with
  `a_ray_through_the_center_hits_the_near_surface` (asserts `hit.t == 1.5` and
  `hit.normal == Vec3::new(0.0, 0.0, 1.0)`) and `a_ray_beside_the_sphere_misses`.
- `src/main.rs:24-30` - `ray_colour` reads `hit.normal` only: `(hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5`.
- `src/main.rs:86-97` - `the_center_pixel_shows_the_sphere_facing_the_camera` asserts the centre
  pixel has `z > 0.9`; `the_top_row_is_sky` asserts `pixels[0].z > pixels[0].x`.
- `src/vec3.rs:54-59` - `impl Neg for Vec3` exists, so `-outward_normal` is available.
- `src/ray.rs:6-9` - `pub struct Ray { pub origin: Vec3, pub direction: Vec3 }`, `Copy`.
- `src/main.rs:32-37` - the scene holds two spheres, both hit from outside by camera rays.

**Read first (do not invent):**
- `src/hittable.rs` - the whole file; the record, the trait and the list are the only things this
  phase edits besides the sphere.
- `src/sphere.rs` - the whole file; the quadratic and the test module.
- `src/main.rs:23-30` and `src/main.rs:82-98` - the only reader of `normal` and the tests that pin
  the rendered colours.
- `src/ray.rs` and `src/vec3.rs` - `Ray::direction`, `Vec3::dot`, `Vec3::Neg`, `Vec3::PartialEq`.
- `docs/requirements.md:21-30` - items 7 and 8, the behavior this phase implements.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image conditional gate.

**Build plan:**

1. `src/hittable.rs`, `HitRecord` - add three public fields and keep the derive list unchanged.
   Before:

   ```rust
   /// Where a ray hit a surface: the distance along the ray, the point and the outward normal.
   #[derive(Clone, Copy, Debug)]
   pub struct HitRecord {
       pub t: f64,
       // The origin of the bounce rays the materials plan adds.
       #[allow(dead_code)]
       pub point: Vec3,
       pub normal: Vec3,
   }
   ```

   After:

   ```rust
   /// Where a ray hit a surface: the distance along the ray, the point, the normal against the
   /// ray, the face side and the texture coordinates.
   #[derive(Clone, Copy, Debug)]
   pub struct HitRecord {
       pub t: f64,
       // The origin of the bounce rays the materials plan adds.
       #[allow(dead_code)]
       pub point: Vec3,
       pub normal: Vec3,
       // Read by the tests only.
       #[allow(dead_code)]
       pub front_face: bool,
       #[allow(dead_code)]
       pub u: f64,
       #[allow(dead_code)]
       pub v: f64,
   }
   ```

   `normal` is now the unit normal that points against the hit ray; `front_face` is true when the
   ray hit the outside of the surface; `u` and `v` are texture coordinates in 0..=1. `front_face`,
   `u` and `v` carry `#[allow(dead_code)]` because only the tests read them today. Above those
   three attributes write one short present-tense comment stating that the tests read the fields,
   in the style of the existing `point` comment. Do not name a plan, a later phase or a future
   reader in it. Remove an attribute from a field only when a build without tests reads that field.

2. `src/hittable.rs`, new `impl HitRecord` block placed directly after the struct - one constructor
   computes `front_face` and `normal` together, so a shape cannot record one without the other:

   ```rust
   impl HitRecord {
       /// Return a hit at distance `t` and `point`, with texture coordinates `(u, v)` and a normal
       /// facing `ray`. `outward_normal` is unit length and points away from the surface.
       pub fn new(
           t: f64,
           point: Vec3,
           u: f64,
           v: f64,
           ray: &Ray,
           outward_normal: Vec3,
       ) -> Self {
           let front_face = ray.direction.dot(outward_normal) < 0.0;
           let normal = if front_face { outward_normal } else { -outward_normal };
           HitRecord {
               t,
               point,
               normal,
               front_face,
               u,
               v,
           }
       }
   }
   ```

   `Ray` is already imported at `src/hittable.rs:3`, `Vec3` at `src/hittable.rs:4`; add no import.

3. `src/sphere.rs`, `impl Hittable for Sphere::hit` - replace the record construction at
   `src/sphere.rs:36-41`. Before:

   ```rust
   let point = ray.at(t);
   Some(HitRecord {
       t,
       point,
       normal: (point - self.center) / self.radius,
   })
   ```

   After:

   ```rust
   let point = ray.at(t);
   let outward_normal = (point - self.center) / self.radius;
   let u = 0.5 + outward_normal.z.atan2(outward_normal.x) / (2.0 * PI);
   let v = 0.5 - outward_normal.y.asin() / PI;
   Some(HitRecord::new(t, point, u, v, ray, outward_normal))
   ```

   `u` is longitude: `0.5 + atan2(z, x) / (2 * PI)`, in 0..=1. `v` is latitude with 0 at the north
   pole (`+y`): `0.5 - asin(y) / PI`, in 0..=1. Add `use std::f64::consts::PI;` as a new line at
   the top of `src/sphere.rs` after the file header and before the `use crate::` lines. Leave the
   quadratic solve, the `t_min`/`t_max` comparisons and the miss path untouched.

4. `src/main.rs` - no edit. `ray_colour` keeps reading `hit.normal`, which is now the face-corrected
   normal. Every ray in the current scene hits a sphere from outside, so each `normal` keeps the
   value it has today and the rendered image is byte-identical.

5. `src/sphere.rs`, tests - adapt the existing test and add one:
   - `a_ray_through_the_center_hits_the_near_surface`: keep the existing `t` and `normal`
     assertions and add `assert!(hit.front_face);` plus `assert!((hit.u - 0.75).abs() < 1e-9);` and
     `assert!((hit.v - 0.5).abs() < 1e-9);` (the hit point is `(0, 0, -1.5)`, so the outward normal
     is `(0, 0, 1)`, `atan2(1.0, 0.0)` is `PI / 2` and `asin(0.0)` is zero).
   - New test `a_ray_from_inside_the_sphere_flips_the_normal`: `Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5)`
     and `Ray::new(Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 1.0))`; assert `!hit.front_face`
     and `hit.normal == Vec3::new(0.0, 0.0, -1.0)`.
   - Do not add further test functions. `a_ray_beside_the_sphere_misses` is unchanged.

6. Docs - no page changes. `docs/requirements.md` items 7 and 8 already describe this behavior and
   stay exactly as written. `README.md` makes no claim about the hit record and stays as written.

**Callers / wiring to update:**
- `src/sphere.rs:37` (record construction inside `Sphere::hit`) - builds `HitRecord` field by field
  today; replaces that with `HitRecord::new(t, point, u, v, ray, outward_normal)`.
- `src/main.rs:26` (`ray_colour`) - reads `hit.normal`; compiles and behaves as today, no edit.
- `src/hittable.rs:33-43` (`impl Hittable for HittableList`) - copies `HitRecord` and reads only
  `hit.t`; the added fields copy with the record, no edit.
- No other file constructs or destructures `HitRecord`: a repository-wide search finds
  `HitRecord` only in `src/hittable.rs`, `src/sphere.rs` and `src/main.rs` (as `hit.normal`).

**Config / schema / migrations:**
- None. No configuration key, no schema, no migration and no generated file.
- Retained records and their named consumer/owner: none added. The new fields live in the existing
  in-memory `HitRecord` that `Sphere::hit` returns and `ray_colour` consumes for the pixel colour.

**Touched files:**
<!-- touched-files:start -->
- `src/hittable.rs` - changed: `HitRecord` gains `front_face`, `u`, `v`; new `HitRecord::new` constructor sets the normal against the ray and the face side.
- `src/sphere.rs` - changed: computes the outward normal plus `(u, v)` and calls `HitRecord::new`; adds `std::f64::consts::PI`; adapts one test and adds one back-face test.
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add a dependency or touch `Cargo.toml` (hard invariant 1).
- Do not add planes, triangles, bounding boxes, materials, scattering, a PNG writer, a command line
  or progress output; those belong to later phases and other plans.
- Do not edit `src/main.rs`, `src/vec3.rs`, `src/ray.rs`, `src/camera.rs` or `src/rng.rs`.
- Do not add a second way to build a `HitRecord`. `Sphere::hit` calls `HitRecord::new` and nothing
  constructs the record with a struct literal.
- Do not change the `Hittable` trait signature, the `HittableList` loop, the sphere quadratic, the
  `t_min`/`t_max` comparisons or the miss path.
- Do not clamp, wrap or branch on `(u, v)`; the formulas already land in 0..=1.
- Do not add guards for a non-positive radius, a non-unit outward normal, a NaN value or a
  zero-length ray direction.
- Do not keep more `#[allow(dead_code)]` attributes than the unused fields require, and do not
  remove the existing one on `point`.
- Do not add a comment that names a later phase, a plan or a future reader; keep every comment a
  present-tense fact about the code.
- Do not add more than one new test function; adapt the existing sphere tests first
  (REPO-RULES §3a). Do not delete or weaken `a_ray_beside_the_sphere_misses`.
- Do not add `unsafe`, threads or any seeded randomness; this phase reads none.

**Tests:**
- Changed: `src/sphere.rs` - `a_ray_through_the_center_hits_the_near_surface`: also asserts
  `front_face` is true and `(u, v)` equal 0.75 and 0.5 at the near surface.
- New: `src/sphere.rs` - `a_ray_from_inside_the_sphere_flips_the_normal`: a ray leaving the sphere's
  centre records `front_face == false` and the negated outward normal. (One new test function, in
  budget; no other test file changes.)
- Fast gate:
  - `cargo test sphere`
  - `cargo test`
  - `cargo build`
  - `cargo clippy -- -D warnings`
  - `cargo fmt --check`
  - `git diff --check`
- Conditional gates:
  - Rendered image: required. The phase changes the normal every shape records, which sets every
    hit pixel's colour, so the plan's "normal-colouring keeps working" is proven by one render.
    Run once by the implementer:
    `cargo run --release > /tmp/opencode/render.ppm`, then confirm the header lines are `P3`,
    `200 100` and `255`, and read the centre pixel with
    `sed -n '10104p' /tmp/opencode/render.ppm`, which is row 50 and column 100 (the header takes
    the first three lines). It reads `129 126 255` before and after this phase: the sphere's
    outward normal at that hit is `(0, 0, 1)`, its face side is the front, so the corrected normal
    equals the outward one. A flipped normal would print a dark blue-dominant value such as
    `0 0 0`; a wrong record build would fail the render.
  - Agent generation: not run; no file under `AGENT-INSTRUCTIONS/SUBAGENTS/` changes.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - automated phase gates prove acceptance: `cargo test` covers the face side, the
  flipped back-face normal, the texture coordinates and the centre pixel colour, and the rendered
  image gate above covers the same colours from the real render loop.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `HitRecord` has exactly the fields `t`, `point`, `normal`, `front_face`, `u`, `v`, and
  `normal` points against the ray for both a front-face and a back-face hit.
- [ ] `Sphere::hit` sets `(u, v)` from the two formulas in step 3, and both values lie in 0..=1 for
  every hit the tests exercise.
- [ ] `HitRecord::new` is the only construction of a `HitRecord`.
- [ ] `src/main.rs` is unchanged and `a_ray_through_the_center_hits_the_near_surface` still finds
  the outward normal at the near surface.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a): one new test function, one adapted; the receipt
  reports functions added and deleted.
- [ ] Fast phase gate and the rendered-image conditional gate from
  `AGENT-INSTRUCTIONS/BUILDING/VERIFICATION-COMMITS-DEPLOY.md` §1 pass; no slow gate.
- [ ] `docs/requirements.md` items 7 and 8 and `README.md` are checked and need no edit; no page
  refers to a plan.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists.
- [ ] On passing evidence: compress this file into `done/B1-face-side-and-texture-coordinates.md`
  (frozen contract only: the `HitRecord` fields, the constructor signature, the `(u, v)` formulas)
  and flip the `full-plan.md` status row to `done`.

**Exit:** every sphere hit carries a unit normal that points against the ray, the face side, and
`(u, v)` in 0..=1 from latitude and longitude; the tests, the fast gate and the rendered image pass,
and the rendered image is the same picture as before this phase.
