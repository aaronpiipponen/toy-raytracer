## Phase A2 - Glass

**Status:** active.
**Depends on:** A1 (`done/A1-diffuse-and-metal-materials.md`) and the shared design `PLANS/shared/plan-a-A2+plan-b-B2.md`.
**Scope:** the combined hit record with a face flag and texture coordinates, `Sphere` filling that record, the existing material tests moved onto its constructor, a `Dielectric` glass material, and one glass sphere in the scene. No plane, triangle, bounding box, sampling, command-line or output change.
**Gate level:** standard - bounded runtime behaviour in the hit record, one shape and one material, with no schema, durable state, security boundary or cross-package contract on this branch. The rendered-image conditional gate applies because a material, a shape's record and the scene change.
**Simplicity statement:** One record shape and one seven-argument constructor serve every shape. Glass reads the record's existing `front_face` and ray-facing `normal`, adds no face flag, no per-shape branch and no stored sine or cosine. The refractive index is a plain field with no validation or clamping. A ray that is absorbed is the only `None` result, and glass never absorbs. Nothing is deleted beyond the struct-literal construction sites that the constructor replaces.

**Current state (verified):**
- `src/hittable.rs:11-16` - `HitRecord` holds `pub t: f64`, `pub point: Vec3`, `pub normal: Vec3`, `pub material: Rc<dyn Material>` and derives nothing.
- `src/hittable.rs:18-21` - the `Hittable` trait's `hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>`.
- `src/hittable.rs:29-45` - `HittableList::add` and `HittableList::hit`; line 39 already reads the nearest distance through `nearest.as_ref().map_or(t_max, |hit| hit.t)`, so it stays unchanged.
- `src/sphere.rs:10-24` - `Sphere { pub center: Vec3, pub radius: f64, pub material: Rc<dyn Material> }` and `Sphere::new(center, radius, material)`.
- `src/sphere.rs:26-52` - `impl Hittable for Sphere`; lines 44-50 build the record with a struct literal, `normal: (point - self.center) / self.radius` and `material: Rc::clone(&self.material)`.
- `src/sphere.rs:54-87` - tests `a_ray_through_the_center_hits_the_near_surface` (lines 62-75, asserts `t` and the normal) and `a_ray_beside_the_sphere_misses` (lines 77-86).
- `src/material.rs:8-12` - `Material::scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)>`.
- `src/material.rs:14-56` - `Lambertian` (albedo) and `Metal` (albedo, fuzz), both reading `hit.normal`; `Metal` returns `None` when `scattered.dot(hit.normal) <= 0.0`.
- `src/material.rs:58-76` - the private `reflect(v: Vec3, n: Vec3) -> Vec3` and `random_unit_vector(rng: &mut Rng) -> Vec3`.
- `src/material.rs:78-133` - the test module; `record_with(normal: Vec3, material: Rc<dyn Material>) -> HitRecord` at lines 84-91 builds the record with a struct literal, and the three tests at lines 93-133 use it.
- `src/main.rs:16` - `use material::{Lambertian, Metal};`.
- `src/main.rs:54-72` - `scene()` adds a diffuse centre sphere, a diffuse ground sphere and a metal sphere at `(0.85, -0.25, -1.3)` of radius `0.25`.
- `src/main.rs:29-46` - `ray_colour` reads only `hit.material` and `hit.point` through `scatter`, so the record's new fields do not change it.
- `src/vec3.rs:82` - `pub type Colour = Vec3`; `Vec3` implements `Add`, `Sub`, `Mul<f64>`, `Div<f64>`, `Neg`, `dot`, `unit`.
- `src/rng.rs:19-22` - `next_f64(&mut self) -> f64` returns a value in `0..1`.
- `docs/requirements.md` items 4 and 7 - glass refracts, reflects on total internal reflection, picks reflection by Schlick, does not attenuate, and every hit records its face side with a normal against the ray.
- `Cargo.toml` has no dependency; `src/plane.rs` and `src/triangle.rs` do not exist on this branch.

**Read first (do not invent):**
- `src/hittable.rs` - the `HitRecord` struct, the `Hittable` trait and `HittableList::hit`; this phase changes only the struct and adds its constructor.
- `src/sphere.rs` - the fields, `Sphere::new` and the `hit` body at lines 27-51; the (u, v) formula and the constructor call go there.
- `src/material.rs` - the trait, `Lambertian`, `Metal`, `reflect`, `random_unit_vector` and the whole test module; `Dielectric`, `refract`, `schlick` join them.
- `src/main.rs` - the `material` import at line 16 and `scene()` at lines 54-72; one glass sphere joins the scene.
- `src/vec3.rs` - `Vec3` operators and `Colour`; the glass math uses `dot`, `unit`, `Add`, `Mul<f64>` and `Neg` (or a leading minus in the formula).
- `src/ray.rs` - `Ray { pub origin, pub direction }`, `Ray::new` and `Ray::at`; the constructor reads `ray.direction` and tests build rays.
- `src/rng.rs` - `Rng::new(seed: u64)` and `next_f64`; the Schlick choice is the only random draw in glass.
- `docs/requirements.md` items 4 and 7 - the behaviour this phase must produce.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A2+plan-b-B2.md` - the exact record shape, constructor signature, (u, v) formula and plan-a file list.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1-diffuse-and-metal-materials.md` - the frozen `Material`, `HitRecord` and render contracts.

