## Phase A1r - hit record constructor and materials

**Status:** active.
**Depends on:** none. The starting renderer in `src/` is the only contract this phase consumes.
**Scope:** plan-a's side of the shared hit-record design plus the materials work: one `HitRecord::new`
constructor in `src/hittable.rs` taking `t`, `point`, `normal` and the material last, with `Clone` as
the only derive; the `as_ref()` nearest-distance read in `HittableList::hit`; `src/material.rs` with
the `Material` trait, `Lambertian` and `Metal`; the material on `HitRecord` and on `Sphere`; the
bounce loop in `ray_colour`; gamma 2 on output; the seeded generator; and the scene's two diffuse
spheres plus one metal sphere. Glass, samples per pixel, front faces, texture coordinates, bounding
boxes, the PNG writer, the command line and progress output stay out of this phase.
**Gate level:** standard. The phase changes the shading path of the one command the product has, with
no durable state, no external boundary and no security surface; targeted tests plus the fast gate and
the rendered-image conditional gate fit the risk.
**Simplicity statement:** a scatter direction is used as produced, with no guard for a
near-zero-length direction; `Metal` uses its fuzz value as given and does not clamp or validate it,
and a fuzz value outside `0.0..=1.0` simply produces the direction it produces; the constructor
carries only the four fields this branch's record has, and no code path builds a record any other
way. The normal-colour shading path and the stale `#[allow(dead_code)]` markers that the phase makes
unnecessary are deleted, not kept beside the new path.

**Current state (verified):**
- `src/main.rs:24` - `fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Colour` returns
  `(hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5` on a hit and a blue-white sky gradient on a miss.
  There is no material and no bounce.
- `src/main.rs:32` - `fn scene() -> HittableList` adds two `Sphere`s, `(0.0, 0.0, -1.0)` radius 0.5
  and `(0.0, -100.5, -1.0)` radius 100.0.
- `src/main.rs:40` - `fn render(width: usize, height: usize) -> Vec<Colour>` builds the scene and
  camera, iterates rows `(0..height).rev()` and columns `0..width`, and pushes one
  `ray_colour(&camera.ray(u, v), &world)` per pixel.
- `src/main.rs:54` - `fn to_byte(channel: f64) -> u8` returns
  `(255.999 * channel.clamp(0.0, 1.0)) as u8`, with no gamma correction.
- `src/main.rs:6-8` - a comment naming the plans above `#[allow(dead_code)]` and `mod rng;`. The
  marker exists because nothing calls the generator.
- `src/hittable.rs:7` - `#[derive(Clone, Copy, Debug)] pub struct HitRecord` holds `t: f64`,
  `point: Vec3`, `normal: Vec3`. Lines 10 to 12 carry a comment naming the materials plan and
  `#[allow(dead_code)]` above `point`. There is no constructor; `Sphere::hit` builds the record with
  a struct literal.
- `src/hittable.rs:37` - `HittableList::hit` reads `nearest.map_or(t_max, |hit| hit.t)`, which moves
  the record out of the `Option`.
- `src/sphere.rs:7` - `pub struct Sphere { pub center: Vec3, pub radius: f64 }` with
  `pub fn new(center: Vec3, radius: f64) -> Self`. `hit` returns
  `HitRecord { t, point, normal: (point - self.center) / self.radius }`.
- `src/rng.rs` - `pub struct Rng` with `Rng::new(seed: u64)`, `next_u64(&mut self) -> u64` and
  `next_f64(&mut self) -> f64` in `0..1`. Nothing calls it.
- `src/vec3.rs` - `Vec3::new`, `dot`, `length`, `unit`, `Mul<f64>`, `Mul<Vec3>`, `Neg`, `Sub`;
  `pub type Colour = Vec3`. There is no `length_squared`. `cross` carries `#[allow(dead_code)]` and
  its comment, and this phase does not touch the file.
- `src/material.rs` does not exist.

