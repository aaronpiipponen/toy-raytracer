## Phase C3 - Progress line

**Status:** active.
**Depends on:** `done/C2-command-line.md`, which froze the command-line parser and the `render(width, height, samples)` contract.
**Scope:** the render loop in `src/main.rs` reports the share of rows done on standard error, overwriting one line, and the last update ends the line. The image bytes, the options, the scene, the camera, the sample averaging and the writers do not change.
**Gate level:** standard. The phase changes what the program prints while it runs, with no durable state, no external input and no trust boundary. The fast gate plus the rendered-image conditional gate, which observes the real standard error, prove it.
**Simplicity statement:** the progress line is one `eprint!` per rendered row with no flag to disable it, no rounding choice beyond integer truncation, no terminal detection and no stored history. `eprint!` writes to `io::stderr()`, which Rust does not buffer, so no flush, retry or recovery is added. A very small height still prints one update per row. No test captures or parses the stream: the conditional gate reads the standard error of the built binary directly.

**Current state (verified):**
- `src/main.rs:44` - the doc comment `/// Every pixel's colour, row by row from the top.` above `render`.
- `src/main.rs:45` - `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour> {`.
- `src/main.rs:46-48` - `let world = scene();`, `let camera = Camera::new(width as f64 / height as f64);`, `let mut pixels = Vec::with_capacity(width * height);`.
- `src/main.rs:49` - the row loop `for row in (0..height).rev() {`.
- `src/main.rs:50-59` - the column loop, the `(width - 1)`/`(height - 1)` mapping, the `samples` averaging loop and `pixels.push(colour / samples as f64);`.
- `src/main.rs:60-61` - the row loop's closing brace and `pixels`.
- `src/main.rs:174-196` - `main`, which parses the options, calls `render(config.width, config.height, config.samples)` and writes the chosen file; neither its options nor its two writers change.
- `src/main.rs:202-212` - `the_center_pixel_shows_the_sphere_facing_the_camera` and `the_top_row_is_sky`, which call `render(WIDTH, HEIGHT, 1)`; the test harness captures their standard error, so their assertions and names do not change.
- `README.md:18` - the sentence `` `cargo run` with no options writes `render.png` at 400 by 200 with 16 samples. ``, after which the progress sentence belongs.
- `docs/requirements.md:41` - item 13, `While rendering, a progress line on standard error shows the share of rows done.`, which this phase implements and leaves unchanged.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast phase gate and the rendered-image conditional gate, whose command writes `/tmp/opencode/render.png`.
- `Cargo.toml` - package `toy-raytracer`, an empty `[dependencies]` table.

**Read first (do not invent):**
- `src/main.rs` - the whole file, especially `render`, `main` and the test module; this phase edits only the row loop and one doc comment.
- `docs/requirements.md:32-41` - items 11 to 13 and 14.
- `README.md` - the page's present shape, so the addition is one sentence in it.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image conditional gate.
- `Cargo.toml` - the empty `[dependencies]` table this phase leaves untouched.

**Build plan:**

1. `src/main.rs:44` - extend the doc comment of `render` to state the side effect its signature does not show:
   ```rust
   /// Every pixel's colour, row by row from the top. A progress line on standard error reports the
   /// share of rows done.
   ```

2. `src/main.rs:49-61` - count the finished rows and print the share after each one. The row loop becomes:
   ```rust
   for (done, row) in (0..height).rev().enumerate() {
       // the column loop and the averaging loop stay exactly as they are
       eprint!("\r{:3}%", (done + 1) * 100 / height);
   }
   eprintln!();
   pixels
   ```
   Write the whole `render` function as:
   ```rust
   /// Every pixel's colour, row by row from the top. A progress line on standard error reports the
   /// share of rows done.
   fn render(width: usize, height: usize, samples: usize) -> Vec<Colour> {
       let world = scene();
       let camera = Camera::new(width as f64 / height as f64);
       let mut pixels = Vec::with_capacity(width * height);
       for (done, row) in (0..height).rev().enumerate() {
           for column in 0..width {
               let u = column as f64 / (width - 1) as f64;
               let v = row as f64 / (height - 1) as f64;
               let ray = camera.ray(u, v);
               let mut colour = Colour::new(0.0, 0.0, 0.0);
               for _ in 0..samples {
                   colour = colour + ray_colour(&ray, &world);
               }
               pixels.push(colour / samples as f64);
           }
           eprint!("\r{:3}%", (done + 1) * 100 / height);
       }
       eprintln!();
       pixels
   }
   ```
   The `\r` returns the cursor to the start of the line, so each update replaces the previous one. The trailing `eprintln!()` ends the progress line once, after the last row, so later output starts on a fresh line. The percentage truncates: `height` rows complete the share `(done + 1) * 100 / height`, which reaches 100 on the last row.

3. `README.md:18-19` - add one sentence between the no-options sentence and the `docs/requirements.md` line, keeping the page's shape:
   ```markdown
   While rendering, a progress line on standard error shows the share of rows done.
   ```

