## Phase A3 - Samples per pixel

**Status:** active.
**Depends on:** A1r and A2. Their frozen contracts are `done/A1r-hit-record-constructor-and-materials.md` (the scatter loop, the one seeded generator, gamma 2 in `to_byte`) and `done/A2-glass.md` (the four-sphere scene).
**Scope:** `render` in `src/main.rs` takes a sample count, and each pixel averages that many jittered rays drawn from the generator; `main` passes the default of 16. The scene, the materials, the camera, the bounce limit, the gamma correction, the PPM writer and `ray_colour` are unchanged. The command line, a `--samples` option, the PNG writer and progress output stay out of this phase.
**Gate level:** standard. The phase changes the render loop of the one command the product has, with no durable state, no external boundary and no security surface; targeted tests plus the fast gate and the rendered-image conditional gate fit the risk.
**Simplicity statement:** the sample count is used as given, like the metal fuzz value, with no check for zero. `main` passes the constant `16`, and the command-line plan owns rejecting a bad value from the command line, so a zero count is not reachable from the product. There is no stratification, no adaptive sampling, no pixel filter and no per-pixel generator. No legacy path is added and none is deleted.

**Current state (verified):**
- `src/main.rs:22-25` - `const WIDTH: usize = 200;`, `const HEIGHT: usize = 100;`, `const MAX_BOUNCES: usize = 50;`, `const RNG_SEED: u64 = 2024;`.
- `src/main.rs:75` - `fn render(width: usize, height: usize) -> Vec<Colour>`; it builds `scene()` and `Camera::new(width as f64 / height as f64)`, creates one `Rng::new(RNG_SEED)` at line 78, and pushes one `ray_colour(camera.ray(u, v), &world, &mut rng)` per pixel at lines 79 to 87.
- `src/main.rs:80-84` - the current pixel mapping: `for row in (0..height).rev()` then `for column in 0..width`, with `u = column as f64 / (width - 1) as f64` and `v = row as f64 / (height - 1) as f64`.
- `src/main.rs:28` - `fn ray_colour(ray: Ray, world: &dyn Hittable, rng: &mut Rng) -> Colour` consumes the generator through the materials it scatters. It stays unchanged.
- `src/main.rs:113-116` - `main` calls `render(WIDTH, HEIGHT)` and passes the result to `write_ppm(&mut io::stdout().lock(), WIDTH, HEIGHT, &pixels)`.
- `src/main.rs:124` and `src/main.rs:134` - the two render tests, `the_center_pixel_shows_the_diffuse_sphere` and `the_top_row_is_sky`, each call `render(WIDTH, HEIGHT)`. Those are the only two call sites besides `main` (checked with grep).
- `src/main.rs:139-144` - `a_mid_grey_pixel_is_written_with_gamma_two` calls `write_ppm` only and needs no change.
- `src/rng.rs:57` - `pub fn next_f64(&mut self) -> f64` returns a value in `0..1`.
- `src/camera.rs:31` - `pub fn ray(&self, u: f64, v: f64) -> Ray` takes both coordinates in `0..=1` from the lower left.
- `src/vec3.rs:42` and `src/vec3.rs:77` - `Vec3` implements `Add` and `Div<f64>`; `src/vec3.rs:83` - `pub type Colour = Vec3`.
- `docs/requirements.md` item 5 states the pixel averaging and the default of 16; item 6 fixes the one seeded generator.

**Read first (do not invent):**
- `docs/requirements.md` - item 5 is the contract of this phase, item 6 the reproducibility rule, item 12 the command-line default that the output plan owns.
- `src/main.rs` - the whole file: the constants, `ray_colour`, `render`, `to_byte`, `write_ppm`, `main` and the tests.
- `src/rng.rs` - `Rng::new` and `next_f64`, the phase's only randomness.
- `src/camera.rs` - `Camera::new` and `Camera::ray`.
- `src/vec3.rs` - `Add` and `Div<f64>` for the pixel sum, and the `Colour` alias.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image conditional gate. The command line is the output plan's work, so the render writes PPM to standard output.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1r-hit-record-constructor-and-materials.md` and `done/A2-glass.md` - the render, generator and scene contracts this phase builds on.

**Build plan:**

1. `src/main.rs:22-25` - add `const SAMPLES: usize = 16;` after `const HEIGHT` and before
   `const MAX_BOUNCES`. Update the module header only if it stops being accurate; it describes the
   PPM output and stays true.
