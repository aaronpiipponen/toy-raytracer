## Phase A3 - Samples per pixel

**Status:** active.
**Depends on:** A1 (`done/A1-diffuse-and-metal-materials.md`) and A2 (`done/A2-glass.md`).
**Scope:** `render` in `src/main.rs` takes a sample count and averages that many jittered rays per pixel, with one seeded generator shared across the whole image. `main` passes a new default of 16. No command-line option, no output-format change, no material, scene or bounce-limit change.
**Gate level:** standard - bounded runtime behaviour in the render loop, with no schema, durable state, security boundary or cross-package contract. The rendered-image conditional gate applies because the render loop changes.
**Simplicity statement:** The sample count has one supported range, at least one sample, validated only by the command line in the merged product. `render` carries no sample-count guard. The phase adds no clamping, retry, fallback or per-pixel generator. Width or height of zero goes unhandled by `render`, and the command line validates its own values in the output plan. No code is deleted.

**Current state (verified):**
- `src/main.rs:22-25` - `const WIDTH: usize = 200`, `const HEIGHT: usize = 100`, `const SEED: u64 = 7`, `const MAX_BOUNCES: usize = 50`.
- `src/main.rs:79-93` - the one-sentence doc comment `/// Every pixel's colour, row by row from the top.` and `fn render(width: usize, height: usize) -> Vec<Colour>`, which creates `let mut rng = Rng::new(SEED)`, loops `for row in (0..height).rev()` then `for column in 0..width`, maps `u = column as f64 / (width - 1) as f64` and `v = row as f64 / (height - 1) as f64`, and pushes one `ray_colour(&camera.ray(u, v), &world, &mut rng)` per pixel.
- `src/main.rs:118-121` - `main` calls `render(WIDTH, HEIGHT)` and `write_ppm(&mut io::stdout().lock(), WIDTH, HEIGHT, &pixels)`.
- `src/main.rs:123-132` - the test module and `the_top_row_is_sky`, which calls `render(WIDTH, HEIGHT)` and asserts the first pixel is bluer than it is red.
- `src/main.rs:95-97` - `to_byte` applies gamma 2 with `channel.clamp(0.0, 1.0).sqrt()`; output gamma is already in place and stays unchanged.
- `src/main.rs:54-77` - `scene()` holds four spheres: diffuse red-brown centre, diffuse grey ground, metal at `(0.85, -0.25, -1.3)` and the glass sphere at `(-1.1, 0.0, -1.5)`. The scene stays unchanged.
- `src/rng.rs:8-9` and `src/rng.rs:20-22` - `Rng::new(seed: u64)` and `next_f64(&mut self) -> f64`, which returns a value in `0..1`.
- `src/camera.rs:31-36` - `Camera::ray(&self, u: f64, v: f64) -> Ray` maps viewport coordinates in `0..=1` from the lower left.
- `src/vec3.rs:83` - `pub type Colour = Vec3`, and `Vec3` implements `Add`, `Div<f64>` and `PartialEq`.
- A1 contract: one `Rng` seeded with `SEED` is shared across every pixel, the bounce limit and gamma are in place, and `Cargo.toml` has no dependency.
- A2 contract: `ray_colour` reaches glass through `hit.material`, so the render loop needs no material knowledge.

