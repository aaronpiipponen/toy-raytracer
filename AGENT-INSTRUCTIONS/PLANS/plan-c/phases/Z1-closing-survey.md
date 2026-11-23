## Phase Z1 - Closing survey and user acceptance

**Status:** active.
**Depends on:** `done/C1-png-writer.md`, `done/C2-command-line.md` and `done/C3-progress-line.md`, the three frozen contracts of this plan.
**Scope:** one read-only closing survey of every area this plan added or changed, a dated note in `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/survey-2026-11-23.md`, and the user's acceptance of that note. No product code, test, configuration or permanent document changes. Findings go to the user and are fixed outside this phase only when the user directs it.
**Gate level:** minimal - the phase changes no product code, test or configuration, and its one artifact is a documentation note in the plan folder.
**Simplicity statement:** the survey adds one dated note with evidence for each finding and fixes nothing. It adds no survey framework, checker, command, test, registry, index or second copy of the contracts, and it writes no product record or receipt. A change the survey suggests is either a small checked fix the user approves in a later round or a user decision, never work performed inside Z1.

**Execution owner:** the plan orchestrator, not `phase-implementer`. The binding survey method in `AGENT-INSTRUCTIONS/PLANNING/PLAN-AUTHORING-SCOPING.md` section 3b assigns the scout dispatch and the note writing to the orchestrator, and the read-only `ops-scout` agent is available only to it. Do not hand this phase to `phase_subagent` and do not ask a writer to produce the note.