**Read first (do not invent):**
- `docs/requirements.md` - items 1, 2, 3, 5 and 6 are the contract of this phase; item 4 is A2 and
  items 7 to 13 are other plans.
- `src/main.rs` - the whole file: the two existing tests assert the normal-colour behaviour this
  phase removes or renames.
- `src/hittable.rs` - the whole file: the record, its new constructor and the `HittableList::hit`
  loop.
- `src/sphere.rs` - the whole file: `Sphere::new` gains an argument and the hit builds its record
  through the constructor.
- `src/rng.rs` - `Rng::new` and `next_f64`; the phase's only source of randomness.
- `src/vec3.rs` - `Vec3`, `Colour`, the operators the material maths uses.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image conditional gate, and
  the PPM redirect that stands in for the command line until the output plan lands.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md` - the agreed shared shape; this phase
  lands plan-a's side only and needs no plan-b code.

**Build plan:**

1. `src/hittable.rs` - the imports become `use std::rc::Rc;` in a `std` group, then
   `use crate::material::Material;`, `use crate::ray::Ray;` and `use crate::vec3::Vec3;`.
   `pub struct HitRecord` changes from `#[derive(Clone, Copy, Debug)]` to `#[derive(Clone)]`, keeps
   `t`, `point` and `normal`, and gains `pub material: Rc<dyn Material>` as its last field. Update
   the struct doc comment to name the material of the object hit. Delete the two-line comment and
   `#[allow(dead_code)]` above `point`, because the material code reads it. `Debug` is dropped
   because `dyn Material` does not implement it, and `Copy` is dropped because a shared material
   cannot be copied.
2. `src/hittable.rs` - add one constructor before `pub trait Hittable`, with the doc comment
   "Return a hit at distance `t` and `point`, with the outward `normal` and the material of the
   object hit.":
   ```rust
   impl HitRecord {
       pub fn new(t: f64, point: Vec3, normal: Vec3, material: Rc<dyn Material>) -> Self {
           HitRecord {
               t,
               point,
               normal,
               material,
           }
       }
   }
   ```
   This is the only place that builds a `HitRecord` value. `normal` is the surface's outward
   geometric normal on this branch, passed through unchanged: there is no ray argument, no
   face-side decision and no flipping of the normal against the ray. Those arguments and the
   `front_face`, `u` and `v` fields belong to plan-b and arrive at the merge. It carries no second
   constructor and no `Default`.
3. `src/hittable.rs`, `impl Hittable for HittableList` - change line 37 to
   `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);`, because `HitRecord` is no longer
   `Copy` and the value form would move the record out of the loop.
4. `src/material.rs` (create) - module header
   `//! Materials: how a ray scatters off a surface.` Then:
   - imports `crate::hittable::HitRecord`, `crate::ray::Ray`, `crate::rng::Rng` and
     `crate::vec3::{Colour, Vec3}`. No `Rc` import at module level.
   - `pub trait Material { fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)>; }`
     with a doc comment stating that it returns the attenuation and the scattered ray, and `None`
     when the ray is absorbed. Add the same one-sentence doc comment on the method.
   - `pub struct Lambertian { pub albedo: Colour }` with `pub fn new(albedo: Colour) -> Self` and a
     doc comment saying it scatters in a random direction around the normal and attenuates by its
     albedo. `scatter` names its unused ray parameter `_ray` and returns
     `Some((self.albedo, Ray::new(hit.point, hit.normal + random_unit_vector(rng))))`.
   - `pub struct Metal { pub albedo: Colour, pub fuzz: f64 }` with
     `pub fn new(albedo: Colour, fuzz: f64) -> Self`; the doc comment states the valid range
     `0.0..=1.0` for `fuzz`. `scatter` computes
     `let reflected = reflect(ray.direction.unit(), hit.normal);` and
     `let scattered = Ray::new(hit.point, reflected + random_unit_vector(rng) * self.fuzz);`, then
     returns `None` when `scattered.direction.dot(hit.normal) <= 0.0` and
     `Some((self.albedo, scattered))` otherwise.
   - private `fn reflect(direction: Vec3, normal: Vec3) -> Vec3` returning
     `direction - normal * (2.0 * direction.dot(normal))`.
   - private `fn random_unit_vector(rng: &mut Rng) -> Vec3` looping until it draws a point inside
     the unit ball and returns that point's `unit()`: each attempt builds
     `Vec3::new(2.0 * rng.next_f64() - 1.0, 2.0 * rng.next_f64() - 1.0, 2.0 * rng.next_f64() - 1.0)`
     and accepts it when `candidate.length() < 1.0`. A rejected attempt draws three values. The
     `length` and `unit` methods already exist on `Vec3`.
   - `#[cfg(test)] mod tests` with the two material tests listed under **Tests**. The test module
     imports `std::rc::Rc` and `crate::material`'s own items through `use super::*`, and builds each
     `HitRecord` with `HitRecord::new`, never with a struct literal, because `Metal` reads only the
     hit's point and normal.
