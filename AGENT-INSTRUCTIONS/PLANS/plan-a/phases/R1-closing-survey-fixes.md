## Phase R1 - Closing survey fixes

**Status:** active.
**Depends on:** the accepted closing survey in `benchmarks/closing-survey-2026-11-24.md` and the
contracts in `done/A1r2-shared-render-settlement.md` and `done/A3-samples-per-pixel.md`.
**Scope:** two accepted survey defects. S1: correct the `HitRecord` doc comment so `t` is a ray
parameter, and give the `0.001` self-intersection lower bound in `ray_colour` a comment that does
not call it a distance. S2: shrink `the_top_row_is_sky` to a small image and assert a few pixels of
its top row, and correct the one line of `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md`
that freezes the old call. Nothing else changes.
**Gate level:** minimal. One doc comment, one code comment and one test's size and assertions. No
renderer behavior changes, and the rendered bytes must be identical before and after, which the
phase proves directly. The `minimal` requirements plus the phase-specific byte comparison of the
whole rendered image are the verification. The test-scope edit keeps the equal-behavior guarantee
of the fast gate.
**Simplicity statement:** the phase corrects wording and one test's size, nothing more. It does not
touch `Vec3::cross` or its comment in `src/vec3.rs` (the geometry plan's plane phase and the
release merge resolve it), does not action any rejected survey claim (the `refract` `.abs()`
guard, the `1e-12` rejection bound, the jitter formula, the two 40x20 whole-vector comparisons),
adds no helper to the test, adds no test, and adds no record, receipt or hash. It handles no new
case and creates no second code path.

**Current state (verified):**
- `src/hittable.rs:9-10` - the doc comment above `pub struct HitRecord { pub t: f64, pub point:
  Vec3, pub normal: Vec3, pub material: Rc<dyn Material> }` reads "Where a ray hit a surface: the
  distance along the ray, the point, the outward normal and the material of the object." The
  "distance" word is wrong: `Ray::at` is `self.origin + self.direction * t` (`src/ray.rs:16-18`),
  and `Camera::ray` (`src/camera.rs:31-35`) builds the direction as
  `lower_left + horizontal * u + vertical * v - origin`, which has unit length only at the viewport
  centre. `Sphere::hit` (`src/sphere.rs:29,37-40`) solves in that parameter with
  `a = ray.direction.dot(ray.direction)`.
- `src/main.rs:33` - `if let Some(hit) = world.hit(&ray, 0.001, f64::INFINITY) {` inside
  `ray_colour`. The `0.001` is a lower bound on the same ray parameter and currently carries no
  comment, so nothing claims it is a distance.
- `src/main.rs:134-138` - `#[test] fn the_top_row_is_sky()` calls `render(WIDTH, HEIGHT, 16)` with
  `WIDTH = 200` and `HEIGHT = 100` (`src/main.rs:22-23`) and asserts only
  `pixels[0].z > pixels[0].x`. It renders 320,000 samples and proves one pixel.
- `src/main.rs:140-148` - the two neighbouring tests render `render(40, 20, 4)`,
  `render(40, 20, 4)` and `render(40, 20, 1)`.
- `src/main.rs:87-98` - `render` pushes pixels row-major starting at `row = height - 1`, so the top
  row is `pixels[0..width]`. Every top-row ray misses the scene (`src/main.rs:54-77`) and returns
  `sky_colour`, which is a blend of `(1, 1, 1)` and `(0.5, 0.7, 1.0)` with weight
  `0.5 * (unit(direction).y + 1.0) > 0.5` on the top row, so its blue channel exceeds its red
  channel.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md:93` - under "Tests at the merge" reads
  "- `the_top_row_is_sky` stays with plan-c's signature: `render(200, 100, 16)`."
- `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md:37` - "Rendering tests check a few pixels of a
  small image, never a whole image."
- `benchmarks/closing-survey-2026-11-24.md:141-151` - the user decision accepts R1 and R2 together
  and approves the one-line shared-file correction.

**Read first (do not invent):**
- `src/hittable.rs` - the `HitRecord` doc comment and fields; only the comment changes.
- `src/main.rs` - `ray_colour` and its `0.001` call, `sky_colour`, `render`'s pixel order, the
  `WIDTH`/`HEIGHT` constants and the three tests; the comment and one test change.
- `src/ray.rs` - `Ray::at` and `Ray { pub origin, pub direction }`; the evidence for the parameter
  wording.
- `src/camera.rs` - `Camera::ray(u, v)`; the evidence that the direction is not unit length.
- `src/sphere.rs` - `Sphere::hit` with `a = ray.direction.dot(ray.direction)`; the evidence for the
  parameter wording.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md` - read the whole file, edit only the one
  line at line 93.
