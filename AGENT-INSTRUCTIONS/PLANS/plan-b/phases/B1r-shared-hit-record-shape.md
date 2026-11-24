## Phase B1r - Shared hit record shape

**Status:** active.
**Depends on:** none; this is the starting phase at commit `66097bd`.
**Scope:** make `HitRecord` and `Sphere::hit` match the shape settled with the materials plan in
`AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B1.md`. The record gains `front_face`, `u` and `v`,
its one constructor takes the ray and the outward normal and turns the normal against the ray, and
`Sphere::hit` fills (u, v) from the outward normal's latitude and longitude. On this branch the
record keeps its current derives, has no material and the constructor takes six arguments; the
material is the release merge's addition. No new shape, no bounds, no material, no output or
command-line change.
**Gate level:** standard - a bounded change to one shared record and one shape whose normal-colouring
output must stay identical; no data, credential or cross-package seam is touched.
**Simplicity statement:** the phase handles only spheres with a positive radius, as the existing code
already requires. It deliberately does not clamp (u, v), does not normalise a zero-length normal,
does not add a second way to build a `HitRecord`, and does not handle planes, triangles, materials,
bounding boxes or the render loop. No legacy or defensive code is added or deleted.

**Current state (verified):**
- `src/hittable.rs:7-14` - `#[derive(Clone, Copy, Debug)] pub struct HitRecord { pub t: f64, pub point: Vec3, pub normal: Vec3 }`; `point` carries `#[allow(dead_code)]` because nothing outside tests reads it; `normal` holds the **outward** normal written by the shape.
- `src/hittable.rs:16-19` - `pub trait Hittable { fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>; }` (unchanged by this phase).
- `src/hittable.rs:33-44` - `impl Hittable for HittableList` copies whole `HitRecord` values; it needs no edit when the struct grows.
- `src/sphere.rs:18-43` - `impl Hittable for Sphere`, `hit` returns `Some(HitRecord { t, point, normal: (point - self.center) / self.radius })`; the only construction site of `HitRecord` in the program.
- `src/sphere.rs:45-66` - `mod tests` with `a_ray_through_the_center_hits_the_near_surface` (asserts `hit.t == 1.5` and `hit.normal == Vec3::new(0.0, 0.0, 1.0)`) and `a_ray_beside_the_sphere_misses`.
- `src/main.rs:23-30` - `ray_colour` reads `hit.normal` for the normal-colouring `(hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5`; it works unchanged once `normal` always faces the ray.
- `src/main.rs:86-97` - tests `the_center_pixel_shows_the_sphere_facing_the_camera` and `the_top_row_is_sky` call `render`, which goes through `ray_colour`.
- `src/ray.rs:5-19` - `pub struct Ray { pub origin: Vec3, pub direction: Vec3 }`, `Ray::at`.
- `src/vec3.rs:17-19,54-66` - `Vec3::dot(self, other: Vec3) -> f64` and the `Neg` and `Mul<f64>` operators the face-side and texture formulas use.
- `src/main.rs:32-37` - `scene()` holds a unit-ish sphere at `(0.0, 0.0, -1.0)` and a large sphere at `(0.0, -100.5, -1.0)`; the camera at the origin is outside both, so every current hit is a front-face hit and the rendered image must not change.
- `docs/requirements.md:21-27` - items 7 and 8 already state the face-side and (u, v) behaviour this phase makes real.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B1.md` - the target record, the one construction
  path and plan-b's part in it; the merge shape and the absence of a material on this branch.
- `src/hittable.rs` - whole file; the record and its doc comment are the contract later shapes use.
- `src/sphere.rs` - whole file; the `hit` body and the two existing tests.
- `src/main.rs` - `ray_colour` and `render`; confirm the only `HitRecord` read outside `sphere.rs`.
- `src/vec3.rs` - `dot`, `Neg`, `Mul<f64>`, `Sub`, `Div<f64>` operators used by the steps below.
- `src/ray.rs` - `Ray.direction` is not normalised and may be any length; the face side uses the sign of the dot product only.
- `docs/requirements.md` items 7 and 8 - the target behaviour this phase makes real.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - fast gate and the rendered-image gate commands.

**Build plan:**
1. `src/hittable.rs`, `HitRecord` - before -> after:

   ```rust
   // before
   #[derive(Clone, Copy, Debug)]
   pub struct HitRecord {
       pub t: f64,
       // The origin of the bounce rays the materials plan adds.
       #[allow(dead_code)]
       pub point: Vec3,
       pub normal: Vec3,
   }

   // after
   #[derive(Clone, Copy, Debug)]
   pub struct HitRecord {
       pub t: f64,
       // The origin of the bounce rays.
       #[allow(dead_code)]
       pub point: Vec3,
       pub normal: Vec3,
       #[allow(dead_code)]
       pub front_face: bool,
       #[allow(dead_code)]
       pub u: f64,
       #[allow(dead_code)]
       pub v: f64,
   }
   ```

   Keep the `Clone, Copy, Debug` derives: this branch has no `Rc` field, and the material field that
   removes `Copy` arrives with the release merge. Replace the `///` doc comment above the struct
   (currently `Where a ray hit a surface: the distance along the ray, the point and the outward
   normal.`) with a one-line description stating that `normal` always faces against the ray and that
   the record carries the face side and texture coordinates. Reword the existing `point` comment from
   `// The origin of the bounce rays the materials plan adds.` to `// The origin of the bounce rays.`
   so no code comment refers to a plan; do not otherwise reword existing code. The three new fields
   carry `#[allow(dead_code)]` for the same reason `point` does: nothing outside `#[cfg(test)]` code
   reads them until later phases, and `cargo clippy -- -D warnings` must stay clean.

