# Closing survey, 2026-11-24

Branch `plan-a`, surveyed `e180476..46a5c02`. Method: four independent read-only area surveys in
parallel, one independent cross-check over all their claims, then this note written by the plan
orchestrator. Full returns: `closing-survey-returns-2026-11-24.md`.

Areas: 1 `src/material.rs` and the `Material` trait; 2 `src/hittable.rs` and `src/sphere.rs`;
3 `src/main.rs` and `src/rng.rs`; 4 tests and documentation.

## Area 1 - materials

No finding survived the cross-check.

Validated clean: requirements 1 to 4 and 6; one trait, three implementations, no duplicate scatter
path; all four private helpers used; six tests split 3 and 3 across A1 and A2, each within the §3a
cap; no records, hashes or telemetry; index 1.5, standard library only, no `unsafe`.

Claims rejected by the cross-check (recorded so they are not re-raised):

- The `.abs()` in `refract` (`src/material.rs:102`) is not a speculative guard to remove. The phase
  named the helper and its semantics, not its body; at the Snell boundary the radicand can be a few
  ulps negative through a different rounding path, and `.abs()` yields the tangent direction instead
  of a NaN. It cannot mask wrong output for valid input.
- The `> 1e-12` bound in `random_unit_vector` (`src/material.rs:121`) is not a deviation. The phase
  asked for rejection sampling in the unit ball and did not fix the loop test; excluding a radius
  1e-6 ball cannot bias the direction and prevents dividing by zero. That it is a guard is accepted
  as the standard form of the helper, not as extra machinery.

## Area 2 - hit record and spheres

**S1. Stale doc: `t` is described as a distance, but it is a ray parameter.**
`src/hittable.rs:9-10` says "the distance along the ray". `Ray::at` is `origin + direction * t`
(`src/ray.rs:16-18`), the camera's direction is not unit length except at the viewport centre
(`src/camera.rs:19-21,31-35`), and `Sphere::hit` solves the quadratic in the parameter with
`a = direction·direction` (`src/sphere.rs:29,37-40`). `t` is therefore the ray parameter, not a
distance; the same applies to the `0.001` self-intersection epsilon (`src/main.rs:33`). The paragraph
was rewritten by this plan's A1 commit and kept the wrong word, so it is in this plan's changed area.

Validated clean: every record field consumed; one nearest-hit loop; one construction path; material
cloned once per hit; no dead code; sphere tests focused; the branch's record shape matches the
settled shared design.

## Area 3 - render loop and randomness

**S2. The sky test renders the whole product-size image to assert one pixel, and its name overstates.**
`src/main.rs:135-138` calls `render(WIDTH, HEIGHT, 16)`, 200×100×16 = 320,000 samples, and asserts
only `pixels[0]`. `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md:37` says rendering tests check a few
pixels of a small image, never a whole image. Measured cost: 0.32 s in a debug test run, essentially
the whole suite (the two 40×20 render tests cost 0.01 s each). The test asserts a single top-left
pixel, not a row. The call is frozen by the render settlement (`done/A3-samples-per-pixel.md`,
`done/A1r2-shared-render-settlement.md`), so a fix also corrects one line in
`PLANS/shared/plan-a-A1+plan-c-C3.md`.

Validated clean: 50-bounce loop with black at the limit and absorption; attenuation multiplication;
gamma 2 only in `to_byte`; row/column/sample/bounce nesting; two jitter draws per sample; one
generator per render; no other randomness source; the scene's four objects; determinism.

Claims rejected by the cross-check:

- The jitter does not leave its pixel. Each draw lands within ±0.5 pixel of its own cell's centre
  (`src/main.rs:91-92`). Under the frozen `width - 1` grid the outermost cells overhang the
  viewport (column 199's cell is `[198.5/199, 199.5/199)`); the cells, not the samples, cross the
  edge, and the grid is inherited and frozen. Requirement 5 holds.
- The two 40×20 whole-vector comparisons (`src/main.rs:142,147`) are a literal reading of "never a
  whole image", but that image is small (0.01 s), and the invariants they prove (byte-identical
  reproducibility, sample-count dependence) are whole-buffer properties no few pixels can prove.
  The A3 contract fixes them. Not actionable; recorded as a known, accepted literal deviation.
