## Phase A1 - Diffuse and metal materials

**Status:** active.
**Depends on:** none (starting tree at `e180476`).
**Scope:** add `src/material.rs` with the `Material` trait, `Lambertian` and `Metal`; carry the hit
object's material on `HitRecord`; give `Sphere` a material and put it on its records; make the render
loop in `src/main.rs` follow scattered rays up to 50 bounces and apply gamma 2 when writing bytes; give
the scene two diffuse objects and one metal sphere, with all randomness from `src/rng.rs` under a fixed
seed.
**Gate level:** standard - bounded runtime behaviour (an image renderer), no durable state, config,
security or data-access seam; targeted material tests plus the fast gate, the rendered-image
conditional gate and the repository-map update are enough.
**Simplicity statement:** this phase handles exactly the two materials and one bounce loop. It does
not handle glass, sampling/jitter, face-side flags, texture coordinates, new shapes, bounding boxes,
reflection clamping, near-zero scattered directions, or any command line or PNG output (A2, A3, the
geometry plan and the output plan own those). Albedo and fuzz values are scene literals; nothing is
validated or clamped, and the sky gradient stays one expression. No defensive branch, fallback colour
or duplicate scatter path is added.

**Current state (verified):**
- `src/main.rs:1` - header `//! Render a fixed scene and write it to standard output as a PPM image.`
- `src/main.rs:24` - `fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Colour` colours a hit by
  `(hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5`, and a miss by the gradient
  `Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t` with `t = 0.5 * (ray.direction.unit().y + 1.0)`.
- `src/main.rs:32` - `fn scene() -> HittableList` adds `Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)`
  and `Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)`.
- `src/main.rs:40` - `fn render(width: usize, height: usize) -> Vec<Colour>` builds the scene, the
  camera and one colour per pixel via `ray_colour`; `src/main.rs:54` - `fn to_byte(channel: f64) -> u8`
  is `(255.999 * channel.clamp(0.0, 1.0)) as u8`.
