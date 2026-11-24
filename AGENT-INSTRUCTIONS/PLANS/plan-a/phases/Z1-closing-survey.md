# Phase Z1 - Closing survey and user acceptance

**Status:** active.
**Depends on:** A1, A2 and A3, recorded in `done/A1-diffuse-and-metal-materials.md`,
`done/A2-glass.md` and `done/A3-samples-per-pixel.md`.
**Scope:** a read-only survey of everything this plan added or changed, the dated survey note, and
the user's acceptance of the survey findings and of the final rendered image. No product code,
test, `docs/` page, `README.md` or configuration changes.
**Gate level:** minimal. The phase writes plan records, runs a read-only survey and renders one
image. It changes no runtime behavior, so the documentation-only gate plus the rendered-image
conditional gate is the whole verification. The user acceptance is a product gate owned by the user.
**Simplicity statement:** the phase fixes nothing and opens no repair work. It reports every
finding instead. It adds no second review path, no per-area checker, no receipt, hash or manifest, no
extra record in product code and no test. It does not survey unrelated areas, rerun passing gates or
recheck values the producing code guarantees.

**Current state (verified):**
- Branch `plan-a` is at `46a5c02`, A3's tested head. A3's receipt reports `cargo build`, `cargo test`
  (12 passed), `cargo clippy -- -D warnings`, `cargo fmt --check` and `git diff --check` clean, and two
  byte-identical release PPMs from `cargo run --release`.
- `src/material.rs` - `Material::scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) ->
  Option<(Colour, Ray)>`. `Lambertian` holds `pub albedo: Colour`. `Metal` holds `pub albedo: Colour`
  and `pub fuzz: f64`. `Dielectric` holds `pub refraction_index: f64`. The private helpers are
  `reflect`, `refract`, `schlick_reflectance` and `random_unit_vector`. Six `#[test]` functions.
- `src/hittable.rs` - `HitRecord { pub t: f64, pub point: Vec3, pub normal: Vec3, pub material:
  Rc<dyn Material> }`, which derives nothing. `Hittable::hit` is the trait method.
  `HittableList { pub objects: Vec<Box<dyn Hittable>> }` has `add` and a nearest-hit loop that reads
  the limit from the nearest record found so far.
- `src/sphere.rs` - `Sphere { pub center: Vec3, pub radius: f64, pub material: Rc<dyn Material> }`
  with `Sphere::new(center, radius, material)`. Its hit builds the record with
  `Rc::clone(&self.material)`. Two `#[test]` functions.
- `src/main.rs` - `WIDTH = 200`, `HEIGHT = 100`, `SEED = 7` and `MAX_BOUNCES = 50`. The functions
  are `ray_colour`, `sky_colour`, `scene`, `render(width, height, samples)`, `to_byte` and
  `write_ppm`. `main` calls `render(WIDTH, HEIGHT, 16)` and writes P3 PPM to standard output. Three
  `#[test]` functions.
- `src/rng.rs` - `Rng::new(seed)` with `seed.max(1)`, plus `next_u64` and `next_f64`. One `#[test]`
  function.
- `docs/requirements.md` - 14 numbered items. Items 1 to 4 cover the materials, item 5 the sample
  count, jitter and gamma, item 6 the seeded generator, and item 14 the quality gates.
- `README.md` - four content lines: the description, `cargo run > image.ppm`, `cargo test` and the
  pointer to `docs/requirements.md`.
- The branch's suite is 12 test functions: 3 in `src/main.rs`, 6 in `src/material.rs`, 2 in
  `src/sphere.rs` and 1 in `src/rng.rs`.
- `benchmarks/` does not exist in the plan folder. The phase creates it.
- The plan's product change is confined to `src/material.rs`, `src/hittable.rs`, `src/sphere.rs`,
  `src/main.rs` and the seeded generator in `src/rng.rs` that the render loop uses. Areas 1 to 3
  cover those files and area 4 covers the tests and documents.

**Read first (do not invent):**
- `full-plan.md` - the Z1 row and its inline section. The row's exit state and move-on gate are the
  source of this phase.
- `done/A1-diffuse-and-metal-materials.md`, `done/A2-glass.md`, `done/A3-samples-per-pixel.md` -
  the frozen contracts the survey checks the tree against.
- `src/material.rs` - `Material`, `Lambertian`, `Metal`, `Dielectric` and the four private helpers,
  as listed in Current state.
- `src/hittable.rs` - `HitRecord`, `Hittable` and `HittableList`, as listed in Current state.
- `src/sphere.rs` - `Sphere` and its hit record construction, as listed in Current state.
- `src/main.rs` - `ray_colour`, `sky_colour`, `scene`, `render`, `to_byte`, `write_ppm` and `main`,
  as listed in Current state.
- `src/rng.rs` - `Rng`, `next_u64` and `next_f64`, as listed in Current state.
- `docs/requirements.md` - items 1 to 6 and item 14 are the survey's requirements baseline. Items 7
  to 13 describe the geometry and output plans and are outside this branch's product change.
