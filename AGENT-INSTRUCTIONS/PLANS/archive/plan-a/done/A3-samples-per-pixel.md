# A3 - Samples per pixel (frozen contract)

## Guarantees later phases build on

- `render` in `src/main.rs` is
  `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>`. It builds the scene, a
  `Camera::new(width as f64 / height as f64)`, exactly one `Rng::new(SEED)` with `SEED = 7`, and the
  row-major pixel vector.
- The nesting is row (`(0..height).rev()`), then column, then sample, then bounce. For each sample,
  before its trace, it draws two offsets from that one generator:
  `let u = (column as f64 + rng.next_f64() - 0.5) / (width - 1) as f64;` then
  `let v = (row as f64 + rng.next_f64() - 0.5) / (height - 1) as f64;`. The `- 0.5` centres every
  draw inside its own pixel, so a single sample lands within half a pixel of the pixel centre. The
  pixel grid keeps the `width - 1` and `height - 1` denominators. Consumption is row-major across
  the image, so a fixed size and sample count give the same pixels.
- The per-pixel sum starts at `Colour::new(0.0, 0.0, 0.0)`, adds
  `ray_colour(&camera.ray(u, v), &world, &mut rng)` in linear space for each sample, and is divided
  once by `samples as f64` after the sample loop. Gamma 2 stays in `to_byte` and is never applied
  per sample. `to_byte`, `write_ppm`, the scene, the materials, `HitRecord`, `Sphere`, `Camera`,
  `Rng` and `Cargo.toml` are unchanged.
- `main` calls `render(WIDTH, HEIGHT, 16)`. The sample count is that literal, with no `SAMPLES`
  constant. `the_top_row_is_sky` calls `render(WIDTH, HEIGHT, 16)` and keeps its sky assertion and
  message.
- `render` adds no guard, clamp or fallback for `samples`; the caller guarantees a value of at least
  1. The image depends on the sample count, and each fixed sample count reproduces byte-identically.
- No `docs/` page changed. `docs/requirements.md` item 5 (a configurable number of samples, default
  16, jittered inside the pixel, gamma 2 on output) is accurate, and the repository map still names
  `main.rs` as the render loop.

## Release merge obligation

Per `PLANS/shared/plan-a-A1+plan-c-C3.md`, the merged tree starts from plan-c's
`render(width, height, samples)` and replaces its fixed `u` and `v` with the two centred jitter
draws above. Each sample keeps two draws before its trace and the row-major order. The merged loop
keeps plan-c's progress calls, its command line passes `--samples`, `the_top_row_is_sky` keeps
plan-c's signature at `render(200, 100, 16)`, and `tests/cli.rs` is unchanged.

## Tests

- `src/main.rs` tests: `the_top_row_is_sky` adapted to `render(WIDTH, HEIGHT, 16)`, plus the new
  `the_same_settings_render_the_same_pixels` (two `render(40, 20, 4)` calls are equal) and
  `the_sample_count_changes_the_pixel_colours` (`render(40, 20, 1)` differs from
  `render(40, 20, 4)`).
- Budget used: 2 new test functions, 1 adapted, 0 deleted.
- Verification at tested head `46a5c02`, base `80d833c`: `cargo build`, `cargo test` (12 passed),
  `cargo clippy -- -D warnings`, `cargo fmt --check` and `git diff --check` all clean. The
  rendered-image gate produced two byte-identical release PPMs, and the image shows the diffuse,
  metal and glass spheres and the ground intact, with edges smoother than the single-sample image
  rendered from `80d833c`.

## Carry-overs into Z1

- None.
