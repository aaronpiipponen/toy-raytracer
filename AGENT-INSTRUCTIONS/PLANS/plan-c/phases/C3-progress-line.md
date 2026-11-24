## Phase C3 - Progress line

**Status:** active.
**Depends on:** C1 (`done/C1-png-writer-and-command-line.md`): the resolved settings, the render loop
that takes the sample count, and the rule that standard output stays clean.
**Scope:** while rendering, report the share of rows done on standard error, overwriting one line, and
end that line with a newline when the render finishes. The image bytes and everything else the
program writes stay exactly as they are.
**Gate level:** standard - the render loop changes and it emits output, so the fast gate plus the
rendered-image conditional gate and a byte comparison of the image before and after the change prove
it. No durable state, no trust boundary, no migration.
**Simplicity statement:** the line is `\r` + "N of M rows", rewritten after each row, with no
percentage rounding, padding, timing, throttling, verbosity switch, terminal-width or tty detection,
and no flush call beyond what standard error already does. The phase deletes nothing and adds no
fallback path.

**Current state (verified):**
- `src/main.rs:60-78` - `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>` builds
  the scene and camera once, then fills `pixels` in `for row in (0..height).rev() { for column in
  0..width { ... } }`, pushing `colour / samples as f64` for each pixel. It writes nothing to any
  stream.
- `src/main.rs:60` - the doc comment reads `/// Every pixel's colour, row by row from the top,
  averaging `samples` rays per pixel.`
- `src/main.rs:168-176` - `fn run(settings: &Settings) -> io::Result<()>` calls `render`, encodes the
  pixels into `bytes` and writes the file once with `fs::write`.
- `src/main.rs:178-192` - `fn main()` prints errors through `eprintln!` and exits; it writes nothing
  to standard output.
- `src/main.rs:194-210` - the unit tests call `render(200, 100, 16)` directly; the test harness
  captures whatever the test prints.
- `tests/cli.rs:37-80` - `writes_a_png_whose_pixels_match_the_ppm_render` runs the binary with
  `--width 8 --height 4 --samples 1`, asserts success and compares three PNG pixels with a PPM run.
  It makes no assertion about standard output or standard error.
- Cargo and its tools are not on `PATH` in this worktree; call `~/.cargo/bin/cargo`.

**Read first (do not invent):**
- `src/main.rs` - the whole file, especially `render` and the doc comment above it.
- `tests/cli.rs` - the existing binary test adapted below and its `run` helper.
- `docs/requirements.md` item 13 - the progress line on standard error showing the share of rows done.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the rendered-image gate command. Read only, never edited.

**Build plan:**

1. `src/main.rs`, `render` - inside the outer loop, after the inner column loop closes and before the
   outer loop closes, add `eprint!("\r{} of {} rows", height - row, height);`. `row` counts down from
   `height - 1` to 0, so `height - row` is the number of rows finished, from 1 to `height`. Standard
   error is unbuffered in `std`, so each update reaches the terminal without a flush call.
2. `src/main.rs`, `render` - after the outer loop and before `pixels` is returned, add `eprintln!();`
   so the finished progress line is terminated and later output starts on its own line.
3. `src/main.rs`, the doc comment above `render` - extend it to
   `/// Every pixel's colour, row by row from the top, averaging `samples` rays per pixel. Reports the
   /// share of rows done on standard error.` Keep the first sentence in its noun-phrase form.
4. `tests/cli.rs`, `writes_a_png_whose_pixels_match_the_ppm_render` - after the first successful
   `run`, add two assertions on its `Output`: `result.stdout.is_empty()` with the message that the
   image goes only to the file, and `!result.stderr.is_empty()` with the message that the render
   reports progress. Keep every existing assertion; do not assert the progress wording, the row
   numbers or the carriage return.

**Callers / wiring to update:**
- `src/main.rs:169` - `run` is the only caller of `render` and needs no edit: the progress writes
  happen inside `render`, and `run` already returns the pixels unchanged.
- `src/main.rs:200` and `src/main.rs:207` - the two unit tests call `render` and need no edit. The
  test harness captures standard error, so the progress line does not disturb them.
- No other module calls `render` (grep over `*.rs` finds `run` and the two unit tests only).

**Config / schema / migrations:**
- None. No new option, constant or configuration key; the progress line has no setting.
- Retained records and their named consumer/owner: none. The progress line is transient terminal
  output and is stored nowhere.

**Touched files:**
<!-- touched-files:start -->
- `src/main.rs` - changed: `render` writes the row progress to standard error and terminates the line; its doc comment states that.
- `tests/cli.rs` - changed: the existing PNG/PPM test also asserts empty standard output and non-empty standard error.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Write progress only to standard error. Standard output and the image file stay byte for byte what
  they are, and no newline or text enters the image file.
