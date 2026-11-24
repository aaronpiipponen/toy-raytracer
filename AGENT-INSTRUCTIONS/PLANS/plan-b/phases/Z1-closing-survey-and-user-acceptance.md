## Phase Z1 - Closing survey and user acceptance

**Status:** active.
**Depends on:** B1r (`done/B1r-shared-hit-record-shape.md`), B2 (`done/B2-planes-and-triangles.md`)
and B3 (`done/B3-bounding-boxes.md`): every one of this plan's product commits is on the branch.
Also B2r (`done/B2r-shared-scene-settlement.md`) for the release merge's obligations.
**Scope:** the binding closing survey of everything this plan added or changed, and the user's
acceptance of the result. The survey is read-only. This phase writes two plan files, the saved
scout returns and the dated note, and runs the acceptance render. It changes no product code,
test, `docs/` page, `Cargo.toml` or configuration, and fixes no finding.
**Gate level:** minimal. The phase changes no product behavior: it reads the tree, runs one release
render and writes plan records. The rendered-image conditional gate runs because the image is the
user's acceptance subject and the survey's evidence.
**Simplicity statement:** a survey, not a fix list. Every finding is recorded with its evidence and
handed to the user. No remediation opens here, no extra test or checker is added, and no scout is
asked to design a repair. One release render is the only runtime work.

**Current state (verified):**
- Survey base: `git merge-base main HEAD` is `e180476`, and the plan's product changes span
  `e180476..c25007c`. `git diff --stat e180476..HEAD` reports 7 files, 368 insertions and 11
  deletions: `src/aabb.rs`, `src/hittable.rs`, `src/main.rs`, `src/plane.rs`, `src/sphere.rs`,
  `src/triangle.rs`, `src/vec3.rs`. The worktree's `AGENTS.md` and `AGENT-INSTRUCTIONS` paths are
  excluded by `project.git/info/exclude`, so product checks scope `git status` to the project paths
  named in build step 7.
- `src/hittable.rs:7-43` - `HitRecord` holds `t`, `point`, `normal`, `front_face`, `u`, `v` and
  derives `Clone, Copy, Debug`; `HitRecord::new` is the one constructor and sets the normal against
  the ray. `src/hittable.rs:45-51` - `Hittable` has `hit` and `bounding_box`.
  `src/hittable.rs:65-87` - `HittableList::hit` tests a box over `t_min..t_max`, then calls the
  object with the running `limit`; `HittableList::bounding_box` returns `None`.
- `src/sphere.rs:20-52` - `Sphere::hit` maps (u, v) from the outward normal at `:40-43` and reports
  its box at `:47-51`. `src/plane.rs:24-58` - `Plane::hit` normalises in `Plane::new`, maps (u, v)
  at `:35-50` and reports no box at `:54-57`. `src/triangle.rs:22-64` - Möller-Trumbore `hit` records
  barycentric (u, v) at `:33-47` and reports its vertex box at `:50-63`.
- `src/aabb.rs:6-48` - `Aabb` with `new` and the slab test `hit`. `src/main.rs:3-13` - the module
  list; `:28-35` - `ray_colour`; `:37-50` - `scene()` with the centre sphere, the ground plane and
  the triangle; `:53-65` - `render`; `:67-93` - output and `main`.
- Tests: 13 functions. `src/main.rs:95-110` (2), `src/aabb.rs:50-61` (1), `src/hittable.rs:89-109`
  (1), `src/sphere.rs:54-98` (4), `src/plane.rs:60-75` (1), `src/triangle.rs:66-109` (3),
  `src/rng.rs:25-37` (1).
- `docs/requirements.md:21-30` - items 7 to 10 state the face side and normal, the texture
  coordinates, planes and triangles, and the box test. `README.md:1-10` - the overview, the run
  commands and the requirements link. `AGENT-INSTRUCTIONS/LOCAL/project.md:5` - the repository map
  omits `aabb.rs`, `plane.rs` and `triangle.rs`. `AGENT-INSTRUCTIONS/PLANS/plan-b/docs-pass.md:5-12`
  records that map update as the release's item together with plan-c's `png.rs` half.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md:26-45` - the fast gate and the conditional gates; the
  rendered-image gate's fallback command applies because this branch writes PPM to standard output
  and has no command line.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/` does not exist, so the note is created new at
  `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/2026-11-24-closing-survey.md`. The returns file is
  created new at `AGENT-INSTRUCTIONS/PLANS/plan-b/survey-returns-2026-11-24.md`. If the phase runs on
  a later date, both filenames use that run date.
