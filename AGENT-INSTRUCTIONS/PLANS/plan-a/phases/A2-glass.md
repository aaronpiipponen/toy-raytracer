## Phase A2 - Glass

**Status:** active.
**Depends on:** A1, frozen in `done/A1-diffuse-and-metal-materials.md`, and the shared face-side
design in `PLANS/shared/plan-a-A1+plan-b-B1.md`.
**Scope:** add `Dielectric` to `src/material.rs` with Snell's law refraction, total internal
reflection and Schlick's approximation, deriving the face side locally from the outward normal, and
add one glass sphere to `scene()`.
**Gate level:** standard - bounded runtime behaviour in an image renderer, no durable state, config,
security or data-access seam; the material tests, the fast gate, the rendered-image conditional gate
and the repository-map update are enough.
**Simplicity statement:** this phase adds exactly one material and one scene object. It does not add
a face flag, texture coordinates, a new shape, sampling or jitter, a command line, PNG output,
fresnel configurability, or any clamp or validation of the refractive index. The face-side
derivation is two lines in `Dielectric::scatter`; no second scatter path or per-object special case
appears in `src/main.rs`, and no other material changes.

**Current state (verified):**
- `src/material.rs:8-11` - `pub trait Material` with
  `fn scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)>`.
- `src/material.rs:13-29` - `Lambertian { pub albedo: Colour }`, `Lambertian::new(albedo)`.
  `src/material.rs:31-53` - `Metal { pub albedo: Colour, pub fuzz: f64 }`,
  `Metal::new(albedo, fuzz)`. `src/material.rs:55-57` - `fn reflect(v: Vec3, n: Vec3) -> Vec3`.
  `src/material.rs:59-72` - `fn random_unit_vector(rng: &mut Rng) -> Vec3`.
  `src/material.rs:74-133` - the tests module, which imports `std::rc::Rc` and `super::*`.
- `src/hittable.rs:8-17` - `HitRecord { t, point, normal, material }`; its doc comment says
  `normal` is the outward normal on this branch. Any number of records may be built as literals
  while this branch is un-merged.
- `src/sphere.rs:10-24` - `Sphere { center, radius, material }`, `Sphere::new(center, radius, material)`.
- `src/main.rs:16` - `use material::{Lambertian, Metal};`.
  `src/main.rs:54-72` - `scene()` adds the centre diffuse sphere, the ground diffuse sphere whose top
  is the plane `y = -0.5`, and the metal sphere.
- `src/main.rs:27-46` - `ray_colour` follows `hit.material.scatter` for `MAX_BOUNCES = 50`;
  `src/main.rs:90-92` - `to_byte` applies gamma 2.
- `src/rng.rs:18-20` - `pub fn next_f64(&mut self) -> f64` in 0..1. The first value from
  `Rng::new(7)` is `0.820247`.
- `src/vec3.rs` - `Vec3::new`, `dot`, `unit`, `length`; operators `Add`, `Sub`, `Neg`, `Mul<f64>`,
  `Mul<Vec3>`, `Div<f64>`; there is no `f64 * Vec3`, so write the vector first.
- `src/camera.rs:14-21` - the viewport has height 2 and sits at `z = -1`, so
  `direction = (2u - 1, 2v - 1, -1)`: a point at depth `|z|` is inside the frame when
  `|x| < |z|` and `|y| < |z|`.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` - the repository map line reads
  ``material.rs` (the `Material` trait and the diffuse and metal materials)`.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md` - hard invariants: no dependency, reproducible
  randomness from `src/rng.rs` under a fixed seed, single-threaded, no `unsafe`; materials shared as
  `Rc<dyn Material>`.
- `docs/requirements.md` item 4 - the glass behaviour this phase implements: Snell's law, total
  internal reflection, Schlick's approximation, no attenuation.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B1.md` - the face-side rule and the exact lines
  the release merge replaces.
- `src/material.rs`, `src/hittable.rs`, `src/main.rs`, `src/rng.rs`, `src/vec3.rs` - the signatures
  above; use them exactly and add nothing else.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate, the rendered-image conditional gate and its
  command.
- `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md` - the prose rules for the doc comments
  and test names this phase writes.

