# Shared design: plan-a-A3 + plan-c-Z1 (jittered samples × progress line)

Agreed by the plan-a and plan-c orchestrators. Both plans read this as the one copy. It settles
implementation detail only; it opens no user-owned decision, because jittered sampling (requirement
5) and the progress line (requirement 13) are already user-approved requirements.

## Target shape of the shared code

The merged `render` in `src/main.rs` takes plan-a's scene, camera, generator and `ray_colour`, and
plan-c's progress write:

```rust
/// Every pixel's colour, row by row from the top, each pixel averaging `samples` jittered rays, with
/// the share of finished rows reported on standard error.
fn render(width: usize, height: usize, samples: usize) -> Vec<Colour> {
    let world = scene();
    let camera = Camera::new(width as f64 / height as f64);
    let mut rng = Rng::new(SEED);
    let mut pixels = Vec::with_capacity(width * height);
    for row in (0..height).rev() {
        for column in 0..width {
            let mut colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (column as f64 + rng.next_f64()) / width as f64;
                let v = (row as f64 + rng.next_f64()) / height as f64;
                colour = colour + ray_colour(&camera.ray(u, v), &world, &mut rng);
            }
            pixels.push(colour / samples as f64);
        }
        let done = height - row;
        eprint!("\r{}%", done * 100 / height);
    }
    eprintln!();
    pixels
}
```

- **Progress:** the three progress lines keep plan-c's exact form and position: the `done`/`eprint!`
  pair is the last statement before the row loop's closing brace, after the column loop, and the
  single `eprintln!()` follows the row loop. A3 edits only the `render` doc comment among plan-c's
  landed lines.
- **Doc comment:** exactly the one sentence above. It is the only doc-comment sentence for `render`.
- **Jitter mapping:** `u` and `v` are drawn inside the sample loop as
  `(column + rng.next_f64()) / width` and `(row + rng.next_f64()) / height`, `u` first, then `v`,
  before that sample's scattering draws. This replaces plan-c's unjittered `(width - 1)` mapping.
- **Samples:** the merged code has no `SAMPLES` constant and no `WIDTH`/`HEIGHT` constants. The single
  default is plan-c's `DEFAULT_SAMPLES = 16` (beside `DEFAULT_WIDTH = 400` and `DEFAULT_HEIGHT =
  200`), and `main` -> `run` passes `settings.samples` into `render`. `render` carries no sample-count
  validation: `parse_args` is the single validation path and rejects a count below 1.

## Which plan changes what

**plan-a, A3, branch `plan-a`:**

- Adds plan-a's `const SAMPLES: usize = 16` beside `WIDTH`/`HEIGHT`; that constant is this branch's
  local default only, and its one use is the `main` call `render(WIDTH, HEIGHT, SAMPLES)`.
- Changes `render` to average `samples` jittered rays per pixel with the one `Rng::new(SEED)` created
  before both loops, using the jitter mapping above.
- Drops the planned `assert!(samples > 0)`; `render` does not validate the count.
- Replaces the `render` doc comment with the one agreed sentence, and only that clause about progress
  is added to what A3 would otherwise write.
- Adapts `the_top_row_is_sky` to the literal call `let pixels = render(200, 100, 16);` (byte-identical
  to plan-c's landed line) and keeps its assertion unchanged.
- Adds its two new tests with literal calls only: `render(20, 10, 2)` twice for
  `two_renders_with_the_same_options_give_the_same_pixels`, and `render(20, 10, 1)` versus
  `render(20, 10, 4)` for `the_sample_count_changes_the_pixels`. No test names or calls
  `WIDTH`, `HEIGHT` or `SAMPLES`.
- Does not touch the module header, `mod` list, `main` beyond the sample argument, the writers,
  `to_byte`, the scene, the materials or any output behaviour.

**plan-c, Z1r, branch `plan-c`:**

- Changes no product code. Z1r records this design, confirms plan-c's landed `render` seam and its
  frozen contract, and re-runs plan-c's acceptance render; plan-c's own branch keeps the unjittered
  loop and its existing doc sentence until the merge, because jitter is plan-a's work.
- The closing survey note and the user's acceptance of it stay as they are.

## Release merge

When plan-a and plan-c merge into the integration branch:

- Take plan-a's `scene`, `ray_colour`, `Rng`, `SEED`, `MAX_BOUNCES` and `to_byte` gamma body, with
  plan-c's `pub(crate)` on `to_byte` so `png::write_png` shares it.
- Take plan-c's module header (`//! Render a fixed scene and write it to a file as a PNG or PPM
  image.`), `mod png;`, `Format`, `Settings`, `USAGE`, `DEFAULT_WIDTH`, `DEFAULT_HEIGHT`,
  `DEFAULT_SAMPLES`, `DEFAULT_OUTPUT`, `parse_args`, `parse_dimension`, `resolve_format`, `run`,
  `main` and the progress write.
- Take the merged `render` above, including the agreed doc sentence.
- Delete plan-a's `SAMPLES`, `WIDTH` and `HEIGHT` constants; plan-c has no such constants.
- The test module after the merge holds `the_top_row_is_sky`,
  `two_renders_with_the_same_options_give_the_same_pixels` and
  `the_sample_count_changes_the_pixels`. plan-c's
  `the_center_pixel_shows_the_sphere_facing_the_camera` does not survive, because materials replaced
  normal colouring in A1; plan-a's branch already dropped it.
- `tests/cli.rs` is plan-c's and needs no change: PNG-versus-PPM pixel equality is material-agnostic.
- README and `docs/requirements.md` stay plan-c's; the scene, materials and gamma are plan-a's.

## What each may rely on

- plan-a may rely on `render(width, height, samples)` and on `main`/`run` passing the resolved count
  after the merge, and on the progress writes being plan-c's.
- plan-c may rely on jitter, the generator and materials arriving through plan-a's branch without any
  change to its command line, writers or progress contract.
- Neither plan adds a second `render` path, a second default for the sample count, a second generator,
  or a second progress write.
