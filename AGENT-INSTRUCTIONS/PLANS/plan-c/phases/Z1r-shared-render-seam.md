## Phase Z1r - Shared render seam resolution

**Status:** active.
**Depends on:** C1 and C3, whose frozen contracts are in `done/C1-png-writer-and-command-line.md`
and `done/C3-progress-line.md`, and the shared design
`AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md`, which both plans read as the one copy.
**Scope:** record the shared render design in this plan, confirm plan-c's landed `render` seam, its
doc comment and progress placement against that design, and re-run plan-c's acceptance render. The
phase changes no product code, test, configuration, README or `docs/` file, adds no test function
and does not reopen the closing survey note or the user's acceptance of it.
**Gate level:** minimal. The phase writes one plan record and runs one read-only render, so no code
behaviour, schema or contract changes. The rendered-image conditional gate runs as the resolution
evidence for a render-related seam, not because any pixel or output behaviour changes.
**Simplicity statement:** the phase records the settlement and observes one render. It adds no
sample constant, no second render path, no second progress write, no receipt store and no test. It
does not restate the shared design as a copy, it cites it. It fixes nothing in the survey note and
opens no remediation. A defect found while checking the seam goes to the user-review list instead of
being patched here.

**Current state (verified):**
- `src/main.rs:62` holds `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>`.
  This signature is the seam plan-a's A3 consumes and the shared design keeps.
- `src/main.rs:60-61` holds the `render` doc comment, one sentence wrapped over two lines: "Every
  pixel's colour, row by row from the top, each pixel averaging `samples` rays, with the share of
  finished rows reported on standard error."
- `src/main.rs:66-80` holds the row loop. The `let done = height - row;` and
  `eprint!("\r{}%", done * 100 / height);` pair is the last statement of the row loop, after the
  column loop, and the single `eprintln!()` follows the loop. This placement and form are the
  shared design's progress target.
- `src/main.rs:23-25` holds `DEFAULT_WIDTH = 400`, `DEFAULT_HEIGHT = 200` and
  `DEFAULT_SAMPLES = 16`. `src/main.rs:111` reads `DEFAULT_SAMPLES` into the parsed `samples`, the
  `--samples` arm at `src/main.rs:126-133` rejects a count below 1, and `src/main.rs:185-186`
  `run` passes `settings.samples` into `render`. There is no `SAMPLES`, `WIDTH` or `HEIGHT`
  constant anywhere in `src/`, which matches the shared design's constant rule.
- `src/main.rs:5` holds `mod png;` and `src/main.rs:1` holds the module header
  `//! Render a fixed scene and write it to a file as a PNG or PPM image.`
- `src/main.rs:60-83` renders the unjittered `column / (width - 1)` and `row / (height - 1)`
  mapping, because jitter is plan-a's A3 work and arrives through the merge.
- `tests/cli.rs:43`, `:104` and `:129` hold the three binary tests; `src/main.rs` holds
  `the_center_pixel_shows_the_sphere_facing_the_camera` and `the_top_row_is_sky`.
- Branch `plan-c` is at `a0c585b`, where the fast gate and the C1 remediation evidence were taken.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md` - the whole file. The sections "Target
  shape of the shared code", "Which plan changes what", "Release merge" and "What each may rely
  on" are the settlement this phase records.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/done/C1-png-writer-and-command-line.md` and
  `AGENT-INSTRUCTIONS/PLANS/plan-c/done/C3-progress-line.md` - the frozen contracts the check reads
  the code against.