**Build plan:**
1. `src/material.rs` - add `Dielectric` between `Metal` and the private helpers (order: trait,
   `Lambertian`, `Metal`, `Dielectric`, `reflect`, `refract`, `schlick_reflectance`,
   `random_unit_vector`, tests module):
   - `/// A transparent surface that refracts by its refractive index.`
     `pub struct Dielectric { pub refraction_index: f64 }`.
   - `impl Dielectric` with
     `/// Create a dielectric material with the given refractive index.`
     `pub fn new(refraction_index: f64) -> Self { Dielectric { refraction_index } }`.
   - `impl Material for Dielectric` with this exact body:
     ```rust
     fn scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)> {
         let front_face = ray_in.direction.dot(hit.normal) < 0.0;
         let normal = if front_face { hit.normal } else { -hit.normal };
         let refraction_ratio = if front_face {
             1.0 / self.refraction_index
         } else {
             self.refraction_index
         };
         let unit_direction = ray_in.direction.unit();
         let cos_theta = (-unit_direction).dot(normal).min(1.0);
         let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
         let cannot_refract = refraction_ratio * sin_theta > 1.0;
         let direction = if cannot_refract
             || schlick_reflectance(cos_theta, refraction_ratio) > rng.next_f64()
         {
             reflect(unit_direction, normal)
         } else {
             refract(unit_direction, normal, refraction_ratio)
         };
         Some((Vec3::new(1.0, 1.0, 1.0), Ray::new(hit.point, direction)))
     }
     ```
     The face side comes only from that first line and the flip on the second; `hit.normal` stays
     the outward normal, and no face flag is added to `HitRecord`. `ray_colour` in `src/main.rs`
     passes the incoming ray as `ray_in`. Attenuation is white, so glass does not tint (requirement
     4). The `||` order matters: it skips the random draw when the ray cannot refract out.
   - Add these two private helpers next to `reflect`:
     ```rust
     fn refract(unit_direction: Vec3, normal: Vec3, refraction_ratio: f64) -> Vec3 {
         let cos_theta = (-unit_direction).dot(normal).min(1.0);
         let perpendicular = (unit_direction + normal * cos_theta) * refraction_ratio;
         let parallel = normal * -(1.0 - perpendicular.dot(perpendicular)).abs().sqrt();
         perpendicular + parallel
     }

     /// Return Schlick's reflection share at `cos_theta` for an index ratio of `refraction_ratio`.
     fn schlick_reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
         let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
         r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
     }
     ```
     Do not add any other helper, clamp or random source. `reflect` and `random_unit_vector` are
     unchanged.
   - Carry-over from A1: add the missing doc comments on the public items while editing this file:
     `Material::scatter` gets
     `/// Scatter an incoming ray, returning the attenuation and the scattered ray, or `None` when the surface absorbs the ray.`
     (prose, wrapped), and `Lambertian::new` and `Metal::new` get one imperative sentence each
     (`/// Create a matte material with the given albedo.`, `/// Create a metal material with the given albedo and fuzz factor in 0..=1.`).
     No other existing line changes.
2. `src/main.rs`:
   - `src/main.rs:16` becomes `use material::{Dielectric, Lambertian, Metal};`.
   - `scene()` appends one sphere after the metal sphere, before `world`:
     ```rust
     world.add(Box::new(Sphere::new(
         Vec3::new(-0.85, -0.1, -1.3),
         0.4,
         Rc::new(Dielectric::new(1.5)),
     )));
     ```
     The centre `y = -0.1` puts the bottom at `y = -0.5`, on the ground; the distance to the centre
     sphere's centre is `sqrt(0.85^2 + 0.1^2 + 0.3^2) = 0.907`, above the `0.9` sum of the radii, and
     the widest point `x = -1.25` at depth `1.3` is inside the frame. No other scene line changes.
3. `AGENT-INSTRUCTIONS/LOCAL/project.md` - load the `agent-instructions` skill first (edits under
   `AGENT-INSTRUCTIONS/` outside `PLANS/` are held otherwise), then change the `src/` map line entry
   from ``material.rs` (the `Material` trait and the diffuse and metal materials)`` to
   ``material.rs` (the `Material` trait and the diffuse, metal and glass materials)``. The rest of
   the line and file is unchanged.

