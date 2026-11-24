## Z1r - Shared render seam resolution

**Status:** done.
**Depends on:** C1 and C3, whose frozen contracts are in `done/C1-png-writer-and-command-line.md`
and `done/C3-progress-line.md`, and the shared design
`AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md`, which is the one copy of the settlement.
**Gate level:** minimal.

**Frozen contract:**
- `render` keeps the signature `fn render(width: usize, height: usize, samples: usize) ->
  Vec<Colour>` in `src/main.rs`, with the parameter names `width`, `height` and `samples`. This
  signature is the seam plan-a's A3 consumes. `render` carries no sample-count validation:
  `parse_args` is the single validation path.
- The `render` doc comment is one sentence: "Every pixel's colour, row by row from the top, each
  pixel averaging `samples` rays, with the share of finished rows reported on standard error." On
  branch `plan-c` the sentence has no "jittered". The merged sentence adds the word "jittered"
  before "rays", because the merged loop jitters, and that clause is the only line A3 writes among
  plan-c's landed lines.
- Progress keeps plan-c's exact form and position: the `done` value `height - row` and the
  `eprint!("\r{}%", done * 100 / height)` call are the last statements of the row loop, after the
  column loop, and the single `eprintln!()` follows the row loop. Plan-c writes no second progress
  line.
- The merged code has no `SAMPLES`, `WIDTH` or `HEIGHT` constant, and branch `plan-c` has none
  either. `DEFAULT_SAMPLES = 16` beside `DEFAULT_WIDTH = 400` and `DEFAULT_HEIGHT = 200` is the
  single sample default, read into the parsed `samples`. `main` to `run` passes `settings.samples`
  into `render`, and `parse_args` rejects a count below 1.
- Branch `plan-c` keeps the unjittered `column / (width - 1)` and `row / (height - 1)` mapping.
  Jitter arrives through plan-a's A3 in the merge as `(column as f64 + rng.next_f64()) / width as
  f64` and `(row as f64 + rng.next_f64()) / height as f64`, drawn `u` first then `v` inside the
  sample loop before that sample's scattering draws, with one `Rng::new(SEED)` before both loops.
- Release merge split. Plan-a brings `scene`, `ray_colour`, `Rng`, `SEED`, `MAX_BOUNCES`, the
  `to_byte` gamma body, the jitter mapping and the `samples` averaging. Plan-c brings the module
  header `//! Render a fixed scene and write it to a file as a PNG or PPM image.`, `mod png;`,
  `Format`, `Settings`, `USAGE`, `DEFAULT_WIDTH`, `DEFAULT_HEIGHT`, `DEFAULT_SAMPLES`,
  `DEFAULT_OUTPUT`, `parse_args`, `parse_dimension`, `resolve_format`, `run`, `main`,
  `to_byte` with `pub(crate)` so `png::write_png` shares it, and the progress write. The merge
  deletes plan-a's `SAMPLES`, `WIDTH` and `HEIGHT` constants.
- `README.md`, `docs/requirements.md` and `tests/cli.rs` are plan-c's. `tests/cli.rs` needs no
  change in the merge, because PNG-versus-PPM pixel equality does not depend on the materials.
- The merged test module holds `the_top_row_is_sky` adapted to the literal call
  `render(200, 100, 16)` with its assertion unchanged, `two_renders_with_the_same_options_give_the_same_pixels`
  and `the_sample_count_changes_the_pixels`, both with literal calls only.
  `the_center_pixel_shows_the_sphere_facing_the_camera` does not survive the merge, because
  materials replace normal colouring.

**Proof in the tree (branch `plan-c`, head `a0c585b`):**
- `src/main.rs:62` holds the signature, `:60-61` the doc sentence, `:77-78` the progress pair as
  the last statements of the row loop, and `:80` the single `eprintln!()`.
- `src/main.rs:23-25` holds the three defaults, `:111` reads `DEFAULT_SAMPLES` into `samples`,
  `:131-133` rejects a count below 1, and `:186` passes `settings.samples` into `render`.
- A search of `src/` finds no `SAMPLES`, `WIDTH` or `HEIGHT` constant.
- The acceptance render at `a0c585b` through
  `target/release/toy-raytracer --width 400 --height 200 --samples 16 --output
  /tmp/opencode/resolution.png` exited with status 0. The PNG is 400 by 200, 8-bit RGB,
  non-interlaced, and opens showing the sphere over the blue-to-white sky and the green ground
  plane. Standard output is 0 bytes. Standard error is 783 bytes of percentages separated by `\r`,
  ending `100%` plus one newline.
- This phase changes no code or test, so the fast-gate evidence at `a0c585b` stands. The
  documentation gate and the acceptance render are its evidence.