- This branch has no material field and no `Rc`; the release merge adds them. `docs/requirements.md`
  items 1 to 6 and 11 to 14 describe the merged product and the other plans, so their absence here
  is branch state, not drift introduced by this plan.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANS/plan-b/full-plan.md` - the plan's scope, the phase table and Z1's intent.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/B1r-shared-hit-record-shape.md`,
  `done/B2-planes-and-triangles.md` and `done/B3-bounding-boxes.md` - the contracts the survey checks
  the code against. `done/B2r-shared-scene-settlement.md` - the release merge's obligations, which
  this phase leaves untouched.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B1.md` and
  `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md` - the shared designs for the record and
  the scene.
- `src/hittable.rs`, `src/aabb.rs`, `src/sphere.rs`, `src/plane.rs`, `src/triangle.rs`,
  `src/main.rs`, `src/vec3.rs`, `src/ray.rs`, `src/camera.rs`, `src/rng.rs` - the whole product tree
  the survey covers.
- `docs/requirements.md`, `README.md`, `AGENT-INSTRUCTIONS/LOCAL/project.md`,
  `AGENT-INSTRUCTIONS/PLANS/plan-b/docs-pass.md`, `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the prose and
  release records the survey checks.
- `AGENT-INSTRUCTIONS/SUBAGENTS/ops-scout.md` - the scout's read-only limits and its
  answer/evidence/unknowns output, so each brief fits it.

**Build plan:**
1. Fix the survey list from the branch, not from this document: run
   `git diff --name-only e180476..HEAD` and `git rev-parse HEAD`, and record the base, head and file
   list in the note. Expected list: `src/aabb.rs`, `src/hittable.rs`, `src/main.rs`, `src/plane.rs`,
   `src/sphere.rs`, `src/triangle.rs`, `src/vec3.rs`. If the list differs from this document, stop
   and report the drift instead of narrowing the survey.
