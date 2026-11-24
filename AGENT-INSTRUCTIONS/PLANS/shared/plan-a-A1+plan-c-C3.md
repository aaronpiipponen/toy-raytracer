# Shared design: plan-a-A1 + plan-c-C3 (`render` loop, generator, gamma, progress line)

Settled between the plan-a and plan-c orchestrators. Both plans reach this design on their own
branches; commits never move between branches, no branch is merged into another, and the release
merges both into the integration branch and resolves the textual conflicts in `src/main.rs` by the
shape below. plan-c leads this pair; plan-c's C1 is already landed, so the merged `render` starts
from plan-c's code and A1's material loop lands inside it.

## Target merged `render`

```rust
fn render(width: usize, height: usize, samples: usize) -> Vec<Colour> {
    let world = scene();
    let camera = Camera::new(width as f64 / height as f64);
    let mut rng = Rng::new(SEED);
    let mut pixels = Vec::with_capacity(width * height);
    for row in (0..height).rev() {
        for column in 0..width {
            let u = column as f64 / (width - 1) as f64; // A3 jitters these later
            let v = row as f64 / (height - 1) as f64;
            let ray = camera.ray(u, v);
            let mut colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                colour = colour + ray_colour(&ray, &world, &mut rng);
            }
            pixels.push(colour / samples as f64);
        }
        eprint!("\r{} of {} rows", height - row, height);
    }
    eprintln!();
    pixels
}
```

## Rules the merge follows

1. `render(width, height, samples) -> Vec<Colour>` is frozen by plan-c's C1. On plan-a's branch A1
   still builds `render(width, height)` with the `WIDTH` and `HEIGHT` constants, because that is the
   branch's API at its base; that is not a conflict, it is branch state. The merged tree takes C1's
   signature and its settings/command-line path, and A1's material loop lands inside `ray_colour`.
   `WIDTH` and `HEIGHT` do not survive the merge.
2. Nesting is row, then column, then sample, then bounce: one trace per sample, with the same
   generator threaded through every bounce of that trace.
3. Exactly one `Rng::new(SEED)` is created at the top of `render` and passed `&mut` down into
   `ray_colour`. No second generator, no global, no per-pixel or per-sample reseeding. Consumption
   order is row-major, so a fixed sample count reproduces byte-identically.
4. `ray_colour` keeps its name and its 50-bounce material loop, returns linear radiance, returns
   black at the bounce limit and the sky gradient on a miss.
5. Averaging happens in linear space in `render`. `to_byte` applies gamma 2 (`sqrt`) once to the
   averaged channel and stays the single output conversion that both the PNG and PPM writers use.
   Gamma is never applied per sample.
6. `render` returns the pixel vector unchanged to `run(&Settings)`, which encodes and writes it as
   C1 leaves it. Neither writer changes in this settlement.
7. The progress line is the two `eprint`/`eprintln` calls inside the row loop after the column loop,
   inside `render`. It counts rows only and is independent of samples and bounces. Neither A1 nor A3
   moves, renames or re-implements it.
8. A3 keeps this signature and the command-line-owned defaults (400 by 200, 16 samples). Its only
   change is replacing the fixed `u`, `v` with jittered ones drawn from the same generator.

## Sample count guarantees

- `samples` is always at least 1 by the time `render` runs: C1's option parser rejects a value below
  1 with the usage line and exit status 2, and `tests/cli.rs` already covers `--samples 0`. Nothing
  in `render`, A1 or A3 may add a second guard for it.
- Once materials consume the generator, the image legitimately depends on the sample count; that is
  requirement 5. C1's recorded "same bytes at every sample count" described the pre-randomness state
  and is not preserved, and no test asserts that invariance (`tests/cli.rs` uses the option only as a
  value and as a rejected bad value), so nothing is deleted for it. What must hold afterwards: each
  fixed sample count reproduces byte-identically across runs.

## A3 draw order

For each sample, before the bounce trace: two `next_f64` draws offset `u` and `v` inside the pixel,
then the bounce scattering draws for that trace. Row-major across the image. Two draws per sample is
the fixed order, so A3's detail and the merge cannot disagree about where the draws happen.

## What each plan does on its own branch

- **plan-c / C1 (landed):** the command line, the settings, the PNG and PPM writers, and
  `render(width, height, samples)` averaging identical rays. `to_byte` is `pub(crate)` and shared by
  both writers.
- **plan-c / C3:** adds the two progress calls and the doc-comment sentence to `render` only. No
  signature, loop-shape, pixel or stream change.
- **plan-a / A1:** unchanged as detailed on its branch: `src/material.rs`, the material field on
  `HitRecord`, materials on `Sphere`, the 50-bounce `ray_colour`, `sky_colour`, `SEED = 7`, gamma in
  `to_byte`, the scene's diffuse and metal objects. At the merge its loop lives inside the sample
  loop above and its `to_byte` body is the one both writers use.
- **plan-a / A3:** jittered `u`, `v` per sample in the existing sample loop, using the generator
  created in `render`.

## Tests at the merge

- A1's deletion of `the_center_pixel_shows_the_sphere_facing_the_camera` stands.
- `the_top_row_is_sky` stays with plan-c's signature: `render(200, 100, 16)`.
- `tests/cli.rs` stays exactly as plan-c's C3 leaves it.
- plan-a's `src/material.rs`, `src/sphere.rs` and `src/hittable.rs` tests are unaffected by this pair
  (the `HitRecord` shape is the other pair's settlement).

## Documentation state

- No `docs/` page changes: requirement 5 already states averaging, jitter and gamma 2, and
  requirement 13 the progress line.
- Neither branch's permanent files name this settlement; the release merge is bound by this file.