**Callers / wiring to update:**
- `src/main.rs:183` - `main` calls `render` unchanged: same arguments, same return value, no error path added for the progress output.
- `src/main.rs:204` and `src/main.rs:211` - the two render tests keep their calls and assertions; the harness captures the progress writes.
- No other file calls `render`: the only calls are in `main` and the test module.
- No signature, import, module or `Cargo.toml` change.

**Config / schema / migrations:**
- None. The progress line has no setting, no environment variable and no flag.
- Retained records and their named consumer/owner: none. The progress text is written once per row and is not stored.

**Touched files:**
<!-- touched-files:start -->
- `src/main.rs` - changed: `render` counts finished rows, writes the share to standard error with `eprint!`, ends the line with `eprintln!`, and its doc comment names the progress line.
- `README.md` - changed: one sentence that the program shows the share of rows done on standard error while rendering.
- Adds: nothing (no dependency, no configuration key, no generated file).
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not write anything to standard output. Only standard error carries the progress line, so a caller redirecting standard output sees nothing.
- Do not add a flag, environment variable or setting that disables or changes the progress line, and do not detect whether standard error is a terminal. Requirement 13 asks for the line unconditionally.
- Do not change `main`, `Config`/`Format`, `parse_value`, `parse_number`, `usage`, `to_byte`, `write_ppm`, `png::write_png`, `scene`, `ray_colour`, `Camera`, the column loop, the pixel mapping or the averaging loop. The image bytes for the same options stay identical.
- Do not add a test that captures, splits or asserts the standard-error text, and do not extract a percentage formatter helper that exists only to be tested. The conditional gate observes the real stream.
- Do not add a dependency to `Cargo.toml`, `unsafe`, threads or seeded randomness.
- Do not print progress from `main` around the `render` call. The single loop in `render` owns it.
- Do not mention a plan, a phase or a future change in the code, the doc comment, the README or a test name.
- Do not rename the existing tests or change their assertions.

**Tests:**
- New/changed/deleted: none. The progress line has no return value a unit test can read, and capturing the standard error inside a test would need a subprocess or a new seam for no extra confidence beyond the conditional gate. The existing tests keep covering the render result, and REPO-RULES §3a allows zero new functions when none is justified.
- Test budget: 0 new test functions, 0 changed, 0 deleted.
- Fast gate, run in this order: `cargo test`, `cargo build`, `cargo clippy -- -D warnings`, `cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image, required because the render loop changed.
  1. `cargo build --release`, then run the binary directly so cargo's own messages stay out of the captured stream:
     ```bash
     ./target/release/toy-raytracer --width 20 --height 10 --samples 1 \
       --output /tmp/opencode/progress.ppm 2>/tmp/opencode/progress.err
     ```
     It exits 0, writes nothing to standard output, and writes `/tmp/opencode/progress.ppm` whose first two lines are `P3` and `20 10`. The progress file must contain ten carriage returns and end with a completed `100%` line:
     ```bash
     python3 -c "d=open('/tmp/opencode/progress.err','rb').read(); print(d.count(b'\r'), d.endswith(b'100%\n'))"
     ```
     which prints `10 True`. `cat -v /tmp/opencode/progress.err` shows the percentages separated by `^M`, so each update overwrote the previous line, and the line ends with a newline.
  2. `cargo run --release -- --width 80 --height 40 --samples 4 --output /tmp/opencode/render.png` exits 0, shows the progress line on standard error, and writes `/tmp/opencode/render.png`. Check it with `python3 -c "d=open('/tmp/opencode/render.png','rb').read();print(d[:8]==bytes([137,80,78,71,13,10,26,10]), d[16:24].hex())"`, which must print `True 0000005000000028`, and open the file with the image reader: the sphere sits over the sky gradient at 80 by 40.
  3. Remove `/tmp/opencode/progress.ppm`, `/tmp/opencode/progress.err` and `/tmp/opencode/render.png` after the checks.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - automated phase gates prove acceptance. The conditional gate runs the built binary and reads the standard-error stream, its carriage-return count and its ending `100%` line, and the image file it writes.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `render` writes one overwriting progress update per rendered row to standard error and ends the line with a newline after the last row.
- [ ] The progress line reaches `100%` on the last row and the count of updates equals `height`.
- [ ] Standard output stays empty, and the image file for the same options is unchanged in content.
- [ ] No flag, setting, terminal detection or dependency is added, and `main` still calls `render` with the same arguments.
- [ ] `src/main.rs` still has the two render tests unchanged and no test asserts the standard-error text.
- [ ] `README.md` states the progress line on standard error in its existing shape and names no plan.
- [ ] `docs/requirements.md`, `AGENT-INSTRUCTIONS/LOCAL/documentation-pages.md` and `AGENT-INSTRUCTIONS/LOCAL/gates.md` were read and stay accurate, and none refers to a plan.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports 0 functions added, 0 deleted and 0 adapted.
- [ ] Fast phase gate and the rendered-image conditional gate pass, including the carriage-return count and the `100%` ending.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists with the progress-line observations.
- [ ] On passing evidence: compress this file into `done/C3-progress-line.md` (frozen contract only) and flip the `full-plan.md` status row to `done`.

**Exit:** the fixed scene renders at the parsed size and sample count while standard error carries one line that overwrites its percentage on every row and ends at `100%` with a newline, and the image file is written as before.
