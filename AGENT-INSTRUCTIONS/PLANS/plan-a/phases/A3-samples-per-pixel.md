## Phase A3 - samples per pixel

**Status:** active.
**Depends on:** A1 (`done/A1-diffuse-and-metal-materials.md`) and A1r2
(`done/A1r2-shared-render-settlement.md`); the render contract is
`PLANS/shared/plan-a-A1+plan-c-C3.md`.
**Scope:** give `render` a sample count, add the per-pixel sample loop with two jitter offset
draws per sample from the one seeded generator, average in linear space, and pass 16 samples from
`main`. No other file changes beyond the render tests the signature forces or the two new tests.
**Gate level:** standard. The change is bounded runtime behavior in one function, with no external
boundary, stored state or security surface. It changes the rendered image, so the rendered-image
conditional gate applies.
**Simplicity statement:** `samples` is at least 1 by contract, so `render` adds no guard and no
error path. There is one generator, created once in `render`, and no second generator, global,
reseed or per-sample gamma. The phase adds no adaptive sampling, no sample-count invariance, no
`SAMPLES` constant, no precomputed per-pixel ray and no progress line.

**Current state (verified):**
- `src/main.rs:22-25` - `const WIDTH: usize = 200;`, `HEIGHT: usize = 100;`, `SEED: u64 = 7;`,
  `MAX_BOUNCES: usize = 50;`.
- `src/main.rs:29` - `fn ray_colour(ray: &Ray, world: &dyn Hittable, rng: &mut Rng) -> Colour`
  follows `hit.material.scatter` for `MAX_BOUNCES` with the passed generator.
- `src/main.rs:80-93` - `fn render(width: usize, height: usize) -> Vec<Colour>` creates
  `let mut rng = Rng::new(SEED);` once before the loops, computes fixed
  `u = column as f64 / (width - 1) as f64` and `v = row as f64 / (height - 1) as f64` once per
  pixel, and pushes one `ray_colour(&camera.ray(u, v), &world, &mut rng)` per pixel. There is no
  sample loop.
- `src/main.rs:96-98` - `fn to_byte(channel: f64) -> u8` applies
  `(255.999 * channel.clamp(0.0, 1.0).sqrt()) as u8`, the only gamma 2 conversion.
- `src/main.rs:119-122` - `main` calls `render(WIDTH, HEIGHT)` and writes the PPM through
  `write_ppm`.
- `src/main.rs:128-133` - `the_top_row_is_sky` calls `render(WIDTH, HEIGHT)` and asserts
  `pixels[0].z > pixels[0].x`.
- `src/camera.rs:31-38` - `pub fn ray(&self, u: f64, v: f64) -> Ray` takes viewport coordinates in
  0..=1.
- `src/rng.rs:19-22` - `pub fn next_f64(&mut self) -> f64` returns a value in 0..1 from the seeded
  xorshift64* state.
- `src/vec3.rs:5` and `src/vec3.rs:83` - `Vec3` derives `PartialEq` and `Debug`, and `Colour` is a
  `Vec3` alias, so pixel vectors compare with `assert_eq!` and `assert_ne!`.
- `docs/requirements.md:17-18` - item 5 states that each pixel averages a configurable number of
  samples jittered inside the pixel with gamma 2 correction on output.

**Read first (do not invent):**
- `src/main.rs` - the whole file, especially `render`, `main` and the test module; the phase edits
  only these.