5. `src/sphere.rs` - add `use std::rc::Rc;` and `use crate::material::Material;`.
   `pub struct Sphere` keeps `center` and `radius` and gains `pub material: Rc<dyn Material>`.
   `new` changes from `pub fn new(center: Vec3, radius: f64) -> Self` to
   `pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self`.
   The hit's struct literal is replaced by
   `Some(HitRecord::new(t, point, (point - self.center) / self.radius, Rc::clone(&self.material)))`.
   Both existing tests pass a material of their own (see **Tests**).
6. `src/main.rs` - the module list gains `mod material;` between `mod hittable;` and `mod ray;`.
   Delete the comment line above `mod rng;` on line 6 and the `#[allow(dead_code)]` on line 7.
   Imports gain `use std::rc::Rc;` in the `std` group, and `use material::{Lambertian, Metal};` and
   `use rng::Rng;` in the crate group, keeping the existing imports.
7. `src/main.rs` - add `const MAX_BOUNCES: usize = 50;` and `const RNG_SEED: u64 = 2024;` after the
   existing `WIDTH` and `HEIGHT` constants. `ray_colour` changes from
   `fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Colour` to
   `fn ray_colour(ray: Ray, world: &dyn Hittable, rng: &mut Rng) -> Colour`. Its body becomes:
   - `let mut attenuation = Colour::new(1.0, 1.0, 1.0);` and `let mut ray = ray;`;
   - `for _ in 0..MAX_BOUNCES { ... }` where the body calls
     `world.hit(&ray, 0.001, f64::INFINITY)`;
   - on `Some(hit)`, `hit.material.scatter(&ray, &hit, rng)` yields
     `Some((factor, scattered))`, which sets
     `attenuation = attenuation * factor; ray = scattered;`, and `None`, which returns
     `Colour::new(0.0, 0.0, 0.0)`;
   - on `None`, compute the existing sky gradient from `ray.direction.unit().y` (the two lines at
     `src/main.rs:28` and `src/main.rs:29` move into this arm unchanged) and return
     `attenuation * sky`;
   - after the loop, return `Colour::new(0.0, 0.0, 0.0)`, which makes a ray that uses all 50 bounces
     black.
8. `src/main.rs` - `render` creates `let mut rng = Rng::new(RNG_SEED);` once before the row loop and
   calls `ray_colour(camera.ray(u, v), &world, &mut rng)`. One generator, one seed, pixel order
   `(0..height).rev()` then `0..width`, so every run produces the same sequence.
9. `src/main.rs` - `to_byte` applies gamma 2 before quantizing:
   `(255.999 * channel.clamp(0.0, 1.0).sqrt()) as u8`.
10. `src/main.rs` - `scene` gives the centre sphere
    `Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))`, the ground sphere
    `Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))`, and adds
    `world.add(Box::new(Sphere::new(Vec3::new(1.1, -0.1, -1.0), 0.4, Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3)))));`
    after the ground sphere. Those coordinates put the metal sphere right of the diffuse sphere and
    above the ground surface at `y = -0.5`.