- `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md` (the `documentation-style` skill) -
  the rules for the new doc comment, code comment and test assertion message.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast phase gate and the pre-command-line render
  command.
- `benchmarks/closing-survey-2026-11-24.md` - the accepted findings and the user decision.

**Build plan:**
1. `src/hittable.rs`, the `HitRecord` doc comment - replace the two lines with a comment that
   states what `t` is and what the other fields are, for example:
   `/// Where a ray hit a surface: the point `origin + direction * t`, the outward normal and the`
   `/// material of the object. `t` is the ray parameter, which is the distance along the ray when`
   `/// the direction has unit length.`
   Keep the field order covered by the words in the same order the struct lists them if that reads
   better, keep it to three comment lines or fewer, and make no other change to the struct. Fix any
   wording in the finished comment that the style guide's self-check rejects.
2. `src/main.rs`, `ray_colour` line 33 - add one comment line directly above the `world.hit` call
   that states the `0.001` lower bound in ray-parameter units and the effect it has, for example:
   `// The lower bound of 0.001 in ray-parameter units keeps a scattered ray from re-hitting the`
   `// surface it left.`
   Do not change the expression, the call or any other line in `ray_colour`.
3. `src/main.rs`, `the_top_row_is_sky` - `render(40, 20, 4)` replaces `render(WIDTH, HEIGHT, 16)`,
   and the single `pixels[0]` assertion becomes an assertion over a few top-row pixels with
   `blue > red`, for example:
   ```rust
   let pixels = render(40, 20, 4);
   for column in [0, 20, 39] {
       let pixel = pixels[column];
       assert!(
           pixel.z > pixel.x,
           "the sky is bluer than it is red at column {column}"
       );
   }
   ```
   The top row is `pixels[0..40]` because `render` pushes rows from the top. Keep the test name
   `the_top_row_is_sky`, keep the assertion message's meaning, add no helper and no second test,
   and leave the two neighbouring tests unchanged.
4. `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md`, line 93 only - replace the bullet
   with a line stating the corrected test scope, for example:
   `- `the_top_row_is_sky` renders a small image, `render(40, 20, 4)`, and asserts a few pixels of`
   `  its top row.`
   Edit no other line in the file, and do not reopen the pair or change the `A3` and plan-c
   sections.
5. No `docs/` page, `README.md` or `AGENT-INSTRUCTIONS/LOCAL/project.md` change: the renderer's
   behaviour and the repository map are unaffected.

**Callers / wiring to update:**
- None in product code. The two comment edits and the test edit are file-local.
- The corrected test scope supersedes the corresponding statements in `done/A3-samples-per-pixel.md`
  (its guarantee "`the_top_row_is_sky` calls `render(WIDTH, HEIGHT, 16)`" and its release-merge
  obligation "keeps plan-c's signature at `render(200, 100, 16)`"). Those files are frozen and are
  not edited here. When this phase is compressed, `done/R1-closing-survey-fixes.md` records that it
  supersedes that test call, so the release merge follows the corrected scope.

**Config / schema / migrations:**
- None.
- Retained records and their named consumer/owner: none.

**Touched files:**
<!-- touched-files:start -->
- `src/hittable.rs` - changed: the `HitRecord` doc comment states the ray parameter, not a distance.
- `src/main.rs` - changed: one comment on the `0.001` lower bound; `the_top_row_is_sky` renders `render(40, 20, 4)` and asserts a few top-row pixels.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md` - changed: one line in "Tests at the merge" records the corrected test scope.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/R1-closing-survey-fixes.md` - created: this phase document.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - changed: the `R1` row is added and `Z1` depends on it.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Doc comments and comments only for the `t` correction and the `0.001` note. Do not change a
  signature, struct field, expression, statement or constant in `src/hittable.rs` or `src/main.rs`,
  and do not change the produced bytes in any way.