**Read first (do not invent):**
- `src/main.rs` - the constants at lines 22-25 (a new `SAMPLES` joins them), the `render` doc comment and body at lines 79-93, `main` at lines 118-121 and the test module at lines 123-132. This phase edits only those items. `ray_colour` at lines 27-46 and `to_byte` at lines 95-97 are read for context and stay unchanged.
- `src/rng.rs` - `Rng::new(seed: u64)` and `next_f64(&mut self) -> f64`; the generator is the only source of randomness.
- `src/camera.rs` - `Camera::ray`; sample coordinates are passed unchanged into it.
- `src/vec3.rs` - `Colour`, `Add` and `Div<f64>`; the accumulation uses them.
- `docs/requirements.md` items 5 and 6 - the per-pixel sample average, default 16, gamma 2, and the seeded generator.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1-diffuse-and-metal-materials.md` - the frozen render and generator contract.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A2-glass.md` - the frozen material and scene contract, including the glass sphere the rendered image must still show.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-b-Z1.md` - the geometry union's release contract recorded below; read-only context, because A3 changes no file for the union.

**Build plan:**
1. `src/main.rs:23` - add `const SAMPLES: usize = 16;` directly below `const HEIGHT: usize = 100;`. On branch `plan-a` the constant is this branch's local default, and its only use is the `main` call `render(WIDTH, HEIGHT, SAMPLES)`. The merged code has no `SAMPLES`, and the release takes plan-c's `DEFAULT_SAMPLES = 16`.
2. `src/main.rs:80` - change the signature from `fn render(width: usize, height: usize) -> Vec<Colour>` to `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>`.
3. `src/main.rs:80-93`, inside `render` - keep the single `let mut rng = Rng::new(SEED);` before both loops. `render` adds no sample-count validation. For each pixel, replace the single `ray_colour` call with an accumulation over `for _ in 0..samples`:
   - `let u = (column as f64 + rng.next_f64()) / width as f64;`
   - `let v = (row as f64 + rng.next_f64()) / height as f64;`
   - `pixel = pixel + ray_colour(&camera.ray(u, v), &world, &mut rng);`
   Start `pixel` at `Vec3::new(0.0, 0.0, 0.0)` before the sample loop, and push `pixel / samples as f64` after it. Each sample draws exactly two `next_f64` values, `u` before `v`, before any scattering draws for that sample. Keep the row loop as `for row in (0..height).rev()`.
4. `src/main.rs:119` - change `let pixels = render(WIDTH, HEIGHT);` to `let pixels = render(WIDTH, HEIGHT, SAMPLES);`.
5. `src/main.rs:79` - replace the `render` doc comment with exactly these two lines:
   ```rust
   /// Every pixel's colour, row by row from the top, each pixel averaging `samples` jittered rays, with
   /// the share of finished rows reported on standard error.
   ```
   This sentence describes the merged product. Branch `plan-a` adds no progress line: the release merge takes plan-c's progress write from `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md` and keeps this sentence unchanged, so the sentence is edited only here.
6. `src/main.rs:129` - change the test call to the literal `let pixels = render(200, 100, 16);`, the same line plan-c landed, and keep the assertion unchanged.
7. Add the two tests named under Tests at the end of the test module, after the closing brace of `the_top_row_is_sky` (line 132), using small dimensions so the suite stays fast.

**Release merge contract (from the shared design's "Release merge" section):**
- From plan-a's branch: `scene`, `ray_colour`, `Rng` usage, `SEED`, `MAX_BOUNCES`, the `to_byte` gamma body, the one `Rng::new(SEED)` before both loops, the jitter mapping and the `samples` averaging, and the three test functions below.
- From plan-c's branch: the module header, `mod png;`, `Format`, `Settings`, `USAGE`, `DEFAULT_WIDTH`, `DEFAULT_HEIGHT`, `DEFAULT_SAMPLES`, `DEFAULT_OUTPUT`, `parse_args`, `parse_dimension`, `resolve_format`, `run`, `main`, the progress write, the `pub(crate)` visibility on `to_byte`, `README.md`, `docs/requirements.md` and `tests/cli.rs`.
- The merged `render` is the shared design's function: plan-a's scene, camera, generator, `ray_colour` and jitter with plan-c's progress write and doc sentence. The merged code has no `SAMPLES`, `WIDTH` or `HEIGHT` constant. `DEFAULT_SAMPLES = 16` is the single default, `run` passes `settings.samples`, and `parse_args` rejects a count below 1.
- The merged test module holds `the_top_row_is_sky`, `two_renders_with_the_same_options_give_the_same_pixels` and `the_sample_count_changes_the_pixels`. Plan-c's `the_center_pixel_shows_the_sphere_facing_the_camera` does not survive, because A1 replaced normal colouring with materials.
- Neither branch adds a second `render` path, a second sample default, a second generator or a second progress write.

**Release merge dependency - the geometry union (plan-b; `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-b-Z1.md`):**
A3 implements no part of this union: it records the agreed contract so the release merge has one target. A3's own sampling scope, build steps, tests and gates are unchanged, and A3 changes no product code for the union.

- `src/hittable.rs` - the merged `HitRecord` is not `Copy` and holds `t`, `point`, the ray-facing `normal`, `front_face`, `u`, `v` and `material: Rc<dyn Material>`. `point` carries no `#[allow(dead_code)]`; `u` and `v` keep it. The merged `HitRecord::new(ray: &Ray, t: f64, point: Vec3, outward_normal: Vec3, u: f64, v: f64, material: Rc<dyn Material>) -> Self` is the only constructor, the seven-argument form.
- `Hittable` declares `hit(...)` plus the default `fn bounding_box(&self) -> Option<Aabb> { None }`. `Sphere` and `Triangle` override it; `Plane` and `HittableList` take the default.
- The merged `HittableList::hit` combines plan-a's borrowed nearest record with plan-b's box skip between the limit and the object test: `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);`, then skip with `continue` when `object.bounding_box()` is `Some(bounds)` and `!bounds.hit(ray, t_min, limit)`, then test `object.hit(ray, t_min, limit)` and keep the record. One loop, one limit, no second culling path.
- `Sphere { center, radius, material }` keeps plan-a's material and gains plan-b's `bounding_box` (center minus radius to center plus radius) and its exact `(u, v)` formula. `Plane` and `Triangle` gain a `material` field and pass it to the seven-argument constructor. `src/aabb.rs` stays plan-b's landed file, unchanged.
- `src/main.rs` takes plan-a's material render and scene plus plan-b's one triangle at the exact landed vertices `(0.6, 0.15, -1.0)`, `(1.95, 0.15, -1.0)` and `(1.275, 0.92, -1.0)`. The placement is not moved or "improved" at the merge.
- Test reconciliation, one test per behaviour: drop plan-b's `the_center_pixel_shows_the_sphere_facing_the_camera`, whose normal-colour premise disappears when material scattering replaces normal colouring; no replacement test is added, and the release's own rendered-image gate proves the fixed scene still shows the sphere. Drop plan-b's `the_near_surface_has_texture_coordinates_from_latitude_and_longitude`, which duplicates the `(u, v)` assertions plan-a's `a_ray_through_the_center_hits_the_near_surface` already makes. `the_top_row_is_sky` stays, and plan-b's shape and box tests pass a material after the merge.
- Open user-owned decision: the scene triangle's material changes the finished image, so the release asks the user (recommendation: a Lambertian in the scene spheres' family). A3 records it and does not decide it.