- `src/main.rs` - `render`, `parse_args`, the sample default and the progress write, at the lines
  named above.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/full-plan.md` - the status table and the binding design rules.

**Build plan:**
1. Read the shared design section by section and read the current `src/main.rs` names above. Do not
   copy the design into the plan. The plan record names the design path and the facts plan-c relies
   on.
2. Write the resolution record in this phase file. These are the facts plan-c's branch holds and
   that the release merge takes from it:
   - The `render(width, height, samples) -> Vec<Colour>` signature, byte for byte, including the
     parameter names `width`, `height` and `samples`.
   - The progress write: the `done` value `height - row`, the `eprint!("\r{}%", done * 100 /
     height)` call as the last statement of the row loop, and the single `eprintln!()` after it.
   - The `render` doc comment as one sentence. The merged sentence adds the word "jittered" before
     "rays" because the merged loop jitters, and that is the only clause A3 writes among plan-c's
     landed lines. On branch `plan-c` the sentence has no "jittered", because the branch has no
     jitter.
   - The module header, `mod png;`, `Format`, `Settings`, `USAGE`, `DEFAULT_WIDTH`,
     `DEFAULT_HEIGHT`, `DEFAULT_SAMPLES`, `DEFAULT_OUTPUT`, `parse_args`, `parse_dimension`,
     `resolve_format`, `run`, `main` and the progress write.
   - `to_byte` with `pub(crate)`, so `png::write_png` shares it.
   - `README.md` and `docs/requirements.md` as this plan's text, and `tests/cli.rs` as this plan's
     test file.
3. Record what the release merge takes from plan-a: `scene`, `ray_colour`, `Rng`, `SEED`,
   `MAX_BOUNCES`, the `to_byte` gamma body, the jitter mapping
   `(column as f64 + rng.next_f64()) / width as f64` and
   `(row as f64 + rng.next_f64()) / height as f64` drawn `u` first then `v` inside the sample
   loop before that sample's scattering draws, one `Rng::new(SEED)` before both loops, and the
   `samples` averaging. Record that the merged code has no `SAMPLES`, `WIDTH` or `HEIGHT` constant,
   that `DEFAULT_SAMPLES = 16` is the single default, that `main` to `run` passes
   `settings.samples`, and that `render` carries no sample-count validation because `parse_args`
   rejects a count below 1.
4. Record the merged test module. It holds `the_top_row_is_sky`,
   `two_renders_with_the_same_options_give_the_same_pixels` and
   `the_sample_count_changes_the_pixels`. A3 adapts `the_top_row_is_sky` to the literal call
   `render(200, 100, 16)` with its assertion unchanged, and its two tests use literal calls only.
   `the_center_pixel_shows_the_sphere_facing_the_camera` does not survive the merge, because
   materials replace normal colouring. `tests/cli.rs` needs no change, because PNG-versus-PPM
   pixel equality does not depend on the materials.
5. Check plan-c's branch against steps 2 to 4 by reading the named lines. Confirm the signature,
   the progress pair placement, the single sentence, the absence of the three constants and the
   presence of `DEFAULT_SAMPLES` and the `parse_args` rejection. Record the result. Make no edit:
   the branch holds the unjittered loop and the sentence without "jittered" until the merge.
6. Build the release binary and run the acceptance render once on the current head, then record the
   evidence:

   ```bash
   ~/.cargo/bin/cargo build --release
   target/release/toy-raytracer --width 400 --height 200 --samples 16 \
     --output /tmp/opencode/resolution.png \
     > /tmp/opencode/resolution.stdout 2> /tmp/opencode/resolution.stderr
   ```

   The build produces the binary this step runs and is not the fast gate, which stays unrun. The
   command was checked at `a0c585b` while this phase was detailed, so opening the existing
   `/tmp/opencode/resolution.png` and rereading the two captures is enough at the same revision.
   Expected result: exit status 0, the PNG is 400 by 200, 8-bit RGB, non-interlaced, and opens
   showing the sphere over the blue-to-white sky and the green ground plane. Standard output is
   0 bytes. Standard error is 783 bytes of percentages separated by `\r`, ending `100%` plus one
   newline. If the file or captures are absent, run the command once.
7. Documentation-only gate: reread this phase file and the `full-plan.md` edits, and run
   `git diff --check` at the workspace root `/home/opc/projects/toy-raytracer-runs/run004`. That
   directory is the target of the worktree's `AGENT-INSTRUCTIONS/` link and is the tree that tracks
   the plan files, so the command sees the tracked edits. A created file is untracked until the
   workflow plugin commits the round, so the reread is its check, and the command must print nothing
   for the tracked edits. Do not run the fast gate. No code changes, so the passing evidence at
   `a0c585b` stands and is not reproduced.
8. On passing evidence, compress this file into `done/Z1r-shared-render-seam.md` as the frozen
   resolution contract: the design path, the facts plan-c's branch holds, the release merge split,
   the merged test module, the no-constant rule and the acceptance-render evidence. Then set the
   status bookkeeping exactly:
   - In the status table, the Z1r row becomes `done` with `done/Z1r-shared-render-seam.md` as its
     phase file, and the Z1 row becomes `active` with `phases/Z1-closing-survey.md` as its phase
     file. The C1, C2 and C3 rows do not change.
   - In the scoped phases section of `full-plan.md`, the Z1r section's `**Status:**` line becomes
     `done` and the Z1 section's line becomes `active`.
   - In `phases/Z1-closing-survey.md`, the `**Status:**` line becomes `active`, matching its
     returned row and phase file.
   The closing survey note `benchmarks/closing-survey-2026-11-24.md` is unchanged, and the user's
   acceptance of it is given. This phase restates neither and records no new acceptance claim. With
   Z1 back to `active`, the advancer re-details Z1 fresh from `phases/Z1-closing-survey.md` before
   any further implementer, on the settled acceptance. The plan is not closed or archived here,
   because closure needs explicit user authorization.

**Callers / wiring to update:**
- None. The phase changes no code, test, configuration, README or `docs/` file.

**Config / schema / migrations:**
- No configuration key, schema or migration.
- Retained record and its consumer: the frozen resolution contract in `done/Z1r-shared-render-seam.md`
  is the record this phase produces. Its consumers are the release merge and the advancer that
  reads plan-c's landed contract, and it lives once in the plan folder. The acceptance PNG and the
  two stream captures under `/tmp/opencode/` are transient evidence and are removed after the done
  contract is written. No receipt, hash or manifest is added.

**Touched files:**
<!-- touched-files:start -->
- `AGENT-INSTRUCTIONS/PLANS/plan-c/phases/Z1r-shared-render-seam.md` - created: this resolution
  phase, and the frozen contract source when compressed.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/full-plan.md` - changed: the Z1r and Z1 status rows and the two
  scoped phase status lines.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/phases/Z1-closing-survey.md` - changed: the `**Status:**` line,
  `planned` while Z1 is held and `active` when the row returns to it.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/done/Z1r-shared-render-seam.md` - created on passing evidence:
  the frozen resolution contract.
