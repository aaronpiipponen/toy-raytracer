## Phase A1 - diffuse and metal materials

**Status:** active.
**Depends on:** none. The starting renderer in `src/` is the only contract this phase consumes.
**Scope:** a `Material` trait with two implementations, the material on the hit record and on
`Sphere`, the bounce loop in `ray_colour`, gamma 2 on output, and the scene's two diffuse spheres
plus one metal sphere. Glass, samples per pixel, front faces, texture coordinates, bounding boxes,
the PNG writer, the command line and progress output stay out of this phase.
**Gate level:** standard. The phase changes the shading path of the one command the product has,
with no durable state, no external boundary and no security surface; targeted tests plus the fast
gate and the rendered-image conditional gate fit the risk.
**Simplicity statement:** a scatter direction is used as produced, with no guard for a
near-zero-length direction and no retry that would consume extra random numbers; `Metal` uses its
fuzz value as given and does not clamp or validate it; the material interface carries only what
requirement 2 and requirement 3 need. The normal-colour shading path and the stale
`allow(dead_code)` markers that the phase makes unnecessary are deleted, not kept beside the new
path.

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
- `src/main.rs:6` - `mod rng;` carries `#[allow(dead_code)]`; `src/hittable.rs:11` carries
  `#[allow(dead_code)]` above `point`. Both markers exist because those items are unused.
- `src/hittable.rs:7` - `#[derive(Clone, Copy, Debug)] pub struct HitRecord` holds `t: f64`,
  `point: Vec3`, `normal: Vec3`. `HittableList::hit` reads `nearest.map_or(t_max, |hit| hit.t)` at
  line 37, which moves the record out of the `Option`.
- `src/sphere.rs:7` - `pub struct Sphere { pub center: Vec3, pub radius: f64 }` with
  `pub fn new(center: Vec3, radius: f64) -> Self`; `hit` returns
  `HitRecord { t, point, normal: (point - self.center) / self.radius }`.
- `src/rng.rs` - `pub struct Rng` with `Rng::new(seed: u64)`, `next_u64(&mut self) -> u64` and
  `next_f64(&mut self) -> f64` in `0..1`. Nothing calls it.
- `src/vec3.rs` - `Vec3::new`, `dot`, `length`, `unit`, `Mul<f64>`, `Mul<Vec3>`, `Neg`, `Sub`;
  `pub type Colour = Vec3`. There is no `length_squared`.

**Read first (do not invent):**
- `docs/requirements.md` - items 1, 2, 3, 5 and 6 are the contract of this phase; item 4 is A2 and
  items 7 to 13 are other plans.
- `src/main.rs` - the whole file: the two existing tests assert the normal-colour behaviour this
  phase removes.
- `src/hittable.rs` - `HitRecord` and `HittableList::hit`; the record loses `Copy` and the list must
  read it by reference.
- `src/sphere.rs` - `Sphere::new` gains an argument, so both existing tests change.
- `src/rng.rs` - `Rng::new` and `next_f64`; the phase's only source of randomness.
- `src/vec3.rs` - `Vec3`, `Colour`, the operators the material maths uses.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image conditional gate, and
  the PPM redirect that stands in for the command line until the output plan lands.

**Build plan:**

1. `src/material.rs` (create) - module header
   `//! Materials: how a ray scatters off a surface.` Then:
   - `use std::rc::Rc;` only where a test needs it, otherwise no import.
   - `pub trait Material { fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)>; }`
     with a doc comment stating that it returns the attenuation and the scattered ray, and `None`
     when the ray is absorbed.
   - `pub struct Lambertian { pub albedo: Colour }` with `pub fn new(albedo: Colour) -> Self`.
     `scatter` returns `Some((self.albedo, Ray::new(hit.point, hit.normal + random_unit_vector(rng))))`.
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
   - `#[cfg(test)] mod tests` with the two material tests listed under **Tests**.
