## Phase A1r - Materials on the shared hit record

**Status:** active.
**Depends on:** none.
**Scope:** plan-a's half of `shared-plan-a+plan-b.md`. It adds the `material` field to `HitRecord`, gives `Sphere` a material and puts it on the record, adds `src/material.rs` with the `Material` trait, a diffuse and a metal material, and replaces the normal-colouring loop in `src/main.rs` with the scatter loop and gamma 2 output. It does not add `front_face`, `u`, `v`, a `HitRecord` constructor, or any change to how the normal is computed; plan-b's B1r owns those. This phase executes plan row `A1`'s material scope as the pair resolution. Row `A1` stays planned and is re-detailed against the landed contract after this phase; it is not implemented from its current scoped text.
**Gate level:** standard - the phase changes runtime rendering behaviour across the record, the shape and the render loop, with no durable state, security or migration surface.
**Simplicity statement:** the trait object is shared as `Rc<dyn Material>` with no generic parameter and no lifetime. The bounce limit is a fixed constant, the seed is a fixed constant, and a ray that reaches either is returned as produced, not retried. There is no path tracer accumulator, no sampling average, no clamping of material parameters and no second colouring path; the normal-colouring code is deleted, not kept beside the new loop. No legacy format and no defensive re-check is added.

**Current state (verified):**
- `src/hittable.rs` - `pub struct HitRecord { pub t: f64, pub point: Vec3, pub normal: Vec3 }` with `#[derive(Clone, Copy, Debug)]`; `point` carries `#[allow(dead_code)]`.
- `src/hittable.rs` - `impl Hittable for HittableList::hit` uses `nearest.map_or(t_max, |hit| hit.t)`, which needs `Copy`.
- `src/sphere.rs` - `pub struct Sphere { pub center: Vec3, pub radius: f64 }`, `pub fn new(center: Vec3, radius: f64) -> Self`, and `hit` builds `HitRecord { t, point, normal: (point - self.center) / self.radius }`.
- `src/sphere.rs` tests - both call `Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5)`; `a_ray_through_the_center_hits_the_near_surface` asserts the hit distance and the outward normal, `a_ray_beside_the_sphere_misses` asserts a miss.
- `src/main.rs` - `fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Colour` returns `(hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5` on a hit and the sky gradient otherwise.
- `src/main.rs` - `scene()` adds two spheres, `render()` calls `ray_colour` once per pixel, `to_byte(channel)` returns `(255.999 * channel.clamp(0.0, 1.0)) as u8`, and `mod rng` carries `#[allow(dead_code)]` because nothing uses it.
- `src/main.rs` tests - `the_center_pixel_shows_the_sphere_facing_the_camera` asserts `center.z > 0.9`; `the_top_row_is_sky` asserts `pixels[0].z > pixels[0].x`.
- `src/rng.rs` - `pub struct Rng`, `Rng::new(seed: u64) -> Self`, `next_u64(&mut self) -> u64`, `next_f64(&mut self) -> f64` in 0..1.
- `src/vec3.rs` - `Vec3` is `Copy` with `dot`, `length`, `unit`, `Add`, `Sub`, `Neg`, `Mul<f64>`, `Mul<Vec3>` and `Div<f64>`; `pub type Colour = Vec3`.
- `src/ray.rs` - `pub struct Ray { pub origin: Vec3, pub direction: Vec3 }` with `Ray::new(origin, direction) -> Self` and `at(t)`.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/shared-plan-a+plan-b.md` - the settled shared design this phase implements on the plan-a branch.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANS/plan-a/shared-plan-a+plan-b.md` - the `HitRecord` field order, the `Clone`-only derive, the additive record construction, and the ownership split; this phase is plan-a's half.
- `docs/requirements.md` items 1 to 6 - material behaviour, the bounce limit of 50, gamma 2, and the seeded generator.
- `src/hittable.rs` - the exact record and the `nearest.map_or` line that stops compiling without `Copy`.
- `src/sphere.rs` - the current hit body and the two sphere tests that need the new constructor argument.
- `src/main.rs` - `ray_colour`, `scene`, `render`, `to_byte` and the two tests.
- `src/rng.rs` - `Rng::new` and `next_f64`, the only source of randomness.
- `src/vec3.rs` - the operator set the new code multiplies and reflects with.