2. `src/hittable.rs`, add `impl HitRecord` directly below the struct with exactly this associated
   function and no other constructor:

   ```rust
   impl HitRecord {
       /// A hit record whose normal faces against the ray, with the face side and texture coordinates.
       pub fn new(ray: &Ray, t: f64, point: Vec3, outward_normal: Vec3, u: f64, v: f64) -> Self {
           let front_face = ray.direction.dot(outward_normal) < 0.0;
           let normal = if front_face { outward_normal } else { -outward_normal };
           HitRecord { t, point, normal, front_face, u, v }
       }
   }
   ```

   `Ray` is already imported at `src/hittable.rs:3`. This is the one place a record is built on this
   branch; every shape calls it so the face side is recorded the same way everywhere. The constructor
   takes six arguments here because no material exists on this branch; the release merge adds the
   object's material as the final argument.

3. `src/sphere.rs`, `impl Hittable for Sphere::hit` - keep the discriminant/root/t selection lines
   unchanged; replace only the final block:

   ```rust
   // before
   let point = ray.at(t);
   Some(HitRecord {
       t,
       point,
       normal: (point - self.center) / self.radius,
   })

   // after
   let point = ray.at(t);
   let outward_normal = (point - self.center) / self.radius;
   let theta = (-outward_normal.y).acos();
   let phi = (-outward_normal.z).atan2(outward_normal.x) + PI;
   let u = phi / (2.0 * PI);
   let v = theta / PI;
   Some(HitRecord::new(ray, t, point, outward_normal, u, v))
   ```

   Add `use std::f64::consts::PI;` next to the existing `use` lines at the top of `src/sphere.rs`
   (after the `crate::` imports). The (u, v) values come from the outward normal, never from the
   ray-facing `normal`, so a back-face hit still gets the same texture coordinates as the same point
   seen from outside.

4. `src/sphere.rs`, `mod tests` - adapt and add exactly the tests named in `Tests` below; do not
   change `a_ray_beside_the_sphere_misses`.

5. `docs/` - no page changes. `docs/requirements.md` items 7 and 8 already state the target
   behaviour, nothing it says becomes wrong, and no module is added, so `README.md` and the
   repository map stay accurate.

**Callers / wiring to update:**
- `src/main.rs:26` - reads `hit.normal` only; no edit. It now receives a ray-facing normal, which is
  what the existing normal-colouring already assumes.
- `src/hittable.rs:34-43` - `HittableList::hit` copies `HitRecord` values; no edit beyond compiling
  against the larger struct.
- No other file reads or builds `HitRecord` (verified by grep across `src/` and `README.md`).

**Config / schema / migrations:**
- None: no configuration key, migration, generated file or persisted record.
- Retained records and their named consumer/owner: none; `HitRecord` is an in-memory value, not a
  stored record.

