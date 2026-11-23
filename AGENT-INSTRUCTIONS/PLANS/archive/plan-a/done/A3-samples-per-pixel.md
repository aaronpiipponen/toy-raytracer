## Phase A3 - Samples per pixel (done)

**Status:** done.
**Gate level:** standard.

**Frozen contract:**
- `src/main.rs` holds `const SAMPLES: usize = 16;` between `HEIGHT` and `MAX_BOUNCES`, and `main` calls `render(WIDTH, HEIGHT, SAMPLES)`.
- `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>` builds one `Rng::new(RNG_SEED)` per call, iterates rows from the top and columns from the left, and pushes one colour per pixel.
- Each pixel sums `samples` terms. For every sample it draws `u = (column as f64 + rng.next_f64()) / width as f64` and then `v = (row as f64 + rng.next_f64()) / height as f64`, and adds `ray_colour(camera.ray(u, v), &world, &mut rng)`. The pixel is `total / samples as f64`.
- The two jitter draws come before `ray_colour`, `u` before `v`, once per sample, and the same generator continues into `ray_colour`. This draw order and the single seeded generator are the reproducibility contract.
- The sample count is used as given. `render` holds no check, clamp or fallback, and `main` passes the constant `16`. A command-line `--samples` option belongs to the output plan.
- `ray_colour`, `scene`, the camera, the materials, `to_byte`, `write_ppm` and the PPM header keep their signatures and behaviour. `src/main.rs` is the only changed source file, `Cargo.toml` gained no dependency, and rendering stays single-threaded with no `unsafe`.
- Tests: `a_pixel_averages_four_jittered_samples` asserts that `render(1, 1, 4)[0]` equals a recomputation from the same seeded draws. `the_center_pixel_shows_the_diffuse_sphere` and `the_top_row_is_sky` pass `1` as the sample count with their assertions unchanged. The suite holds 12 test functions.
- `docs/requirements.md` and `README.md` stay accurate, no `docs/` page changed, and no `docs/` page refers to a plan.

**Evidence:** base `d079bb0`, tested head `dca633c`. `cargo build`, `cargo test` (12 passed, 0 failed), `cargo clippy -- -D warnings`, `cargo fmt --check` and `git diff --check` passed. The rendered-image conditional gate rendered 200 by 100 at 16 samples: the opened image showed smooth sphere silhouettes, the ground horizon and the metal reflection with no single-sample speckle or isolated dot, and the four spheres kept their materials. Test functions added 1, deleted 0.

**Carry-overs:** none in code. Documentation drift for the closing survey's user-review list, not fixed here:
- `AGENT-INSTRUCTIONS/LOCAL/project.md:5` does not name `src/material.rs` in the repository map.
- `src/vec3.rs:21-22` carries a comment naming the geometry plan.