2. Start six fresh read-only `ops-scout` sessions, one per area below, all six in parallel before
   any return is read. Each brief names the worktree
   `/home/opc/projects/toy-raytracer-runs/run003/plan-b`, the one area, its files and questions,
   and the output contract: **Answer** in two to five sentences, **Evidence** as `path:line` or
   `command -> fact`, **Unknowns**, and **Out-of-scope observations** with evidence. Tell each scout
   it is read-only, that it must not run tests, builds, formatters or providers, must not edit or
   change git state, must not judge a diff, and must report missing evidence rather than guess. Give
   each scout the whole changed-file list so it can place adjacent observations outside its area.

   **Survey areas (one `ops-scout` each, in parallel):**

   - **Area 1 - Hit record and face side.** `src/hittable.rs:7-43`,
     `src/main.rs:28-35`, and each shape's record construction at `src/sphere.rs:44`,
     `src/plane.rs:44-51`, `src/triangle.rs:47`. Check against `done/B1r-shared-hit-record-shape.md`:
     one construction path, `front_face` from the sign of `ray.direction.dot(outward_normal)`,
     the normal always against the ray, the `Clone, Copy, Debug` derives, and whether each
     `#[allow(dead_code)]` on `point`, `front_face`, `u` and `v` at `src/hittable.rs:11-21` names a
     real consumer or is unexplained dead code on this branch. Check the doc comments at
     `src/hittable.rs:7-8,25-26` and the tests at `src/sphere.rs:58-80`. Report any second
     construction path, a face-side sign that contradicts the contract, or plan history in a
     comment.
   - **Area 2 - Texture coordinates.** `src/sphere.rs:40-43`, `src/plane.rs:35-50`,
     `src/triangle.rs:33-38,47` and the fields at `src/hittable.rs:19-21`. Check each mapping
     against `docs/requirements.md:25-27` (item 8) and `done/B1r` and `done/B2`: spheres by latitude
     and longitude from the outward normal, planes on the two axes the dominant normal component
     does not use with `rem_euclid(1.0)`, triangles by barycentric weights, all values in 0..=1.
     Check the tests at `src/sphere.rs:82-91`, `src/plane.rs:64-74` and `src/triangle.rs:70-86`.
     Report a mapping that can leave the contracted range, a coordinate taken from the ray-facing
     normal, or a coordinate that is recomputed rather than recorded.
   - **Area 3 - Plane and triangle.** `src/plane.rs:1-75`, `src/triangle.rs:1-109` and the scene
     constructions at `src/main.rs:40-48`, plus `src/vec3.rs:21-27` for `cross`. Check against
     `done/B2-planes-and-triangles.md`: plane normalisation and parallel epsilon, triangle
     determinant epsilon and barycentric bounds, two-sided records, the degenerate rejection, and
     the exact scene vertices and normals. Check for a duplicate hit path, a material or `Rc`
     field, dead helpers, and comments that describe history or a plan. Check the tests at
     `src/plane.rs:60-75` and `src/triangle.rs:66-97`.
   - **Area 4 - Bounding boxes.** `src/aabb.rs:1-61`, `src/hittable.rs:45-51,65-109`,
     `src/sphere.rs:47-51`, `src/plane.rs:54-57`, `src/triangle.rs:50-63,99-109` and
     `docs/requirements.md:29-30` (item 10). Check against `done/B3-bounding-boxes.md`: `Aabb::new`
     orders the corners, the slab test rejects a missed box over `t_min..t_max` and handles a ray
     parallel to an axis, every shape implements the required `bounding_box`, the plane and the
     list return `None` and are still tested, and `HittableList::hit` tests the box over the full
     range before the object with the running limit so the chosen record cannot change. Report any
     padding, shrink, merge, ordering, hierarchy, cached box, second hit path, or a box that can
     drop a real hit. State whether the tests prove each claim and whether the current code still
     explains the byte-identical image.
   - **Area 5 - Scene and render output.** `src/main.rs:25-93`, `src/vec3.rs:5-35`,
     `src/ray.rs:5-19`, `src/camera.rs:1-37` and `src/rng.rs:1-37`. Check the scene against
     `done/B2` (centre sphere at `(0, 0, -1)` radius `0.5`, ground plane at `y = -0.5` with normal
     `(0, 1, 0)`, triangle at `(-2, -0.5, -2)`, `(2, -0.5, -2)`, `(0, 1.5, -2)`), the row order and
     aspect in `render`, the single output path in `write_ppm`, and the two render tests at
     `src/main.rs:95-110`. Check that every random value would come from the seeded generator and
     that no material, `Rc`, command line, PNG or progress line has appeared on this branch. Report
     a scene value that differs from the contract, a second output path, output on standard output
     beyond the PPM bytes, or a comment that is stale.
   - **Area 6 - Tests and documents.** Every test module at `src/main.rs:95-110`,
     `src/aabb.rs:50-61`, `src/hittable.rs:89-109`, `src/sphere.rs:54-98`,
     `src/plane.rs:60-75`, `src/triangle.rs:66-109`, `src/rng.rs:25-37`; `docs/requirements.md`;
     `README.md:1-10`; `AGENT-INSTRUCTIONS/LOCAL/project.md:5`;
     `AGENT-INSTRUCTIONS/PLANS/plan-b/docs-pass.md`; `AGENT-INSTRUCTIONS/LOCAL/gates.md:20-45`.
     Check test count and necessity against REPO-RULES §3a (13 functions, no duplicated behavior,
     no test of a private helper or exact wording, no plan id in a test name), per-phase budgets
     (B1r, B2 and B3 each add at most three), and whether the requirements pages are accurate for
     this branch. Items 1 to 6 and 11 to 14 describe the merged product and are other plans' work,
     so record their absence as expected branch state, not as this plan's drift. Confirm the
     repository-map omission is recorded in `docs-pass.md` and the rendered-image gate command still
     applies. Report stale prose, a missing release record, test bloat, and a docs page that names a
     plan or a module.