**Current state (verified):**
- The plan's code head is `525196b`; `Cargo.toml` has an empty `[dependencies]` table and package `toy-raytracer`.
- `src/main.rs:44-65` - the doc comment and `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>`, whose row loop writes the progress line to standard error and whose last statement ends the line.
- `src/main.rs:71-88` - `fn write_ppm(out: &mut impl Write, width: usize, height: usize, pixels: &[Colour]) -> io::Result<()>`.
- `src/main.rs:90-161` - `enum Format { Png, Ppm }`, `struct Config`, its `Default` and `Config::parse(args: impl Iterator<Item = String>) -> Result<Config, String>`.
- `src/main.rs:163-175` - `parse_value`, `parse_number` and `fn usage() -> &'static str`.
- `src/main.rs:177-199` - `fn main() -> io::Result<()>`, which parses, calls `render`, opens the output through one `io::BufWriter` and writes PNG or PPM.
- `src/main.rs:201-255` - five tests: two render tests, the defaults test, the full-command-line test and the unknown-option test. The other modules add five tests, for 10 in total.
- `src/png.rs:11` - `pub fn write_png(out: &mut impl Write, width: usize, height: usize, rgb: &[u8]) -> io::Result<()>`, the only PNG encoder.
- `README.md:5-20` - the commands, the five options with their defaults, the no-options sentence and the standard-error progress sentence.
- `docs/requirements.md:32-41` - items 11 to 13, the output, command line and progress line this plan implements; item 14 at line 45 lists the gates.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` and `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the repository map and the gate commands.
- `AGENT-INSTRUCTIONS/LOCAL/documentation-pages.md` - the documentation set is `README.md` and `docs/requirements.md`.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md` - the one release-time documentation item, the root `README.md` shape.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/` does not exist. `plan-a` and `plan-b` have no `benchmarks/` folder either, so no earlier survey note exists to base finding classes on.
- `src/` names no `unsafe` and no thread type.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANNING/PLAN-AUTHORING-SCOPING.md` section 3b - the binding survey method, the survey stops and the closing-survey rule; follow it exactly and add no steps.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/full-plan.md` - the plan's scope, its binding design rules and the phase rows.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/done/C1-png-writer.md`, `done/C2-command-line.md` and `done/C3-progress-line.md` - the frozen contracts the survey checks against.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md` - the release-time documentation item that is not a plan-c defect.
- `docs/requirements.md:32-45` - items 11 to 14.
- `README.md` - the whole page.
- `AGENT-INSTRUCTIONS/LOCAL/documentation-pages.md`, `AGENT-INSTRUCTIONS/LOCAL/project.md` and `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the documentation set, the repository map and the gates.
- `Cargo.toml` and every file under `src/` - the source the survey reads.
- Earlier survey notes: none exist. State that in the note instead of inventing a base.

**Build plan:**

1. Read the plan's scope, its binding design rules and the three done contracts, then name the areas this plan added or changed:
   - **PNG output:** `src/png.rs` in full, the `Format::Png` branch and the byte conversion in `main`.
   - **Command line and output selection:** `Config`, `Format`, `Config::parse`, `parse_value`, `parse_number`, `usage`, `main` and `write_ppm`.
   - **Progress line and render loop:** `render`, the two render tests, and the rendered image at the default options.
   - **Plan-facing documentation, tests and state:** `README.md`, `docs/requirements.md`, `AGENT-INSTRUCTIONS/LOCAL/project.md`, `AGENT-INSTRUCTIONS/LOCAL/gates.md`, `docs-pass.md`, the three done contracts and the ten tests across `src/`.
2. Dispatch one read-only `ops-scout` per area in parallel through the orchestrator's background task mechanism. Each prompt gives its area, the plan's scoped contract, the finding classes and the evidence format:
   - the finding classes are unnecessary complexity, duplication, dead code, redundant checks or records, test bloat, stale documentation and drift from the user's decisions;
   - every finding carries `file:line` evidence and the class it belongs to;
   - an out-of-scope observation is reported with the evidence already in hand and is not investigated further;
   - the scout reads whatever context outside the area it needs to judge it, and no prompt limits it to the diff.
3. After all four returns arrive, dispatch ONE further read-only `ops-scout` that receives every first-round finding and cross-checks each claim against the source, the documents and the done contracts. It marks each finding `CONFIRMED`, `WRONG` or `UNCLEAR` with evidence, and reports any finding the first round missed from the same evidence. Areas get no per-area checker.
4. The orchestrator writes `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/survey-2026-11-23.md` itself from the saved returns. No scout, implementer or other child writes it. The note contains, in this order:
   - the date `2026-11-23`, the plan scope, the four areas and the statement that no earlier survey note exists;
   - one row per real finding with an id, the finding class, `file:line` evidence, the impact and a recommendation;
   - the out-of-scope observations, kept separate from plan-c findings;
   - the cross-check table with one verdict and its evidence per finding, and the missed findings it added;
   - the checks performed, recording the requirement comparison, the hard invariants, the rendered image, the test budget, the documentation accuracy and the `docs-pass.md` state;
   - a user-review list of every finding that needs the user's decision, and every finding the cross-check marked `UNCLEAR`.
   Drop every finding the cross-check marks `WRONG`. Do not record a finding without `file:line` evidence.
5. Check the requirements and the hard invariants in the note:
   - requirement 11: the default and an explicit `.png` path both write a valid PNG through `src/png.rs`; a `.ppm` path writes P3 PPM through `write_ppm`;
   - requirement 12: the five options and their defaults from `Config::default`, the extension-derived format, and the usage line with a non-zero exit on a bad option;
   - requirement 13: the standard-error progress line ends at `100%` with a newline;
   - requirement 14 and the fast gate: the receipt at `525196b` already reports `cargo build`, `cargo test` (10 passed), `cargo clippy -- -D warnings`, `cargo fmt --check` and `git diff --check`, so do not rerun them on the unchanged revision;
   - the hard invariants: `Cargo.toml` has no dependency, `src/rng.rs` supplies all seeded randomness and two identical runs write identical bytes, and `src/` uses no `unsafe` and no thread.
6. Render the default image once for the comparison and the reproducibility check, from `~/.cargo/bin/cargo` when `cargo` is not on `PATH`:
   ```bash
   ~/.cargo/bin/cargo build --release
   ./target/release/toy-raytracer --output /tmp/opencode/survey-a.png 2>/tmp/opencode/survey.err >/tmp/opencode/survey.out
   ./target/release/toy-raytracer --output /tmp/opencode/survey-b.png 2>/dev/null
   cmp /tmp/opencode/survey-a.png /tmp/opencode/survey-b.png
   python3 -c "d=open('/tmp/opencode/survey-a.png','rb').read(); print(d[:8]==bytes([137,80,78,71,13,10,26,10]), d[16:24].hex())"
   python3 -c "d=open('/tmp/opencode/survey.err','rb').read(); print(d.count(b'\r'), d.endswith(b'100%\n'))"
   test ! -s /tmp/opencode/survey.out
   ```
   `cmp` exits 0, the header prints `True 00000190000000c8` (400 by 200), the progress file prints `200 True`, and standard output is empty. Open `/tmp/opencode/survey-a.png` with the image reader and confirm the sphere sits over the sky gradient at 400 by 200. Record all of it in the note.
7. Compare the note against the plan and the requirements one more time, then report to the user: the note path, the finding summary by class, the cross-check result and the user-review list. State plainly that the findings are not fixed inside Z1.
8. On the user's acceptance, the advancer compresses this file into `done/Z1-closing-survey.md` and flips the status row. If the user names changes, return their direction to the orchestrator for a later survey or remediation round under section 3b; do not make them here. The plan is not archived on acceptance alone: closure needs the finished release and the user's explicit authorization (`AGENT-INSTRUCTIONS/PLANS/README.md` section Archive).

**Callers / wiring to update:**
- None. No product file calls anything new and no signature changes.
- The note is read by the user and by the release's closing-survey acceptance; `full-plan.md` and `done/Z1-closing-survey.md` change only in the advancer's completion step.

**Config / schema / migrations:**
- None. No configuration key, schema, migration or generated product file is added.
- Retained records and their named consumer/owner: the dated survey note only. Its consumer is the user's acceptance decision and the release closing-survey gate, and the plan folder's `benchmarks/` directory owns it. No product record, receipt or hash is stored.

**Touched files:**
<!-- touched-files:start -->
- `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/survey-2026-11-23.md` - created: the closing survey note with cross-checked findings, the checks performed and the user-review list.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/full-plan.md` - changed after acceptance: the advancer flips the Z1 row to done.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/done/Z1-closing-survey.md` - created after acceptance: the advancer compresses this phase into the frozen contract.
- Adds: nothing in the project repository (no dependency, configuration key or generated file).
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not edit `src/`, `Cargo.toml`, `README.md`, `docs/requirements.md`, `AGENT-INSTRUCTIONS/LOCAL/**` or any permanent document. The only new file is the survey note.
- Do not fix any finding inside Z1, not even a one-line documentation or comment correction. Findings go to the user's review list, and a fix needs the user's direction in a later round.
- Do not add a test, gate, checker, command, registry, index file or receipt to run or record the survey.
- Do not claim a finding without `file:line` evidence confirmed by the cross-checking scout. Drop `WRONG` findings and put `UNCLEAR` ones on the user-review list.
- Do not rerun the fast gate on the unchanged `525196b` revision; reuse its receipt. Do not rerun the 80 by 40 rendered-image evidence from C3; the default render above is the one Z1 owns.
- Do not recount a contract in the note as a second source of truth; cite the done contract and the current source.
- Do not write history, review narration or plan identifiers into a permanent file, and do not name a plan in code or `docs/`.
- Do not move or archive the plan folder. Closure follows the release and the user's explicit authorization.
- Do not add a dependency, `unsafe`, threads or a second randomness source.

**Tests:**
- New/changed/deleted: none. The plan's gate level is minimal and the phase changes no product code, so REPO-RULES section 3a allows zero new test functions.
- Test budget: 0 new test functions, 0 changed, 0 deleted.
- Fast gate: not run on the unchanged `525196b` revision, whose receipt already reports every fast command. The note itself is re-read for accuracy and its paths are checked.
- Conditional gates: rendered image, run once by the plan orchestrator at the default 400 by 200 and 16 samples (step 6), because the survey compares the scene with the requirements and checks reproducibility.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** user.
- The plan's move-on gate is the user's acceptance of the closing survey.
- 1. Open `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/survey-2026-11-23.md` and read the findings, the cross-check verdicts and the user-review list.
- 2. Open `/tmp/opencode/survey-a.png` and confirm it shows the sphere over the sky gradient at 400 by 200.
- 3. Reply that the survey results are accepted, or name each change to make. Any change is scheduled outside Z1.
- After the reply, the orchestrator removes `/tmp/opencode/survey-a.png`, `/tmp/opencode/survey-b.png`, `/tmp/opencode/survey.err` and `/tmp/opencode/survey.out`. Acceptance alone does not close or archive the plan.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` section 3a for `minimal` are satisfied.
- [ ] `benchmarks/survey-2026-11-23.md` exists, is dated `2026-11-23`, and covers the four areas the plan added or changed.
- [ ] Every finding carries `file:line` evidence and a cross-check verdict. No `WRONG` finding remains, and every `UNCLEAR` finding is on the user-review list.
- [ ] The note records the requirement checks (items 11 to 14), the three hard invariants, the test budget, the documentation accuracy and the `docs-pass.md` state, and states that no earlier survey note exists.
- [ ] The default 400 by 200 render is a valid PNG whose header is `True 00000190000000c8`, shows the sphere over the sky, writes 200 carriage returns ending `100%\n` on standard error, writes nothing to standard output, and two runs are byte-identical.
- [ ] No product source, test, configuration or permanent document changed, and `git status --porcelain` in the worktree lists no product file.
- [ ] No test, dependency, configuration key, checker, command or receipt was added.
- [ ] Findings sit only in the note's user-review list and none was fixed inside Z1.
- [ ] The user accepted the survey results, or named the changes to make.
- [ ] On acceptance: the advancer compresses this file into `done/Z1-closing-survey.md` and flips the `full-plan.md` row to done. The plan is not archived until the release finishes and the user authorizes closure.

**Exit:** the closing survey note records the plan's areas with cross-checked findings and a user-review list, the user has accepted the results or named the changes to make, and no phase of the plan remains open.
