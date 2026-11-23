## Phase B1r - Face side and texture coordinates

**Status:** active.
**Depends on:** none.
**Scope:** plan-b's half of `shared-plan-a+plan-b.md`. It adds `front_face`, `u` and `v` to `HitRecord`, adds the one `HitRecord::new` constructor that sets the normal against the ray, and fills `(u, v)` in `Sphere::hit` from latitude and longitude. It adds no material, no `Rc` and no change to `src/main.rs`; plan-a's A1r owns those, and the merged constructor gains `material` as its last parameter at merge. This phase executes plan row `B1`'s geometry scope as the pair resolution. Row `B1` stays planned and is re-detailed against the landed contract after this phase; it is not implemented from its current scoped text, and row `Z1`'s dependency on `B1` stays as written.
**Gate level:** standard - the phase changes the shared record contract and the sphere hit, with no durable state, security or migration surface.
**Simplicity statement:** there is one construction path, `HitRecord::new`, and every shape uses it. The helper does not validate `(u, v)`, does not clamp the normal and does not handle a degenerate ray. `front_face`, `u` and `v` carry `#[allow(dead_code)]` because the normal-colouring renderer does not read them, and the phase adds no reader to justify their use. No material, no second record and no texture mapping is built here.

**Current state (verified):**
- `src/hittable.rs` - `pub struct HitRecord { pub t: f64, pub point: Vec3, pub normal: Vec3 }` with `#[derive(Clone, Copy, Debug)]`; `point` carries `#[allow(dead_code)]` and a comment.
- `src/hittable.rs` - `impl Hittable for HittableList::hit` returns the nearest `HitRecord`; it never reads `front_face`, `u` or `v`.
- `src/sphere.rs` - `pub struct Sphere { pub center: Vec3, pub radius: f64 }`, `pub fn new(center: Vec3, radius: f64) -> Self`, and `hit` builds `HitRecord { t, point, normal: (point - self.center) / self.radius }`.
- `src/sphere.rs` tests - `a_ray_through_the_center_hits_the_near_surface` asserts `hit.t` near `1.5` and `hit.normal == Vec3::new(0.0, 0.0, 1.0)`; `a_ray_beside_the_sphere_misses` asserts `None`.
- `src/main.rs` - `ray_colour` still colours a hit by `(hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5`; this phase does not touch it.
- `docs/requirements.md` items 7 and 8 - the front-face flag, the against-ray normal and `(u, v)` in 0..=1 by latitude and longitude.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/shared-plan-a+plan-b.md` - the settled shared design this phase implements on the plan-b branch.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANS/plan-b/shared-plan-a+plan-b.md` - the exact field order, the helper signature, the `Clone, Copy, Debug` derive to keep, the dead-code allowances, and the ownership split; this phase is plan-b's half. The shared text writes `outward_normal.dot(&ray.direction)`; `Vec3::dot` takes its argument by value (`src/vec3.rs:17`), so step 2's `outward_normal.dot(ray.direction)` is the compiling form and overrides that line.
- `docs/requirements.md` items 7 and 8 - what `front_face`, the against-ray normal and `(u, v)` must mean.
- `src/hittable.rs` - the record, its derive, and the one construction site that moves to the helper.
- `src/sphere.rs` - the hit body and both tests.
- `src/ray.rs` - `pub struct Ray { pub origin: Vec3, pub direction: Vec3 }` and `Ray::at`.
- `src/vec3.rs` - `Vec3::dot` and `Neg` used by the helper, and `Vec3::new`.

**Build plan:**
1. `src/hittable.rs`, `HitRecord` - keep `#[derive(Clone, Copy, Debug)]` and the existing `t`, `point`, `normal` fields. Add after `normal`, in this order:
   - `// Which side of the surface the ray hit.` then `#[allow(dead_code)]` then `pub front_face: bool`;
   - `// The hit point's texture coordinates in 0..=1.` then `#[allow(dead_code)]` then `pub u: f64`;
   - `#[allow(dead_code)]` then `pub v: f64`.
   Update the doc comment to "Where a ray hit a surface: the distance along the ray, the point, the normal pointing against the ray, the face side and the texture coordinates."
   Do not add `material`, do not import `Rc` and do not change the derive.
2. `src/hittable.rs`, new `impl HitRecord` after the struct - add exactly one constructor:
   `pub fn new(t: f64, point: Vec3, outward_normal: Vec3, ray: &Ray, u: f64, v: f64) -> Self` with doc comment "Build a record whose normal points against `ray`, with `front_face` saying which side was hit." Body: `let front_face = outward_normal.dot(ray.direction) < 0.0;`, `let normal = if front_face { outward_normal } else { -outward_normal };`, then `HitRecord { t, point, normal, front_face, u, v }`.