- `README.md` - the run and test lines and the pointer to the requirements.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the documentation-only gate and the rendered-image command
  for a branch without a command line.

**Build plan:**

The orchestrator, not an implementer, runs this phase. It performs every step below itself, except
that the four area surveys and the one cross-check are read-only `ops-scout` work.

Method (binding, overrides any generic wording):

- Dispatch one independent read-only `ops-scout` per area, the areas in parallel. Then dispatch ONE
  independent read-only `ops-scout` that cross-checks ALL findings together, claim by claim with
  evidence. There is no per-area checker, no implementer, and no other child writes the note.
- Each scout reads whatever context outside its area it needs. If it notices a problem outside its
  area it does not investigate further, but must report it as an out-of-scope observation with the
  evidence it already has. Dispatch prompts must not instruct a scout to report only on the diff.
- The orchestrator writes the dated note `benchmarks/closing-survey-<date>.md` from the saved scout
  returns: confirmed findings, out-of-scope observations, unnecessary complexity, duplication, dead
  code, redundant checks or records, test bloat against REPO-RULES §3a, stale docs, and drift from
  the user's decisions.
- Findings go to the user and are not fixed in this phase or in this document. Clear defects and
  drift from the user's decisions become separate remediation units. A change that would need a
  whole phase of its own, and every user-owned decision, go to the note's user-review list.
- The final gate is the user's acceptance of the survey findings and of the rendered image. The plan
  is not closed or archived here. The release closes it after the user accepts.

Areas (one scout each, exactly these four):

1. `src/material.rs` and the `Material` trait: the matte, metal and glass behaviour against
   requirements 1 to 4, unnecessary complexity, duplication and dead code.
2. `src/hittable.rs` and `src/sphere.rs`: the material seam on the hit record and how objects get
   their material.
3. `src/main.rs` and `src/rng.rs`: the bounce loop, gamma, the sample loop and its jitter order, the
   scene, and that every random value comes from the seeded generator.
4. Tests and docs: `docs/requirements.md` and `README.md` accurate, test bloat against REPO-RULES
   §3a, drift from the user's decisions, and permanent comments and docs never naming a plan.

Steps:

1. Confirm the worktree is on branch `plan-a` at head `46a5c02` and the suite is 12 test functions. If
   the head, the file list or the suite differs, stop and report the drift instead of surveying.
2. Start the four independent read-only `ops-scout` sessions, one per area above, with all four
   started before any return is read. Each prompt names its one area, says the scout may read any
   context outside it, asks for an out-of-scope observation rather than further investigation, and
   does not describe the task as a diff review. Each return gives confirmed findings with
   `path:line` evidence, the facts it checked and found clean, and any out-of-scope observation.
3. Save each returned report verbatim in `benchmarks/closing-survey-returns-<date>.md`, one section
   per area, and do not edit the text. `<date>` is the run date as `YYYY-MM-DD`.
4. Start ONE further independent read-only `ops-scout` with all saved returns, the four areas and the
   branch revision. It checks every finding claim by claim against the tree, marks each claim
   confirmed, rejected or corrected with its evidence, checks that every changed file and
   requirements 1 to 6 sit in at least one area, and adds any claim the areas missed. No per-area
   checker runs.
5. Write `benchmarks/closing-survey-<date>.md` from the saved returns and the cross-check, in this
   order:
   - the date, branch, surveyed base `e180476` to head `46a5c02`, the method sentence and the area
     list;
   - one section per area with its confirmed findings, each with `path:line` evidence and one line
     on why it is a finding, and the facts validated clean;
   - a Categories section with one entry each for unnecessary complexity, duplication, dead code,
     redundant checks or records, test bloat against REPO-RULES §3a, stale docs and drift from the
     user's decisions, naming the finding ids or `none`;
   - out-of-scope observations;
   - the cross-check outcome, including rejected, corrected and added claims and their resolution;
   - the acceptance render result;
   - a remediation-candidates list of clear defects and drift from the user's decisions;
   - a user-review list for each change that needs a whole phase of its own and every user-owned
     decision;
   - a user decision section, empty until the user accepts.
   Write no finding that lacks evidence, no history rundown and no plan-close-out claim.
6. Render the final scene once at the default settings and show the image to the user:
   `~/.cargo/bin/cargo run --release > /tmp/opencode/z1-accept.ppm`. Convert the PPM to PNG with
   `/tmp/opencode/z1_ppm_to_png.py`, a scratch standard-library Python script outside the repository
   that reads the P3 file and writes an RGB PNG. Open the PNG with the image reader and check it
   against requirements 1 to 6 before showing it. Record the command, exit status, byte size, image
   dimensions and what the image shows in the note's acceptance section. Do not run the render twice
   and do not redo A3's byte comparison: A3's receipt holds two byte-identical release PPMs at
   `46a5c02`.
