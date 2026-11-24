## Phase A3r - Samples per pixel resolution [DONE]

Implemented and verified. Build instructions retired. Later work consumes these
frozen contracts:

- The shared sampling design is `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md`. Both
  plans read it as the one copy, and this plan cites it rather than copying it.
- `phases/A3-samples-per-pixel.md` is amended in place to that design and sits at `planned`. The
  amendments are: a branch-local `const SAMPLES: usize = 16` below `const HEIGHT`, used only by the
  `main` call `render(WIDTH, HEIGHT, SAMPLES)`; `render` takes `samples` and averages that many
  jittered rays per pixel with the one `Rng::new(SEED)` created before both loops; each sample draws
  `u = (column + rng.next_f64()) / width` first, then `v = (row + rng.next_f64()) / height`, before
  that sample's scattering draws; `render` carries no sample-count validation; the `render` doc
  comment is the shared design's two-line sentence; `the_top_row_is_sky` calls the literal
  `render(200, 100, 16)`; and the two new tests use literal calls only,
  `render(20, 10, 2)` twice and `render(20, 10, 1)` versus `render(20, 10, 4)`.
- A3 adds no progress write, changes the module header and `mod` list only through plan-c's merge,
  and touches no writer, `to_byte`, scene or material. The merged command line is the single
  sample-count validation path.
- Release merge split: plan-a's branch contributes `scene`, `ray_colour`, `Rng` usage, `SEED`,
  `MAX_BOUNCES`, the `to_byte` gamma body, the one `Rng::new(SEED)` before both loops, the jitter
  mapping, the `samples` averaging and the three test functions. Plan-c's branch contributes the
  module header, `mod png;`, `Format`, `Settings`, `USAGE`, `DEFAULT_WIDTH`, `DEFAULT_HEIGHT`,
  `DEFAULT_SAMPLES`, `DEFAULT_OUTPUT`, `parse_args`, `parse_dimension`, `resolve_format`, `run`,
  `main`, the progress write, the `pub(crate)` visibility on `to_byte`, `README.md`,
  `docs/requirements.md` and `tests/cli.rs`.
- The merged `render` takes plan-a's scene, camera, generator, `ray_colour` and jitter with plan-c's
  progress write and doc sentence. The merged code has no `SAMPLES`, `WIDTH` or `HEIGHT` constant.
  `DEFAULT_SAMPLES = 16` is the single default, `run` passes `settings.samples`, and `parse_args`
  rejects a count below 1.
- The merged test module holds `the_top_row_is_sky`,
  `two_renders_with_the_same_options_give_the_same_pixels` and
  `the_sample_count_changes_the_pixels`. Plan-c's
  `the_center_pixel_shows_the_sphere_facing_the_camera` does not survive, because A1 replaced normal
  colouring with materials.
- Neither branch adds a second `render` path, a second sample default, a second generator or a
  second progress write.
- Verification: the documentation-only gate passed with a changed-file reread and a clean
  `git diff --check` at the workspace root. No fast gate, build, test or render ran. The A1 and A2
  fast-gate evidence at A2's landed head `a1d640a` stands unchanged. Test budget: 0 functions
  added, 0 deleted, 0 changed.
- Exit state: A3 remains `planned` with `phases/A3-samples-per-pixel.md` and is implemented after
  the pair returns `RESOLVED`.