- `Rng::new`'s `seed.max(1)` (`src/rng.rs:9`) is unreachable with the fixed seed and guards the
  xorshift zero fixed point; unchanged from the base.
- A Lambertian direction of near-zero length when the random unit vector nearly opposes the normal
  has probability effectively zero; explicitly not a finding under the hobby-simplicity rule.

## Area 4 - tests and documentation

**S3. `Vec3::cross` is dead code behind a comment that names a plan and a feature absent here.**
`src/vec3.rs:21-29`: `// For the triangle normals the geometry plan adds.` above
`#[allow(dead_code)] pub fn cross`, with no caller on this branch. REPO-RULES §4 item 13 forbids plan
ids in permanent files. This is not this plan's code and not this plan's fix: the geometry plan's
plane-and-triangle phase (plan-b) deletes the comment and the allowance, and its branch already has
the corrected line. The release merge resolves it.

**Observation, no action: the glass "does not attenuate" fact is asserted in each of the three glass
tests** (`src/material.rs:201,221,243`) although the return is unconditional. Each test stands alone
and the assertions are free; not bloat.

Validated clean: 12 test functions, no suite duplication, no test of a private helper; assertions
exercise distinct paths; README and `docs/requirements.md` accurate for this branch and unchanged by
it; no other plan or phase names anywhere in `src/`, `docs/` or `README.md`.

## Categories

- Unnecessary complexity: none.
- Duplication: repeated white-attenuation assertions in the three glass tests - accepted, no action.
- Dead code: `Vec3::cross` - real, owned by the geometry plan, resolved at the merge.
- Redundant check or record: none.
- Test bloat: the whole-image sky render (S2).
- Stale docs: the hit record's "distance" (S1); the `Vec3::cross` comment (S3).
- Drift from the user's decisions: none. All settled decisions hold - glass 1.5, 16 samples, centred
  jitter, gamma once, one seeded generator, single-threaded, standard library only, no `unsafe`.

## Out-of-scope observations

- `Vec3::cross` and its comment (S3), owned by the geometry plan and already corrected on its
  branch.
- Requirement 5's "configurable number of samples" is a render parameter here with 16 passed from
  `main`; the user-facing `--samples` belongs to the output plan and is not on this branch.

## Cross-check outcome

All four changed files (`src/hittable.rs`, `src/main.rs`, `src/material.rs`, `src/sphere.rs`) and
requirements 1 to 6 are covered by at least one area. Of the area claims: the material guards and
the jitter-overhang finding were rejected, the edge-cell range and the "inherited unchanged" wording
were corrected, the top-row runtime was measured (0.32 s) and 12 tests were re-run (12 passed). The
cross-check added: the sky test's name overstates its one-pixel assertion; the `HitRecord` sentence
was touched by this plan; the `0.001` epsilon shares the parameter-versus-distance issue; and the
near-zero Lambertian direction, reported as explicitly not a finding.

## Acceptance render

`~/.cargo/bin/cargo run --release > /tmp/opencode/plan-a-z1-accept.ppm`, exit 0. Converted to
`/tmp/opencode/plan-a-z1-accept.png` at 200×100 and inspected: light blue gradient sky, grey ground,
matte red-brown centre sphere, small grey metal sphere on the right with a blurred reflection, and a
transparent glass sphere on the left refracting the scene; smooth pixel edges from the 16 jittered
samples. No normal-gradient colours and no black regions. Matches requirements 1 to 6.

## Remediation candidates (clear defects)

- **R1 (S1):** correct the hit record's doc comment so `t` is described as the ray parameter, not a
  distance, and make the `0.001` epsilon's doc avoid the same unit claim. Doc-only.
- **R2 (S2):** shrink the sky test to a small image (40×20) and assert a few pixels of its top row,
  so the name is true and the domain test rule holds; update the one line in
  `PLANS/shared/plan-a-A1+plan-c-C3.md` that freezes `render(200, 100, 16)`. Test-only plus one plan
  line; needs plan-c's awareness because the line is in a closed cross-plan settlement.

## User-review list

- Whether to do R2 at all. It is a one-line test change plus one line in the closed render settlement
  with plan-c; leaving it costs 0.32 s and keeps a literal deviation from the local test rule.
  Recommendation: do it, and note the corrected call in the shared design.

## User decision

Pending. The user's acceptance of these findings and of the render above is the phase's final gate.
