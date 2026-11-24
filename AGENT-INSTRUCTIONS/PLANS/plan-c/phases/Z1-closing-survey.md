## Phase Z1 - Closing survey and user acceptance

**Status:** active.
**Depends on:** C1 and C3, whose frozen contracts are in `done/C1-png-writer-and-command-line.md`
and `done/C3-progress-line.md`.
**Scope:** the binding closing survey of every area this plan added or changed, a dated note in the
plan's `benchmarks/`, the acceptance render through the real command line, and the user's
acceptance. The survey is read-only. Findings go to the user and are not fixed here. The plan
orchestrator runs the survey with read-only `ops-scout` sessions and writes the note. No
`phase-implementer` session runs for this phase, and no code, test, configuration, README or
`docs/` change is made in it.
**Gate level:** minimal. The phase writes one plan-folder note and runs one read-only render, so
`minimal` fits. The rendered-image conditional gate runs as survey evidence and for the user's
acceptance.
**Simplicity statement:** the survey runs once per area, then one cross-check, then one note. It
does not add a report generator, a checklist tool, a findings store or a test. It does not narrow
its scope to a diff-only review, and it fixes nothing. A defect confirmed in the survey goes to a
remediation round or to the user-review list, and a change that needs a whole phase of its own goes
to the user-review list. The plan is neither closed nor archived here: the release closes it after
the user accepts.

**Current state (verified):**
- The plan's changes sit in the branch range `e1293ef..0ad0f2b`: `src/png.rs` is a new module
  (`write_png`, `write_chunk`, `crc32`, `adler32`, `SIGNATURE`, `MAX_STORED_BLOCK`), `src/main.rs`
  gains `Format`, `Settings`, `parse_args`, `parse_dimension`, `resolve_format`, `run` and the
  progress output in `render`, `tests/cli.rs` is a new binary test file with three tests, and
  `README.md` shows the real run command.
- `docs/requirements.md` items 11 to 13 state the output and command-line behaviour. That file and
  `AGENT-INSTRUCTIONS/LOCAL/gates.md` are unchanged by this plan.
- `docs-pass.md` records one deferred item: deleting the rendered-image fallback sentence in
  `AGENT-INSTRUCTIONS/LOCAL/gates.md` during the release.
- No `benchmarks/` directory exists under `AGENT-INSTRUCTIONS/PLANS/plan-c/` yet.
- This workspace holds no earlier plan survey note to consult. The survey method is the binding one
  in `AGENT-INSTRUCTIONS/PLANNING/PLAN-AUTHORING-SCOPING.md` §3b: one read-only `ops-scout` per
  area in parallel, then one read-only `ops-scout` cross-checking every finding, then the
  orchestrator writes the note from the saved returns.
