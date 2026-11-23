## Phase B1 - Face side and texture coordinates

**Status:** active.
**Depends on:** B1r (`done/B1r-face-side-and-texture-coordinates.md`).
**Scope:** close out row `B1` against the contract `B1r` landed. It removes the two source comments that still name plans (`src/main.rs:6`, `src/vec3.rs:21`), asserts the front-face side in the existing sphere hit test so requirement 7 is proven for both sides, and records that requirements 7 and 8 are satisfied by the landed code. It adds no field, no constructor, no shape and no production reader of `front_face`, `u` or `v`; the one new read is an assertion inside a `#[cfg(test)]` module. Planes, triangles and bounding boxes stay in B2 and B3.
**Gate level:** standard - the phase changes a test and two source comments, so it runs the fast gate, but it changes no runtime behavior and no contract.
**Simplicity statement:** the phase deletes text and adds one assertion. It adds no test function, no validation, no clamp and no production reader of the new record fields. The three `#[allow(dead_code)]` attributes on `front_face`, `u` and `v` stay, because a read inside `#[cfg(test)]` does not count as a non-test reader and `cargo build` would otherwise fail on `-D warnings`.

**Current state (verified):**
- `src/hittable.rs` - `HitRecord` holds `t`, `point`, `normal`, `front_face`, `u`, `v` in that order, derives `Clone, Copy, Debug`, and is built only by `HitRecord::new(t, point, outward_normal, ray, u, v)`; `front_face`, `u` and `v` carry `#[allow(dead_code)]`.
- `src/hittable.rs` - `impl Hittable for HittableList::hit` returns the nearest record and never reads the three new fields.
- `src/sphere.rs` - `Sphere::hit` builds its record through `HitRecord::new` with `u` and `v` from latitude and longitude; the module imports `std::f64::consts::PI`.
- `src/sphere.rs` tests - `a_ray_through_the_center_hits_the_near_surface` asserts `hit.t` near `1.5` and `hit.normal == Vec3::new(0.0, 0.0, 1.0)` but does not assert `hit.front_face`; `a_ray_beside_the_sphere_misses` asserts `None`; `a_ray_inside_the_sphere_hits_the_back_face_with_the_normal_against_it` asserts `!hit.front_face`; `a_sphere_hit_reports_latitude_and_longitude_texture_coordinates` asserts `u` near `0.25` and `v` near `0.5`.
- `src/main.rs:6` - the comment `// Seeded randomness for the scattering and sampling the plans add.` sits above `#[allow(dead_code)]` and `mod rng;`; it names plans.
- `src/vec3.rs:21` - the comment `// For the triangle normals the geometry plan adds.` sits above `#[allow(dead_code)]` and `pub fn cross`; it names a plan.
- `src/main.rs` - `ray_colour` colours a hit by `(hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5`; every hit in the fixed scene is a front face, so the image does not change.
- `docs/requirements.md` items 7 and 8 - the front-face flag, the against-ray normal, and `(u, v)` in 0..=1 by latitude and longitude.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/B1r-face-side-and-texture-coordinates.md` - the frozen contract this phase closes out, including the merge obligation that belongs to plan-a's merge, not to this phase.
- `docs/requirements.md` items 7 and 8 - the behavior this phase confirms is landed.
- `src/hittable.rs` - the record, the derive and the one constructor.
- `src/sphere.rs` - the hit body and all four tests.
- `src/main.rs` - the module block and its two comment lines above `mod rng;`.
- `src/vec3.rs` - the comment line above `pub fn cross` and the `#[allow(dead_code)]` it explains.

**Build plan:**
1. `src/main.rs`, module block - delete only the comment line `// Seeded randomness for the scattering and sampling the plans add.`. Keep `#[allow(dead_code)]` and `mod rng;` exactly as they are.
2. `src/vec3.rs`, `Vec3::cross` - delete only the comment line `// For the triangle normals the geometry plan adds.`. Keep `#[allow(dead_code)]` and the function body unchanged.
3. `src/sphere.rs`, test `a_ray_through_the_center_hits_the_near_surface` - add `assert!(hit.front_face);` after the existing `t` assertion and before the `normal` assertion. Keep both existing assertions. This is an adaptation of an existing test, not a new function.
4. `src/hittable.rs` - no change. Do not remove or add any `#[allow(dead_code)]`, do not change the derive, and do not touch `HitRecord::new`.
5. `docs/` - no page changes. Re-read `docs/requirements.md` items 7 and 8 and `README.md`; report drift instead of editing. Both already state the landed behavior.