**Build plan:**
1. `src/material.rs` - create the module with header `//! Materials: how a surface scatters the ray that hit it.` and these exact public items. Every public item carries a doc comment in the style guide's form; the wording below is the minimum:
   - `pub trait Material` with doc comment "A surface that scatters the ray that hits it." and method `fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)>` with its own doc comment "Scatter `ray` at a hit, returning the attenuation and the scattered ray, or nothing when the material absorbs the ray."
   - `pub struct Lambertian { pub albedo: Colour }` with doc comment "A diffuse surface that scatters in a random direction around the normal." and `pub fn new(albedo: Colour) -> Self` with doc comment "Build a diffuse material with the given albedo."; `impl Material for Lambertian` computes `let direction = hit.normal + random_in_unit_sphere(rng);` and returns `Some((self.albedo, Ray::new(hit.point, direction.unit())))`. The `ray` parameter is named `_ray`.
   - `pub struct Metal { pub albedo: Colour, pub fuzz: f64 }` with doc comment "A reflective surface, the reflection blurred by `fuzz` in 0..=1." and `pub fn new(albedo: Colour, fuzz: f64) -> Self` with doc comment "Build a metal material with the given albedo and fuzz factor."; `impl Material for Metal` computes `let unit = ray.direction.unit();`, `let reflected = unit - hit.normal * (2.0 * unit.dot(hit.normal));`, `let direction = reflected + random_in_unit_sphere(rng) * self.fuzz;`, and returns `Some((self.albedo, Ray::new(hit.point, direction)))` when `direction.dot(hit.normal) > 0.0`, otherwise `None`.
   - `fn random_in_unit_sphere(rng: &mut Rng) -> Vec3` draws three coordinates with `2.0 * rng.next_f64() - 1.0` in a loop and returns the first whose `dot` with itself is `< 1.0`.
   - Imports: `crate::hittable::HitRecord`, `crate::ray::Ray`, `crate::rng::Rng`, `crate::vec3::{Colour, Vec3}`.
   - No `Debug` bound on `Material` and no interior mutability; the `&mut Rng` parameter is the only randomness path.
2. `src/hittable.rs`, `HitRecord` - change `#[derive(Clone, Copy, Debug)]` to `#[derive(Clone)]`; add `use std::rc::Rc;` and `use crate::material::Material;`; remove the `#[allow(dead_code)]` line and its comment from `point`; add `pub material: Rc<dyn Material>` as the last field after `normal`. The doc comment becomes "Where a ray hit a surface: the distance along the ray, the point, the surface normal and the surface's material."
3. `src/hittable.rs`, `HittableList::hit` - change `nearest.map_or(t_max, |hit| hit.t)` to `nearest.as_ref().map_or(t_max, |hit| hit.t)`.
4. `src/sphere.rs`, `Sphere` - add `pub material: Rc<dyn Material>` after `radius`; change `pub fn new(center: Vec3, radius: f64) -> Self` to `pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self` and store it. Add `use std::rc::Rc;` and `use crate::material::Material;`.
5. `src/sphere.rs`, `Sphere::hit` - replace the literal record with `Some(HitRecord { t, point, normal: (point - self.center) / self.radius, material: Rc::clone(&self.material) })`. Do not add a constructor helper and do not change the normal computation.
6. `src/sphere.rs` tests - pass a material to both `Sphere::new` calls: `Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))`, adding `use crate::material::Lambertian;` to the test module; `Rc` comes from the parent's `use super::*;`. Keep both assertions as they are.
7. `src/main.rs` module block - add `mod material;` in alphabetical order, and delete the two lines above `mod rng;` (the comment `// Seeded randomness for the scattering and sampling the plans add.` and `#[allow(dead_code)]`), because `rng` now has a reader and the comment names the plans.
8. `src/main.rs` imports - add `use std::rc::Rc;`, `use material::{Lambertian, Material, Metal};` and `use rng::Rng;`. `Material` is imported so the trait is in scope for the `Rc<dyn Material>` values.
9. `src/main.rs` constants - add `const MAX_BOUNCES: u32 = 50;` and `const SEED: u64 = 42;` beside `WIDTH` and `HEIGHT`.
10. `src/main.rs`, `ray_colour` - before -> after: `fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Colour` becomes `fn ray_colour(ray: &Ray, world: &dyn Hittable, depth: u32, rng: &mut Rng) -> Colour`. Body: return `Colour::new(0.0, 0.0, 0.0)` when `depth == 0`; on a hit call `hit.material.scatter(ray, &hit, rng)` and return `attenuation * ray_colour(&scattered, world, depth - 1, rng)` when it is `Some((attenuation, scattered))`, otherwise `Colour::new(0.0, 0.0, 0.0)`; keep the sky gradient as the final branch. The doc comment becomes "The colour seen along a ray: the material's scattered colour up to the bounce limit, the sky gradient otherwise."
11. `src/main.rs`, `scene` - bind each material as a local typed trait object and add three spheres in this order: `let red: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)));` on `Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::clone(&red))`; `let ground: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));` on `Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::clone(&ground))`; `let metal: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3));` on `Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.3, Rc::clone(&metal))`. The explicit `Rc<dyn Material>` annotation is required: `Rc::clone` on an `Rc<Lambertian>` cannot infer the unsizing coercion at the call site.
12. `src/main.rs`, `render` - create `let mut rng = Rng::new(SEED);` before the loops and call `ray_colour(&camera.ray(u, v), &world, MAX_BOUNCES, &mut rng)`. The signature of `render` does not change.
13. `src/main.rs`, `to_byte` - return `(255.999 * channel.clamp(0.0, 1.0).sqrt()) as u8`.
14. `src/main.rs` tests - rename `the_center_pixel_shows_the_sphere_facing_the_camera` to `the_center_pixel_shows_the_red_diffuse_sphere` and assert `center.x > center.y && center.x > center.z`, with the message `"the centre sphere is diffuse red: {center:?}"`. Leave `the_top_row_is_sky` unchanged.
15. `src/material.rs` tests - add the two test functions listed under Tests, in a `#[cfg(test)] mod tests` with `use super::*;`, a private `fn hit_with_normal(normal: Vec3, material: Rc<dyn Material>) -> HitRecord` returning `HitRecord { t: 1.0, point: Vec3::new(0.0, 0.0, 0.0), normal, material }`, and `use std::rc::Rc;`.
16. `docs/` - no page changes. `docs/requirements.md` items 1 to 6 already state the final material behaviour, the bounce limit, gamma 2 and the seeded generator, and `README.md` still describes `cargo run` and `cargo test`. Re-read both and report any drift instead of editing them.