2. `src/hittable.rs` - `use std::rc::Rc;` plus `use crate::material::Material;`.
   `pub struct HitRecord` changes from `#[derive(Clone, Copy, Debug)]` to `#[derive(Clone)]`, keeps
   `t`, `point` and `normal`, and gains `pub material: Rc<dyn Material>`. `Debug` is dropped because
   `dyn Material` does not implement it, and `Copy` is dropped because a shared material cannot be
   copied. Delete the two-line `#[allow(dead_code)]` and its comment above `point`, which the
   material code reads. In `impl Hittable for HittableList`, `hit` changes line 37 to
   `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);`, because `HitRecord` is no longer
   `Copy`.
3. `src/sphere.rs` - add `use std::rc::Rc;`, `use crate::material::Material;`.
   `pub struct Sphere` keeps `center` and `radius` and gains `pub material: Rc<dyn Material>`.
   `new` changes from `pub fn new(center: Vec3, radius: f64) -> Self` to
   `pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self`. The returned
   `HitRecord` gains `material: Rc::clone(&self.material)`. Both existing tests pass a material of
   their own (see **Tests**).
4. `src/main.rs` - `mod material;` in the module list, then `mod rng;` without its
   `#[allow(dead_code)]`. Delete the comment line above `mod rng;` (`// Seeded randomness for the
   scattering and sampling the plans add.`), because code carries no plan reference. Imports gain
   `use std::rc::Rc;`, `use material::{Lambertian, Metal};` and `use rng::Rng;` (keep the existing
   imports).
5. `src/main.rs` - add `const MAX_BOUNCES: usize = 50;` and `const RNG_SEED: u64 = 2024;`.
   `ray_colour` changes from `fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Colour` to
   `fn ray_colour(ray: Ray, world: &dyn Hittable, rng: &mut Rng) -> Colour`. Its body becomes:
   - `let mut attenuation = Colour::new(1.0, 1.0, 1.0);` and `let mut ray = ray;`;
   - `for _ in 0..MAX_BOUNCES { ... }` where the body calls
     `world.hit(&ray, 0.001, f64::INFINITY)`;
   - on `Some(hit)`, `hit.material.scatter(&ray, &hit, rng)` yields
     `Some((factor, scattered))`, which sets
     `attenuation = attenuation * factor; ray = scattered;`, and `None`, which returns
     `Colour::new(0.0, 0.0, 0.0)`;
   - on `None`, compute the existing sky gradient from `ray.direction.unit().y` (the three lines at
     `src/main.rs:28` to `src/main.rs:29` move into this arm unchanged) and return
     `attenuation * sky`;
   - after the loop, return `Colour::new(0.0, 0.0, 0.0)`, which makes a ray that uses all 50 bounces
     black.
6. `src/main.rs` - `render` creates `let mut rng = Rng::new(RNG_SEED);` once before the row loop and
   calls `ray_colour(camera.ray(u, v), &world, &mut rng)`. One generator, one seed, pixel order
   `(0..height).rev()` then `0..width`, so every run produces the same sequence.
7. `src/main.rs` - `to_byte` applies gamma 2 before quantizing:
   `(255.999 * channel.clamp(0.0, 1.0).sqrt()) as u8`.
8. `src/main.rs` - `scene` gives the centre sphere
   `Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))`, the ground sphere
   `Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))`, and adds
   `world.add(Box::new(Sphere::new(Vec3::new(1.1, -0.1, -1.0), 0.4, Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3)))));`
   after the ground sphere. Those coordinates put the metal sphere right of the diffuse sphere and
   above the ground surface at `y = -0.5`.
9. `src/main.rs` - doc comments follow `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md`:
   `MAX_BOUNCES` and `RNG_SEED` carry none (their names say it), `ray_colour` keeps a one-sentence
   doc comment saying it returns the colour seen along a ray after bounces, `render` keeps its
   existing header sentence, and the file header `//! Render a fixed scene and write it to standard
   output as a PPM image.` stays true and unchanged.