**Build plan:**
1. `src/hittable.rs:9-16` - replace the struct doc comment with one sentence stating that the record holds the distance, the point, the normal facing the ray, the face side, the texture coordinates and the object's material. Change the struct to exactly these fields in this order:
   ```rust
   pub struct HitRecord {
       pub t: f64,
       pub point: Vec3,
       /// Faces against the incoming ray.
       pub normal: Vec3,
       pub front_face: bool,
       pub u: f64,
       pub v: f64,
       pub material: Rc<dyn Material>,
   }
   ```
2. `src/hittable.rs`, after the struct and before the `Hittable` trait - add the one constructor:
   ```rust
   impl HitRecord {
       /// A record for a hit by `ray` at `point`, with `outward_normal` facing away from the object.
       pub fn new(
           ray: &Ray,
           t: f64,
           point: Vec3,
           outward_normal: Vec3,
           u: f64,
           v: f64,
           material: Rc<dyn Material>,
       ) -> Self {
           let front_face = ray.direction.dot(outward_normal) < 0.0;
           HitRecord {
               t,
               point,
               normal: if front_face { outward_normal } else { -outward_normal },
               front_face,
               u,
               v,
               material,
           }
       }
   }
   ```
3. `src/sphere.rs:3-8` - add `use std::f64::consts::PI;` beside the other imports. Leave `Sphere`, `Sphere::new` and the discriminant code at lines 27-43 unchanged.
4. `src/sphere.rs:44-50`, after `let point = ray.at(t);` - replace the struct literal with:
   ```rust
   let outward_normal = (point - self.center) / self.radius;
   let theta = (-outward_normal.y).acos();
   let phi = (-outward_normal.z).atan2(outward_normal.x) + PI;
   let u = phi / (2.0 * PI);
   let v = theta / PI;
   Some(HitRecord::new(
       ray,
       t,
       point,
       outward_normal,
       u,
       v,
       Rc::clone(&self.material),
   ))
   ```
   The formula is the shared one and applies to both roots, so `(u, v)` is computed once after `t` is settled.
5. `src/material.rs`, after `impl Material for Metal` (line 56) and before `fn reflect` (line 58) - add the glass material with a doc comment saying it refracts by its refractive index and reflects by total internal reflection or Schlick's approximation:
   ```rust
   pub struct Dielectric {
       pub refractive_index: f64,
   }

   impl Dielectric {
       /// A glass with the given refractive index.
       pub fn new(refractive_index: f64) -> Self {
           Dielectric { refractive_index }
       }
   }

   impl Material for Dielectric {
       fn scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)> {
           let unit_direction = ray_in.direction.unit();
           let eta_ratio = if hit.front_face {
               1.0 / self.refractive_index
           } else {
               self.refractive_index
           };
           let cos_theta = (-unit_direction.dot(hit.normal)).min(1.0);
           let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
           let direction = if eta_ratio * sin_theta > 1.0
               || rng.next_f64() < schlick(cos_theta, eta_ratio)
           {
               reflect(unit_direction, hit.normal)
           } else {
               refract(unit_direction, hit.normal, eta_ratio)
           };
           Some((Colour::new(1.0, 1.0, 1.0), Ray::new(hit.point, direction)))
       }
   }
   ```
   `eta_ratio` is the ratio of the index the ray leaves to the index it enters. On a front face the ray enters glass, so it is `1.0 / refractive_index`; on a back face it leaves glass, so it is `refractive_index`.
6. `src/material.rs:58-61`, below `reflect` - add the two private helpers:
   ```rust
   /// The refracted direction of a unit ray `v` crossing a surface with unit normal `n`, from a
   /// medium with index 1 into one with index `1 / eta_ratio`.
   fn refract(v: Vec3, n: Vec3, eta_ratio: f64) -> Vec3 {
       let cos_theta = (-v.dot(n)).min(1.0);
       let perpendicular = (v + n * cos_theta) * eta_ratio;
       let parallel = n * -(1.0 - perpendicular.dot(perpendicular)).max(0.0).sqrt();
       perpendicular + parallel
   }

   /// Schlick's approximation of the reflectance at `cosine` for index ratio `eta_ratio`.
   fn schlick(cosine: f64, eta_ratio: f64) -> f64 {
       let r0 = (1.0 - eta_ratio) / (1.0 + eta_ratio);
       let r0 = r0 * r0;
       r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
   }
   ```