**Callers / wiring to update:**
- `src/main.rs:119` (`main`) - pass `SAMPLES` as the third argument on branch `plan-a`.
- `src/main.rs:129` (`the_top_row_is_sky`) - pass the literal `16` in `render(200, 100, 16)`.
- No other caller: `render` is called only at `src/main.rs:119` and `src/main.rs:129`.
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
- Do not change `to_byte`, gamma 2, the 200 by 100 size, the PPM writer, the four scene spheres, the materials, `MAX_BOUNCES` or `SEED`.
- `render` does not validate the sample count, and the merged command line is the single validation path. Do not clamp, round or otherwise adjust the sample count elsewhere.
- Do not add a progress write, change the module header or the `mod` list, or touch the writers, `to_byte`, the scene or the materials.
- Do not add a second code path for one sample, and do not keep the old no-jitter mapping alongside the new one.
- Do not add version or history words, plan names or phase ids to code comments.

**Tests:**
- Changed: `src/main.rs` - `the_top_row_is_sky`: calls the literal `let pixels = render(200, 100, 16);` and keeps the assertion that the first pixel is bluer than it is red.
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
  Open `/tmp/opencode/render.png` with the image reader and confirm all four spheres are visible: the glass sphere on the left refracts and inverts the background, the diffuse red-brown centre sphere, the diffuse grey ground sphere and the grey metal sphere still render. Their edges blend over several pixels, the sky and the flat sphere areas show no salt-and-pepper grain, and the image has no flat `NaN` grey pixels. A single absorbed metal sample no longer leaves a black pixel, because the other samples of that pixel stay visible.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** agent.
- Run the rendered-image conditional gate above and inspect `/tmp/opencode/render.png`: a glass sphere on the left of the centre, a diffuse red-brown centre sphere, a diffuse grey ground sphere and a grey metal sphere, with soft edges and without heavy grain. The plan's final user acceptance of the image is Z1.

**Completion checklist (gate):**
- [ ] Gate level `standard` requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `render` takes `samples`, averages that many jittered rays per pixel with the one seeded generator, and `main` passes `SAMPLES` equal to 16.
- [ ] `render` carries no sample-count guard, and its doc comment is the agreed sentence, which describes the merged render and is not edited at the merge.
- [ ] `the_top_row_is_sky`, `two_renders_with_the_same_options_give_the_same_pixels` and `the_sample_count_changes_the_pixels` pass.
- [ ] Test budget respected (REPO-RULES §3a); receipt reports functions added/deleted. Two new functions and one adapted test, no deletions.
- [ ] Fast phase gate passes, and the rendered-image conditional gate was run once and the image inspected, with the glass, diffuse and metal spheres all present.
- [ ] No `docs/` page changes are needed: `docs/requirements.md` item 5 already states this behaviour and `README.md` does not name the sample count.
- [ ] `Cargo.toml` has no dependency, and the seeded generator remains the only randomness.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the agent user test has passing evidence.
- [ ] On passing evidence: compress this file into `done/A3-samples-per-pixel.md` (frozen contract only) and flip the `full-plan.md` status row to `done`.

**Exit:** each pixel is the average of 16 jittered samples from the one seeded generator, the same options give the same bytes, and the rendered image has smooth sphere edges without heavy grain.
