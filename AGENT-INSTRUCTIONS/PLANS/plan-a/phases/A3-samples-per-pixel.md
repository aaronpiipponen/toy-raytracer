## Phase A3 - Samples per pixel

**Status:** active.
**Depends on:** A1 (`done/A1-diffuse-and-metal-materials.md`); independent of A2.
**Scope:** `render` in `src/main.rs` takes a sample count and averages that many jittered rays per pixel, with one seeded generator shared across the whole image. `main` passes a new default of 16. No command-line option, no output-format change, no material, scene or bounce-limit change.
**Gate level:** standard - bounded runtime behaviour in the render loop, with no schema, durable state, security boundary or cross-package contract. The rendered-image conditional gate applies because the render loop changes.
**Simplicity statement:** The sample count has one supported range, at least one sample, guarded by a single assertion. The phase adds no clamping, retry, fallback or per-pixel generator. Width or height of zero remains unhandled as it is today; the command line validates its own values in the output plan. No code is deleted.

**Current state (verified):**
- `src/main.rs:22-25` - `const WIDTH: usize = 200`, `const HEIGHT: usize = 100`, `const SEED: u64 = 7`, `const MAX_BOUNCES: usize = 50`.
- `src/main.rs:75-88` - `fn render(width: usize, height: usize) -> Vec<Colour>` creates `let mut rng = Rng::new(SEED)`, loops `for row in (0..height).rev()` then `for column in 0..width`, maps `u = column as f64 / (width - 1) as f64` and `v = row as f64 / (height - 1) as f64`, and pushes one `ray_colour(&camera.ray(u, v), &world, &mut rng)` per pixel.
- `src/main.rs:113-116` - `main` calls `render(WIDTH, HEIGHT)` and `write_ppm(&mut io::stdout().lock(), WIDTH, HEIGHT, &pixels)`.
- `src/main.rs:122-127` - `the_top_row_is_sky` calls `render(WIDTH, HEIGHT)` and asserts the first pixel is bluer than it is red.
- `src/main.rs:90-92` - `to_byte` applies gamma 2 with `channel.clamp(0.0, 1.0).sqrt()`; output gamma is already in place and stays unchanged.
- `src/rng.rs:20-22` - `Rng::next_f64(&mut self) -> f64` returns a value in `0..1`.
- `src/camera.rs:31-36` - `Camera::ray(&self, u: f64, v: f64) -> Ray` maps viewport coordinates in `0..=1` from the lower left.
- `src/vec3.rs:82` - `pub type Colour = Vec3`, and `Vec3` implements `Add`, `Div<f64>` and `PartialEq`.
- A1 contract: one `Rng` seeded with `SEED` is shared across every pixel, the bounce limit and gamma are in place, and `Cargo.toml` has no dependency.