7. `src/material.rs:84-91` - change the test helper to the constructor and a self-consistent point:
   ```rust
   fn record_with(ray: &Ray, outward_normal: Vec3, material: Rc<dyn Material>) -> HitRecord {
       HitRecord::new(ray, 1.0, ray.at(1.0), outward_normal, 0.0, 0.0, material)
   }
   ```
8. `src/material.rs:93-107` and `109-124` - in `a_diffuse_material_keeps_its_albedo_and_scatters_away_from_the_surface` and `a_metal_with_no_fuzz_reflects_about_the_normal`, build `ray` before `hit` and call `record_with(&ray, Vec3::new(0.0, 0.0, 1.0), Rc::new(...))`. Keep every assertion.
9. `src/material.rs:126-133` - replace `a_metal_reflection_into_the_surface_is_absorbed` with a fuzz case, because with a ray-facing normal an unfuzzed reflection always points away from the surface. Use direction `Vec3::new(1.0, 0.0, -0.05)` for a grazing hit, `Metal::new(albedo, 1.0)`, and loop up to 50 draws until `scatter` returns `None`; assert that it did. Rename it `a_fuzzy_metal_reflection_into_the_surface_is_absorbed`.
10. `src/material.rs`, after the metal tests - add the two glass tests named under Tests, built with `record_with` and `Rc::new(Dielectric::new(1.5))`.
11. `src/sphere.rs:62-75` - keep the existing assertions and add `assert!(hit.front_face);`, `assert!((hit.u - 0.25).abs() < 1e-9);` and `assert!((hit.v - 0.5).abs() < 1e-9);`.
12. `src/sphere.rs`, after the miss test - add `a_ray_from_inside_hits_the_back_face` named under Tests.
13. `src/main.rs:16` - change the import to `use material::{Dielectric, Lambertian, Metal};`.
14. `src/main.rs:66-70`, after the metal sphere in `scene()` - add:
    ```rust
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.1, 0.0, -1.5),
        0.5,
        Rc::new(Dielectric::new(1.5)),
    )));
    ```
15. `docs/` - no page changes. `docs/requirements.md` item 4 already states the glass behaviour and item 7 already states the face flag and the ray-facing normal. `README.md` does not enumerate the scene. Re-read both after the code change and confirm neither is stale.

**Callers / wiring to update:**
- `src/sphere.rs:45` - the only production construction of `HitRecord`; it moves to `HitRecord::new` and passes `ray`, `t`, `point`, `outward_normal`, `u`, `v` and the cloned material.
- `src/material.rs:85` - the test helper construction moves to `HitRecord::new`; the three existing tests are its only users.
- `src/main.rs:16` and `src/main.rs:54-72` - import `Dielectric` and add the glass sphere.
- `src/main.rs:29-46` - no change; `ray_colour` reaches the material through `hit.material`.
- `src/hittable.rs:36-44` - no change; the nearest-distance borrow already compiles with the non-`Copy` record.
- No other construction site: `grep -rn "HitRecord\s*{" src/` finds `src/hittable.rs:11`, `src/sphere.rs:45` and `src/material.rs:85` only.

**Config / schema / migrations:**
- None. The refractive index is a material field set to `1.5` at the scene, not a configuration key.
- Retained records and their named consumer/owner: none. `front_face`, `u` and `v` are in-memory hit data, not stored records.

**Touched files:**
<!-- touched-files:start -->
- `src/hittable.rs` - changed: adds `front_face`, `u`, `v`, the ray-facing normal and `HitRecord::new`.
- `src/sphere.rs` - changed: computes (u, v) by the shared formula and builds the record through `HitRecord::new`; tests assert the face side and coordinates.
- `src/material.rs` - changed: adds `Dielectric`, `refract` and `schlick`; moves the existing tests to `HitRecord::new` and adds two glass tests.
- `src/main.rs` - changed: imports `Dielectric` and adds one glass sphere to the scene.
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not touch `src/plane.rs`, `src/triangle.rs`, `src/vec3.rs`, `src/camera.rs`, `Cargo.toml` or any plan-b file. The geometry plan owns planes, triangles, bounding boxes and `Vec3::cross`.
- Do not add a second `Material` trait, a material placeholder field, a material-less construction path or an `Aabb`.
- Do not change `Material::scatter`, `Lambertian` or `Metal` behaviour; only their tests move to the constructor.
- `HitRecord.normal` always faces against `ray.direction`. Compute `front_face` as `ray.direction.dot(outward_normal) < 0.0` in the one constructor, and never flip a normal anywhere else.
- Glass derives entering from leaving from `hit.front_face` alone and adds no face flag of its own. It returns unit attenuation `Colour::new(1.0, 1.0, 1.0)` and never `None`.
- Do not clamp or validate `refractive_index`, and do not clamp a scattered direction. The only numeric guard is the `max(0.0)` inside `refract`.
- Keep the seeded generator as the only randomness, and draw exactly one value per glass scatter, only for the Schlick choice. Do not reseed.
- Keep rendering single-threaded, keep `unsafe` absent, and add no dependency.
- Do not add version or history words, plan names or phase ids to code comments.
- Do not change the bounce limit, the scene's other three spheres, gamma or the PPM writer.

