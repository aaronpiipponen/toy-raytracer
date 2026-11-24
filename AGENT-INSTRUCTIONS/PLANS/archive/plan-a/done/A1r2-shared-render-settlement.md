# A1r2 - Shared render settlement (frozen contract)

## Guarantees later phases build on

- The shared render design settled with the output plan lives in
  `PLANS/shared/plan-a-A1+plan-c-C3.md`. This branch records it and implements no part of it.
- On this branch A1 stays as detailed: `render(width, height)` with the `WIDTH` and `HEIGHT`
  constants, one `Rng::new(SEED)` at the top of `render`, the 50-bounce material `ray_colour` with
  `sky_colour`, gamma 2 in `to_byte`, and the scene's diffuse centre, diffuse ground and metal
  sphere. The merged tree takes plan-c's `render(width, height, samples)` and its settings and
  command-line path; that is the release merge's work, not a change to A1 on this branch.
- The merged `render` starts from plan-c's C1: `render(width: usize, height: usize, samples: usize)`
  builds the scene, the camera and exactly one `Rng::new(SEED)`, then for each row from the top and
  each column computes `u` and `v`, traces `samples` rays through `ray_colour` with that one
  generator threaded `&mut` through every bounce, and pushes their linear average. There is no
  second generator, no global, and no per-pixel or per-sample reseeding.
- Nesting is row, then column, then sample, then bounce: one trace per sample with the same
  generator. Consumption is row-major, so each fixed sample count reproduces byte-identically.
- `ray_colour` keeps its name and its 50-bounce material loop, returns linear radiance, returns
  black at the bounce limit and the sky gradient on a miss.
- Averaging happens in linear space in `render`. Gamma 2 comes from A1 on this branch: `to_byte`
  applies `sqrt` once to the averaged channel, and the merge keeps that one `to_byte` body as the
  single conversion both the PNG and PPM writers use. Gamma is never applied per sample.
- `render` returns the pixel vector unchanged to `run(&Settings)`, which encodes and writes it as
  plan-c's C1 leaves it; neither writer changes in this settlement.
- The progress line is plan-c's C3 pair of calls inside `render`: an `eprint!` after the column loop
  inside the row loop, and a trailing `eprintln!` after the row loop. It counts rows only and is
  independent of samples and bounces. A1 and A3 do not move, rename or re-implement it; the merge
  keeps plan-c's calls.
- `samples` is at least 1 before `render` runs: plan-c's option parser rejects a value below 1 with
  the usage line and exit status 2, and `tests/cli.rs` covers `--samples 0`. Neither `render`, A1 nor
  A3 adds a second guard. Once materials consume the generator the image depends on the sample
  count, and each fixed sample count reproduces byte-identically.
- A3 creates the sample loop on this branch, because A1 leaves `render(width, height)` with no
  sample loop. A3's branch shape is the frozen `render(width, height, samples)` signature, with 16
  passed from `main`, one `Rng::new(SEED)` before the loops, and the nesting above. Per sample,
  before the bounce trace, it draws two `next_f64` offsets for `u` and `v` inside the pixel, averages
  in linear space, leaves gamma in `to_byte`, and adds no sample-count guard. After the merge A3
  keeps the frozen signature and the command-line-owned defaults (400 by 200, 16 samples), and its
  only remaining change is the jitter.
- At the release merge plan-c's identical sample loop wins textually, so the merge deduplicates the
  loop rather than making A3 rely on plan-c's. A3's contribution to the merged tree is the jitter
  draws: for each sample, two `next_f64` draws offset `u` and `v` inside the pixel, then the bounce
  scattering draws for that trace, row-major across the image.
- The release merge's obligations on plan-a's side: call A1's `ray_colour` loop from inside plan-c's
  sample loop; make A1's `to_byte` body the one both writers use; drop `WIDTH` and `HEIGHT`; keep
  plan-c's progress calls; keep `the_top_row_is_sky` calling `render(200, 100, 16)` and keep the
  deletion of `the_center_pixel_shows_the_sphere_facing_the_camera`; leave `tests/cli.rs` alone; add
  no `samples` guard.
- Tests at the merge: A1's deletion of `the_center_pixel_shows_the_sphere_facing_the_camera` stands.
  `the_top_row_is_sky` stays with plan-c's signature, `render(200, 100, 16)`. `tests/cli.rs` stays as
  plan-c's C3 leaves it. This pair leaves plan-a's `src/material.rs`, `src/sphere.rs` and
  `src/hittable.rs` tests alone; the `HitRecord` shape is the other pair's settlement.

## Documentation state

- No `docs/` page changes. `docs/requirements.md` item 5 already states averaging, jitter and gamma 2,
  and item 13 the progress line.
- Neither branch's permanent files name this settlement; the release merge is bound by the shared
  file.
- This phase changed plan documents only: no product code, test, `docs/` page, `Cargo.toml` or
  configuration. Its verification was the documentation-only gate: the changed plan documents were
  re-read and `git diff --check` was clean.