3. Save the six verbatim returns, in area order, to
   `AGENT-INSTRUCTIONS/PLANS/plan-b/survey-returns-2026-11-24.md`, opened by a short header naming
   the branch, worktree, base and head commits and stating that the returns are the saved finding
   set for the note. Change no wording in a return and add no finding of your own.
4. Dispatch one fresh read-only `ops-scout` as the cross-checker. Give it the saved returns file and
   the changed-file list, not the area sessions. It checks every finding claim by claim against the
   tree and reports, for each numbered claim, `CONFIRMED`, `REJECTED` or `CORRECTED` with
   `path:line` evidence, then lists claims the area returns missed. It also checks coverage: every
   path in `git diff --name-only e180476..HEAD` belongs to at least one area, and every
   requirement item 7 to 10 is covered. It must not run tests or builds, must not edit, and must say
   when evidence is missing. Start it as a new session, never a resumed area scout.
5. Write `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/2026-11-24-closing-survey.md` from the saved
   returns and the cross-check. It holds, in this order: a header naming the branch, worktree, base
   and head commits, and the method (six parallel read-only `ops-scout` area returns, one
   independent `ops-scout` cross-check, the orchestrator wrote the note); a coverage table mapping
   each changed file to its area; one section per area with every confirmed finding as
   `path:line` plus its evidence and one sentence on the behavior; the validated clean facts worth
   keeping; the out-of-scope observations; the cross-check outcome with every rejected, corrected
   and added claim; the acceptance render; the user's decision once given; and a final
   **For the user** list. State plainly that nothing was fixed in this phase.
6. Run the acceptance render and record the exact commands and observations in the note:
   ```bash
   ~/.cargo/bin/cargo build --release
   ./target/release/toy-raytracer > /tmp/opencode/z1-accept.ppm
   ./target/release/toy-raytracer > /tmp/opencode/z1-accept-2.ppm
   cmp /tmp/opencode/z1-accept.ppm /tmp/opencode/z1-accept-2.ppm
   head -2 /tmp/opencode/z1-accept.ppm
   ```
   Expect exit status 0 for both runs, no output from `cmp`, and a header `P3` then `200 100`.
   Open `/tmp/opencode/z1-accept.ppm` with an image viewer available in the environment and record
   what opens it. Confirm against `docs/requirements.md` items 7 to 10: a sky gradient in the top
   band, the normal-shaded centre sphere, one flat light-green ground across the lower image
   because the ground normal is constant, and the light-blue triangle beside and above the sphere.
   Record the image size, the viewer used, and anything that does not match.
7. Run the documentation-only gate: reread both new plan files in full, run
   `git status --short -- src Cargo.toml docs README.md` and expect no output, and run
   `git diff --check` and expect no output. These prove no product file changed. Do not run the fast
   gate: no code changed.
8. Report the note and the image path to the user and ask for acceptance. State that no finding was
   fixed. If the user names a defect, add it to the note's **For the user** list and record the
   decision; do not fix it in this phase. A defect needing a repair goes to a remediation round the
   orchestrator decides after acceptance, not to Z1.

**Callers / wiring to update:**
- None. No product symbol, caller, test, `docs/` page or configuration changes. The two plan files
  are new; the status table and the `done/` compression belong to the advancer after acceptance.

**Config / schema / migrations:**
- None. No configuration key, migration, generated file or persisted product record. The saved
  returns and the note are plan records, not product data.

**Touched files:**
<!-- touched-files:start -->
- `AGENT-INSTRUCTIONS/PLANS/plan-b/survey-returns-2026-11-24.md` - created: verbatim returns of the six area scouts
- `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/2026-11-24-closing-survey.md` - created: the dated survey result, cross-check outcome and acceptance record
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Change no product code, test, `docs/` page, `README.md`, `Cargo.toml`, `AGENT-INSTRUCTIONS/LOCAL/`
  file or other plan's file. `git status --short -- src Cargo.toml docs README.md` stays empty.