**Read first (do not invent):**
- `src/main.rs` - `render` at lines 75-88, its doc comment, `main` at lines 113-116 and the test at lines 118-127. This phase edits only those items. `ray_colour` at lines 29-46 and `to_byte` at lines 90-92 are read for context and stay unchanged.
- `src/rng.rs` - `Rng::new(seed: u64)` and `next_f64(&mut self) -> f64`; the generator is the only source of randomness.
- `src/camera.rs` - `Camera::ray`; sample coordinates are passed unchanged into it.
- `src/vec3.rs` - `Colour`, `Add` and `Div<f64>`; the accumulation uses them.
- `docs/requirements.md` items 5 and 6 - the per-pixel sample average, default 16, gamma 2, and the seeded generator.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1-diffuse-and-metal-materials.md` - the frozen render and generator contract.

**Build plan:**
1. `src/main.rs:22-25` - add `const SAMPLES: usize = 16;` directly below `const HEIGHT: usize = 100;`. It is the default sample count and the value `main` passes.
2. `src/main.rs:75` - change the signature from `fn render(width: usize, height: usize) -> Vec<Colour>` to `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>`.
3. `src/main.rs:75-88`, inside `render` - keep the single `let mut rng = Rng::new(SEED);` before both loops. At the top of the function add `assert!(samples > 0, "the sample count must be at least 1");`. For each pixel, replace the single `ray_colour` call with an accumulation over `for _ in 0..samples`:
   - `let u = (column as f64 + rng.next_f64()) / width as f64;`
   - `let v = (row as f64 + rng.next_f64()) / height as f64;`
   - `pixel = pixel + ray_colour(&camera.ray(u, v), &world, &mut rng);`
   Start `pixel` at `Vec3::new(0.0, 0.0, 0.0)` before the sample loop, and push `pixel / samples as f64` after it. Each sample draws exactly two `next_f64` values, `u` before `v`, before any scattering draws for that sample. Keep the row loop as `for row in (0..height).rev()`.
4. `src/main.rs:114` - change `let pixels = render(WIDTH, HEIGHT);` to `let pixels = render(WIDTH, HEIGHT, SAMPLES);`.
5. `src/main.rs:74` - update the doc comment on `render` to state that every pixel is the average of `samples` jittered rays, row by row from the top. Keep it one sentence.
6. `src/main.rs:124` - change the test call `render(WIDTH, HEIGHT)` to `render(WIDTH, HEIGHT, SAMPLES)`.
7. `src/main.rs:118-127` - add the two tests named under Tests below, after `the_top_row_is_sky`, using small dimensions so the suite stays fast.

**Callers / wiring to update:**
- `src/main.rs:114` (`main`) - pass `SAMPLES` as the third argument.
- `src/main.rs:124` (`the_top_row_is_sky`) - pass `SAMPLES` as the third argument.
- No other caller: `render` is called only at `src/main.rs:114` and `src/main.rs:124`.
- No command-line, configuration or output wiring changes; the output plan owns `--samples`.

**Config / schema / migrations:**
- None. `SAMPLES` is a source constant, not a configuration key.
- Retained records and their named consumer/owner: none.

**Touched files:**
<!-- touched-files:start -->
- `src/main.rs` - changed: adds `SAMPLES`, changes `render` to average jittered samples, updates the `main` call, the `render` doc comment and the tests.
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Keep `Cargo.toml` free of dependencies; the standard library only.
- Use the one `Rng` seeded with `SEED` before both loops and pass `&mut rng` to every `ray_colour` call. Do not reseed per pixel, per sample or per row, and do not add a second generator.
- Keep rendering single-threaded and keep `unsafe` absent.
- Do not add or parse `--samples`, and do not read environment or configuration; the command line belongs to the output plan.
- Do not change `to_byte`, gamma 2, the 200 by 100 size, the PPM writer, the scene, the materials, `MAX_BOUNCES` or `SEED`.
- Do not clamp, round or otherwise adjust the sample count beyond the one assertion; zero samples must fail loudly rather than render a blank or `NaN` image.
- Do not add a second code path for one sample, and do not keep the old no-jitter mapping alongside the new one.

**Tests:**
- Changed: `src/main.rs` - `the_top_row_is_sky`: calls `render(WIDTH, HEIGHT, SAMPLES)` and keeps the assertion that the first pixel is bluer than it is red.
- New: `src/main.rs` - `two_renders_with_the_same_options_give_the_same_pixels`: calls `render(20, 10, 2)` twice and asserts the two `Vec<Colour>` values are equal, proving the shared seeded generator gives identical bytes for identical options.
- New: `src/main.rs` - `the_sample_count_changes_the_pixels`: calls `render(20, 10, 1)` and `render(20, 10, 4)` and asserts the results differ, proving the sample count reaches the jittered sampling loop.
- Deleted: none. Two new test functions plus one adapted test are within the REPO-RULES §3a maximum of three new functions; no existing test is removed.
- Fast gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image, required because the render loop changes. Run `cargo run --release > /tmp/opencode/render.ppm`, then convert and inspect it:
  ```bash
  python3 - <<'PY'
  import struct, zlib

  with open("/tmp/opencode/render.ppm") as f:
      assert f.readline().strip() == "P3"
      width, height = map(int, f.readline().split())
      assert f.readline().strip() == "255"
      data = [int(v) for v in f.read().split()]

  raw = b"".join(
      b"\x00" + bytes(data[(r * width + c) * 3 + k] for c in range(width) for k in range(3))
      for r in range(height)
  )

  def chunk(tag, body):
      return struct.pack(">I", len(body)) + tag + body + struct.pack(">I", zlib.crc32(tag + body))

  png = (b"\x89PNG\r\n\x1a\n"
         + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0))
         + chunk(b"IDAT", zlib.compress(raw))
         + chunk(b"IEND", b""))
  with open("/tmp/opencode/render.png", "wb") as f:
      f.write(png)
  PY
  ```
  Open `/tmp/opencode/render.png` with the image reader and confirm the three spheres are visible, their edges blend over several pixels, and the sky and the flat sphere areas show no salt-and-pepper grain.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** agent.
- Run the rendered-image conditional gate above and inspect `/tmp/opencode/render.png`: a diffuse red-brown centre sphere, a diffuse grey ground sphere and a grey metal sphere, with soft edges and without heavy grain. The plan's final user acceptance of the image is Z1.

**Completion checklist (gate):**
- [ ] Gate level `standard` requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `render` takes `samples`, averages that many jittered rays per pixel with the one seeded generator, and `main` passes `SAMPLES` equal to 16.
- [ ] `the_top_row_is_sky`, `two_renders_with_the_same_options_give_the_same_pixels` and `the_sample_count_changes_the_pixels` pass.
- [ ] Test budget respected (REPO-RULES §3a); receipt reports functions added/deleted. Two new functions and one adapted test, no deletions.
- [ ] Fast phase gate passes, and the rendered-image conditional gate was run once and the image inspected.
- [ ] No `docs/` page changes are needed: `docs/requirements.md` item 5 already states this behaviour and `README.md` does not name the sample count.
- [ ] `Cargo.toml` has no dependency, and the seeded generator remains the only randomness.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the agent user test has passing evidence.
- [ ] On passing evidence: compress this file into `done/A3-samples-per-pixel.md` (frozen contract only) and flip the `full-plan.md` status row to `done`.

**Exit:** each pixel is the average of 16 jittered samples from the one seeded generator, the same options give the same bytes, and the rendered image has smooth sphere edges without heavy grain.