- `src/main.rs:6-8` - `#[allow(dead_code)] mod rng;` with a comment mentioning "the plans".
- `src/main.rs:82` - tests `the_center_pixel_shows_the_sphere_facing_the_camera` (asserts the centre
  pixel's normal colour `center.z > 0.9`; that behaviour disappears this phase) and
  `the_top_row_is_sky`.
- `src/hittable.rs:7-14` - `#[derive(Clone, Copy, Debug)] pub struct HitRecord { pub t: f64, #[allow(dead_code)] pub point: Vec3, pub normal: Vec3 }`.
- `src/hittable.rs:34-42` - `HittableList::hit` does `let limit = nearest.map_or(t_max, |hit| hit.t);`,
  which needs `HitRecord: Copy`.
- `src/sphere.rs:7-15` - `pub struct Sphere { pub center: Vec3, pub radius: f64 }`,
  `pub fn new(center: Vec3, radius: f64) -> Self`.
- `src/sphere.rs:36-41` - builds `HitRecord { t, point, normal: (point - self.center) / self.radius }`.
- `src/sphere.rs:49-65` - two tests call `Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5)`.
- `src/rng.rs:3-22` - `pub struct Rng`, `Rng::new(seed: u64)`, `next_u64(&mut self) -> u64`,
  `next_f64(&mut self) -> f64` in 0..1.
- `src/vec3.rs` - `Vec3::new`, `dot`, `unit`, `length`; operators `Add`, `Sub`, `Neg`, `Mul<f64>`,
  `Mul<Vec3>`, `Div<f64>`; `pub type Colour = Vec3`; there is no `f64 * Vec3`, so write the scalar
  first inside `Vec3 * f64` order.
- `src/ray.rs:5-18` - `#[derive(Clone, Copy, Debug)] pub struct Ray { pub origin: Vec3, pub direction: Vec3 }`,
  `Ray::new`, `Ray::at`.
- `Cargo.toml` - `[dependencies]` is empty.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md` - hard invariants: no dependency, reproducible
  randomness from `src/rng.rs` under a fixed seed, single-threaded, no `unsafe`; one concept per
  module and materials shared as `Rc<dyn Material>`.
- `docs/requirements.md` items 1, 2, 3 and 6 - the material, bounce-limit and seed behaviour this
  phase implements, and item 5's gamma clause.
- `src/main.rs`, `src/hittable.rs`, `src/sphere.rs`, `src/rng.rs`, `src/vec3.rs`, `src/ray.rs` - the
  signatures above; use them exactly and add nothing else.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate, the rendered-image conditional gate and its
  command.
- `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md` - the prose rules for the new module
  header, doc comments and test names.

**Build plan:**
1. `src/material.rs` (create) - header `//! Materials: how a ray scatters when it hits a surface.`
   Contents:
   - `use std::rc::Rc;` is not needed here; import only what is used:
     `use crate::hittable::HitRecord; use crate::ray::Ray; use crate::rng::Rng; use crate::vec3::{Colour, Vec3};`
   - `/// A surface material: how an incoming ray scatters, or nothing when it is absorbed.`
     `pub trait Material { fn scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)>; }`
     - return value: `Some((attenuation, scattered_ray))`; attenuation is the colour multiplied into
       the running colour, `scattered_ray` starts at `hit.point`.
   - `pub struct Lambertian { pub albedo: Colour }` with `pub fn new(albedo: Colour) -> Self`.
     `impl Material for Lambertian`: attenuation `self.albedo`; direction
     `hit.normal + random_unit_vector(rng)`; ray `Ray::new(hit.point, direction)`; always `Some`.
     Do not normalise the summed direction and add no near-zero guard.
   - `pub struct Metal { pub albedo: Colour, pub fuzz: f64 }` with
     `pub fn new(albedo: Colour, fuzz: f64) -> Self`; doc comment says the fuzz factor is in 0..=1.
     `impl Material for Metal`: `let reflected = reflect(ray_in.direction.unit(), hit.normal);`
     `let scattered = reflected + random_unit_vector(rng) * self.fuzz;`
     `if scattered.dot(hit.normal) <= 0.0 { None } else { Some((self.albedo, Ray::new(hit.point, scattered))) }`.
     Do not clamp `fuzz`.
   - private helpers in this module:
     `fn reflect(v: Vec3, n: Vec3) -> Vec3 { v - n * (2.0 * v.dot(n)) }` and
     `fn random_unit_vector(rng: &mut Rng) -> Vec3` - rejection sampling: in a loop draw
     `Vec3::new(2.0 * rng.next_f64() - 1.0, 2.0 * rng.next_f64() - 1.0, 2.0 * rng.next_f64() - 1.0)`,
     compute `let len_squared = p.dot(p);`, and return `p.unit()` when `len_squared > 1e-12 && len_squared <= 1.0`.
     No other helper and no second random source.
2. `src/hittable.rs` - `pub trait Hittable` unchanged, but the record and the list change:
   - `HitRecord` becomes `pub struct HitRecord { pub t: f64, pub point: Vec3, pub normal: Vec3, pub material: Rc<dyn Material> }`
     with `use std::rc::Rc;` and `use crate::material::Material;`.
   - Delete the `#[derive(Clone, Copy, Debug)]` line and both the `#[allow(dead_code)]` and its
     "bounce rays the materials plan adds" comment on `point`; the point is now the scatter origin.
     Update the doc comment to name the material: "where a ray hit a surface: the distance along the
     ray, the point, the outward normal and the material of the object".
   - `HittableList::hit` keeps its logic but reads the limit without moving the record:
     `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);` (the only change from
     `nearest.map_or(...)`).
3. `src/sphere.rs` - `pub struct Sphere` gains `pub material: Rc<dyn Material>`;
   `pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self`. In `hit`, add
   `material: Rc::clone(&self.material)` to the returned `HitRecord`. Imports:
   `use std::rc::Rc; use crate::material::Material;`.
4. `src/main.rs` - wire the new module and the bounce loop:
   - module list: add `mod material;` (alphabetically before `mod ray;`); drop `#[allow(dead_code)]`
     from `mod rng;` and its plan-referencing comment, leaving the bare `mod rng;`.
   - imports: `use material::{Lambertian, Metal, Material};` (the trait must be in scope to call
     `scatter`), `use rng::Rng;`, and `use std::rc::Rc;`.
   - constants: keep `WIDTH`/`HEIGHT`; add `const SEED: u64 = 7;` and
     `const MAX_BOUNCES: usize = 50;`.
   - `ray_colour`'s doc comment `/// The colour seen along a ray: the surface normal where it hits, the sky gradient otherwise.`
     becomes `/// The colour a ray sees: its scattered path attenuated by each material, the sky gradient when it misses.`
     Then `ray_colour` becomes
     `fn ray_colour(ray: &Ray, world: &dyn Hittable, rng: &mut Rng) -> Colour`: start
     `let mut ray = *ray; let mut colour = Vec3::new(1.0, 1.0, 1.0);`, then `for _ in 0..MAX_BOUNCES`:
     on a hit (`world.hit(&ray, 0.001, f64::INFINITY)`), `match hit.material.scatter(&ray, &hit, rng)` -
     `Some((attenuation, scattered))` sets `colour = colour * attenuation; ray = scattered;`,
     `None` returns `Vec3::new(0.0, 0.0, 0.0)`; on a miss return
     `colour * sky_colour(&ray)`. After the loop return `Vec3::new(0.0, 0.0, 0.0)` (the bounce limit is black).
   - keep the existing gradient as one small private function used only here:
     `fn sky_colour(ray: &Ray) -> Colour` holding the current `t`/gradient expression unchanged; do
     not duplicate it inside the loop.
   - `render` creates `let mut rng = Rng::new(SEED);` once before the pixel loops and passes
     `&mut rng` to `ray_colour(&camera.ray(u, v), &world, &mut rng)`. No other change to the loops.
   - `to_byte` applies gamma 2 by square root:
     `(255.999 * channel.clamp(0.0, 1.0).sqrt()) as u8`. `write_ppm` is unchanged.
   - `scene()` adds materials with `Rc::new(...)`:
     - centre: `Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.3))))`;
     - ground: `Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.8))))`;
     - metal: `Sphere::new(Vec3::new(0.85, -0.25, -1.3), 0.25, Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.3)))`
       (radius 0.25 rests on the ground at y = -0.5 and does not intersect the centre sphere).
     Keep the add order above.
5. Tests - see the Tests section; update the two `Sphere::new` calls in `src/sphere.rs` and delete the
   obsolete centre-pixel test in `src/main.rs`.
6. `AGENT-INSTRUCTIONS/LOCAL/project.md` - repository map: load the `agent-instructions` skill first
   (edits under `AGENT-INSTRUCTIONS/` outside `PLANS/` are held otherwise), then add the new module to
   the `src/` line: `material.rs` (the `Material` trait and the diffuse and metal materials), keeping
   the rest of the line and file unchanged.

**Callers / wiring to update:**
- `src/main.rs:32` `scene()` - both existing `Sphere::new` calls gain a material; one metal sphere is added.
- `src/main.rs:40` `render()` - owns the one `Rng` and passes it to `ray_colour`.
- `src/main.rs:24` `ray_colour()` - its only caller is `render()`; both change together.
- `src/main.rs:82` tests - one test deleted; the sky test is unchanged.
- `src/hittable.rs:37` `HittableList::hit` - the `map_or` closure must borrow (`as_ref`).
- `src/sphere.rs:37` `Sphere::hit` - adds the material to `HitRecord`; `src/sphere.rs` tests update
  their `Sphere::new` calls.
- No other caller of `Sphere::new`, `HitRecord`, `ray_colour` or `to_byte` exists (grep the tree).

**Config / schema / migrations:**
- None: no configuration key, migration, generated file or CLI option. Scene values stay literals in
  `scene()`.
- Retained records and their named consumer/owner: none. Nothing is logged, stored or hashed; the
  render returns pixels to `write_ppm` and exits.

**Touched files:**
<!-- touched-files:start -->
- `src/material.rs` - created: the `Material` trait, `Lambertian`, `Metal`, `reflect` and `random_unit_vector`, with a tests module.
- `src/hittable.rs` - changed: `HitRecord` carries `Rc<dyn Material>`; the copy-free `map_or` borrow in `HittableList::hit`.
- `src/sphere.rs` - changed: `Sphere` holds and returns a material; tests pass one.
- `src/main.rs` - changed: `mod material`, bounce loop, `sky_colour`, seeded `Rng`, gamma in `to_byte`, scene materials and the metal sphere; one test deleted.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` - changed: repository map names `material.rs`.
- Adds: nothing (no dependency, config key, migration or generated file).
<!-- touched-files:end -->

**Rules / MUST NOT:**
- No dependency in `Cargo.toml`; standard library only (`std::rc::Rc`).
- All randomness goes through the one `Rng` in `render` via `next_f64`; never call an OS or time
  source, never re-seed per pixel, never add a second generator; keep the seed constant (`SEED = 7`).
- Single-threaded, no `unsafe`.
- No glass, no refractive index, no face flag, no texture coordinates, no new object type, no bounding
  box, no sampling loop or jitter, no command line, no PNG writer, no progress output - later phases own
  those.
- No second scatter path or per-material special case in `main.rs`; the loop knows only `Material`.
- Do not clamp, validate or debug-print albedo, fuzz or the bounce count; no near-zero-direction guard.
- Do not re-derive the sky colour in more than one place.
- Permanent code comments and docs never mention a plan or phase.

**Tests:**
- New/changed/deleted:
  - `src/material.rs` - `a_diffuse_material_keeps_its_albedo_and_scatters_away_from_the_surface`:
    one `Rng::new(7)`, a `HitRecord` with normal `Vec3::new(0.0, 0.0, 1.0)`, point
    `Vec3::new(0.0, 0.0, -1.0)` and its own `Lambertian`; three `scatter` calls assert attenuation
    equals the albedo and each scattered direction has a positive dot against the normal.
  - `src/material.rs` - `a_metal_with_no_fuzz_reflects_about_the_normal`: `Metal::new(albedo, 0.0)`,
    incoming direction `Vec3::new(0.0, 0.0, -1.0)`, normal `Vec3::new(0.0, 0.0, 1.0)`, asserts the
    attenuation equals the albedo and the scattered direction equals `Vec3::new(0.0, 0.0, 1.0)`
    (compare component-wise within `1e-9`).
  - `src/material.rs` - `a_metal_reflection_into_the_surface_is_absorbed`: `Metal::new(albedo, 0.0)`
    with incoming direction `Vec3::new(0.0, 0.0, -1.0)` and normal `Vec3::new(0.0, 0.0, -1.0)` (the
    reflected direction points into the surface) asserts `scatter` returns `None`.
  - Deleted: `src/main.rs` - `the_center_pixel_shows_the_sphere_facing_the_camera` (it asserted normal
    colouring, which this phase removes).
  - Adapted: `src/sphere.rs` - both tests keep their assertions and only pass
    `Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.8)))` to `Sphere::new`.
  - Kept unchanged: `src/main.rs` - `the_top_row_is_sky`; `src/rng.rs` - `the_same_seed_gives_the_same_sequence`.
  - Budget: 3 new test functions (the §3a cap; not a quota), 1 deleted; no other test is added.
- Fast gate: `~/.cargo/bin/cargo test material`, `~/.cargo/bin/cargo test`, then
  `~/.cargo/bin/cargo build`, `~/.cargo/bin/cargo clippy -- -D warnings`,
  `~/.cargo/bin/cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image - required, because the render loop, the colours and the scene
  changed; agent generation - not affected, not run.