2. `src/main.rs:74-88` - change the signature and the loop:
   before `fn render(width: usize, height: usize) -> Vec<Colour>`, after
   `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>`. The doc comment becomes
   `/// Every pixel's colour averaged over `samples` jittered rays, row by row from the top.`
   Keep `scene()`, `Camera::new(width as f64 / height as f64)`, the single
   `let mut rng = Rng::new(RNG_SEED);` and `Vec::with_capacity(width * height)`. Replace the inner
   push with:
   ```rust
   let mut total = Colour::new(0.0, 0.0, 0.0);
   for _ in 0..samples {
       let u = (column as f64 + rng.next_f64()) / width as f64;
       let v = (row as f64 + rng.next_f64()) / height as f64;
       total = total + ray_colour(camera.ray(u, v), &world, &mut rng);
   }
   pixels.push(total / samples as f64);
   ```
   The two generator draws happen once per sample, `u` before `v`, and the same `rng` continues into
   `ray_colour`, so each pixel consumes `samples` jittered rays in a fixed order. Pixel `column`
   covers `u` in `column / width .. (column + 1) / width` and pixel `row` covers `v` in
   `row / height .. (row + 1) / height`, so a jittered sample stays inside its pixel and never
   exceeds `1.0`. The rows still run from the top because `row` counts down.
3. `src/main.rs:114` - `let pixels = render(WIDTH, HEIGHT, SAMPLES);`.
4. `src/main.rs:124` and `src/main.rs:134` - `render(WIDTH, HEIGHT, 1)` in both existing tests. One
   sample keeps the two full-image renders fast, and both assertions are coarse enough to hold with
   a jittered sample.
5. `src/main.rs`, tests - append the new test listed under **Tests** to `mod tests`, after
   `a_mid_grey_pixel_is_written_with_gamma_two`. It needs no new import: the module already has
   `use super::*;`, `scene`, `Camera`, `Rng`, `Colour`, `ray_colour` and `render` in scope.
6. Prose - the changed doc comment follows `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md`.
   Add no comment naming a plan, a phase or history.
7. No `docs/` page is created or changed. `docs/requirements.md` item 5 already states this
   behaviour and `README.md` stays accurate; read both before claiming the docs step complete.

**Callers / wiring to update:**
- `src/main.rs:114` - `main` passes `SAMPLES`.
- `src/main.rs:124` and `src/main.rs:134` - the two tests pass `1`.
- No other caller changes. `ray_colour`, `scene`, `Camera::ray`, `to_byte`, `write_ppm`, `main`'s
  output path and the material, hit-record and generator signatures stay as they are.

**Config / schema / migrations:**
- None. No configuration key, no command-line option, no migration, no generated file. `SAMPLES` is a
  private constant in `src/main.rs`, not a configuration key.
- Retained records and their named consumer/owner: none. Pixels are per-render working data and
  nothing is written to a store.

**Touched files:**
<!-- touched-files:start -->
- `src/main.rs` - changed: the `SAMPLES` constant, the `render` sample loop, the `main` call, the two adapted test calls and one new test.
- Adds: nothing (no dependency, no config key, no migration, no generated file).
<!-- touched-files:end -->

**Rules / MUST NOT:**
- `Cargo.toml` gains no dependency. Use `std` only: `usize`, `f64`. No `rand`, no threads.
- Do not add `unsafe`. Rendering stays single-threaded.
- Do not add a command-line option, a `--samples` flag, argument parsing, a progress line, a PNG
  writer or a sample count on any other type. The command-line plan owns those.
- Do not create a generator per pixel or per sample, do not reseed, and do not reorder the draws:
  the two jitter draws come before `ray_colour`, `u` before `v`. Reproducibility depends on this one
  sequence.
- Do not change `MAX_BOUNCES`, `RNG_SEED`, `to_byte`, `write_ppm`, the PPM header, `scene`, the four
  spheres, `ray_colour`, `Camera`, or any material.
- Do not validate, clamp or reject the sample count, and do not fall back to a default when it is
  zero. Use it as given.
- Do not change the existing tests' assertions; only their `render` arguments change.
- Do not add `length_squared`, a pixel filter, stratification, adaptive sampling or any other helper.
  Keep `#[allow(dead_code)]` and its comment on `Vec3::cross`.
- No comment or doc comment this phase writes or edits mentions a plan, a phase or the history of the
  code. Prose follows `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md`.