**Touched files:**
<!-- touched-files:start -->
- `src/hittable.rs` - changed: `HitRecord` gains `front_face`, `u`, `v`; `HitRecord::new` sets the face side and turns the normal against the ray
- `src/sphere.rs` - changed: `Sphere::hit` fills (u, v) from latitude and longitude and builds the record through `HitRecord::new`; its tests gain a back-face and a texture-coordinate case
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add any dependency to `Cargo.toml` and do not use `unsafe` (domain invariants 1 and 3).
- Do not add a second way to build a `HitRecord`: no struct literal outside `HitRecord::new` and no
  `set_face_normal` method alongside the constructor.
- Do not add a material field or `Rc` on this branch, and keep the constructor at six arguments; the
  release merge adds the object's material as the final argument.
- Do not change the `Hittable` trait, `HittableList`, `ray_colour`, `render`, the scene, or the
  output; the rendered image must stay byte-identical.
- Do not clamp, wrap or normalise `u` and `v` beyond the formula above, and do not handle a zero
  radius or a degenerate ray direction.
- Do not add fields beyond `front_face`, `u`, `v`, and do not add plan or phase references in code
  comments.
- Do not touch `docs/`, `README.md`, `Cargo.toml` or another plan's files.

**Tests:**
- New (2 functions) in `src/sphere.rs` `mod tests`:
  - `a_ray_from_inside_the_sphere_hits_the_back_face`: sphere `Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5)`, ray `Ray::new(Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 1.0))`; the hit is `Some`, `t` is about `0.5`, `front_face` is `false`, and `normal == Vec3::new(0.0, 0.0, -1.0)` (points against the ray).
  - `the_near_surface_has_texture_coordinates_from_latitude_and_longitude`: sphere `Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5)`, ray `Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0))`; the hit's `u` is about `0.25` and `v` is about `0.5`, each within `1e-9`.
- Changed (1 existing function, no new function): `src/sphere.rs`
  `a_ray_through_the_center_hits_the_near_surface` - keep its two assertions and add
  `assert!(hit.front_face)`.
- Deleted: none.
- Fast gate: `cargo test`, then `cargo build`, `cargo clippy -- -D warnings`, `cargo fmt --check`,
  `git diff --check`.
- Conditional gates: rendered image is required even though the camera, materials and output format
  are untouched, because `ray_colour` consumes `hit.normal` and the image must stay unchanged: from
  the base commit `66097bd`, render `cargo run --release > /tmp/opencode/render-base.ppm` before
  editing and `cargo run --release > /tmp/opencode/render-b1r.ppm` after, then require
  `cmp /tmp/opencode/render-base.ppm /tmp/opencode/render-b1r.ppm` to report no difference.
  Agent-generation gate: none.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - automated phase gates prove acceptance: the named tests prove the face side and
  (u, v), and the byte-identical PPM comparison proves the normal colouring still works.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `HitRecord` keeps its `Clone, Copy, Debug` derives, has `front_face`, `u`, `v`, its `normal`
  always points against the ray, and the only constructor is the six-argument `HitRecord::new`.
- [ ] The branch carries no material field and no `Rc`, matching the shared design for this branch.
- [ ] `Sphere::hit` fills (u, v) from the outward normal's latitude and longitude, so a front-face
  and a back-face hit of the same point give the same (u, v).
- [ ] `src/main.rs` compiles and its two render tests pass with no edit to the file.
- [ ] The post-change PPM is byte-identical to the base-commit PPM (`cmp` prints nothing).
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a); receipt reports 2 functions added, 1 changed, 0
  deleted. Any line counts are observations, not gates.
- [ ] Fast phase gate and the rendered-image conditional gate from `AGENT-INSTRUCTIONS/LOCAL/gates.md`
  pass; no slow gate applies.
- [ ] No `docs/` page needed a change; `docs/requirements.md` items 7 and 8 are accurate and no page
  refers to a plan.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; no separate user test was required.
- [ ] On passing evidence: compress this file into `done/B1r-shared-hit-record-shape.md` (frozen
  contract only) and flip the `full-plan.md` status row to `done`.

**Exit:** plan-b's branch matches the shared design: every hit records `front_face` and (u, v), the
normal always faces the ray, the constructor takes six arguments without a material, the rendered
image is unchanged and all gates are green.
