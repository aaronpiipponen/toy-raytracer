## Phase Z1 - Closing survey and user acceptance

**Status:** active.
**Depends on:** C1 (`done/C1-png-writer-and-command-line.md`) and C3 (`done/C3-progress-line.md`),
both done. Their frozen contracts and the current tree are the survey subject.
**Scope:** a read-only closing survey of everything this plan added or changed (the PNG writer, the
command line and settings, the render loop and progress line, and the documents the plan touched),
written as one dated note in the plan folder, plus one acceptance render through the real command
line. Findings are reported to the user. Nothing is fixed here.
**Gate level:** minimal - the phase writes one note in the plan folder and runs read-only
inspection plus one render. It changes no product code, test, configuration or `docs/` page, and it
deletes nothing.
**Simplicity statement:** the survey covers the four areas this plan owns and stops at them. A scout
reports a problem outside its area as an out-of-scope observation with the evidence it already has
and does not investigate it. A finding is recorded with its `path:line` evidence and left for the
user, never repaired in this phase. The note has no severity scoring, no confidence scale, no second
survey round and no re-survey after a fix, because no fix happens here. No new test is written.

**Execution model:** Z1 is orchestrator-executed. The orchestrator dispatches one read-only
`ops-scout` per area, all four in parallel, then one read-only `ops-scout` that cross-checks every
finding together, then writes the note itself and runs the acceptance render. There is no
`phase-implementer` unit for Z1 and no `phase-implementation-reviewer` cross-check of the note. The
phase's evidence is the four saved scout returns, the cross-check return, the note the orchestrator
writes, and the acceptance render it runs.

**Current state (verified):**
- `src/png.rs:9-51` - `pub fn write_png(out: &mut impl Write, width: usize, height: usize,
  pixels: &[Colour]) -> io::Result<()>` writes the signature, one `IHDR` (8-bit, colour type 2, no
  interlace), one `IDAT` of zlib data with stored blocks of at most 65535 bytes, and `IEND`.
  `src/png.rs:54-62` writes a chunk with its CRC-32, `src/png.rs:65-78` computes CRC-32,
  `src/png.rs:81-89` computes Adler-32. `src/png.rs:1` is the module doc line.
- `src/main.rs:23-26` - defaults 400 by 200, 16 samples, `render.png`. `src/main.rs:28` - the usage
  line. `src/main.rs:30-42` - `Format` and `Settings`. `src/main.rs:107-146` - `parse_args(&[String])
  -> Result<Settings, String>`. `src/main.rs:149-157` - `parse_number`. `src/main.rs:160-169` -
  `resolve_format`. `src/main.rs:171-179` - `run(&Settings) -> io::Result<()>`. `src/main.rs:181-195`
  - `main`, which prints errors on standard error and exits with status 2 for a bad command line and
  1 for a failed write.
- `src/main.rs:62-81` - `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>`
  averages `samples` rays per pixel, writes `\r` plus `"{rows done} of {height} rows"` on standard
  error after each row (`src/main.rs:77`) and a final newline after the row loop (`src/main.rs:79`).
  `src/main.rs:83-85` - `to_byte`. `src/main.rs:87-104` - `write_ppm`. `src/main.rs:45-51` -
  `ray_colour`, `src/main.rs:53-58` - `scene`.
- `tests/cli.rs:30-35` - the `run(args: &[&str]) -> Output` helper. Three test functions:
  `writes_a_png_whose_pixels_match_the_ppm_render` (`tests/cli.rs:37-88`),
  `rejects_an_unknown_option_and_an_unparsable_value_with_a_non_zero_exit` (`tests/cli.rs:90-106`)
  and `selects_the_format_from_the_output_extension_and_lets_an_explicit_format_win`
  (`tests/cli.rs:108-165`). `tests/cli.rs:168-249` - `decode_png`, which re-derives the chunk CRCs,
  the stored-block framing, the single `IDAT`, the filter bytes and the Adler-32 trailer from the
  written bytes. `src/main.rs:197-212` - the two unit tests that call `render(200, 100, 16)`.