- `AGENT-INSTRUCTIONS/SUBAGENTS/README.md` describes `ops-scout` as the general-purpose read-only
  scout with a full read-only shell.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANNING/PLAN-AUTHORING-SCOPING.md` §3b - the survey method, the scout scope
  rules, the note's finding classes and the closing rule. Binding.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/full-plan.md` - the status table, the binding design rules 1 to
  8 and the carry-overs list.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/done/C1-png-writer-and-command-line.md` and
  `AGENT-INSTRUCTIONS/PLANS/plan-c/done/C3-progress-line.md` - the frozen contracts the survey
  checks the code against.
- `src/png.rs`, `src/main.rs`, `tests/cli.rs`, `README.md` - the changed tree.
- `docs/requirements.md` items 11 to 13 and `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the required
  behaviour and the gate commands. Both are read only.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md` - the deferred documentation item.

**Build plan:**
1. Create the directory `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/`. The note is
   `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/closing-survey-<date>.md`, where the date is the day
   the survey runs, for example `closing-survey-2026-11-24.md`.
2. The orchestrator dispatches one independent read-only `ops-scout` per area below, all launched
   together so they run in parallel. Each dispatch prompt names its area, tells the scout it may
   read whatever context outside the area it needs to judge it, tells it to report an out-of-scope
   problem as an out-of-scope observation with the evidence it already has without investigating
   further, and does not restrict it to the diff:
   - **Area 1, the PNG writer.** `src/png.rs` against requirement 11: one `IDAT` chunk of stored
     deflate blocks of at most 65535 bytes, one filter byte of 0 per scanline, a CRC-32 per chunk,
     the Adler-32 trailer and the zlib header, the `IHDR` fields, no dependency and no real
     compression. Look for unnecessary complexity, duplication with `tests/cli.rs` helpers, dead
     code and redundant checks.
   - **Area 2, the command line and settings.** `src/main.rs` `Format`, `Settings`, `USAGE`,
     `DEFAULT_WIDTH`, `DEFAULT_HEIGHT`, `DEFAULT_SAMPLES`, `DEFAULT_OUTPUT`, `parse_args`,
     `parse_dimension`, `resolve_format`, `run` and `main` against requirement 12 and design rule 3:
     the five options, the defaults, the extension rule, an explicit `--format` winning, one
     resolved settings value, the usage line plus non-zero exit on bad input, and one error line
     plus a non-zero exit on a writer or path failure. Look for duplicate validation, unnecessary
     branches and invented fallbacks.
   - **Area 3, the render loop, the progress line and the writers.** `src/main.rs` `render`,
     `ray_colour`, `scene`, `to_byte`, `write_ppm` and the progress write against requirement 13 and
     design rules 4 and 5: one render path, the row share on standard error on one overwriting line
     ended by one newline, empty standard output, unchanged image bytes, unjittered samples that are
     the materials plan's seam, and no second output path. Look for duplication, dead code,
     defensive re-checks and anything that breaks reproducibility.
   - **Area 4, tests and documentation.** `tests/cli.rs`, the `src/main.rs` unit tests, `README.md`,
     `docs/requirements.md` items 11 to 13 and `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md`: test
     necessity and focus against REPO-RULES §3a, no fake product behaviour, current and accurate
     README and requirements text, and nothing stale. The `AGENT-INSTRUCTIONS/LOCAL/gates.md`
     fallback sentence is deferred through `docs-pass.md` and is not a finding.
3. The orchestrator dispatches ONE independent read-only `ops-scout` that cross-checks every
   finding from every area, claim by claim, against the code and documents with evidence. It returns
   each claim as confirmed or rejected with `file:line` evidence. There is no per-area checker.
4. The orchestrator writes the dated note from the saved returns. The note states the surveyed
   areas and the revision surveyed, then lists:
   - confirmed findings with `file:line` evidence and their class: unnecessary complexity,
     duplication, dead code, a redundant check or record, test bloat, a stale document, or drift
     from the plan's design rules or the user's decisions;
   - claims the cross-check rejected, so a later reader does not reopen them;
   - out-of-scope observations with the evidence the area scout had;
   - a user-review list for anything that needs a phase of its own or a user-owned decision;
   - the rendered-image evidence from step 5.
   The note holds no fix and no plan id.
5. Rendered-image gate on the surveyed revision, run by the orchestrator. Build and run the real
   command line, capture the streams separately and inspect the image:
   ```bash
   ~/.cargo/bin/cargo build --release
   target/release/toy-raytracer --width 400 --height 200 --samples 16 --output /tmp/opencode/acceptance.png > /tmp/opencode/acceptance.stdout 2> /tmp/opencode/acceptance.stderr
   ```
   Confirm the standard output capture is empty, the standard error capture holds the row share
   with `\r` separators ending in `100%` plus one newline, and open
   `/tmp/opencode/acceptance.png` with the image reader to confirm the sphere and the sky. Compare
   what runs with requirements 11 to 13. Remove the scratch files under `/tmp/opencode/` after the
   inspection.
6. Report the note and the render to the user with the note's findings, the rejected claims and the
   user-review list. Fix nothing in this phase. Clear defects and drift from the design rules go to
   a remediation round after the survey, and the plan is neither closed nor archived here.

**Callers / wiring to update:**
- None. The phase changes no code, test, configuration or published document.

**Config / schema / migrations:**
- No configuration key, schema or migration.
- Retained record and its consumer: the dated survey note in `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/`
  is the record this phase exists to produce. Its consumers are the user's acceptance decision, the
  release's review of `docs-pass.md`, and any later remediation round. It is the only copy and lives
  in the plan folder. The acceptance PNG and the stream captures are transient evidence under
  `/tmp/opencode/` and are removed after inspection.

**Touched files:**
<!-- touched-files:start -->
- `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/closing-survey-2026-11-24.md` - created: the dated
  closing survey note, named with the day the survey runs.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/phases/Z1-closing-survey.md` - changed: this phase file, only
  when compressing it after acceptance.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/done/Z1-closing-survey.md` - created on acceptance: the frozen
  survey outcome and the user's acceptance, with the note path.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/full-plan.md` - changed: the Z1 status row and its phase file
  path.
