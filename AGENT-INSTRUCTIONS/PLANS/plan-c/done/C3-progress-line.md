## Phase C3 - Progress line (done)

**Status:** done.
**Gate level:** standard.

**Frozen contract:**
- `render(width, height, samples)` in `src/main.rs` writes one progress update per rendered row to standard error with `eprint!("\r{:3}%", (done + 1) * 100 / height)`, and `eprintln!()` ends the line after the last row. Standard output stays empty.
- The carriage return overwrites one line, so a render of `height` rows writes `height` updates and the last states `100%`. The percentage truncates toward zero.
- No flag, environment variable, setting, terminal detection, flush, stored history or retry exists. `eprint!` writes the unbuffered standard error.
- `main`, `Config`, `Format`, `Config::parse`, `parse_value`, `parse_number`, `usage`, `write_ppm`, `png::write_png`, `scene`, `ray_colour`, the camera, the column loop, the pixel mapping, the averaging loop and both writers are unchanged. The image bytes for the same options are unchanged.
- `render`'s doc comment names the standard-error progress line.
- `README.md` states that the program shows the share of rows done on standard error while rendering.
- `docs/requirements.md` item 13 describes the progress line and stays unchanged.
- The five tests in `src/main.rs` and the five tests in the other modules are unchanged. No test captures or asserts the standard-error text.

**Evidence:** `cargo build`, `cargo test` (10 passed), `cargo clippy -- -D warnings`, `cargo fmt --check` and `git diff --check` passed at `525196b`. The rendered-image gate at 20 by 10 with 1 sample exited 0, wrote nothing to standard output, wrote a P3 file headed `20 10`, and its standard error held 10 carriage returns and ended `100%\n`. The 80 by 40 run with 4 samples wrote a PNG whose header decoded as `True 0000005000000028`, showed the sphere over the sky, and ended its standard error at `100%`. The 80 by 40 image is byte-identical to the same render at the phase base `928af8e`.

**Carry-overs:** none in `src/`. The release-time `README.md` shape item stays in `docs-pass.md`.