**Callers / wiring to update:**
- `src/main.rs:34` and `src/main.rs:35` - the two `Sphere::new` scene calls take the third material argument.
- `src/main.rs:24` and `src/main.rs:48` - `ray_colour` gains `depth` and `rng`; the call in `render` passes `MAX_BOUNCES` and `&mut rng`.
- `src/sphere.rs:51` and `src/sphere.rs:62` - the two `Sphere::new` test calls take the third material argument.
- `src/hittable.rs:37` - the only `map_or` on `Option<HitRecord>` must switch to `as_ref` because the record is no longer `Copy`.
- No other caller exists: `grep -rn "Sphere::new\|HitRecord {" src/` finds only these sites.

**Config / schema / migrations:**
- none.
- Retained records and their named consumer/owner: none.

**Touched files:**
<!-- touched-files:start -->
- `src/material.rs` - created: the `Material` trait, `Lambertian`, `Metal` and the random-in-sphere helper.
- `src/hittable.rs` - changed: `material` field, `Clone`-only derive, `point` allowance removed, `HittableList` uses `as_ref`.
- `src/sphere.rs` - changed: material field and constructor argument, record carries the material, tests pass a material.
- `src/main.rs` - changed: scatter loop with the bounce limit, gamma 2 output, scene materials, module and import wiring, tests adapted.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add `front_face`, `u`, `v`, or a `HitRecord` constructor; plan-b's B1r owns them and the merged helper gains `material` only at merge.
- Do not change how `Sphere::hit` computes the normal; it stays the outward normal on this branch.
- Do not keep the normal-colouring path, a material-second-path flag, or any fallback colour; replace the hit branch in `ray_colour`.
- Do not add a dependency to `Cargo.toml`; `Rc` and the standard library only.
- Do not use `RefCell`, `Cell`, `unsafe`, threads, or a seed from the clock; all randomness goes through `&mut Rng`.
- Do not add `Debug` to `Material` or `Debug` to `HitRecord`.
- Do not clamp or validate `fuzz` and do not add recovery for a zero-length scatter direction.
- Do not write plan or phase references into source comments.
- Stay inside `src/material.rs`, `src/hittable.rs`, `src/sphere.rs` and `src/main.rs`; leave the geometry and output work for their plans.

