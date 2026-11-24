## Phase A3 - Samples per pixel [DONE]

Implemented and verified. Build instructions retired. Later work consumes these
frozen contracts:

- `render(width: usize, height: usize, samples: usize) -> Vec<Colour>` in `src/main.rs` averages
  `samples` jittered rays per pixel: one `let mut rng = Rng::new(SEED);` before both loops, and each
  sample draws `u = (column as f64 + rng.next_f64()) / width as f64` first, then
  `v = (row as f64 + rng.next_f64()) / height as f64`, before that sample's scattering draws, then
  adds `ray_colour(&camera.ray(u, v), &world, &mut rng)`. The pixel is pushed as
  `pixel / samples as f64`. The row loop stays `for row in (0..height).rev()`.
- Branch `plan-a` keeps `const SAMPLES: usize = 16;` directly below `const HEIGHT: usize = 100;`,
  used only by `main`'s `render(WIDTH, HEIGHT, SAMPLES)`. The release merge deletes `SAMPLES`,
  `WIDTH` and `HEIGHT`; `DEFAULT_SAMPLES = 16` is the single default there and `run` passes
  `settings.samples`.
- `render` carries no sample-count validation. `parse_args` in the merged command line is the single
  validation path and rejects a count below 1. There is no clamping, rounding, retry or fallback.
- The `render` doc comment is the shared design's two-line sentence: "Every pixel's colour, row by
  row from the top, each pixel averaging `samples` jittered rays, with the share of finished rows
  reported on standard error." It describes the merged product; branch `plan-a` writes no progress
  line, and the sentence is edited only here and never again at the merge.
- Tests in `src/main.rs`: `the_top_row_is_sky` calls the literal `render(200, 100, 16)` with its
  assertion unchanged, `two_renders_with_the_same_options_give_the_same_pixels` asserts
  `render(20, 10, 2)` twice is equal, and `the_sample_count_changes_the_pixels` asserts
  `render(20, 10, 1)` differs from `render(20, 10, 4)`. No test names `WIDTH`, `HEIGHT` or `SAMPLES`.
- Unchanged by this phase: `ray_colour`, `scene`, `to_byte` and gamma 2, `write_ppm`, the four scene
  spheres and their materials, `MAX_BOUNCES`, `SEED`, `Cargo.toml` (still no dependency), and the
  module header and `mod` list.
- Verification at branch head `566b11c`, base `a1d640a`: `cargo build` clean, `cargo test` 12 passed,
  `cargo clippy -- -D warnings` clean, `cargo fmt --check` clean, `git diff --check` clean. The
  rendered-image conditional gate ran once: the 200 by 100 image at 16 samples converted to PNG and
  inspected shows the glass sphere left inverting the background over a near-uniform sky (mean
  adjacent-pixel difference under 0.1 per channel), the diffuse red-brown centre sphere, the diffuse
  grey ground and the grey metal sphere, with soft edges, no sky grain and no pure-black pixel. Test
  budget: 2 functions added, 0 deleted, 1 adapted.
- Known carry-over: the release merge takes plan-c's progress write into this `render` with no other
  change, per `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md`, and takes plan-b's record and
  shapes per `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-b-Z1.md`. The glass sphere and the
  sphere's `(u, v)` must keep working when the geometry shapes join the same record.
