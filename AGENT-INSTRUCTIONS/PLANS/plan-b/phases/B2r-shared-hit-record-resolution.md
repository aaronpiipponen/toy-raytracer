## Phase B2r - shared hit record resolution

**Status:** active.
**Depends on:** `done/B1-face-side-and-texture-coordinates.md`, which froze `HitRecord` and
`HitRecord::new`.
**Scope:** one edit in `src/hittable.rs`: `HittableList::hit` reads the nearest distance through
`nearest.as_ref()` instead of moving the nearest record out of the loop. No other line of any file
changes. In particular there is no material type, no record field, no derive change, no
`src/plane.rs`, no `src/triangle.rs`, no `src/main.rs` and no test change.
**Gate level:** minimal - one behaviour-neutral mechanical edit that cannot change runtime
behaviour. The loop still returns the same nearest record for the same input.
**Simplicity statement:** the edit keeps one nearest-hit path and does not add a second loop, a
clone, a helper or a comment. Plan-a owns the material field, the `Copy` removal and the seventh
constructor parameter. This phase only makes the loop correct whether or not the record stays
`Copy`, per `PLANS/shared/plan-a-A1+plan-b-B2.md`.

**Current state (verified):**
- `src/hittable.rs:8` - `#[derive(Clone, Copy, Debug)]` above `pub struct HitRecord`. The record is
  `Copy` on this branch. The edit below is correct with or without `Copy`, so no derive changes.
- `src/hittable.rs:9-22` - `pub struct HitRecord { t, point, normal, front_face, u, v }`. No field
  is added, removed or renamed.
- `src/hittable.rs:24-43` - `HitRecord::new(t: f64, point: Vec3, u: f64, v: f64, ray: &Ray,
  outward_normal: Vec3) -> Self`. The signature is unchanged.
- `src/hittable.rs:45-48` - `pub trait Hittable { fn hit(&self, ray: &Ray, t_min: f64, t_max: f64)
  -> Option<HitRecord>; }`.
- `src/hittable.rs:62-72` - `impl Hittable for HittableList`. Line 66 is the edited line:
  `let limit = nearest.map_or(t_max, |hit| hit.t);`. Line 64 declares `let mut nearest:
  Option<HitRecord> = None;`.
- `src/main.rs:24-30` - `ray_colour` calls `world.hit(ray, 0.001, f64::INFINITY)` and reads
  `hit.normal`. The value returned by `HittableList::hit` is unchanged.
- `src/main.rs:86-97` - `the_center_pixel_shows_the_sphere_facing_the_camera` and
  `the_top_row_is_sky` render through `HittableList::hit`, so they exercise this loop.

**Read first (do not invent):**
- `src/hittable.rs`, whole file - the record, the trait and the `HittableList` loop being edited.
- `src/main.rs:24-30` and `src/main.rs:82-98` - the only production reader of the list's result and
  the tests that cover it.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md` - the agreed target loop line and why the
  value form stops compiling once the record is not `Copy`.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/B1-face-side-and-texture-coordinates.md` - the frozen
  record contract.

**Build plan:**
1. `src/hittable.rs:66`, inside `impl Hittable for HittableList` - change the nearest-distance read
   from the value form to the reference form. Before:
   ```rust
   let limit = nearest.map_or(t_max, |hit| hit.t);
   ```
   After:
   ```rust
   let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);
   ```
   `nearest` has type `Option<HitRecord>`. `as_ref()` borrows it, so `map_or` returns `f64` and the
   record is not moved. The closure receives `&HitRecord` and field access reads `t` through the
   reference. Change no other line, rename nothing and add no comment.

**Callers / wiring to update:**
- `src/hittable.rs:62-72` - the edited line is inside `HittableList::hit`. Its signature, return
  type and behaviour stay the same, so no caller changes.
- `src/main.rs:24-30` - `ray_colour` consumes the returned `Option<HitRecord>` unchanged.
- No other file calls `HittableList::hit` or refers to the loop.

**Config / schema / migrations:**
- None. No configuration key, schema, migration or generated file.
- Retained records and their named consumer/owner: none added or changed. The record is in-memory
  and consumed by `ray_colour` for the pixel colour.

**Touched files:**
<!-- touched-files:start -->
- `src/hittable.rs` - changed: `HittableList::hit` reads the nearest distance through `nearest.as_ref()`.
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not change the `#[derive(...)]` list, add a field, or change `HitRecord::new`. The material
  field, `Rc<dyn Material>`, `Copy` removal and the seventh constructor parameter belong to the
  materials plan.
- Do not clone the nearest record, restructure the loop, or add a second nearest-hit path.
- Do not edit `src/main.rs`, `src/sphere.rs`, `src/plane.rs`, `src/triangle.rs` or any test.
- Do not add, change or delete a test function (REPO-RULES §3a). The existing tests prove the loop.
- Do not add a comment or a doc comment on the edited line.
- Do not add a dependency or touch `Cargo.toml` (hard invariant 1).

**Tests:**
- New/changed/deleted: none. The existing `src/main.rs` tests render through the loop and the
  `src/sphere.rs` tests exercise the shape side. All must pass unchanged.
- Fast gate, run in this order:
  - `cargo build`
  - `cargo test`
  - `cargo clippy -- -D warnings`
  - `cargo fmt --check`
  - `git diff --check`
- Conditional gates: none. The rendered image is unchanged: the loop selects the same nearest
  record and `ray_colour` reads the same `normal`, so no before/after image comparison is needed.
  No file under `AGENT-INSTRUCTIONS/SUBAGENTS/` changes.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - automated phase gates prove acceptance. The edit is behaviour-neutral and the
  existing build, test, clippy and format gates cover the compiled loop.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `src/hittable.rs:66` reads `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);` and no
  other line in any file changes.
- [ ] `HitRecord`, its derive list, its fields and `HitRecord::new` are unchanged.
- [ ] Existing tests pass unchanged and the test budget is zero new functions (receipt reports
  functions added/deleted).
- [ ] Fast phase gate from `AGENT-INSTRUCTIONS/BUILDING/VERIFICATION-COMMITS-DEPLOY.md` §1 passes.
  No conditional or slow gate applies.
- [ ] No `docs/` page changes, and no file refers to a plan or phase.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists.
- [ ] On passing evidence: compress this file into
  `done/B2r-shared-hit-record-resolution.md` (frozen contract only: the `as_ref()` nearest-distance
  read and the unchanged record) and flip the `full-plan.md` status row to `done`.

**Exit:** `HittableList::hit` reads the nearest distance through `nearest.as_ref()`, so the loop
compiles once the record stops being `Copy`, and the rendered image and all existing tests are
unchanged.
