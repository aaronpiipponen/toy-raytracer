# Z1 - Closing survey and user acceptance (frozen contract)

## Guarantees later phases build on

- The closing survey record is `benchmarks/closing-survey-2026-11-24.md`: branch `plan-a`, surveyed
  range `e180476..46a5c02`, four independent read-only area surveys, one independent cross-check,
  findings S1 to S3 with `path:line` evidence, the seven categories, out-of-scope observations, the
  cross-check outcome, the acceptance render, the remediation candidates, the user-review list and
  the user decision.
- The four area returns and the cross-check return are saved verbatim in
  `benchmarks/closing-survey-returns-2026-11-24.md`. Both survey files are plan records, stay
  unedited, and are the evidence the plan's release reads.
- On 2026-11-24 the user accepted the survey findings and the final rendered image. The note's user
  decision section records that date, that acceptance and every decision below. The accepted image
  is the default-settings render at 200x100 inspected against requirements 1 to 6: light blue
  gradient sky, grey ground, matte red-brown centre sphere, small grey metal sphere with a blurred
  reflection and a glass sphere refracting the scene, with no normal-gradient colours and no black
  regions.
- The two accepted clear defects are fixed by the remediation unit R1, whose frozen contract is
  `done/R1-closing-survey-fixes.md`: the `HitRecord` doc comment and the `0.001` bound's comment
  describe `t` as the ray parameter rather than a distance, and `the_top_row_is_sky` renders
  `render(40, 20, 4)` and asserts the top-row pixels at columns 0, 20 and 39. R1 changed no rendered
  byte.
- R1 supersedes the statements in the frozen `done/A3-samples-per-pixel.md` that
  `the_top_row_is_sky` calls `render(WIDTH, HEIGHT, 16)` and that the release merge keeps
  `render(200, 100, 16)`. That file stays frozen and unedited, and the release merge follows
  `render(40, 20, 4)`. `PLANS/shared/plan-a-A1+plan-c-C3.md` already states that call.
- One item is deferred to another owner: `Vec3::cross` and its plan-naming comment in `src/vec3.rs`
  belong to the geometry plan's plane-and-triangle phase and are resolved at the release merge.
  Nothing in plan-a's `src/vec3.rs` changed, and the deferred item is not plan-a work.
- The rejected survey claims stay rejected: no change to `refract`'s `.abs()`, to the `1e-12` lower
  bound in `random_unit_vector`, to the jitter formula, or to the two 40x20 whole-vector
  comparisons. They are recorded so they are not re-raised.

## Test surface

- No test function was added, changed or deleted by Z1; the suite stays at 12 and Z1's own budget is
  zero. R1 changed one existing test function, `the_top_row_is_sky`, with no addition or deletion.

## Verification

- Z1's gate was the documentation-only gate - every created plan file re-read and `git diff --check`
  clean - plus the single default-settings acceptance render at the phase's head.
- No product code, test, `docs/` page, `README.md`, `Cargo.toml` or configuration changed in Z1.
- The plan is not archived or closed by this phase; the release owns closure and archiving.