- Do not touch `src/vec3.rs`, its `cross` method or its comment. The geometry plan's plane phase
  and the release merge resolve it.
- Do not action any rejected survey claim: the `refract` `.abs()` guard (`src/material.rs:102`),
  the `1e-12` bound in `random_unit_vector` (`src/material.rs:121`), the jitter formula, and the
  two 40x20 whole-vector comparisons stay as they are.
- Do not add a helper function, a second test, a parameter or a fixture to the sky test. One lean
  test with a loop over a few top-row pixels.
- Do not edit any line of `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md` except line 93,
  and do not reopen the pair with plan-c.
- Do not name a plan, phase or survey id in `src/` comments, `docs/`, `README.md` or the shared
  file line.
- No new record, receipt, hash or manifest.

**Tests:**
- New/changed/deleted: `src/main.rs` - `the_top_row_is_sky`: renders `render(40, 20, 4)` and asserts
  `blue > red` at a few top-row pixels. Budget: 0 added, 1 changed, 0 deleted, within REPO-RULES
  §3a.
- Fast gate: `~/.cargo/bin/cargo build`, `~/.cargo/bin/cargo test`, `~/.cargo/bin/cargo clippy --
  -D warnings`, `~/.cargo/bin/cargo fmt --check`, `git diff --check`. `cargo test` proves the
  corrected sky test passes and the other 11 tests still pass.
- Conditional gates:
  - Rendered bytes: required by this phase as its proof that the doc-only product change is inert.
    Render once at the tested head and once at base `46a5c02` in a detached worktree, then compare
    the two PPM files byte for byte. Exact commands:
    ```bash
    ~/.cargo/bin/cargo run --release > /tmp/opencode/r1-head.ppm
    git worktree add --detach /tmp/opencode/r1-base 46a5c02
    (cd /tmp/opencode/r1-base && ~/.cargo/bin/cargo run --release > /tmp/opencode/r1-base.ppm)
    cmp /tmp/opencode/r1-head.ppm /tmp/opencode/r1-base.ppm
    git worktree remove /tmp/opencode/r1-base
    ```
    `cmp` must report no difference. Remove the three `/tmp/opencode/r1-*` scratch files and the
    worktree afterwards, and confirm both are gone. Do not inspect the image subjectively and do
    not run the render twice at one revision.
  - The rendered-image gate's seam rule does not otherwise apply, because the render loop, the
    materials, the camera and the output format are unchanged.
- Slow gate (over two minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - automated phase gates prove acceptance. The fast gate proves the test change, and
  byte-identical PPMs against `46a5c02` prove the comment-only product change renders the same
  image. The image's appearance was accepted at the closing survey.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `src/hittable.rs`'s `HitRecord` doc comment states that `t` is the ray parameter and when it
  equals a distance, and no longer calls it a distance unconditionally.
- [ ] `src/main.rs`'s `0.001` lower bound carries a comment that states it in ray-parameter units
  and claims no distance, and the expression is unchanged.
- [ ] `the_top_row_is_sky` renders `render(40, 20, 4)`, asserts `blue > red` at a few top-row
  pixels, keeps its name, adds no helper and no second test.
- [ ] `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md` differs from base only in the one
  line under "Tests at the merge".
- [ ] `src/vec3.rs` is unchanged, and no rejected survey claim was actioned.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a): 0 added, 1 changed, 0 deleted; the receipt reports
  the functions added/deleted. Any line counts are observations, not gates.
- [ ] Fast phase gate and the rendered-byte comparison pass; the detached worktree and the scratch
  files are removed.
- [ ] No `docs/` page, `README.md`, `AGENT-INSTRUCTIONS/LOCAL/project.md` or `Cargo.toml` change,
  and none of them is stale.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists.
- [ ] On passing evidence: compress this file into `done/R1-closing-survey-fixes.md` (frozen
  contract only, including the corrected `the_top_row_is_sky` call) and flip the `full-plan.md`
  status row to `done`, which leaves `Z1` as the next phase.

**Exit:** the two accepted survey defects are fixed, the shared render note records the corrected
sky-test call, and the fast gate plus the byte comparison pass.
**Move-on gate:** the fast gate and the named test pass, and the survey note records the fix.