- No timing, throttling, `--quiet`, `--verbose`, percentage formatting, terminal detection or third
  stream. One line, overwritten as rows finish, terminated at the end.
- Do not change the pixels, the sampling, the scene, the camera, the settings or the file writing.
  `render`'s signature and return value stay as C1 leaves them.
- Do not add a test function. Do not assert the exact progress wording or the refresh cadence; the
  existing binary test only proves that a successful render keeps standard output empty and writes
  something to standard error.
- No `unsafe`, no threads, no new dependency, no new module.
- Do not edit `AGENT-INSTRUCTIONS/LOCAL/gates.md` or `docs/requirements.md`. Requirement 13 already
  states this behaviour and the README usage line does not change.

**Tests:**
- Changed: `tests/cli.rs` - `writes_a_png_whose_pixels_match_the_ppm_render`: after the first run
  succeeds, assert `result.stdout.is_empty()` and `!result.stderr.is_empty()`. Existing pixel
  comparisons stay.
- New/deleted: none. Test functions added 0, deleted 0, changed 1. The progress wording and cadence
  are not tested; the agent-run acceptance below observes the line itself.
- Fast gate, run from the worktree: `~/.cargo/bin/cargo test --test cli`, `~/.cargo/bin/cargo test`,
  `~/.cargo/bin/cargo build`, `~/.cargo/bin/cargo clippy -- -D warnings`, `~/.cargo/bin/cargo fmt
  --check`, `git diff --check`.
- Conditional gates: rendered image, required because the render loop changes. Before editing, render
  a baseline at the phase's base revision:
  `~/.cargo/bin/cargo run --release -- --width 80 --height 40 --samples 4 --output /tmp/opencode/c3-before.png`.
  After the change, run the same command to `/tmp/opencode/c3-after.png`, compare the two files with
  `cmp` (identical bytes), open the PNG with the image reader and confirm the sphere over the sky and
  ground. The PPM path is unaffected and needs no second render. Agent generation: not affected.
- Slow gate (>2 minutes): none. The 80 by 40 render at 4 samples finishes in well under a second in
  release mode.

**User test (acceptance):**
- **Executor:** agent.
- Not a separate manual test: the rendered-image gate and the stderr observation below are the
  acceptance evidence.
1. Run `~/.cargo/bin/cargo build --release`, then `./target/release/toy-raytracer --width 80 --height
   40 --samples 4 --output /tmp/opencode/c3-accept.png 2>/tmp/opencode/c3-progress.err` from the
   worktree root.
2. Expect exit status 0, an empty standard output, and `/tmp/opencode/c3-accept.png` identical to the
   baseline from the rendered-image gate (`cmp /tmp/opencode/c3-before.png
   /tmp/opencode/c3-accept.png`).
3. Read `/tmp/opencode/c3-progress.err` and expect the progress updates `1 of 40 rows` through
   `40 of 40 rows`, separated by carriage returns, and a final newline. The capture is not empty and
   its last line is terminated.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a (standard) are satisfied.
- [ ] A render writes its row share to standard error, overwriting one line, and finishes that line
      with a newline.
- [ ] The image bytes for `--width 80 --height 40 --samples 4` match the pre-change render, and the
      PNG opens showing the sphere over the sky and ground.
- [ ] Standard output stays empty and `render`'s signature and return value are what C1 left.
- [ ] Named tests pass, including the adapted `tests/cli.rs` test and the two unit tests.
- [ ] Test budget respected: 0 test functions added, 1 changed; the receipt reports those counts.
      Any line counts are observations, not gates.
- [ ] Fast phase gate and the rendered-image conditional gate from
      `AGENT-INSTRUCTIONS/BUILDING/VERIFICATION-COMMITS-DEPLOY.md` §1 pass; no slow gate applies.
- [ ] No `docs/` page is stale: `docs/requirements.md` item 13 matches the behaviour and the README
      needs no change. `AGENT-INSTRUCTIONS/LOCAL/gates.md` is unchanged.
- [ ] Safety commit on branch `plan-c`.
- [ ] Revision-bound verification receipt exists; the agent-run `User test` steps have passing
      evidence.
- [ ] On passing evidence: compress this file into `done/C3-progress-line.md` (frozen contract only)
      and flip the `full-plan.md` status row to `done`.

**Exit:** a render reports its row share on standard error while it runs, the finished line ends with
a newline, and the image bytes for the same options are identical to the pre-change render.