11. `src/main.rs` - doc comments follow `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md`:
    `MAX_BOUNCES` and `RNG_SEED` carry none (their names say it), `ray_colour` carries a
    one-sentence doc comment saying it returns the colour seen along a ray after its bounces with
    the sky gradient on a miss, `render` keeps its existing header sentence, and the file header
    `//! Render a fixed scene and write it to standard output as a PPM image.` stays true and
    unchanged.
12. No `docs/` page is created or changed. `docs/requirements.md` already states this behaviour and
    `README.md` keeps `cargo run > image.ppm` accurate, so neither needs an edit; verify both by
    reading them before claiming the docs step complete.

**Callers / wiring to update:**
- `src/main.rs:34` and `src/main.rs:35` - both `Sphere::new` calls gain a third argument.
- `src/main.rs:48` - the `ray_colour` call gains the generator and passes the ray by value.
- `src/main.rs:26` - the normal-colour return is deleted; nothing else reads `hit.normal` for
  display.
- `src/sphere.rs:37-41` - the record struct literal becomes `HitRecord::new(...)`.
- `src/sphere.rs:51` and `src/sphere.rs:63` - both existing tests gain a material argument.
- `src/hittable.rs:37` - `map_or` becomes `as_ref().map_or`.
- `src/main.rs:7` and `src/hittable.rs:11` - the two `#[allow(dead_code)]` markers and their comments
  are removed.
- `src/material.rs` - every `HitRecord` in its tests is built through `HitRecord::new`.

**Config / schema / migrations:**
- None. No configuration key, no migration, no generated file.
- Retained records and their named consumer/owner: none. The hit record is per-ray working data and
  nothing is written to a store.

**Touched files:**
<!-- touched-files:start -->
- `src/material.rs` - created: `Material` trait, `Lambertian`, `Metal`, `reflect`, `random_unit_vector`, and their tests.
- `src/hittable.rs` - changed: `HitRecord` gains `material` and one `new` constructor, loses `Copy` and `Debug`; `HittableList::hit` reads the record by reference.
- `src/sphere.rs` - changed: `Sphere` stores a material, `new` takes it, `hit` builds the record through the constructor, tests adapted.
- `src/main.rs` - changed: material module, bounce loop, gamma 2 output, seeded generator, scene materials, tests adapted.
- Adds: nothing (no dependency, no config key, no migration, no generated file).
<!-- touched-files:end -->

**Rules / MUST NOT:**
- `Cargo.toml` gains no dependency. Use `std` only: `Rc`, `f64` maths. No `rand`, no threads.
- Do not add `unsafe`. Rendering stays single-threaded.
- Do not add a second `HitRecord` constructor and do not build a `HitRecord` with a struct literal
  outside `HitRecord::new`, in product code or in a test. `HitRecord::new` is the only construction.
- Do not add `front_face`, `u`, `v`, a ray argument, or an outward-normal argument with face
  flipping to the constructor or the record. The constructor's `normal` is the outward geometric
  normal passed through unchanged. The face-side arguments and fields belong to plan-b and arrive
  at the merge.
- Do not keep the record `Copy` or add `Debug` as a `Material` supertrait. `Rc<dyn Material>` on the
  record is the agreed contract, so `HitRecord` is `Clone` only and `HittableList::hit` borrows it.
- Do not store `&dyn Material` or a boxed material; shared materials are `Rc<dyn Material>` and
  objects clone the record's `Rc`.
- Do not add `#[allow(dead_code)]` to silence a warning. After this phase every field of
  `HitRecord` is read by the non-test build, so `src/hittable.rs` carries none and `mod rng;`
  carries none. If the change leaves an item unused, delete the item or wire it.
- Do not reseed or re-create `Rng` inside `ray_colour`, per pixel or per bounce. One generator per
  render with `RNG_SEED` keeps requirement 6 true.
