## Phase C3 - Progress line

**Status:** active.
**Depends on:** C1, which leaves the resolved settings, the `samples` render seam and the file-only
output path.
**Scope:** while `render` walks the rows, write the share of rows done on standard error on one
line that overwrites itself, then terminate the line when the last row finishes. Pixels, sampling,
the command line, the writers and the docs do not change.
**Gate level:** standard. The phase changes runtime output, so `minimal` does not apply. There is no
durable state, trust boundary or cross-package contract, so `standard` is the right bar.
**Simplicity statement:** the progress line is one integer percentage prefixed by `\r` and closed by
one newline. There is no terminal detection, no timing, throttling or verbosity option, no buffered
writer and no explicit flush (the standard error stream is unbuffered), no progress module, and no
test of the text or its cadence. Nothing is deleted.

**Current state (verified):**
- `src/main.rs:60-78` - `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>`
  iterates `for row in (0..height).rev()` at line 65, `for column in 0..width` at line 66, pushes
  each pixel at line 74 and returns `pixels` at line 77. The render loop is the only place the rows
  are known.
- `src/main.rs:182-190` - `fn run(settings: &Settings) -> io::Result<()>` calls
  `render(settings.width, settings.height, settings.samples)` at line 183 and builds the image bytes
  in memory before `fs::write`.
- `src/main.rs:192-206` - `fn main()` writes only errors to standard error (`eprintln!` at lines 197,
  198 and 203) and nothing to standard output. No image path touches standard output.
- `src/main.rs:208-224` - the two unit tests call `render(200, 100, 16)` at lines 214 and 221.
- `tests/cli.rs` - the three tests assert standard error non-empty only for rejected input; no test
  expects standard error to be empty on a successful render, so the new output breaks none of them.
- `src/` has no progress output of any kind today.
- `docs/requirements.md:41` - item 13 already states the progress line on standard error.
- `README.md:5-8` - the run example stays correct because the command line does not change.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md:33-39` - the rendered-image gate command and its deferred
  fallback sentence. The file is read only and is not edited by this phase.

**Read first (do not invent):**
- `src/main.rs` - the whole file; the row loop, `run` and the tests are the places this phase
  touches or relies on.
- `tests/cli.rs` - the three binary tests, to confirm none asserts an empty standard error on
  success.
- `docs/requirements.md` item 13 - the required progress behaviour.
- `README.md` lines 5 to 8 - the run example that must stay accurate.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the rendered-image gate command this phase runs. Read only.

**Build plan:**

1. `src/main.rs`, `render` - add the progress line inside the existing loops, with no signature
   change. After the closing brace of the `for column` loop and inside the `for row` loop, add:
   ```rust
   let done = height - row;
   eprint!("\r{}%", done * 100 / height);
   ```
   `row` counts down from `height - 1`, so `height - row` counts completed rows from 1 to `height`
   and the last row prints `100%`. After the closing brace of the `for row` loop and before
   `pixels`, add:
   ```rust
   eprintln!();
   ```
   This writes the line break that ends the progress line. `eprint!` and `eprintln!` write to
   standard error, which is unbuffered, so no `flush` call and no buffered writer are needed.
2. `src/main.rs`, `render` doc comment - extend it to say the row share is reported on standard
   error, for example "Every pixel's colour, row by row from the top, each pixel averaging `samples`
   rays, with the share of finished rows reported on standard error." Keep it one sentence in the
   existing present-tense style.
3. No caller, signature, import or wiring change follows from step 1. `run` and the two unit tests
   keep calling `render` exactly as they do.
4. Docs: no page changes. `docs/requirements.md` item 13 already matches the behaviour, and the
   `README.md` run example stays valid. Do not edit either file.

**Callers / wiring to update:**
- `src/main.rs:183` - `run` calls `render`; unchanged.
- `src/main.rs:214` and `src/main.rs:221` - the two unit tests call `render`; unchanged. Their
  standard error output is captured by the test harness.
- `src/main.rs:192-206` - `main` is unchanged; it keeps printing only errors.

**Config / schema / migrations:**
- No configuration key, schema or migration.
- Retained records and their named consumer/owner: none. The progress line is transient terminal
  output, not a stored field, log or receipt.

**Touched files:**
<!-- touched-files:start -->
- `src/main.rs` - changed: `render` writes the share of finished rows to standard error on one
  overwriting line and terminates it; its doc comment states the progress output.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Write progress only to standard error. Do not write it to standard output or into the image file.
- Do not print a new line per row. The `\r` prefix makes one line overwrite itself and only the
  final `eprintln!()` breaks it.
- Do not add a second rendering path, a progress module, a buffered writer or an explicit flush.
- Do not add an option: no `--quiet`, no verbosity, no timing, no throttling, no terminal detection.
- Do not change `render`'s signature, the pixel computation, the sample loop, the ray, the camera,
  the scene, the RNG or either writer.
- Do not test the progress text or its refresh cadence, and do not add or change a test function.
- Do not touch `docs/requirements.md`, `README.md`, `AGENT-INSTRUCTIONS/LOCAL/gates.md`,
  `Cargo.toml`, `Cargo.lock` or `.gitignore`.
- No history, plan id or phase id in source comments.
- No dependency, no `unsafe`, no threads and no new randomness.

**Tests:**
- New/changed/deleted: none. No test asserts progress text or cadence, so the budget is 0 added / 0
  deleted / 0 changed. The existing tests in `src/main.rs` and `tests/cli.rs` must still pass
  unchanged.
- Fast gate, run from the worktree (cargo is not on `PATH` here, so `~/.cargo/bin/cargo`):
  `~/.cargo/bin/cargo test`, `~/.cargo/bin/cargo build`,
  `~/.cargo/bin/cargo clippy -- -D warnings`, `~/.cargo/bin/cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image, required because the render loop changes. Before editing,
  produce the baseline `~/.cargo/bin/cargo run --release -- --width 80 --height 40 --samples 4
  --output /tmp/opencode/before.png`; after the change, run the same command with `--output
  /tmp/opencode/after.png` and confirm `cmp /tmp/opencode/before.png /tmp/opencode/after.png`
  reports no difference. Then run
  `~/.cargo/bin/cargo run --release -- --width 80 --height 40 --samples 4 --output
  /tmp/opencode/progress.png` and observe the progress line on standard error updating in place,
  reaching `100%` and ending with a newline, with nothing on standard output. Open
  `/tmp/opencode/progress.png` with the image reader and confirm the sphere and sky at 80 by 40.
  Agent generation: not affected.