- Slow gate (>2 minutes): none; the release render of 200x100 pixels finishes in seconds.

**User test (acceptance):**
- **Executor:** agent (all steps run from this worktree with the available tools).
- Rendered image and reproducibility:
  1. `~/.cargo/bin/cargo run --release > /tmp/opencode/a1.ppm` and again
     `~/.cargo/bin/cargo run --release > /tmp/opencode/a1-repeat.ppm`; `cmp /tmp/opencode/a1.ppm /tmp/opencode/a1-repeat.ppm`
     must print nothing (same bytes, requirement 6).
  2. Convert for viewing with a throwaway stdlib script in `/tmp/opencode` (never committed or added
     to the repository): write `/tmp/opencode/ppm2png.py` containing
     `import struct,sys,zlib` and a `P3` parser that reads the header and pixel integers, then writes
     an 8-bit RGB PNG (`IHDR`/`IDAT`/`IEND` chunks, `IDAT` = `zlib.compress` of each row prefixed by a
     zero filter byte, CRC via `zlib.crc32`); run
     `python3 /tmp/opencode/ppm2png.py /tmp/opencode/a1.ppm /tmp/opencode/a1.png` and open
     `/tmp/opencode/a1.png` with the file-read image viewer.
  3. Expected picture: a horizontal gradient sky that is white at the horizon and light blue above; a
     grey ground plane filling the lower half with a soft shadowless gradient; a matte red-brown
     sphere in the centre with smooth shading; a small grey metal sphere on the right resting on the
     ground, showing a sharp reflection of the sky above and the ground below. No normal-gradient
     colours, no pure-white blowout, no visible black speckles.