**Tests:**
- New/changed/deleted:
  - `src/material.rs` - `a_diffuse_hit_scatters_into_the_normal_hemisphere`: bind `let material: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)));`, build the hit with `hit_with_normal(Vec3::new(0.0, 0.0, 1.0), Rc::clone(&material))`, and scatter `Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0))` with `&mut Rng::new(1)`; the returned scattered ray's direction has a positive dot with the normal.
  - `src/material.rs` - `a_metal_reflection_below_the_surface_is_absorbed`: bind `let material: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));`, build the same hit with `hit_with_normal(Vec3::new(0.0, 0.0, 1.0), Rc::clone(&material))`, and scatter `Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0))`; the result is `None`.
  - `src/main.rs` - `the_center_pixel_shows_the_sphere_facing_the_camera` is renamed to `the_center_pixel_shows_the_red_diffuse_sphere` and asserts the red channel is the largest. This adapts an existing test; it does not add to the budget.
  - `src/sphere.rs` - both existing tests keep their assertions and only gain the material argument (adaptation, no new function).
  - Deleted: none.
  - Two new test functions; within the three-function phase cap.
- Fast gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image, because a material and the render loop changed. Run `cargo run --release > /tmp/opencode/render.ppm` once; the file is text (P3 PPM), so acceptance reads named pixel lines rather than opening a viewer. This same render is the acceptance evidence.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** agent.
- The output is a text P3 PPM: header lines 1 to 3 are `P3`, `200 100` and `255`, and pixel `k` (counted from the top-left, row by row) is line `4 + k`. Pixels are read directly from the text, so no viewer or conversion is needed.
- 1. Run `cargo run --release > /tmp/opencode/render.ppm`. Expected: exit status 0 and header lines `P3`, `200 100`, `255`.
- 2. Read the centre pixel with `sed -n '10104p' /tmp/opencode/render.ppm` (pixel index `50*200+100`). Expected: three integers with the first (red) largest, showing the diffuse red sphere.
- 3. Read the metal-sphere pixel with `sed -n '9749p' /tmp/opencode/render.ppm` (pixel index `48*200+145`) and a sky pixel same column with `sed -n '2149p' /tmp/opencode/render.ppm` (pixel index `10*200+145`). Expected: the metal pixel's third (blue) value is at most 150 while the sky pixel's third value is 255, because the metal albedo's blue of 0.2 caps the metal blue after gamma.
- 4. Read a ground pixel with `sed -n '16024p' /tmp/opencode/render.ppm` (pixel index `80*200+20`). Expected: three integers whose third (blue) is `0`, because the yellow ground albedo has no blue and every later bounce keeps blue at `0`.
- 5. Read the top-left pixel with `sed -n '4p' /tmp/opencode/render.ppm`. Expected: three integers with the third (blue) largest, the sky.
- 6. Run the render again to `/tmp/opencode/render2.ppm` and run `cmp /tmp/opencode/render.ppm /tmp/opencode/render2.ppm`. Expected: no output, so the fixed seed reproduces the image byte for byte.
- 7. For a visual check the agent may convert `/tmp/opencode/render.ppm` to PNG in `/tmp/opencode` with a throwaway standard-library script and open it with the image reader; the blocking evidence is steps 1 to 6.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied: targeted tests, fast gate, affected conditional gate, docs checked.
- [ ] `HitRecord` derives `Clone` only and its last field is `pub material: Rc<dyn Material>`.
- [ ] `HittableList::hit` compiles without `Copy` through `nearest.as_ref().map_or`.
- [ ] `ray_colour` follows scattered rays up to `MAX_BOUNCES` and returns black at the limit and on absorption.
- [ ] `to_byte` applies gamma 2.
- [ ] The scene has two diffuse materials and one metal material on three spheres, and the normal-colouring branch is gone.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a); receipt reports two functions added and zero deleted.
- [ ] Fast phase gate and the rendered-image conditional gate pass; no slow gate applies.
- [ ] `docs/requirements.md` and `README.md` were re-read and need no change; no docs page refers to a plan.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the agent acceptance render passed steps 1 to 6 and the second render matched byte for byte.
- [ ] On passing evidence: compress this file into `done/A1r-materials-on-the-shared-hit-record.md` (frozen contract only) and flip the `full-plan.md` row `A1r` to `done`; row `A1` stays planned for re-detailing, not implementation.

**Exit:** every sphere and the ground carry a material, the record carries it without `Copy`, and the rendered image is the scatter loop with gamma 2, showing a red diffuse sphere, a yellow diffuse ground and a metal sphere.