**Tests:**
- New: `src/material.rs` - `a_glass_ray_inside_reflects_on_total_internal_reflection`: a unit ray `(0.8, 0.0, 0.6)` recorded against outward normal `(0.0, 0.0, 1.0)` is a back face, so `normal` is `(0.0, 0.0, -1.0)` and `eta_ratio` is `1.5`; `1.5 * 0.8 > 1.0` reflects. Assert unit attenuation and direction `(0.8, 0.0, -0.6)` within `1e-9`.
- New: `src/material.rs` - `a_glass_ray_bends_by_snell_when_it_enters`: a unit ray `(0.6, 0.0, -0.8)` recorded against outward normal `(0.0, 0.0, 1.0)` is a front face; `eta_ratio` is `1.0 / 1.5`, the refracted direction is `(0.4, 0.0, -0.916515…)`. Assert unit attenuation and that direction within `1e-9`. With `Rng::new(7)` the first draw is `0.8202`, above the Schlick reflectance `0.0403`, so the refraction branch is the stable outcome.
- New: `src/sphere.rs` - `a_ray_from_inside_hits_the_back_face`: `Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5, <Lambertian>)` hit from origin `(0.0, 0.0, -2.0)` along `(0.0, 0.0, -1.0)` returns `t` 0.5, `!hit.front_face`, normal `(0.0, 0.0, 1.0)`, `v` 0.0 and `u` 0.75.
- Changed: `src/sphere.rs` - `a_ray_through_the_center_hits_the_near_surface` keeps its `t` and normal assertions and adds `front_face`, `u` 0.25 and `v` 0.5.
- Changed: `src/material.rs` - the three existing tests and `record_with` use `HitRecord::new`; the absorbed test now uses `Metal::new(albedo, 1.0)` at a grazing angle. No assertion is weakened except the absorbed test's fuzz, which is required for the branch to be reachable with a ray-facing normal.
- Deleted: none. Three new test functions, four adapted tests, no deletion, within the REPO-RULES §3a maximum of three new functions.
- Fast gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image, required because a material, the record and the scene change. Run `cargo run --release > /tmp/opencode/render.ppm`, then convert it with the PNG snippet from `phases/A3-samples-per-pixel.md` and open `/tmp/opencode/render.png`; confirm the left sphere shows the background refracted and inverted through it, the diffuse and metal spheres still render, and no pixel is black or `NaN` grey.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** agent.
- Run the rendered-image conditional gate above and inspect `/tmp/opencode/render.png`: a glass sphere on the left of the centre, a diffuse red-brown centre sphere, a diffuse grey ground sphere and a grey metal sphere on the right. The plan's final user acceptance of the image is Z1.

**Completion checklist (gate):**
- [ ] Gate level `standard` requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `HitRecord` has `t`, `point`, the ray-facing `normal`, `front_face`, `u`, `v` and `material`, and `HitRecord::new` is the only production construction path.
- [ ] `Sphere` computes `(u, v)` by the shared formula on both roots and records the face side.
- [ ] `Dielectric` refracts by Snell, reflects on total internal reflection, chooses by Schlick with the seeded generator and returns unit attenuation.
- [ ] The scene has one glass sphere of refractive index 1.5 and the other three spheres are unchanged.
- [ ] The six named tests pass; the test budget is respected and the receipt reports functions added/deleted.
- [ ] Fast phase gate passes, and the rendered-image conditional gate was run once and the image inspected.
- [ ] `docs/requirements.md` and `README.md` were re-read and need no change.
- [ ] `Cargo.toml` has no dependency, the seeded generator remains the only randomness, and no plan-b file changed.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the agent user test has passing evidence.
- [ ] On passing evidence: compress this file into `done/A2-glass.md` (frozen contract only) and flip the `full-plan.md` status row to `done`.

**Exit:** a glass sphere in the scene refracts its background, reflects at grazing angles inside the glass, and every hit record carries its face side and texture coordinates with the normal against the ray.