- `README.md:5-8` - the run example `cargo run --release -- --output image.png` and `cargo test`.
- `docs/requirements.md` - items 11 and 12 under "Output and command line" state the PNG writer and
  the command line, item 13 states the progress line, item 14 the quality gate, and items 1 to 6 the
  materials and sampling this plan does not implement.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast phase gate, the rendered-image gate command and
  the fallback sentence that the release removes.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md` - the release item for that fallback sentence.
- The plan's product changes are the PNG writer in `src/png.rs`, the command line and render changes
  in `src/main.rs`, the CLI tests in `tests/cli.rs`, and the README run example; nothing else in the
  tree was added or changed by this plan.
- Cargo and its tools are not on `PATH`; call `~/.cargo/bin/cargo`.

**Read first (do not invent):**
- `docs/requirements.md` - items 11, 12 and 13 are the acceptance subject, item 14 the quality gate.
- `src/png.rs` - the whole file, against requirement 11.
- `src/main.rs` - the whole file, against requirements 12 and 13 and C1's frozen contract.
- `tests/cli.rs` - the whole file, the public test surface the survey checks for real coverage.
- `README.md` - the two commands it shows.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the rendered-image gate command to copy.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/done/C1-png-writer-and-command-line.md` and
  `AGENT-INSTRUCTIONS/PLANS/plan-c/done/C3-progress-line.md` - the guarantees the survey checks
  against the tree.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md` - the one deferred documentation item.

**Build plan:**
1. The orchestrator dispatches one independent read-only `ops-scout` per area below, all four in
   parallel. Each dispatch names its area, the questions listed, and the files to start from, and
   states that the scout reads whatever context outside its area it needs to judge it; a problem it
   notices outside its area is reported as an out-of-scope observation with the evidence it already
   has and is not investigated further. No dispatch tells a scout to report only on the diff. The
   four returns are kept as the saved finding set. The areas and their questions:
   - PNG writer: does `src/png.rs` produce what requirement 11 states, and does
     `tests/cli.rs:168-249` independently check chunk CRCs, the stored-block framing, the single
     `IDAT`, the filter bytes and the Adler-32 trailer rather than trusting the writer's own code?
   - Command line and settings: do `parse_args`, `parse_number`, `resolve_format`, `run` and `main`
     honour requirement 12 and C1's frozen contract (all five options, the four defaults, explicit
     format over the extension, a bad option or value printing the usage line with status 2, an
     unwritable path reporting with status 1, PPM reachable, nothing on standard output)?
   - Render loop and progress line: does `render` match C3's frozen contract (row-major pixels,
     `samples` rays averaged per pixel, the progress write after each row, the terminating newline,
     no output on standard output), and does the same command produce identical bytes twice?
   - Documents: are `README.md`, `docs/requirements.md` items 11 to 13, `gates.md` and
     `docs-pass.md` accurate against the tree, and does any product file carry plan history or an
     unfinished marker?
   A clean area is recorded as clean with the files and symbols that were checked. A finding names
   the exact `path:line`, what the tree does and which requirement or frozen guarantee it misses.
2. The orchestrator dispatches ONE independent read-only `ops-scout` that cross-checks ALL saved
   findings together, claim by claim, against the named `path:line` evidence, and reports every claim
   that does not match the tree plus any finding the survey missed only where it already has the
   evidence. Areas get no per-area checker, and this step does not expand the survey beyond the four
   areas.
3. The orchestrator writes `benchmarks/2026-11-24-closing-survey.md` in the plan folder from the
   saved returns and the cross-check: one section per area above, each with its confirmed findings as
   `path:line` plus a short statement or an explicit sentence that the area produced no finding, and
   a separate section for out-of-scope observations. A claim the cross-check rejects is corrected or
   removed. Add a final section for the acceptance render. Do not include review narration,
   timestamps beyond the file name, or suggestions for future work beyond the finding itself.
4. The orchestrator produces the acceptance evidence through the real command line, from the
   worktree root: `~/.cargo/bin/cargo build --release`, then
   `./target/release/toy-raytracer --output /tmp/opencode/z1-accept.png 2>/tmp/opencode/z1-progress.err`,
   run a second time to `/tmp/opencode/z1-accept-2.png`, and `cmp` the two PNGs. Expect exit status
   0, empty standard output, identical bytes, a progress capture whose updates run from `1 of 200
   rows` to `200 of 200 rows` separated by carriage returns and ending with a newline, and a PNG
   that opens showing the sphere over the sky and the ground. Record the observed result in the
   note's final section.
5. The orchestrator presents the note path and summary, the full finding list (or an explicit
   no-finding statement for an area) and `/tmp/opencode/z1-accept.png` to the user, and changes
   nothing else.

**Callers / wiring to update:**
- None. The phase changes no code, so no caller, import or configuration changes. Area three reads
  `render`'s caller `run` (`src/main.rs:172`) and the two unit-test callers
  (`src/main.rs:203`, `src/main.rs:210`).

**Config / schema / migrations:**
- None. No configuration key, schema or migration exists in this project, and the phase adds none.
- Retained records and their named consumer/owner: none. The note is plan working state, read by the
  user, not product data.

**Touched files:**
<!-- touched-files:start -->
- `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/2026-11-24-closing-survey.md` - created: the dated closing survey with every finding, the out-of-scope observations and the acceptance render record. Its parent `benchmarks/` directory is created with it.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Fix nothing. A finding is reported to the user with its evidence. Do not edit product code, tests,
  `Cargo.toml`, `docs/`, `README.md`, `gates.md`, `docs-pass.md`, the phase files or the status table.
- Do not delegate the note to any child. No `phase-implementer` implements this phase and no
  `phase-implementation-reviewer` cross-checks the note; the orchestrator writes it.
- Every scout is read-only and writes no file. Do not add a per-area checker: each area gets one
  scout, and the single cross-check step covers all findings together.
- Do not tell a scout to report only on the diff; each covers its whole area. Do not let a scout
  investigate outside its area: an out-of-scope problem is reported with the evidence it already has.
- Do not write a finding from memory or from another document. Every claim is read from the current
  tree and cites `path:line`, and a claim that cannot be confirmed is reported as unconfirmed rather
  than asserted.
- Do not invent history, severity scores, review narration or future work beyond the finding.
- Do not survey outside the four areas this plan owns. Materials, scattering, jitter, geometry and
  bounding boxes belong to other plans and are named only when a stated requirement of this plan
  depends on them.
- Do not add a test, run the test suite, or create a second survey round. The minimal gate checks the
  note and the acceptance render only.
- Keep the phase single-purpose and read-only apart from the note and the two renders in
  `/tmp/opencode/`.

**Tests:**
- New/changed/deleted: none. This phase writes no test and changes no test function. Test functions
  added 0, deleted 0, changed 0.
- Fast gate for this documentation-only phase: reread the created note for accuracy, check every
  `path:line` it cites, and run `git diff --check` in the worktree. The release build in build plan
  step 4 is required only to run the acceptance binary. `cargo test`, `cargo clippy` and `cargo fmt`
  are not required because no Rust source changes.
- Conditional gates: the rendered-image gate does not apply, because this phase changes neither the
  render loop, a material, the camera nor the output format. The plan instead requires an acceptance
  render through the real command line: `./target/release/toy-raytracer --output
  /tmp/opencode/z1-accept.png` at the defaults (400 by 200, 16 samples), the PNG opened to confirm the
  sphere over the sky and ground, and a second identical render compared with `cmp`. Agent
  generation: not touched.
- Slow gate (>2 minutes): none. The default release render finishes in seconds.

**User test (acceptance):**
- **Executor:** user, for the final acceptance only. The four scout returns, the cross-check, the
  note and the acceptance render are produced by the orchestrator as evidence; the user judges
  whether the result satisfies the plan.
1. The orchestrator presents the note summary, the finding list and the
   `/tmp/opencode/z1-accept.png` render it produced.
2. Expect the user to accept the result, or to name each defect that must be addressed. A named
   defect becomes follow-up work; this phase does not fix it.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a (minimal) are satisfied.
- [ ] Four independent read-only `ops-scout` returns exist, one per area, covering their whole area
   rather than a diff, and any out-of-scope observation in them carries the evidence already had.
- [ ] One independent read-only `ops-scout` cross-check ran over all findings, claim by claim, and
   every claim it reported as mismatched was corrected or removed from the note.
- [ ] `benchmarks/2026-11-24-closing-survey.md` exists, was written by the orchestrator, and holds
   one section per area with every finding backed by a `path:line` read from the current tree.
- [ ] Each clean area states that it produced no finding and names what was checked.
- [ ] The note ends with the acceptance-render section, recording exit status 0, empty standard
   output, the observed progress range and terminating newline, the `cmp` result and the observed
   image.
- [ ] No `phase-implementer` implemented this phase and no `phase-implementation-reviewer`
   cross-checked the note.
- [ ] The acceptance render was produced through the real command line, opens showing the sphere
   over the sky and ground, keeps standard output empty, ends the progress line with a newline,
   and a second identical render is byte-identical by `cmp`.
- [ ] No product code, test, configuration, `docs/` page, `README.md`, `gates.md`, `docs-pass.md`,
   phase file or status row changed.
- [ ] Test budget respected: 0 test functions added, 0 deleted, 0 changed.
- [ ] The note was reread and `git diff --check` is clean. No Git commit is made from the worktree:
   the plan folder is outside the project repository and the workflow plugin commits it.
- [ ] The user test has passing evidence: the user accepted the survey and the result, or named the
   defects.
- [ ] After the user accepts, the advancer compresses this file into
   `done/Z1-closing-survey-and-user-acceptance.md` and flips the `full-plan.md` status row. This
   phase does not edit either.

**Exit:** the dated closing survey note is written by the orchestrator with evidence-backed findings,
the acceptance render and progress line are observed, and the user has accepted the result or named
the remaining defects.