- `src/rng.rs` - `next_f64` is the only jitter source.
- `src/camera.rs` - `Camera::ray(u, v)` consumes the jittered coordinates.
- `docs/requirements.md` - item 5 is the behavior this phase implements; no page changes follow.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md` - the frozen `render` shape, the draw
  order and the merge obligations.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the rendered-image command for a branch without a command
  line.

**Build plan:**
1. `src/main.rs`, `render` - change the signature from
   `fn render(width: usize, height: usize) -> Vec<Colour>` to
   `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>` and replace the body
   with exactly this shape:
   ```rust
   /// Return every pixel's colour, row by row from the top, averaging `samples` jittered rays per
   /// pixel. One seeded generator supplies the jitter and every bounce, so a fixed size and sample
   /// count give the same image.
   fn render(width: usize, height: usize, samples: usize) -> Vec<Colour> {
       let world = scene();
       let camera = Camera::new(width as f64 / height as f64);
       let mut rng = Rng::new(SEED);
       let mut pixels = Vec::with_capacity(width * height);
       for row in (0..height).rev() {
           for column in 0..width {
               let mut colour = Colour::new(0.0, 0.0, 0.0);
               for _ in 0..samples {
                   let u = (column as f64 + rng.next_f64()) / (width - 1) as f64;
                   let v = (row as f64 + rng.next_f64()) / (height - 1) as f64;
                   colour = colour + ray_colour(&camera.ray(u, v), &world, &mut rng);
               }
               pixels.push(colour / samples as f64);
           }
       }
       pixels
   }
   ```
   The nesting is row, then column, then sample, then bounce. Each sample draws `u` then `v` from
   the same `rng` before its trace, so the generator sequence is row-major and reproducible. The
   jitter keeps the existing `width - 1` and `height - 1` denominators of the pixel grid. The sum
   is divided once in linear space after the sample loop.
2. `src/main.rs`, `main` - change `let pixels = render(WIDTH, HEIGHT);` to
   `let pixels = render(WIDTH, HEIGHT, 16);`. Add no constant for the sample count.
3. `src/main.rs`, tests - after step 1 the module has exactly three test functions:
   ```rust
   #[test]
   fn the_top_row_is_sky() {
       let pixels = render(WIDTH, HEIGHT, 16);
       assert!(pixels[0].z > pixels[0].x, "the sky is bluer than it is red");
   }

   #[test]
   fn the_same_settings_render_the_same_pixels() {
       assert_eq!(render(40, 20, 4), render(40, 20, 4));
   }

   #[test]
   fn the_sample_count_changes_the_pixel_colours() {
       assert_ne!(render(40, 20, 1), render(40, 20, 4));
   }
   ```
   Adapt `the_top_row_is_sky` to the new signature and keep its assertion and message unchanged.
   The second test proves the seeded generator makes a fixed size and sample count deterministic.
   The third proves the per-sample draws vary inside a pixel, so the sample average is not a
   repeated identical ray.
4. No `docs/` page changes. `docs/requirements.md` item 5 already states jitter, averaging and
   gamma 2, and the repository map in `AGENT-INSTRUCTIONS/LOCAL/project.md` still names
   `main.rs` as the render loop.

**Callers / wiring to update:**
- `src/main.rs:120`, `main` - pass `16` as the third argument.
- `src/main.rs:130`, `the_top_row_is_sky` - pass `16` as the third argument.
- No other caller of `render` exists in `src/` or the repository.

**Config / schema / migrations:**
- No config keys, schema or migrations. On this branch the sample count is the literal `16` at the
  `main` call; the command-line default belongs to the release merge.
- Retained records and their named consumer/owner: none.

**Touched files:**
<!-- touched-files:start -->
- `src/main.rs` - changed: `render` gains `samples` and the jittered per-pixel sample loop, `main` passes 16, and the render tests adapt and gain two functions.
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Create exactly one `Rng::new(SEED)` at the top of `render` and thread `&mut` into `ray_colour`.
  Add no second generator, global, thread-local, reseed or per-pixel generator.
- Keep the draw order: two `next_f64` draws for `u` and `v` per sample, before that sample's
  bounce trace, rows from the top, then columns, then samples.
- Average in linear space with one `colour / samples as f64` after the sample loop. Do not apply
  gamma per sample and do not change `to_byte`.
- Add no guard, clamp or fallback for `samples`. The caller guarantees a value of at least 1.
- Do not assert or preserve sample-count invariance: the image legitimately depends on the sample
  count.
- Do not touch `HitRecord`, `Sphere`, `Camera`, `Rng`, the materials, the scene, `write_ppm`, the
  output or `Cargo.toml`, and do not add a command line.
- Keep the test budget at two new functions. Do not add tests, helpers, fixtures or a rendered-file
  artifact to the repository.
- Write no plan, phase, task or history wording into `src/main.rs`.

**Tests:**
- Changed: `src/main.rs` - `the_top_row_is_sky`: same sky assertion at `render(WIDTH, HEIGHT, 16)`.
- New: `src/main.rs` - `the_same_settings_render_the_same_pixels`: two renders at 40 by 20 with 4
  samples produce equal pixel vectors.
- New: `src/main.rs` - `the_sample_count_changes_the_pixel_colours`: renders at 40 by 20 with 1 and
  4 samples produce different pixel vectors.
- Deleted: none. Budget: 2 new test functions, 1 adapted, 0 deleted.
- Fast gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`,
  `git diff --check`. Use `~/.cargo/bin/cargo` when `cargo` is not on `PATH`.