- Adds: nothing. `/tmp/opencode/resolution.png`, `/tmp/opencode/resolution.stdout` and
  `/tmp/opencode/resolution.stderr` are transient evidence outside the repository.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Change no product code, test, `Cargo.toml`, `README.md` or `docs/` file, and add no dependency.
- Do not add a `SAMPLES`, `WIDTH` or `HEIGHT` constant, a second render path, a second sample
  default or a second progress write. The merged code has none of the three constants.
- Do not edit the shared design, `benchmarks/closing-survey-2026-11-24.md`, `docs-pass.md`,
  `AGENT-INSTRUCTIONS/LOCAL/gates.md`, `AGENT-INSTRUCTIONS/LOCAL/project.md`,
  `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md` or anything outside
  `AGENT-INSTRUCTIONS/PLANS/plan-c/`.
- Do not reopen the survey, add a finding, fix anything, or record a new acceptance claim. The
  user's acceptance of the survey note and the rendered result is given.
- No new test function, no receipt, hash or manifest, and no extra evidence file.
- Do not run the fast gate or rerun any gate for independent confidence. The documentation gate and
  the one acceptance render are the verification.
- Do not close or archive the plan in this phase.
- This phase record carries no history narration beyond what the settlement requires, and no plan id
  reaches a source comment or a permanent document.

**Tests:**
- New/changed/deleted: none. Test functions added 0, deleted 0, changed 0, so the REPO-RULES §3a
  budget is untouched. The existing tests and `tests/cli.rs` are read only.
- Fast gate: not run. No code or test changes, and the passing evidence at `a0c585b` stands.
- Documentation-only gate: reread this phase file plus `git diff --check`, at the workspace root
  step 7 names.
- Conditional gates: rendered image, required once in step 6 as the resolution evidence for the
  render seam. Agent generation: not affected.
- Slow gate (over two minutes): none. The 400 by 200, 16-sample render finishes in well under a
  minute.

**User test (acceptance):**
- **Executor:** not required.
- Not required - the user's acceptance of the closing survey note and its rendered result is given,
  and this phase records the settlement plus an agent-run render with no user-owned judgment. The
  acceptance render is observed directly in step 6, and no new acceptance claim is recorded.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a (minimal) are satisfied.
- [ ] The resolution record cites the shared design and states the `render` signature, the single
  doc-comment sentence (including the merged "jittered" clause), the progress placement, the
  no-constant rule, the release merge split and the merged test module.
- [ ] The check against branch `plan-c` confirms the signature, the doc sentence, the progress pair
  placement, the absence of `SAMPLES`, `WIDTH` and `HEIGHT`, `DEFAULT_SAMPLES = 16`, the `run`
  sample argument and the `parse_args` rejection below 1.
- [ ] The acceptance render ran at the current head or its existing evidence at that head was read:
  the PNG opens, standard output is 0 bytes, and standard error ends `100%` plus one newline.
- [ ] `git diff --check` is clean and the phase file was reread after the edit.
- [ ] No product code, test, configuration, README, `docs/` file or file outside
  `AGENT-INSTRUCTIONS/PLANS/plan-c/` changed.
- [ ] Test budget respected (REPO-RULES §3a); the record states 0 added, 0 deleted and 0 changed.
- [ ] Revision-bound evidence is recorded: the head is `a0c585b` and no code or test moved, so the
  fast-gate evidence at that revision stands, and the documentation gate plus the acceptance-render
  observation are this phase's evidence. No test receipt is produced for a phase with no code or
  test change.
- [ ] The closing survey note and the user's acceptance are untouched, and no new acceptance claim
  is recorded.
- [ ] On passing evidence: compress this file into `done/Z1r-shared-render-seam.md`, set the Z1r
  status table row to `done` with that path, set the Z1 row to `active` with
  `phases/Z1-closing-survey.md`, set the Z1r scoped status line to `done`, set the Z1 scoped status
  line to `active`, and set the `**Status:**` line in `phases/Z1-closing-survey.md` to `active`.
  C1, C2 and C3 stay as they are.
- [ ] The plan is not closed or archived.

**Exit:** the shared design is recorded as plan-c's resolution contract, plan-c's landed `render`
seam and frozen C1/C3 contracts are confirmed against it, the acceptance render is observed and
recorded, and Z1 returns to `active` awaiting the final close under explicit user authorization.