7. Report the note and the rendered image to the user, present each finding, and ask for acceptance
   of the survey findings and the image. Fix nothing. On acceptance, record the date and the accepted
   findings in the note's user decision section, then compress this phase into
   `done/Z1-closing-survey-and-user-acceptance.md` and flip the `full-plan.md` row to `done`. Do not
   archive the plan and do not open remediation work; the release owns closure.

**Callers / wiring to update:**
- None in product code. The phase adds plan records only.

**Config / schema / migrations:**
- None.
- Retained records and their named consumer/owner: the dated note and the saved returns are plan
  records the user reads during acceptance. They stay in the plan folder. No product field, log,
  receipt or telemetry is added, and REPO-RULES §0b's product-consumer test does not apply to plan
  records.

**Touched files:**
<!-- touched-files:start -->
- `phases/Z1-closing-survey.md` - created: this phase document.
- `benchmarks/closing-survey-<date>.md` - created: the dated survey note.
- `benchmarks/closing-survey-returns-<date>.md` - created: the four area returns and the cross-check return, saved verbatim.
- `full-plan.md` - changed: the Z1 `Phase file` cell points at this document, and on acceptance the Z1 row flips to `done`.
- `done/Z1-closing-survey-and-user-acceptance.md` - created on acceptance: the frozen contract.
- Adds: nothing outside the plan folder.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- The orchestrator runs this phase. Do not hand it to a `phase-implementer`.
- Only the orchestrator writes the note. The four area scouts and the cross-check scout are read-only
  and write nothing.
- Do not fix, edit or delete product code, tests, `docs/` pages, `README.md`, `Cargo.toml` or
  configuration in this phase. Report findings instead.
- Do not narrow the survey to a diff-only review. Each area covers its whole files as the branch
  holds them.
- Do not add a per-area checker, a second cross-check, a receipt, a hash or a manifest.
- Do not add a test. The budget is zero new, zero changed, zero deleted.
- Do not record a finding without `path:line` evidence, and do not pre-judge a finding in this phase
  document.
- Do not close or archive the plan. The release closes it after the user accepts.
- Do not name a plan, phase or survey id in a permanent file, a source comment or a `docs/` page.

**Tests:**
- New/changed/deleted: none. Test budget: 0 added, 0 changed, 0 deleted.
- Fast gate: documentation-only gate. Re-read every created or changed plan file and run
  `git diff --check`. The 12 product tests passed at `46a5c02` in A3's receipt and are reused, not
  rerun.
- Conditional gates: rendered image, required because the phase owns the plan's final acceptance
  render. One render at the default settings, sole execution owner the orchestrator.
- Slow gate (over two minutes): none.

**User test (acceptance):**
- **Executor:** user. Acceptance is a user-owned decision and the image's appearance is a subjective
  judgment no automated check replaces.
- 1. Open `benchmarks/closing-survey-<date>.md` and read every confirmed finding, the cross-check
  outcome, the remediation candidates and the user-review list.
- 2. Open the rendered image from `/tmp/opencode/` at 200 by 100 pixels. Confirm requirements 1 to 6:
  the red-brown matte centre sphere and the light matte ground, the grey metal sphere with its
  blurred reflection, the glass sphere showing a refracted image of the scene, and edges smoothed by
  the 16 jittered samples.
- 3. Accept the survey findings and the image, or name each finding and each image defect to record.
  The plan is not closed here.

**Completion checklist (gate):**
- [ ] Gate level minimal requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] The note exists at `benchmarks/closing-survey-<date>.md` with the method, revision, one section
  per area, every finding backed by `path:line` evidence, the Categories section, out-of-scope
  observations, the cross-check outcome, the acceptance render, the remediation candidates, the
  user-review list and the user decision.
- [ ] The four area returns and the cross-check return are saved verbatim in
  `benchmarks/closing-survey-returns-<date>.md`.
- [ ] The four areas ran in parallel, one cross-check covered all findings claim by claim, and no
  per-area checker, implementer or other child wrote the note.
- [ ] Every changed file and requirements 1 to 6 sit in at least one area, as the cross-check
  confirms.
- [ ] Every finding is tagged with its category and each of unnecessary complexity, duplication,
  dead code, redundant checks or records, test bloat against REPO-RULES §3a, stale docs and drift
  from the user's decisions is either named or recorded as `none`.
- [ ] The final scene was rendered once at the default settings, the image was opened and inspected
  against requirements 1 to 6, and it was shown to the user.
- [ ] No product code, test, `docs/` page, `README.md`, `Cargo.toml` or configuration changed.
- [ ] The user accepted the survey findings and the rendered image, and the note's user decision
  section records the date and the accepted findings.
- [ ] The plan is not archived in this phase. The release owns closure and archiving.
- [ ] Changed plan files were re-read and `git diff --check` is clean.
- [ ] On passing evidence: compress this file into
  `done/Z1-closing-survey-and-user-acceptance.md` (frozen contract only) and flip the
  `full-plan.md` status row to `done`.

**Exit:** the survey note and its saved returns exist, the final image was rendered and shown, and
the user accepted the survey findings and the image.
**Move-on gate:** the user accepts the survey findings and the rendered image.