- Slow gate (>2 minutes): none. An 80 by 40 render at 4 samples finishes well under a second.

**User test (acceptance):**
- **Executor:** agent.
- No separate manual test: the rendered-image gate above is the acceptance step.
1. Run `~/.cargo/bin/cargo run --release -- --width 80 --height 40 --samples 4 --output
   /tmp/opencode/accept.png`. Expect exit status 0, a progress line on standard error that reaches
   `100%` and ends with a newline, no bytes on standard output, and the file `/tmp/opencode/accept.png`.
2. Open `/tmp/opencode/accept.png` with the image reader. Expect the 80 by 40 scene with the sphere
   over the blue-to-white sky gradient, not a corrupt or blank image, and the same bytes as the
   pre-change baseline from the rendered-image gate.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a (standard) are satisfied.
- [ ] Rendering reports the share of finished rows on standard error on one line that overwrites
      itself and ends with a newline.
- [ ] Standard output is empty during a successful render and the image file is unchanged.
- [ ] The image bytes for identical options match the pre-change baseline.
- [ ] Named tests pass unchanged; no test function is added, changed or deleted.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports 0 added, 0 deleted and 0 changed.
- [ ] Fast phase gate and the rendered-image conditional gate from
      `AGENT-INSTRUCTIONS/BUILDING/VERIFICATION-COMMITS-DEPLOY.md` §1 pass; no slow gate applies.
- [ ] `docs/requirements.md` item 13 still describes the behaviour and no `docs/` page or the
      README is stale; `AGENT-INSTRUCTIONS/LOCAL/gates.md` is unchanged.
- [ ] Safety commit on branch `plan-c`.
- [ ] Revision-bound verification receipt exists; the agent-run `User test` steps have passing
      evidence.
- [ ] On passing evidence: compress this file into `done/C3-progress-line.md` (frozen contract
      only) and flip the `full-plan.md` status row to `done`.

**Exit:** a render shows its share of finished rows on one standard-error line that reaches `100%`
and ends with a newline, standard output stays empty, and the image bytes for the same options are
unchanged.