- Fix nothing. Every finding is recorded with evidence and handed to the user. Do not open a
  remediation, do not add a test to cover a finding, and do not edit `docs-pass.md`.
- Do not narrow an area scout to a diff review. Each scout reads the whole area the plan changed and
  reports its own out-of-scope observations.
- Do not run the six area scouts sequentially and do not reuse an area session as the cross-checker.
  The cross-checker is a fresh `ops-scout`.
- Do not ask a scout to write, edit, run tests or builds, or judge the diff. `ops-scout` is
  read-only.
- Do not accept the survey or the image on the user's behalf. The plan's move-on gate is the user's
  acceptance.
- Do not mark this phase done, flip the status table, compress this file or archive the plan. Those
  are the advancer's work after the user's acceptance.
- Do not change the release merge's obligations: the box test and the `bounding_box` methods stay as
  written, and the borrowed nearest record and the material arguments arrive at the merge.
- Keep the acceptance render at the branch's fixed 200 by 100 size; there is no command line to
  vary it.

**Tests:**
- New: none. Changed: none. Deleted: none. This phase adds no test function; the suite stays at 13.
- Fast gate: not required, because no product file changes. Documentation-only gate instead: reread
  both new plan files, `git status --short -- src Cargo.toml docs README.md` empty,
  `git diff --check` clean.
- Conditional gates: rendered image, required as the acceptance subject and the survey's evidence.
  The command and expected observations are build plan step 6. Agent generation: not affected.
- Slow gate (>2 minutes): none. The release build and both renders finish in well under a minute.

**User test (acceptance):**
- **Executor:** user. The plan's move-on gate is the user's acceptance of the survey result and the
  rendered scene, which is a subjective judgment the agent cannot make.
1. The orchestrator presents `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/2026-11-24-closing-survey.md`
   and the render at `/tmp/opencode/z1-accept.ppm`, and states the findings and gaps.
2. The user reads the note and opens the render. The image shows the sky gradient, the
   normal-shaded centre sphere, the flat light-green ground and the light-blue triangle beside and
   above the sphere.
3. The user states acceptance, or names the findings or image points that are not acceptable.
4. Expected: the user accepts the survey and the image, and the note records the acceptance date
   and any accepted gaps. A defect named here is not fixed in Z1.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] The survey base (`e180476`), head and changed-file list are recorded, and every changed file
      maps to at least one of the six areas.
- [ ] Six area returns exist, one per area, saved verbatim in `survey-returns-2026-11-24.md`, and
      each brief was read-only with no test or build run.
- [ ] One fresh cross-checker covered every saved claim and the area coverage, and the note records
      every rejected, corrected and added claim.
- [ ] The note exists at `benchmarks/2026-11-24-closing-survey.md` with the header, coverage table,
      six area sections, cross-check outcome, acceptance render and **For the user** list, and every
      finding carries `path:line` evidence.
- [ ] No finding was fixed, no remediation opened, and no product, test, `docs/`, `Cargo.toml`,
      `AGENT-INSTRUCTIONS/LOCAL/` or other plan file changed:
      `git status --short -- src Cargo.toml docs README.md` is empty and `git diff --check` is
      clean.
- [ ] The acceptance render ran twice at exit status 0, `cmp` found no difference, the header is
      `P3` then `200 100`, and the image matches the expected scene; the commands and observations
      are in the note.
- [ ] Test budget respected: 0 functions added, 0 changed, 0 deleted; the suite stays at 13.
- [ ] The release merge's obligations are unchanged: the box test and `bounding_box` methods stay as
      written, and the borrowed nearest record and the material arguments arrive at the merge.
- [ ] The user's acceptance is recorded in the note with its date and any accepted gaps.
- [ ] On passing acceptance: the advancer compresses this file into
      `done/Z1-closing-survey-and-user-acceptance.md` (frozen contract only) and flips the
      `full-plan.md` status row to `done`.

**Exit:** the closing survey note exists with evidence for every finding, the cross-check is
recorded, the rendered scene is inspected, and the user has accepted the result. Nothing was fixed
in this phase.