- Do not implement glass, refraction, samples per pixel, jitter, front-face flags, texture
  coordinates, bounding boxes, shadow rays, Russian roulette, or any PNG, command-line or progress
  work. Those belong to A2, A3, the geometry plan and the output plan.
- Do not clamp, gamma-correct or tone-map a `Colour` anywhere except `to_byte`; `render` returns
  linear colours.
- Keep `#[allow(dead_code)]` and its comment on `Vec3::cross` in `src/vec3.rs`; that function and
  its geometry-plan comment are outside this phase, and an existing comment this phase does not
  edit is not a reason to touch the file.
- No comment or doc comment this phase writes or edits mentions a plan, a phase or the history of
  the code. Prose follows `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md`.
- Do not touch `src/camera.rs`, `src/ray.rs`, `src/rng.rs` or `src/vec3.rs`.

**Tests:**
- New/changed/deleted, at most three new functions (REPO-RULES §3a):
  - `src/material.rs` - `a_diffuse_scatter_attenuates_by_its_albedo_and_stays_above_the_surface`:
    `Lambertian::new(Vec3::new(0.7, 0.2, 0.1))` with a hit built by
    `HitRecord::new(0.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Rc::new(Lambertian::new(Vec3::new(0.7, 0.2, 0.1))))`
    and a ray `Ray::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0))` returns attenuation
    equal to the albedo and a direction whose dot product with the normal is positive.
  - `src/material.rs` - `metal_absorbs_a_reflection_that_points_into_the_surface`:
    `Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.0)` with a hit built by
    `HitRecord::new(0.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))`
    and a ray direction `Vec3::new(0.0, 0.0, 1.0)` against that `(0.0, 0.0, 1.0)` normal returns
    `None`.
  - `src/main.rs` - `a_mid_grey_pixel_is_written_with_gamma_two`: `write_ppm` into a `Vec<u8>` with
    one pixel `Vec3::new(0.25, 0.25, 0.25)` and `1, 1` dimensions writes a body ending in
    `127 127 127\n`.
- Changed existing tests, no new functions:
  - `src/main.rs` - `the_center_pixel_shows_the_sphere_facing_the_camera` is renamed to
    `the_center_pixel_shows_the_diffuse_sphere`. It asserts the centre pixel after `render` has red
    as its largest channel and red above 0.1, because the centre sphere is a red-brown diffuse
    material: each bounce multiplies blue by at most 0.3 and red by at least 0.5 of a sky value of
    0.5 or a ground value of 0.8, so red dominates unless the ray uses the bounce limit. If that
    assertion fails for the chosen seed, the bounce loop is wrong: report the value and fix the
    loop, never weaken the assertion.
  - `src/main.rs` - `the_top_row_is_sky` stays as written and still holds; the top row misses the
    scene and the sky comparison survives gamma.
  - `src/sphere.rs` - `a_ray_through_the_center_hits_the_near_surface` and
    `a_ray_beside_the_sphere_misses` keep their assertions and pass
    `Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))` to `Sphere::new`. Their test module imports
    `std::rc::Rc` and `crate::material::Lambertian`.