10. No `docs/` page is created or changed. `docs/requirements.md` already states this behaviour and
    `README.md` keeps `cargo run > image.ppm` accurate, so neither needs an edit; verify both by
    reading them before claiming the docs step complete.

**Callers / wiring to update:**
- `src/main.rs:34` and `src/main.rs:35` - both `Sphere::new` calls gain a third argument.
- `src/main.rs:48` - the `ray_colour` call gains the generator and passes the ray by value.
- `src/main.rs:26` - the normal-colour return is deleted; nothing else reads `hit.normal` for
  display.
- `src/sphere.rs:50` and `src/sphere.rs:62` - both existing tests gain a material argument.
- `src/hittable.rs:37` - `map_or` becomes `as_ref().map_or`.
- `src/main.rs:7` and `src/hittable.rs:11` - the two `#[allow(dead_code)]` markers are removed.

**Config / schema / migrations:**
- None. No configuration key, no migration, no generated file.
- Retained records and their named consumer/owner: none. The hit record is per-ray working data and
  nothing is written to a store.

**Touched files:**
<!-- touched-files:start -->
- `src/material.rs` - created: `Material` trait, `Lambertian`, `Metal`, `reflect`, `random_unit_vector`, and their tests.
- `src/hittable.rs` - changed: `HitRecord` gains `material`, loses `Copy`; `HittableList::hit` reads the record by reference.
- `src/sphere.rs` - changed: `Sphere` stores a material, `new` takes it, `hit` puts it on the record, tests adapted.
- `src/main.rs` - changed: material module, bounce loop, gamma 2 output, seeded generator, scene materials, tests adapted.
- Adds: nothing (no dependency, no config key, no migration, no generated file).
<!-- touched-files:end -->

**Rules / MUST NOT:**
- `Cargo.toml` gains no dependency. Use `std` only: `Rc`, `f64` maths. No `rand`, no threads.
- Do not add `unsafe`. Rendering stays single-threaded.
- Do not add a `Material` method, a second shading path or a `HitRecord` copy hack to keep the
  struct `Copy`. `Rc<dyn Material>` on the record is the agreed contract, so `HitRecord` is `Clone`
  only and `HittableList::hit` borrows it.
- Do not store `&dyn Material` or a boxed material; shared materials are `Rc<dyn Material>` and
  objects clone the record's `Rc`.
- Do not reseed or re-create `Rng` inside `ray_colour`, per pixel or per bounce. One generator per
  render with `RNG_SEED` keeps requirement 6 true.
- Do not implement glass, refraction, samples per pixel, jitter, front-face flags, texture
  coordinates, bounding boxes, shadow rays, Russian roulette, or any PNG, command-line or progress
  work. Those belong to A2, A3, the geometry plan and the output plan.
- Do not clamp, gamma-correct or tone-map a `Colour` anywhere except `to_byte`; `render` returns
  linear colours.
- Do not add `#[allow(dead_code)]` to silence a warning. If the change leaves an item unused, delete
  the item or wire it, do not suppress the warning.
- Keep `#[allow(dead_code)]` on `Vec3::cross` in `src/vec3.rs`; that function stays unused in this
  phase.
- No comment or doc comment mentions a plan, a phase or the history of the code. Prose follows
  `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md`.
- Do not touch `src/camera.rs`, `src/ray.rs`, `src/rng.rs` or `src/vec3.rs`.

**Tests:**
- New/changed/deleted, at most three new functions (REPO-RULES §3a):
  - `src/material.rs` - `a_diffuse_scatter_attenuates_by_its_albedo_and_stays_above_the_surface`:
    `Lambertian::new(Vec3::new(0.7, 0.2, 0.1))` with a hit whose `normal` is `(0.0, 0.0, 1.0)` and a
    ray `Ray::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0))` returns attenuation equal
    to the albedo and a direction whose dot product with the normal is positive. Build the hit's
    `material` field with `Rc::new(Lambertian::new(...))` of the same albedo.
  - `src/material.rs` - `metal_absorbs_a_reflection_that_points_into_the_surface`:
    `Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.0)` with the same hit and a ray direction
    `Vec3::new(0.0, 0.0, 1.0)` against the `(0.0, 0.0, 1.0)` normal returns `None`.
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
    `Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))` to `Sphere::new`.