**Callers / wiring to update:**
- None. Grep `rg -n "front_face|\.u\b|\.v\b" src/` finds only `src/hittable.rs` and the `src/sphere.rs` tests; no production caller reads the new fields, and this phase adds none.
- `src/main.rs` and `src/vec3.rs` change only by the removal of one comment line each; no symbol, import or call changes.

**Config / schema / migrations:**
- none.
- Retained records and their named consumer/owner: none.

**Touched files:**
<!-- touched-files:start -->
- `src/main.rs` - changed: one plan-naming comment line above `mod rng;` deleted; no code change.
- `src/vec3.rs` - changed: one plan-naming comment line above `Vec3::cross` deleted; no code change.
- `src/sphere.rs` - changed: `a_ray_through_the_center_hits_the_near_surface` gains one `front_face` assertion; no other test or code change.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add `front_face`, `u`, `v`, a constructor or any field; `B1r` landed them and this phase changes nothing in `src/hittable.rs`.
- Do not remove the `#[allow(dead_code)]` on `front_face`, `u`, `v`, `point`, `mod rng` or `Vec3::cross`; no non-test reader exists on this branch.
- Do not add a production reader of `u` or `v`, a texture lookup, or a clamp of either; the only new read of a record field in this phase is the `front_face` assertion inside the test module.
- Do not change `src/main.rs`'s colouring, the scene, the camera, the render loop or the output.
- Do not name plan-a's symbols (`Material`, `Lambertian`, `Metal`, `Rc<dyn Material>`), and do not add the `material` field or the `Rc` import; those arrive only through the coordinator's merge.
- Do not add a test function; the one assertion is an adaptation of an existing test (REPO-RULES §3a).
- Do not add a dependency to `Cargo.toml`; the standard library only.
- Do not write plan or phase references into source comments.
- Leave planes, triangles and bounding boxes for B2 and B3.

**Tests:**
- New/changed/deleted:
  - `src/sphere.rs` - `a_ray_through_the_center_hits_the_near_surface` - gains `assert!(hit.front_face);`, so the front-face side is proven for the front-face hit that `a_ray_inside_the_sphere_hits_the_back_face_with_the_normal_against_it` proves for the back-face hit. Adaptation, no new function.
  - Changed: that one test; the other three sphere tests keep their assertions.
  - Deleted: none.
  - Zero new test functions; within the three-function phase cap and within the original `B1` cap.
- Fast gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, `git diff --check`.
- Conditional gates: none. Removing two comment lines and adding one assertion changes no render-loop, material, camera or output seam, so the rendered-image gate does not apply.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - automated phase gates prove acceptance. The phase changes no runtime behavior, so no distinct runtime observation exists beyond the fast gate and the assertion.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied: the adapted test and the fast gate pass.
- [ ] `src/main.rs:6` and `src/vec3.rs:21` no longer contain the plan-naming comments, and both `#[allow(dead_code)]` attributes remain.
- [ ] `a_ray_through_the_center_hits_the_near_surface` asserts `hit.front_face` and keeps its `t` and `normal` assertions.
- [ ] `src/hittable.rs` is unchanged in this phase and still holds the `B1r` contract exactly.
- [ ] Requirements 7 and 8 are confirmed against `docs/requirements.md`, and `README.md` needs no change; no doc or comment refers to a plan.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a); receipt reports zero functions added and zero deleted.
- [ ] Fast phase gate passes; no conditional or slow gate applies.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists.
- [ ] On passing evidence: compress this file into `done/B1-face-side-and-texture-coordinates.md` (frozen contract only) and flip the `full-plan.md` row `B1` to `done`.
- [ ] No branch file names plan-a's new symbols; the merged tree is the coordinator's step, following the merge checklist in `shared-plan-a+plan-b.md`.

**Exit:** row `B1` is closed against the landed contract: no source comment names a plan, requirement 7 is proven for both face sides by the sphere tests, requirement 8 is confirmed for spheres, and the branch names none of plan-a's symbols.
