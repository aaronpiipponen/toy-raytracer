## Phase A3r - Samples per pixel resolution

**Status:** active.
**Depends on:** A1 (`done/A1-diffuse-and-metal-materials.md`), A2 (`done/A2-glass.md`) and the shared
design `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md`, which both plans read as the one
copy.
**Scope:** documentation only on branch `plan-a`. The phase amends
`phases/A3-samples-per-pixel.md` in place to the shared sampling design and records the release merge
contract. It changes no product code, test, `Cargo.toml`, `README.md` or `docs/` file, adds no test
function and does not implement A3.
**Gate level:** minimal. Only plan records change, so no code behaviour, schema or contract moves.
**Simplicity statement:** the phase records the settlement and points at the shared design. It adds
no sample constant, no second render path, no sample-count guard, no progress write, no receipt store
and no test. It does not copy the shared design into this plan, it cites it. A product change found
while reading goes to the user through the orchestrator instead of being made here.

**Current state (verified):**
- Branch `plan-a` is at `a1d640a`, where A1 and A2 landed.
- `src/main.rs:22-25` holds `const WIDTH: usize = 200`, `const HEIGHT: usize = 100`,
  `const SEED: u64 = 7` and `const MAX_BOUNCES: usize = 50`. There is no `SAMPLES` constant.
- `src/main.rs:79` holds the one-sentence doc comment `/// Every pixel's colour, row by row from the
  top.` and `src/main.rs:80` holds `fn render(width: usize, height: usize) -> Vec<Colour>`.
- `src/main.rs:81-93` creates one `let mut rng = Rng::new(SEED)`, loops
  `for row in (0..height).rev()` then `for column in 0..width`, maps
  `u = column as f64 / (width - 1) as f64` and `v = row as f64 / (height - 1) as f64`, and pushes
  one `ray_colour` call per pixel.
- `src/main.rs:118-121` holds `main`, which calls `render(WIDTH, HEIGHT)` and writes PPM to standard
  output. There is no command line, no `SAMPLES` and no progress write on this branch.
- `src/main.rs:124-132` holds the test module and `the_top_row_is_sky`, which calls
  `render(WIDTH, HEIGHT)` and asserts the first pixel is bluer than it is red.
- `phases/A3-samples-per-pixel.md` is the detailed A3 phase. Its `**Status:**` line is `active` and
  becomes `planned`. Its jitter mapping and its two new test
  calls already match the shared design. Its assertion, its doc sentence, its `main` and test call
  sites and its constant description need the amendments below.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md:19-20` holds the A3r row `active` with
  `phases/A3r-samples-resolution.md` and the A3 row `planned` with
  `phases/A3-samples-per-pixel.md`. `full-plan.md:65` holds the scoped A3 `**Status:** planned.`
  line.
- The shared design's "Release merge" section fixes the merge split. Plan-c's branch holds the
  command line, the PNG writer and the progress write.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md` - the whole file, in particular "Target
  shape of the shared code", "Which plan changes what", "Release merge" and "What each may rely on".