- Conditional gates: rendered image, required because the render loop and the image change.
  Commands run at the tested head, with `mkdir -p /tmp/opencode` first:
  1. `cargo run --release > /tmp/opencode/render-a3.ppm`
  2. `cargo run --release > /tmp/opencode/render-a3-again.ppm`
  3. `cmp /tmp/opencode/render-a3.ppm /tmp/opencode/render-a3-again.ppm` prints nothing and exits 0.
  4. Convert the default image for viewing with a scratch stdlib-only script, then open the PNG:
     ```bash
     python3 - <<'PY'
     import struct, zlib
     src = "/tmp/opencode/render-a3.ppm"
     dst = "/tmp/opencode/render-a3.png"
     tokens = open(src).read().split()
     assert tokens[0] == "P3"
     width, height, top = int(tokens[1]), int(tokens[2]), int(tokens[3])
     values = [round(int(value) * 255 / top) for value in tokens[4:]]
     rows = [bytes(values[y * width * 3:(y + 1) * width * 3]) for y in range(height)]
     raw = b"".join(b"\x00" + row for row in rows)
     def chunk(kind, data):
         return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)
     png = b"\x89PNG\r\n\x1a\n"
     png += chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0))
     png += chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b"")
     open(dst, "wb").write(png)
     PY
     ```
     Open `/tmp/opencode/render-a3.png` with the image reader and confirm the three spheres show
     their materials and the silhouettes and sphere-ground boundary show intermediate tones from
     the sample average.
  5. Compare with the single-sample image of the previous revision: `git worktree add --detach
     /tmp/opencode/a2-image 80d833c`, run `cargo run --release > /tmp/opencode/render-a2.ppm`
     inside that worktree, convert and open it the same way, then `git worktree remove
     /tmp/opencode/a2-image`. The A2 image has hard stair steps along the sphere edges. The A3
     image shows intermediate tones along those edges.
  6. Small render with few samples: after the safety commit, change only the `main` call to
     `render(80, 40, 4)`, run `cargo run --release > /tmp/opencode/render-a3-small.ppm`, convert
     and open it, confirm it shows the same scene with the same smoother edges, then restore the
     line with `git checkout -- src/main.rs` and confirm `git status --short` is empty.
  7. Remove the scratch files in `/tmp/opencode` this gate created and confirm the `/tmp/opencode`
     PPM and PNG paths above are gone.
- Slow gate: none. The release renders are small and finish well under two minutes.

**User test (acceptance):**
- **Executor:** agent.
- Run the rendered-image commands in the Tests section at the tested head. The observed result is
  the deterministic pair of identical PPM files and an image whose sphere and ground edges are
  smoother than the previous revision's single-sample image.
- The user's acceptance of the final image is Z1, not this phase.

**Completion checklist (gate):**
- [ ] `render` takes `samples`, creates one `Rng::new(SEED)`, nests row, column, sample, bounce,
  draws two jitter offsets per sample before its trace, and divides the sum once in linear space.
- [ ] `main` passes `16`, and `to_byte`, the scene, the materials, the output and `Cargo.toml` are
  unchanged.
- [ ] The three named tests pass and no other test changed.
- [ ] Test budget respected: 2 new functions, 1 adapted, 0 deleted, reported in the receipt.
- [ ] Fast gate and the rendered-image conditional gate pass at the tested head, including the
  identical-bytes comparison, the A2 comparison and the small few-sample render.
- [ ] No `docs/` page changed, and `docs/requirements.md` item 5 remains accurate.
- [ ] The temporary small-render edit is reverted and `git status --short` is empty.
- [ ] Scratch files from the gate are removed.
- [ ] Safety commit on branch `plan-a`.
- [ ] Revision-bound verification receipt exists with base `80d833c` and the tested head.
- [ ] On passing evidence: compress this file into `done/A3-samples-per-pixel.md` (frozen contract
  only) and flip the `full-plan.md` status row to `done`.

**Exit:** every pixel is the average of 16 jittered rays from the one seeded generator, two renders
at the same settings are byte-identical, and the rendered scene has smooth edges with the diffuse,
metal and glass spheres and the ground intact.
