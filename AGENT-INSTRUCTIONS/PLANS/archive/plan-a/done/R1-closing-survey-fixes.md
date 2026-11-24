# R1 - Closing survey fixes (frozen contract)

## Guarantees later phases build on

- The `HitRecord` doc comment in `src/hittable.rs` describes the point as `origin + direction * t`
  and states that `t` is the ray parameter, equal to the distance along the ray when the direction
  has unit length. The fields are `t`, `point`, `normal` and `material`; no field, signature or
  construction path changed.
- `ray_colour` in `src/main.rs` carries a comment above its `world.hit` call stating that the
  `0.001` lower bound is in ray-parameter units and that it keeps a scattered ray from hitting the
  surface it left. The call expression is unchanged.
- `the_top_row_is_sky` in `src/main.rs` renders `render(40, 20, 4)` and asserts `blue > red` at the
  top-row pixels for columns `0`, `20` and `39`, with the message `the sky is bluer than it is red
  at column {column}`. The top row is `pixels[0..40]` because `render` pushes rows from the top. The
  test keeps its name, adds no helper, and the two neighbouring tests are unchanged.
- This corrected call supersedes the statements in `done/A3-samples-per-pixel.md` that
  `the_top_row_is_sky` calls `render(WIDTH, HEIGHT, 16)` and that the release merge keeps plan-c's
  signature at `render(200, 100, 16)`. The release merge follows `render(40, 20, 4)`.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md` states under "Tests at the merge" that
  `the_top_row_is_sky` renders a small image, `render(40, 20, 4)`, and asserts a few pixels of its
  top row. No other line of that file changed.
- The rendered bytes are identical to base `46a5c02`: the change is a doc comment, a code comment
  and a test. `src/vec3.rs` is unchanged, and the rejected survey claims (`refract`'s `.abs()`
  guard, the `1e-12` bound in `random_unit_vector`, the jitter formula, the two 40x20 whole-vector
  comparisons) stay as they are.
- No `docs/` page, `README.md`, `AGENT-INSTRUCTIONS/LOCAL/project.md` or `Cargo.toml` change.
  `benchmarks/closing-survey-2026-11-24.md` is unedited.

## Tests

- `src/main.rs`: `the_top_row_is_sky` changed. Budget: 0 added, 1 changed, 0 deleted.
- Verification at tested head `a7ce19d`, base `46a5c02`: `cargo build`, `cargo test` (12 passed),
  `cargo clippy -- -D warnings`, `cargo fmt --check` and `git diff --check` clean. Release PPMs
  rendered at head and at base `46a5c02` in a detached worktree compared identical.

## Carry-overs into Z1

- None.