**Callers / wiring to update:**
- `src/main.rs:16` import line gains `Dielectric`; nothing else imports `material`.
- `src/main.rs:54` `scene()` gains the one glass sphere; `render`, `ray_colour`, `sky_colour`,
  `to_byte` and `write_ppm` are unchanged.
- `src/material.rs` already exposes `reflect` and `random_unit_vector` to the new material in the
  same module; no visibility change and no import change is needed there (the tests module picks up
  `Dielectric` through `use super::*;`).
- No other caller of `Sphere::new`, `HitRecord` or `Material::scatter` exists (grep the tree).

**Config / schema / migrations:**
- None: no configuration key, migration, generated file or command-line option. The refractive index
  `1.5` is a scene literal in `scene()`.
- Retained records and their named consumer/owner: none. Nothing is logged, stored or hashed; the
  render returns pixels to `write_ppm` and exits.

**Touched files:**
<!-- touched-files:start -->
- `src/material.rs` - changed: `Dielectric`, `refract`, `schlick_reflectance`, three tests and the carry-over doc comments.
- `src/main.rs` - changed: the `Dielectric` import and the glass sphere in `scene()`.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` - changed: the repository map names the glass material.
- Adds: nothing (no dependency, config key, migration or generated file).
<!-- touched-files:end -->

**Rules / MUST NOT:**
- No dependency in `Cargo.toml`; standard library only.
- All randomness goes through the one `Rng` in `render`, created once with `SEED`; glass draws only
  the one `next_f64` inside `scatter`. Never add a second generator or re-seed.
- Single-threaded, no `unsafe`.
- No `front_face` field on `HitRecord`, no second face helper, no face flag on `Sphere`; the local
  derivation inside `Dielectric::scatter` is the only one, and the release merge replaces it.
- No texture coordinates, new shape, bounding box, sampling loop or jitter, command line, PNG
  writer or progress output; later phases own those.
- No clamp, assert, validation or debug print of `refraction_index`, `cos_theta` or
  `refraction_ratio` beyond the `.min(1.0)` shown; no near-zero-direction guard.
- Do not change `Lambertian`, `Metal`, `ray_colour`, `to_byte` or the existing material tests.
- Permanent code comments and docs this phase writes never mention a plan, phase or merge.

**Tests:**
- New, in `src/material.rs` tests module, each building its `HitRecord` literal with
  `material: Rc::clone(&material)` where `material: Rc<dyn Material> = Rc::new(Dielectric::new(1.5))`,
  `t: 1.0` and `point: Vec3::new(0.0, 0.0, -1.0)`:
  1. `a_glass_ray_at_normal_incidence_passes_through_the_surface` - `Rng::new(7)`; outward normal
     `Vec3::new(0.0, 0.0, 1.0)`; ray from `Vec3::new(0.0, 0.0, 0.0)` along
     `Vec3::new(0.0, 0.0, -1.0)`. The first draw `0.820247` exceeds the `0.04` Schlick share, so the
     ray refracts. Assert the attenuation is `Vec3::new(1.0, 1.0, 1.0)` and each scattered direction
     component is within `1e-9` of `(0.0, 0.0, -1.0)`.
  2. `a_glass_ray_entering_at_an_angle_bends_toward_the_normal` - `Rng::new(7)`; same normal and
     point; ray direction `Vec3::new(0.8, 0.0, -0.6)`. Snell's law scales the tangential part by
     `1 / 1.5`, so the expected direction is
     `let x = 0.8 * (2.0 / 3.0); let z = -(1.0 - x * x).sqrt();` i.e. `(0.5333, 0.0, -0.8459)`. The
     entry reflectance is `0.04 + 0.96 * 0.4^5`, about `0.0498`, below the first draw, so the ray
     refracts. Assert the white attenuation and each direction component within `1e-9` of
     `(x, 0.0, z)`.
  3. `a_glass_ray_leaving_beyond_the_critical_angle_reflects_internally` - `Rng::new(7)`; outward
     normal `Vec3::new(0.0, 0.0, 1.0)`; ray direction `Vec3::new(0.8, 0.0, 0.6)`, so the ray leaves
     the surface. `sin_theta = 0.8` and `1.5 * 0.8 > 1.0`, so `cannot_refract` is true and no random
     value is drawn. Assert the white attenuation and each direction component within `1e-9` of
     `(0.8, 0.0, -0.6)`.
- Unchanged: all existing tests keep their assertions and bodies.
- Budget: 3 new test functions (the §3a cap; not a quota), 0 deleted.
- Fast gate: `~/.cargo/bin/cargo test material`, `~/.cargo/bin/cargo test`, then
  `~/.cargo/bin/cargo build`, `~/.cargo/bin/cargo clippy -- -D warnings`,
  `~/.cargo/bin/cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image - required, because the material and the scene changed; agent
  generation - not affected, not run.