3. `src/sphere.rs` - add `use std::f64::consts::PI;`.
4. `src/sphere.rs`, `Sphere::hit` - replace the `Some(HitRecord { ... })` literal with:
   `let outward_normal = (point - self.center) / self.radius;`
   `let theta = (-outward_normal.y).acos();`
   `let phi = (-outward_normal.z).atan2(outward_normal.x) + PI;`
   `let u = phi / (2.0 * PI);`
   `let v = theta / PI;`
   `Some(HitRecord::new(t, point, outward_normal, ray, u, v))`.
   Do not add a material field, do not change `Sphere::new`, and do not change the hit-distance logic.
5. `src/sphere.rs` tests - keep `a_ray_through_the_center_hits_the_near_surface` and `a_ray_beside_the_sphere_misses` as they are; the front-face hit still records the outward normal `(0, 0, 1)`.
6. `src/sphere.rs` tests - add the two new tests listed under Tests.
7. `docs/` - no page changes. `docs/requirements.md` items 7 and 8 already state the final behaviour. Re-read it and `README.md` and report drift instead of editing.

**Callers / wiring to update:**
- `src/sphere.rs:37` - the only `HitRecord` literal construction moves to `HitRecord::new`.
- `src/main.rs` - no change. The renderer keeps reading `hit.normal`, which is unchanged for the front faces the fixed scene hits.
- No other caller exists: `grep -rn "HitRecord {\|Sphere::new" src/` finds only `src/sphere.rs` and the scene in `src/main.rs`, whose `Sphere::new(center, radius)` signature does not change.

**Config / schema / migrations:**
- none.
- Retained records and their named consumer/owner: none.

**Touched files:**
<!-- touched-files:start -->
- `src/hittable.rs` - changed: `front_face`, `u` and `v` fields with dead-code allowances, and the `HitRecord::new` constructor.
- `src/sphere.rs` - changed: `hit` computes the outward normal, latitude and longitude and builds the record through `HitRecord::new`; two tests added.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add `material`, `use std::rc::Rc`, or any material type; plan-a's A1r owns the material field and the record's material placement.
- Do not change `#[derive(Clone, Copy, Debug)]` on `HitRecord`, and do not add a `Debug` bound to any trait.
- Do not change `src/main.rs`, `src/camera.rs`, `src/ray.rs` or `src/vec3.rs`.
- Do not add a second construction path or a second normal computation; `Sphere::hit` uses `HitRecord::new` and nothing else builds the record.
- Do not add `#[allow(dead_code)]` to any field other than `front_face`, `u` and `v`.
- Do not add a dependency to `Cargo.toml`; the standard library only.
- Do not clamp or validate `u`, `v` or the normal, and do not add a texture lookup.
- Do not write plan or phase references into source comments.
- Leave planes, triangles and bounding boxes for B2 and B3.

**Tests:**
- New/changed/deleted:
  - `src/sphere.rs` - `a_ray_inside_the_sphere_hits_the_back_face_with_the_normal_against_it`: the sphere is `Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5)` and the ray starts at its centre `Vec3::new(0.0, 0.0, 0.0)` along `Vec3::new(0.0, 0.0, 1.0)`; the hit is at `t` `0.5`, records `front_face == false`, and records `normal == Vec3::new(0.0, 0.0, -1.0)`.
  - `src/sphere.rs` - `a_sphere_hit_reports_latitude_and_longitude_texture_coordinates`: a ray from `(0, 0, 0)` along `(0, 0, -1)` hits the sphere centred at `(0, 0, -2)` with radius `0.5` at `(0, 0, -1.5)`, whose outward normal is `(0, 0, 1)`, so `theta` is `PI/2`; records `u` near `0.25` and `v` near `0.5`.
  - Changed: none; both existing tests keep their assertions.
  - Deleted: none.
  - Two new test functions; within the three-function phase cap.
- Fast gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, `git diff --check`.
- Conditional gates: none. The render loop, the camera and the output format are untouched, and every hit in the fixed scene is a front face, so normal colouring produces the same image.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - automated phase gates prove acceptance. The phase adds no reader of the new fields, so no distinct runtime observation exists beyond the unit tests and the fast gate.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied: targeted tests, fast gate, docs checked.
- [ ] `HitRecord` has exactly the fields `t`, `point`, `normal`, `front_face`, `u`, `v` and still derives `Clone, Copy, Debug`.
- [ ] `HitRecord::new` sets `front_face` from the sign of `outward_normal.dot(ray.direction)` and flips the normal when the hit is a back face.
- [ ] `Sphere::hit` builds its record through `HitRecord::new` and computes `(u, v)` from latitude and longitude in 0..=1.
- [ ] `src/main.rs` and every other file outside the touched list are unchanged.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a); receipt reports two functions added and zero deleted.
- [ ] Fast phase gate passes; no conditional or slow gate applies.
- [ ] `docs/requirements.md` and `README.md` were re-read and need no change; no docs page refers to a plan.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists.
- [ ] On passing evidence: compress this file into `done/B1r-face-side-and-texture-coordinates.md` (frozen contract only) and flip the `full-plan.md` row `B1r` to `done`; row `B1` stays planned for re-detailing, not implementation.

**Exit:** every sphere hit records its face side, a normal that points against the ray and `(u, v)` from latitude and longitude, built through the one `HitRecord::new` constructor, with the image unchanged.