- Fast gate: the named tests above (run the module tests first:
  `~/.cargo/bin/cargo test material`, `~/.cargo/bin/cargo test main`, `~/.cargo/bin/cargo test sphere`),
  then for code changes:
  `~/.cargo/bin/cargo build`, `~/.cargo/bin/cargo test`,
  `~/.cargo/bin/cargo clippy -- -D warnings`, `~/.cargo/bin/cargo fmt --check`,
  `git diff --check`. Use `cargo` directly when it is on `PATH`; this shell keeps the toolchain in
  `~/.cargo/bin`.
- Conditional gates: Rendered image, required because the render loop, both materials, the camera
  usage through `render` and the output values change. Run it once:
  `~/.cargo/bin/cargo run --release > /tmp/opencode/a1-render.ppm` (the command line is the output
  plan's work, so standard output is the image). Then convert and inspect it as the **User test**
  below describes. No other conditional gate applies.
- Slow gate (>2 minutes): none. The release render of the default 200 by 100 image takes under a
  second.

**User test (acceptance):**
- **Executor:** agent. It runs the commands and opens the image.
- 1. Run `~/.cargo/bin/cargo run --release > /tmp/opencode/a1-render.ppm`.
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
  Run it as `python3 /tmp/opencode/ppm2png.py /tmp/opencode/a1-render.ppm /tmp/opencode/a1-render.png`.
- 3. Observe in the opened image: a blue-white gradient sky above the horizon, a yellow-green ground
  plane filling the lower part, a matte red-brown sphere in the centre with smooth shading between
  its lit and dark sides, and a grey metal sphere to the right of the matte sphere whose surface
  shows a blurred reflection of the ground and sky. The image has no banded normal colours, no
  black patches across a sphere, and no visible white specks on the metal sphere.
- 4. No cast shadows appear under either sphere. That is expected: the phase adds no shadow ray, so
  absence of shadows is not a defect and is not to be fixed here.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `Material`, `Lambertian` and `Metal` exist in `src/material.rs` with the signatures in the
  build plan, and `Sphere` and `HitRecord` carry `Rc<dyn Material>`.
- [ ] `ray_colour` bounces at most `MAX_BOUNCES` (50) times, multiplies the attenuation at each
  bounce, applies the sky only on a miss and returns black when the limit is reached.
- [ ] `render` owns one `Rng::new(RNG_SEED)` and passes it down; no other randomness source exists.
- [ ] `to_byte` applies the square root before quantizing.
- [ ] The two stale `#[allow(dead_code)]` markers are gone and the crate builds without warnings.
- [ ] The scene holds two diffuse spheres and one metal sphere with the values in step 8.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports three test functions added, two
  existing test functions adapted and one renamed.
- [ ] Fast phase gate and the rendered-image conditional gate pass; the agent viewed the rendered
  image and it matches the **User test** description.
- [ ] `docs/requirements.md` and `README.md` were read and stay accurate; no `docs/` page refers to
  a plan.
- [ ] No dependency, no `unsafe`, no thread, no file outside the touched list.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the agent's rendered-image observation is recorded
  in it.
- [ ] On passing evidence: compress this file into `done/A1-diffuse-and-metal-materials.md` (frozen
  contract only) and flip the `full-plan.md` status row to `done`.

**Exit:** every scene object carries a material, a ray follows material scatters up to 50 bounces
and turns black at the limit, output is gamma 2 corrected, and the rendered image shows a matte
centre sphere, a matte ground and a metal sphere with a blurred reflection.