- Slow gate (>2 minutes): none; the release render of 200x100 pixels finishes in seconds.

**User test (acceptance):**
- **Executor:** agent (all steps run from this worktree with the available tools).
- Rendered image and reproducibility:
  1. `~/.cargo/bin/cargo run --release > /tmp/opencode/a2.ppm` and again
     `~/.cargo/bin/cargo run --release > /tmp/opencode/a2-repeat.ppm`; `cmp /tmp/opencode/a2.ppm /tmp/opencode/a2-repeat.ppm`
     must print nothing (same bytes, requirement 6).
  2. Convert for viewing with the throwaway stdlib PPM to PNG script (`/tmp/opencode/ppm2png.py`,
     never committed): `python3 /tmp/opencode/ppm2png.py /tmp/opencode/a2.ppm /tmp/opencode/a2.png`,
     then open `/tmp/opencode/a2.png` with the file-read image viewer.
  3. Expected picture: the A1 scene (gradient sky, grey ground, matte red-brown centre sphere, small
     grey metal sphere on the right) plus a transparent glass sphere on the left resting on the
     ground. The glass sphere shows bright sky above and the grey ground below through its upper and
     lower edges, its interior view is bent and mirrored against the surroundings, and a bright rim
     marks its silhouette. No large black disk and no normal-gradient colours. The image stays
     noisy because one sample per pixel is rendered; A3 owns smooth sampling.
- If step 2's viewer is unavailable, say so and report the failure instead of claiming the image
  looks right; do not substitute unit tests for the visual check.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `src/material.rs` holds only the `Material` trait, `Lambertian`, `Metal`, `Dielectric`,
      `reflect`, `refract`, `schlick_reflectance` and `random_unit_vector`.
- [ ] Glass decides the face side from `ray_in.direction.dot(hit.normal) < 0.0`, uses the normal
      flipped to face the ray, and returns white attenuation; `HitRecord` still has no face flag.
- [ ] `scene()` holds the glass sphere at `(-0.85, -0.1, -1.3)` with radius `0.4` and index `1.5`,
      still with two diffuse spheres and the metal sphere.
- [ ] `Cargo.toml` still has no dependency and `src/` has no `unsafe`; the single seeded `Rng` and
      the 50-bounce loop and gamma 2 are unchanged.
- [ ] Named tests pass; the fast gate and the rendered-image conditional gate pass.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports 3 added and 0 deleted test
      functions.
- [ ] `docs/requirements.md` and `README.md` are still accurate (no wording change is needed), the
      repository map in `AGENT-INSTRUCTIONS/LOCAL/project.md` names the glass material, and no page
      or comment this phase writes refers to a plan.
- [ ] The A1 carry-over doc comments on `Material::scatter`, `Lambertian::new` and `Metal::new`
      exist.
- [ ] Safety commit on branch `plan-a`.
- [ ] Revision-bound verification receipt exists (base revision plus tested head SHA, command
      results, omitted conditional gates with reasons, test-budget line); user test evidence is the
      two-step render check above.
- [ ] On passing evidence: compress this file into `done/A2-glass.md` (frozen contract only) and flip
      the `full-plan.md` A2 row to `done`.

**Exit:** the scene holds a glass sphere that refracts by Snell's law, reflects on total internal
reflection and chooses reflection by Schlick's approximation without attenuating, the same seed
renders identical bytes, and the rendered image shows the glass sphere beside the matte and metal
spheres.
