## Phase C2 - Command line (done)

**Status:** done.
**Gate level:** standard.

**Frozen contract:**
- `src/main.rs` parses `--width N`, `--height N`, `--samples N`, `--output PATH` and `--format png|ppm` by hand through `Config::parse` over the arguments after the program name. No dependency, no `--help`, no subcommand and no configuration file exist.
- The defaults are 400 by 200, 16 samples, `render.png` and PNG. An explicit `--format` wins. Without it the format comes from the output path's extension, and an extension that is neither `png` nor `ppm` is rejected.
- A missing value, a non-numeric number, an unknown option (a bare argument included), a width or height below 2, a sample count below 1 and an output path with no `png`/`ppm` extension each print the message and the `usage:` line on standard error and exit with status 2. Duplicate options are accepted and the last one wins.
- `render(width, height, samples)` returns every pixel colour row by row from the top, averaging `samples` traces of one deterministic camera ray per pixel with the `(width - 1)`/`(height - 1)` mapping unchanged.
- `main` opens the parsed output path once through one `io::BufWriter`, fills it with `png::write_png` or `write_ppm`, and flushes once so a buffered write error reaches the caller. `write_ppm` is live code with no `#[allow(dead_code)]`.
- `Cargo.toml` still has no dependency.
- `README.md` lists the five options with their defaults and the extension-derived format. `docs/requirements.md` item 12, `AGENT-INSTRUCTIONS/LOCAL/project.md` and `AGENT-INSTRUCTIONS/LOCAL/gates.md` describe the command line accurately, and the rendered-image gate command names its own output path.
- No source comment names a plan or a later phase.
- Three parser tests in `src/main.rs` pin the defaults, every option and unknown-option rejection, and the two render tests pass a sample count of 1.

**Evidence:** `cargo build`, `cargo test` (10 passed), `cargo clippy -- -D warnings`, `cargo fmt --check`, `git diff --check` and the rendered-image gate passed at `928af8e`. An 80 by 40 run with 4 samples wrote a PNG whose header decoded as `True 0000005000000028`. A 40 by 20 run without `--format` wrote a P3 file headed `40 20`. Nine bad-input cases each exited 2, printed a message and the usage line on standard error, and wrote no file. The default run in an empty directory wrote `render.png` at 400 by 200. `--format png` with an `x.ppm` output path wrote PNG bytes.

**Carry-overs:** none in `src/`. The root `README.md` shape is listed in `docs-pass.md` for the release.