- Fast gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`,
  `git diff --check`, called as `~/.cargo/bin/cargo` when `cargo` is not on `PATH`. The whole suite
  is the targeted run for this crate (one binary, finishing in well under a second); when one test
  fails, rerun it by node id, for example
  `cargo test tests::the_center_pixel_shows_the_diffuse_sphere`.
- Conditional gates: Rendered image, required because the render loop, both materials, the camera
  usage through `render` and the output values change. The single run is
  `cargo run --release > /tmp/opencode/a1r-render.ppm` (the command line is the output plan's work,
  so standard output is the image); the user test reuses that file and does not run the render a
  second time. No other conditional gate applies.
- Slow gate (>2 minutes): none. The release render of the fixed 200 by 100 image takes under a
  second.

**User test (acceptance):**
- **Executor:** agent. It runs the commands and opens the image.
- 1. Use the image already written by the rendered-image conditional gate at
  `/tmp/opencode/a1r-render.ppm`; do not render a second time.
- 2. Convert it with the throwaway script below (keep it in `/tmp/opencode`, never in the
  repository) and open the resulting PNG with the image reader:
  ```python
  import sys, zlib, struct
  data = open(sys.argv[1], "rb").read().split()
  w, h = int(data[1]), int(data[2])
  vals = [int(v) for v in data[4:]]
  raw = b"".join(b"\x00" + bytes(vals[r * w * 3:(r + 1) * w * 3]) for r in range(h))
  def chunk(t, b):
      return struct.pack(">I", len(b)) + t + b + struct.pack(">I", zlib.crc32(t + b) & 0xFFFFFFFF)
  png = (b"\x89PNG\r\n\x1a\n"
         + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0))
         + chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b""))
  open(sys.argv[2], "wb").write(png)
  ```
  Run it as `python3 /tmp/opencode/ppm2png.py /tmp/opencode/a1r-render.ppm /tmp/opencode/a1r-render.png`.
- 3. Observe in the opened image: a blue-white gradient sky above the horizon, a yellow-green ground
  plane filling the lower part, a matte red-brown sphere in the centre, and a grey metal sphere to
  its right that reflects the sky. Shading varies smoothly across both spheres and no sphere shows
  banded normal colours.
- 4. The image is visibly speckled with isolated black and coloured pixels, and neither sphere casts
  a shadow. Both are expected: each pixel traces one ray, and the phase adds no shadow ray. A3
  averages jittered samples and removes the speckle, so it is not to be fixed here.
- 5. The render at these settings places the two spheres about 55 pixels apart horizontally, with
  the metal sphere's bottom touching the ground plane and a strip of ground visible at the right
  edge.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `Material`, `Lambertian` and `Metal` exist in `src/material.rs` with the signatures in the
  build plan, and `Sphere` and `HitRecord` carry `Rc<dyn Material>`.
- [ ] `impl HitRecord` has exactly one `pub fn new(t: f64, point: Vec3, normal: Vec3, material: Rc<dyn Material>) -> Self`,
  and a grep for `HitRecord {` finds only that constructor's body.
- [ ] `ray_colour` bounces at most `MAX_BOUNCES` (50) times, multiplies the attenuation at each
  bounce, applies the sky only on a miss and returns black when the limit is reached.
- [ ] `render` owns one `Rng::new(RNG_SEED)` and passes it down; no other randomness source exists.
- [ ] `to_byte` applies the square root before quantizing.
- [ ] The stale `#[allow(dead_code)]` markers on `mod rng;`, on `HitRecord::point` and their comments
  are gone, `src/hittable.rs` carries no `#[allow(dead_code)]`, and the crate builds without
  warnings.
- [ ] The scene holds two diffuse spheres and one metal sphere with the values in step 10.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports three test functions added and
  three existing test functions changed, one of them renamed.
- [ ] Fast phase gate and the rendered-image conditional gate pass; the agent viewed the rendered
  image and it matches the **User test** description.
- [ ] `docs/requirements.md` and `README.md` were read and stay accurate; no `docs/` page refers to
  a plan.
- [ ] No dependency, no `unsafe`, no thread, no file outside the touched list.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the agent's rendered-image observation is recorded
  in it.
- [ ] On passing evidence: compress this file into
  `done/A1r-hit-record-constructor-and-materials.md` (frozen contract only), flip the `full-plan.md`
  `A1r` status row to `done`, and keep the `A1` row `not applicable` because A1r carries it.

**Exit:** every scene object carries a material, `HitRecord::new` is the only way a record is built,
a ray follows material scatters up to 50 bounces and turns black at the limit, output is gamma 2
corrected, and the rendered image shows a matte centre sphere, a matte ground and a metal sphere with
a blurred reflection.