**Tests:**
- New/changed/deleted, at most three new functions (REPO-RULES §3a):
  - `src/main.rs` - `a_pixel_averages_four_jittered_samples`: build `scene()` and
    `Camera::new(1.0)`, seed a second `Rng::new(RNG_SEED)`, and for four samples draw `u` then `v`
    with `next_f64`, add `ray_colour(camera.ray(u, v), &world, &mut rng)` to a total, then divide the
    total by `4.0`. Assert `render(1, 1, 4)[0]` equals that expected colour exactly. A 1 by 1 image
    covers `u` and `v` in `0..1`, so each draw is the sample position itself, and four is a power of
    two so dividing by `4.0` and multiplying by `0.25` give the same bits. The test proves the pixel
    is the mean of its jittered samples drawn from the one seeded generator. If the assertion fails,
    the sample loop, the draw order or the averaging is wrong: report both colours and fix the code,
    never weaken the assertion.
  - Changed existing tests: `src/main.rs` - `the_center_pixel_shows_the_diffuse_sphere` and
    `the_top_row_is_sky` pass `1` as the sample count; their assertions are unchanged.
- Fast gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`,
  `git diff --check`, called as `~/.cargo/bin/cargo` when `cargo` is not on `PATH`. The whole suite
  is the targeted run for this crate (one binary, finishing in a few seconds); when one test fails,
  rerun it by node id, for example `cargo test a_pixel_averages_four_jittered_samples`.
- Conditional gates: Rendered image, required because the render loop changes. The single run is
  `cargo run --release > /tmp/opencode/a3-render.ppm` (standard output is the image because the
  command line belongs to the output plan); the user test reuses that file and does not render a
  second time. No other conditional gate applies.
- Slow gate (>2 minutes): none. The release render of the fixed 200 by 100 image at 16 samples
  finishes in under a few seconds.

**User test (acceptance):**
- **Executor:** agent. It runs the command and opens the image.
- 1. Use the image already written by the rendered-image conditional gate at
  `/tmp/opencode/a3-render.ppm`; do not render a second time.
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
  Run it as `python3 /tmp/opencode/ppm2png.py /tmp/opencode/a3-render.ppm /tmp/opencode/a3-render.png`.
- 3. Observe in the opened image: the blue-white gradient sky, the yellow-green ground, the matte
  red-brown centre sphere, the grey metal sphere on the right and the transparent glass sphere on the
  left keep their materials and placement, and the glass interior still shows the bent sky and ground
  with its rim. The single-sample speckle along every edge and on every surface is replaced by smooth
  gradients: the sphere silhouettes, the ground horizon and the metal reflection are clean at the
  200 by 100 size, and no pixel shows an isolated bright or dark dot.
- 4. The scene stays one fixed frame: four spheres, no shadows, bright sky at the top and the ground
  filling the lower part. The default of 16 samples is the count this image used.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `const SAMPLES: usize = 16;` exists in `src/main.rs` and `main` passes it to `render`.
- [ ] `render` takes `(width: usize, height: usize, samples: usize)` and each pixel is the mean of
  `samples` `ray_colour` results over rays jittered inside the pixel, drawn from the one generator.
- [ ] `u` is `(column as f64 + next_f64()) / width as f64` and `v` is
  `(row as f64 + next_f64()) / height as f64`, drawn `u` first, once per sample, before the
  `ray_colour` call that consumes the same generator.
- [ ] The sample count is used as given; no check, clamp or fallback was added, and no command-line
  option, progress line or PNG work was added.
- [ ] `src/main.rs` is the only changed source file; `Cargo.toml` is unchanged and `ray_colour`,
  `scene`, the materials, the camera and `write_ppm` keep their signatures.
- [ ] Named tests pass, including the new `a_pixel_averages_four_jittered_samples` with an exact
  assertion.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports one test function added, two
  existing test calls adapted and no assertion weakened.
- [ ] Fast phase gate and the rendered-image conditional gate pass; the agent viewed the rendered
  image and it matches the **User test** description, with no single-sample speckle left.
- [ ] `docs/requirements.md` and `README.md` were read and stay accurate; no `docs/` page refers to
  a plan.
- [ ] No dependency, no `unsafe`, no thread, no file outside the touched list.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the agent's rendered-image observation is recorded
  in it.
- [ ] On passing evidence: compress this file into `done/A3-samples-per-pixel.md` (frozen contract
  only) and flip the `full-plan.md` `A3` status row to `done`; `A1r`, `A1` and `A2` keep their rows.

**Exit:** every pixel of the rendered image is the mean of 16 jittered rays drawn from the one seeded
generator, the same options still give the same bytes, and the rendered image has smooth edges in
place of single-sample speckle.