- `phases/A3-samples-per-pixel.md` - the file this phase amends. Its build plan, callers, tests,
  rules and completion checklist are the places the amendments below name.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1-diffuse-and-metal-materials.md` and
  `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A2-glass.md` - the frozen render, generator and material
  contracts the merged `render` keeps.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - the status table and the scoped A3 entry.
- `src/main.rs` - the constants, `render`, `to_byte`, `main` and the test module at the lines named
  above.

**Build plan:**
1. Amend `phases/A3-samples-per-pixel.md` in place. Apply exactly these changes and no others:
   - `**Status:**` line: set to `planned.` This is the status bookkeeping for the row returned to
     A3 after this phase.
   - **Simplicity statement** (the `The sample count has one supported range` sentences): replace
     "at least one sample, guarded by a single assertion" with "at least one sample, validated only
     by the command line in the merged product". Add that `render` carries no sample-count guard.
   - **Build plan step 1**: keep the planned `const SAMPLES: usize = 16;` below
     `const HEIGHT`. State that on
     branch `plan-a` the constant is the branch's local default and its only use is the `main` call
     `render(WIDTH, HEIGHT, SAMPLES)`, that the merged code has no `SAMPLES`, and that the release
     takes plan-c's `DEFAULT_SAMPLES = 16`.
   - **Build plan step 3**: delete the `assert!(samples > 0, ...)` instruction and state plainly
     that `render` adds no sample-count validation. Keep the rest of the step: one
     `Rng::new(SEED)` before both loops, `u = (column as f64 + rng.next_f64()) / width as f64` first,
     `v = (row as f64 + rng.next_f64()) / height as f64` second, then that sample's scattering
     draws, then `pixel / samples as f64`.
   - **Build plan step 5**: replace the planned doc comment with exactly these two source lines,
     which are the shared design's one sentence:
     ```rust
     /// Every pixel's colour, row by row from the top, each pixel averaging `samples` jittered rays, with
     /// the share of finished rows reported on standard error.
     ```
     This sentence describes the merged product. Branch `plan-a` has no progress line yet, and the
     sentence is not changed again at the merge.
   - **Build plan step 6** and the **Changed** test bullet: the `the_top_row_is_sky` call is the
     literal `let pixels = render(200, 100, 16);`, the same line plan-c landed, and its assertion is
     unchanged. No test uses `WIDTH`, `HEIGHT` or `SAMPLES`.
   - **Build plan step 7** and the two **New** test bullets: keep the literal calls
     `render(20, 10, 2)` twice for `two_renders_with_the_same_options_give_the_same_pixels`, and
     `render(20, 10, 1)` versus `render(20, 10, 4)` for
     `the_sample_count_changes_the_pixels`.
   - **Callers / wiring**: the `main` bullet passes `SAMPLES` on branch `plan-a`. The
     `the_top_row_is_sky` bullet passes the literal `16` in `render(200, 100, 16)`.
   - **Rules / MUST NOT**: replace the "beyond the one assertion" clause with a rule that `render`
     does not validate the count and the merged command line is the single validation path. Add that
     A3 does not add a progress write, does not change the module header or `mod` list, and does not
     touch the writers, `to_byte`, the scene or the materials. Keep the existing rules on the one
     generator, no dependency, single-threaded rendering and no command-line parsing.
   - **Completion checklist**: the item that says `main` passes `SAMPLES` equal to 16 stays. Add
     that `render` carries no sample-count guard and that the `render` doc comment is the agreed
     sentence.
   The jitter mapping, the sample averaging, the `main` call in build plan step 4, the touched-files
   block, the gate commands and the exit criteria already match the shared design and stay as they
   are. Update the touched-files block only if an amendment changes what it summarises.
2. Record the release merge contract in this phase file, citing the shared design's "Release merge"
   section:
   - From plan-a's branch: `scene`, `ray_colour`, `Rng` usage, `SEED`, `MAX_BOUNCES`, the `to_byte`
     gamma body, the one `Rng::new(SEED)` before both loops, the jitter mapping and the `samples`
     averaging, and the three test functions below.
   - From plan-c's branch: the module header, `mod png;`, `Format`, `Settings`, `USAGE`,
     `DEFAULT_WIDTH`, `DEFAULT_HEIGHT`, `DEFAULT_SAMPLES`, `DEFAULT_OUTPUT`, `parse_args`,
     `parse_dimension`, `resolve_format`, `run`, `main`, the progress write, the `pub(crate)`
     visibility on `to_byte`, `README.md`, `docs/requirements.md` and `tests/cli.rs`.
   - The merged `render` is the shared design's function: plan-a's scene, camera, generator,
     `ray_colour` and jitter with plan-c's progress write and doc sentence. The merged code has no
     `SAMPLES`, `WIDTH` or `HEIGHT` constant. `DEFAULT_SAMPLES = 16` is the single default,
     `run` passes `settings.samples`, and `parse_args` rejects a count below 1.
   - The merged test module holds `the_top_row_is_sky`,
     `two_renders_with_the_same_options_give_the_same_pixels` and
     `the_sample_count_changes_the_pixels`. Plan-c's
     `the_center_pixel_shows_the_sphere_facing_the_camera` does not survive, because A1 replaced
     normal colouring with materials.
   - Neither branch adds a second `render` path, a second sample default, a second generator or a
     second progress write.
3. Confirm the pre-existing status bookkeeping in
   `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md`. No status edit is needed before step 5:
   - `full-plan.md:19` holds the A3r row `active` with `phases/A3r-samples-resolution.md`.
   - `full-plan.md:20` holds the A3 row `planned` with `phases/A3-samples-per-pixel.md`. A3's other
     cells do not change.
   - `full-plan.md:65` holds the scoped A3 `**Status:** planned.` line.
   - The A1, A2 and Z1 rows and scoped sections are unchanged. If any of these values differ when
     the phase runs, stop and report the drift instead of inserting a second row.
4. Documentation-only gate: reread this phase file, the amended `phases/A3-samples-per-pixel.md` and
   the `full-plan.md` edits, then run `git diff --check` at the workspace root
   `/home/opc/projects/toy-raytracer-runs/run004`, which tracks the plan files through the
   worktree's `AGENT-INSTRUCTIONS/` link. A created file is untracked until the workflow plugin
   commits the round, so the reread is its check. Do not run the fast gate, a build, a test or a
   render. No product code moves, so the A1 and A2 fast-gate evidence at their landed revisions
   (A2 head `a1d640a`) stands unchanged.
5. On passing evidence, compress this file into `done/A3r-samples-resolution.md` as the frozen
   resolution contract: the shared design path, the A3 amendments, the merge split and the merged
   test module. Then set the A3r status row to `done` with `done/A3r-samples-resolution.md`, leave
   A3 `planned` with `phases/A3-samples-per-pixel.md`, and keep the other rows unchanged. Plan-a
   implements A3 after the pair returns `RESOLVED`.

**Callers / wiring to update:**
- None in product code. The phase changes no source file.
- The amended A3 phase remains the only place plan-a's sampling work is specified.

**Config / schema / migrations:**
- None. No configuration key, schema or migration.
- Retained record and its consumer: the frozen resolution contract in
  `done/A3r-samples-resolution.md` records the A3 amendments and the release merge split. Its
  consumers are the A3 implementer and the release merge. It lives once in the plan folder. No
  receipt, hash or manifest is added.

**Touched files:**
<!-- touched-files:start -->
- `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A3r-samples-resolution.md` - created: this resolution
  phase, and the frozen contract source when compressed.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A3-samples-per-pixel.md` - changed: amended in place to
  the shared sampling design.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - changed: the A3r and A3 status rows and the
  scoped A3 status line.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A3r-samples-resolution.md` - created on passing evidence:
  the frozen resolution contract.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Change no product code, test, `Cargo.toml`, `README.md` or `docs/` file, and add no dependency.
- Do not implement any part of A3 in this phase. Do not add `SAMPLES` to `src/main.rs`, change
  `render`, or add or edit a test.
- Do not add a `SAMPLES`, `WIDTH` or `HEIGHT` constant to the merged product, a second render path,
  a second sample default, a sample-count guard in `render`, a second generator or a progress write
  on branch `plan-a`.
- Do not edit the shared design, plan-c's files, `done/A1-diffuse-and-metal-materials.md`,
  `done/A2-glass.md`, `AGENT-INSTRUCTIONS/LOCAL/gates.md`, `AGENT-INSTRUCTIONS/LOCAL/project.md` or
  `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md`.
- Do not restate the shared design as a copy. Cite its path and record only the facts the A3
  implementer and the release merge need.
- Do not add a test function, a receipt, a hash or an extra evidence file, and do not run the fast
  gate or a render.
- Do not close or archive the plan, and do not promote A3 to `active` in this phase.
- No plan id or history narration reaches a source comment or a permanent document.

**Tests:**
- New/changed/deleted: none. Test functions added 0, deleted 0, changed 0, so the REPO-RULES §3a
  budget is untouched. The A3 test plan is amended on paper only.
- Fast gate: not run. No code or test changes, so the A1 and A2 evidence at their landed revisions
  (A2 head `a1d640a`) stands unchanged.
- Documentation-only gate: reread the changed plan files plus `git diff --check` at the workspace
  root step 4 names.
- Conditional gates: none. Agent generation is not affected, and no image is rendered because no
  render code moves.
- Slow gate (over two minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - the phase records a design both orchestrators already settled, and automated
  documentation checks prove it. The sampling behaviour is accepted when A3's rendered-image gate
  runs, and the plan's final image acceptance is Z1.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a (minimal) are satisfied.
- [ ] `phases/A3-samples-per-pixel.md` is amended exactly as step 1 lists: branch-local `SAMPLES`
  used only by `main`, jitter order `u` then `v` then scattering, no `assert!`, the agreed doc
  sentence, the literal `render(200, 100, 16)` call and literal-only new test calls, and no touch
  to the module header, `mod` list, writers, `to_byte`, scene, materials or progress.
- [ ] The release merge contract is recorded in this phase file and matches the shared design's
  "Release merge" section, including the merged test module and the dropped plan-c test.
- [ ] `full-plan.md` has the A3r `active` row, the A3 `planned` row with
  `phases/A3-samples-per-pixel.md`, and the scoped A3 status line `planned`. A1, A2 and Z1 are
  unchanged.
- [ ] No product code, test, configuration, `README`, `docs/` file or file outside the plan folder
  changed.
- [ ] Test budget respected (REPO-RULES §3a); the record states 0 added, 0 deleted and 0 changed.
- [ ] The A1 and A2 evidence at their landed revisions (A2 head `a1d640a`) is cited as unchanged,
  and no gate or render was rerun.
- [ ] `git diff --check` is clean and the changed plan files were reread after the edit.
- [ ] On passing evidence: compress this file into `done/A3r-samples-resolution.md`, set the A3r row
  to `done` with that path, and leave A3 `planned` with `phases/A3-samples-per-pixel.md`.
- [ ] The plan is not closed or archived, and A3 is not promoted before the pair returns
  `RESOLVED`.

**Exit:** the shared sampling design is recorded in plan-a, A3 is amended in place to that design,
the release merge split is frozen in the resolution contract, and A3 waits as `planned` for
implementation after the pair resolves.