- Adds: the `benchmarks/` directory.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Read-only survey. Do not change product code, tests, `Cargo.toml`, `README.md`,
  `docs/requirements.md`, `AGENT-INSTRUCTIONS/LOCAL/gates.md` or any other permanent file in this
  phase.
- Do not narrow a scout to a diff-only review. Each area scout covers its whole area and reads
  whatever outside context it needs.
- Do not merge areas into one scout, add a per-area checker, or skip the single cross-check of all
  findings. Do not write the note from memory or from an unsaved return, and no implementer or
  other child writes it.
- Every finding in the note carries `file:line` evidence. A rejected claim stays in the note.
- Do not fix anything here, do not open a remediation phase inside Z1, and do not close or archive
  the plan. The release closes it after the user accepts.
- Do not edit `AGENT-INSTRUCTIONS/LOCAL/gates.md`. Its fallback sentence stays deferred through
  `docs-pass.md`.
- No new dependency, no `unsafe`, no threads, no new randomness, and no change to the scene or the
  camera.
- The note names no plan id and carries no fix. No history or plan id goes into source comments.

**Tests:**
- New/changed/deleted: none. The phase adds no test function, so the budget is 0 added / 0 deleted /
  0 changed. The survey checks the existing tests against REPO-RULES §3a and may report test bloat
  in the note, but it changes nothing.
- Fast gate: not applicable. No code or test changes, so the existing evidence at `0ad0f2b` stands
  and is not rerun.
- Documentation-only gate for the note: reread the note and run `git diff --check`.
- Conditional gates: rendered image, required by step 5 as survey evidence and for the user's
  acceptance. Agent generation: not affected.
- Slow gate (>2 minutes): none. An 80 by 40 render at 4 samples and the 400 by 200 acceptance render
  each finish well under a minute.

**User test (acceptance):**
- **Executor:** user. Accepting the survey findings and judging how the rendered image looks are
  user-owned judgments that no automated check proves.
- 1. Open `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/closing-survey-<date>.md` and read the
  confirmed findings, the rejected claims, the out-of-scope observations and the user-review list.
- 2. Open the acceptance PNG produced by step 5. It shows the sphere over the sky at 400 by 200.
- 3. Run `cargo run --release -- --output /tmp/opencode/acceptance.png` in the worktree and confirm
  one progress line updates in place on standard error and reaches `100%`, and that standard output
  stays empty.
- 4. Answer with acceptance of the note and the render, or with the findings and observations that
  must be handled before acceptance.
- Expected result: the user accepts the survey note and the rendered image, or names the items that
  block acceptance.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a (minimal) are satisfied.
- [ ] All four areas were surveyed by four separate read-only `ops-scout` sessions, and one further
      independent read-only `ops-scout` cross-checked every finding.
- [ ] The dated note exists in `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/` and every finding and
      rejected claim in it carries `file:line` evidence.
- [ ] The note covers unnecessary complexity, duplication, dead code, redundant checks or records,
      test bloat against REPO-RULES §3a, stale documents and drift from the plan's design rules, and
      it lists the out-of-scope observations and the user-review list.
- [ ] The rendered-image gate ran on the surveyed revision: the image was opened and inspected, the
      progress reached `100%` on standard error, and standard output was empty.
- [ ] `docs/requirements.md` items 11 to 13 and the README still describe the behaviour, and
      `docs-pass.md` still lists the `AGENT-INSTRUCTIONS/LOCAL/gates.md` fallback sentence.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports 0 added, 0 deleted and 0 changed.
- [ ] No product code, test, configuration or permanent document changed in this phase, and
      `git diff --check` is clean.
- [ ] The note and the render were reported to the user, and the user's answer is recorded on
      acceptance.
- [ ] The plan is not closed or archived in this phase.
- [ ] On the user's acceptance: compress this file into `done/Z1-closing-survey.md` (the survey
      outcome, the note path and the user's acceptance) and flip the `full-plan.md` status row to
      `done`.

**Exit:** the dated survey note exists with every finding backed by evidence, the acceptance render
was inspected, and the user accepted the survey and the rendered result. The plan is then ready for
the release, which closes it.
