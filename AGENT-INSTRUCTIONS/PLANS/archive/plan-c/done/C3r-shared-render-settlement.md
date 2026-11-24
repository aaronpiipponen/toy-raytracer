# C3r - Shared render settlement (frozen contract)

## Guarantees later phases and the release merge build on

- The merged `render` design settled with the materials plan lives in
  `PLANS/shared/plan-a-A1+plan-c-C3.md`. This branch records it; it does not implement the materials
  plan's half.
- The merged signature is `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>`.
  `WIDTH` and `HEIGHT` do not survive the merge; A1's branch still uses them at its base, which is
  branch state, not a conflict.
- Nesting is row, then column, then sample, then bounce: one trace per sample, with the same
  generator threaded through every bounce of that trace.
- Exactly one `Rng::new(SEED)` is created at the top of `render` and passed `&mut` into `ray_colour`.
  No second generator, no global, no per-pixel or per-sample reseeding. Consumption is row-major, so
  a fixed sample count reproduces byte-identically.
- `ray_colour` keeps its name and its 50-bounce material loop, returns linear radiance, returns black
  at the bounce limit and the sky gradient on a miss.
- Averaging happens in linear space in `render`. `to_byte` applies gamma 2 (`sqrt`) once to the
  averaged channel and stays the single output conversion both the PNG and PPM writers use. Gamma is
  never applied per sample.
- `render` returns the pixel vector unchanged to `run(&Settings)`, which encodes and writes it as C1
  leaves it. Neither writer changes in this settlement.
- The progress line is `eprint!("\r{} of {} rows", height - row, height)` inside the row loop after
  the column loop, plus the terminating `eprintln!()` after the row loop, both inside `render`. It
  counts rows only and is independent of samples and bounces. Neither A1 nor A3 moves, renames or
  re-implements it.
- A3 keeps this signature and the command-line-owned defaults (400 by 200, 16 samples). Its only
  change is replacing the fixed `u`, `v` with jittered ones drawn from the same generator. For each
  sample, before the bounce trace, two `next_f64` draws offset `u` and `v` inside the pixel, then the
  bounce scattering draws for that trace, row-major across the image.

## Sample count guarantees

- `samples` is at least 1 by the time `render` runs: C1's option parser rejects a value below 1 with
  the usage line and exit status 2, and `tests/cli.rs` already covers `--samples 0`. Nothing in
  `render`, A1 or A3 adds a second guard.
- Once materials consume the generator, the image legitimately depends on the sample count; that is
  requirement 5. C1's recorded "same bytes at every sample count" described the pre-randomness state
  and is not preserved. No test asserts that invariance, so nothing is deleted for it. Each fixed
  sample count must reproduce byte-identically across runs.

## Branch state and merge obligations

- On this branch C1's `render` creates no `Rng` and `ray_colour` has no bounce loop; `src/rng.rs` is
  `#[allow(dead_code)]` until the materials plan's code arrives at the merge.
- C3 adds only the two progress calls and the doc-comment sentence inside `render`; no signature,
  loop-shape, pixel or stream change. C3's existing detail stands unchanged.
- The release merge starts from plan-c's code and lands A1's material loop and A3's jitter inside it,
  preserving the progress calls and taking the single top-level generator, the linear averaging and
  the single gamma in `to_byte`.
- A1's deletion of `the_center_pixel_shows_the_sphere_facing_the_camera` stands. `the_top_row_is_sky`
  stays with plan-c's signature, `render(200, 100, 16)`. `tests/cli.rs` stays exactly as C3 leaves it.
  Plan-a's material, sphere and hit-record tests are unaffected by this pair.

## Documentation state

- No `docs/` page changes: requirement 5 already states averaging, jitter and gamma 2, and requirement
  13 the progress line.
- `full-plan.md` records C3r as done and C3 active behind it, states the settlement once in the
  carry-overs, and adds the C3r scoped entry. `phases/C3-progress-line.md` is unchanged.
- This phase changed plan documents only: no product code, test, `docs/` page, `Cargo.toml` or
  configuration. The verification is the documentation-only gate: the changed plan documents were
  re-read and `git diff --check` was clean at the workspace root.