- If step 2's viewer is unavailable, say so and report the failure instead of claiming the image
  looks right; do not substitute unit tests for the visual check.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `src/material.rs` exists with only the `Material` trait, `Lambertian`, `Metal` and the two
      private helpers, and every object in `scene()` has a material.
- [ ] `HitRecord` carries `Rc<dyn Material>` and no longer derives `Copy`; `HittableList::hit` compiles
      with the borrowing `map_or`.
- [ ] A ray still bouncing after 50 scatterings returns black; gamma 2 is applied in `to_byte`; all
      randomness comes from the single `Rng` seeded with `SEED`.
- [ ] `Cargo.toml` still has no dependency and `src/` has no `unsafe`.
- [ ] Named tests pass; the fast gate and the rendered-image conditional gate pass.
- [ ] Test budget respected (REPO-RULES §3a); receipt reports 3 added and 1 deleted test function.
- [ ] `docs/requirements.md` and `README.md` are still accurate (no wording change is needed) and the
      repository map in `AGENT-INSTRUCTIONS/LOCAL/project.md` names `material.rs`; no page or comment
      refers to a plan.
- [ ] Safety commit on branch `plan-a`.
- [ ] Revision-bound verification receipt exists (base revision plus tested head SHA, command results,
      omitted conditional gates with reasons, test-budget line); user test evidence is the two-step
      render check above.
- [ ] On passing evidence: compress this file into `done/A1-diffuse-and-metal-materials.md` (frozen
      contract only) and flip the `full-plan.md` A1 row to `done`.

**Exit:** every object has a material, rays scatter by it for up to 50 bounces with gamma 2 output, the
same seed renders identical bytes, and the rendered image shows a matte centre sphere, a matte ground
and a metal sphere on the right.
