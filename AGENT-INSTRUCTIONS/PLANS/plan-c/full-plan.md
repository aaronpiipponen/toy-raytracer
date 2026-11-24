# Output Plan

> **Status:** scoped forward plan, revised 2026-11-24 on branch `plan-c`.
> **Scope:** how the program is run and what it writes: the PNG writer, the command line and the
> progress line (`docs/requirements.md` items 11 to 13).
> **Out of scope:** materials, sampling, jitter and scattering (the materials plan); shapes, hit
> data and bounding boxes (the geometry plan).
> **Target:** a correct, small hobby raytracer. Every phase here is product work; no phase is an
> experiment.
>
> **How to use this plan:** expand exactly one phase into an implementation-grade phase from
> current code before coding. Discuss and edit phase boundaries here first.

## Phase status

| Phase | Title | Status | Gate level | Depends on | Phase file | Exit state | Move-on gate |
|---|---|---|---|---|---|---|---|
| C1 | PNG writer and command line | active | standard | - | - | the program writes a real PNG at the chosen size, path and format from the command line | fast gate green, the written PNG opens, PPM still writes |
| C2 | (merged into C1) | not applicable | - | - | - | - | - |
| C3 | Progress line | planned | minimal | C1 | - | rendering shows the share of rows done on standard error | fast gate green, progress observed, image unchanged |
| Z1 | Closing survey and user acceptance | planned | minimal | C1 + C3 | - | the closing survey note is written and the user accepts the result | the user accepts |

C2 was folded into C1 so that PPM never becomes unreachable code and the format choice exists in
the same change that makes `render.png` the default.

## How to implement this plan

1. Read `docs/requirements.md` items 11 to 13, the README, `AGENT-INSTRUCTIONS/LOCAL/gates.md` and
   the code the selected phase touches before detailing it.
2. Never invent a name, signature, path or option. If something the phase names does not exist,
   stop and report drift.
3. One phase at a time. Commits whenever useful, no squashing, never on `main`.
4. Product code: no dependency, no hardcoded second path, no `allow(dead_code)` to paper over
   plumbing. `Cargo.toml` stays dependency-free.
5. Run the fast gate in `AGENT-INSTRUCTIONS/LOCAL/gates.md`, plus the rendered-image conditional
   gate whenever the render loop or the output format changes.
6. Fail loudly: a bad option, a bad value or an unwritable output path reports and exits non-zero;
   no silent fallback format or path.
7. Update the README and `gates.md` in the phase that makes their usage lines wrong. A genuine
   deferral to the release goes in `docs-pass.md`.

## Binding design rules

1. No dependency. The PNG is built from stored (uncompressed) deflate blocks, with CRC-32 per chunk
   and Adler-32 in the zlib trailer computed in the writer itself. No crate, no build script, no
   bundled compressor.
2. The PNG writer lives in `src/png.rs`. The PPM writer stays `write_ppm` in `src/main.rs`. `main`
   resolves the options and calls exactly one of them; there is no third output path.
3. Options resolve once into one set of settings before rendering. Defaults are 400 by 200, 16
   samples, `render.png`, and the format from the output file's extension. An explicit `--format`
   wins over the extension. An unknown option, a value that does not parse, an unrecognised format
   or an unrecognised output extension prints a usage line and exits non-zero.
4. `--samples` is accepted and validated here and passed into the render loop as a parameter; each
   pixel averages that many rays. The rays are not jittered yet, because scattering belongs to the
   materials plan, so today every sample count produces the same bytes at `samples` times the cost.
   That is the honest intermediate state: this plan must not add jitter or scattering, and the
   materials plan fills it in. The render signature is the seam the materials plan consumes.
5. Standard output stays clean: the image goes only to the output file and the progress line only to
   standard error. No image bytes on stdout.
6. Rendering stays reproducible: the same options give the same bytes. No new randomness, no time,
   no environment-dependent output.
7. Single-threaded, no `unsafe`, one concept per module, shared types in the module that defines the
   concept. Every module starts with a `//!` line.
8. Docs follow the change: the README run example and the rendered-image fallback line in
   `AGENT-INSTRUCTIONS/LOCAL/gates.md` are updated by the phase that makes them wrong.

## Starting state

- `src/main.rs` renders a fixed 200 by 100 through `render(width, height)` and writes ASCII PPM
  (P3, maximum 255) to standard output through `write_ppm(out, width, height, pixels)`; `to_byte`
  clamps and scales a channel. There is no option parsing and `std::env::args` is unused.
- Two tests in `src/main.rs` pin the centre pixel and the top row from `render(200, 100)`.
- `README.md` shows `cargo run > image.ppm`. `AGENT-INSTRUCTIONS/LOCAL/gates.md` carries the same
  fallback ("until the command-line phase lands, `cargo run --release > render.ppm`").
- There is no `src/png.rs` and `Cargo.toml` has no dependency.
- Known gaps: no PNG writer, no command line, no progress line, and the rendered size is 200 by 100
  rather than the required 400 by 200.

What later phases inherit:

- `render` gains the sample count; `write_ppm` stays reachable through `--format ppm`.
- The output file is the only place the image goes; stdout carries nothing but what the user asks
  for.
- One resolved settings value per render; later phases add behaviour behind it, not a second parse.

## Scoped phases

### C1 - PNG writer and command line

**Status:** active.
**Depends on:** none (the first phase).
**Scope:** the program stops writing PPM to standard output and instead resolves a small set of
command-line options and writes the image to a file as PNG or PPM. This is the phase that makes the
output side of the product match requirements 11 and 12.
**Expected gate level:** standard.

**Starting state / reason:**
- The image only exists as PPM on stdout; there is no file, no size choice and no PNG.
- Requirement 11 wants a dependency-free PNG, requirement 12 the five options and their defaults.
- PPM must stay available, which is exactly why the format choice lands together with the PNG
  writer: switching the default to `render.png` on its own would leave `write_ppm` uncalled.

**Likely areas to touch:**
- `src/png.rs` - new module with the writer.
- `src/main.rs` - option parsing, the resolved settings, the render signature, `main`.
- `README.md` - the run example.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the stale rendered-image fallback line.
- `docs/requirements.md` - only if the change contradicts a stated behaviour; otherwise leave it.

**TODOs:**
1. Add `src/png.rs`: a writer taking the width, the height, the pixels and a `Write`, producing the
   signature, `IHDR` (8-bit RGB, no interlace), one `IDAT` with zlib-wrapped stored deflate blocks
   of at most 65535 bytes, `IEND`, a CRC-32 per chunk, a filter byte of 0 per scanline and the
   Adler-32 trailer.
2. Parse the command line by hand into one resolved settings value: `--width`, `--height`,
   `--samples`, `--output <path>`, `--format png|ppm`, with the required defaults, explicit format
   winning over the extension, and a usage line plus non-zero exit for anything bad.
3. Make `main` render at the chosen size with the chosen sample count and write the chosen format to
   the chosen path, reporting an unwritable path instead of panicking.
4. Give `render` the sample count and average that many rays per pixel; no jitter.
5. Update the README run example and the `gates.md` rendered-image command so both show the real
   command line.
6. Adapt the two existing render tests to the new signature and add at most three new test
   functions covering the public behaviour: the binary writes a file that is a valid PNG whose
   pixels match the PPM rendering of the same scene; a bad option or bad value prints usage and
   exits non-zero; the format follows the output extension and an explicit `--format` wins.

**Must not:**
- Add a dependency, a build script or any real deflate compression.
- Delete `write_ppm` or bypass it with a second PPM path.
- Silence unused plumbing with `allow(dead_code)`; the sample count must reach `render` because it is
  used there, not because it is parked.
- Add jitter, scattering, gamma or any materials work; the materials plan owns those.
- Change the scene contents or the camera.
- Test the exact usage wording or private parse helpers; test the public behaviour through the
  binary.
- Leave the README or the `gates.md` command stale.

**Verification shape:**
- Fast gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`,
  `git diff --check`.
- Rendered-image gate (this phase owns it): a small render through the real command line in both
  formats, the PNG opened and looked at, the PPM compared with the same-size render.
- Documentation reread of the two changed lines plus `git diff --check`.

**Exit:** `cargo run` writes `render.png`; size, sample count, output path and format are honoured;
PPM still writes through `--format ppm`; a bad option prints usage and exits non-zero.
**Move-on gate:** fast gate green, the written PNG opens and looks right, PPM still writes,
work committed.

### C3 - Progress line

**Status:** planned.
**Depends on:** C1 (it consumes the render loop and the resolved settings C1 leaves).
**Scope:** while rendering, the program reports the share of rows done on standard error, updating
one line instead of printing a new one per row.
**Expected gate level:** minimal.

**Starting state / reason:**
- Rendering is silent; at 400 by 200 with 16 samples it takes long enough that silence looks like a
  hang.

**Likely areas to touch:**
- `src/main.rs` - the render loop and its caller.

**TODOs:**
1. Report the row share on standard error as rows finish, overwriting one line.
2. Leave the finished line terminated so later output is not glued onto it.
3. Keep the image bytes exactly what they were for the same options.

**Must not:**
- Write progress to standard output or into the image file.
- Add a timing, throttling or verbosity option; the requirements ask for one line.
- Test the exact progress text or its refresh cadence.
- Change pixels, sampling or the scene.

**Verification shape:**
- Fast gate.
- Rendered-image gate (this phase owns it): a small render with the progress line observed on
  standard error, and the same image bytes as before the change for identical options.
- No separate acceptance test beyond the agent-run checks.

**Exit:** a render shows its row share on standard error while it runs.
**Move-on gate:** fast gate green, progress observed, image unchanged, work committed.

### Z1 - Closing survey and user acceptance

**Status:** planned.
**Depends on:** C1 + C3.
**Scope:** the binding closing survey of everything this plan added or changed, then the user's
acceptance of the result.
**Expected gate level:** minimal.

**Starting state / reason:**
- The plan's last phase is a read-only survey of the whole plan's work, not a fix round.

**Likely areas to touch:**
- `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/` - the dated survey note.
- Nothing else; findings are reported, not fixed.

**TODOs:**
1. One read-only `ops-scout` per area the plan added or changed, in parallel: the PNG writer, the
   command line and settings, the render loop and the progress line, and the docs this plan touched.
2. One independent read-only `ops-scout` cross-checking every finding, claim by claim, with evidence.
3. The orchestrator writes the dated note in `benchmarks/` from those returns.
4. Render the scene, compare it with requirements 11 to 13, and put the result to the user.

**Must not:**
- Fix anything inside this phase; findings go to the user.
- Skip the cross-check, merge areas into one scout, or write the note from memory.
- Change product code, docs or the plan's phase rows beyond flipping its own row.

**Verification shape:**
- The note exists with every finding backed by file:line evidence.
- The acceptance render is produced through the real command line.

**Exit:** the dated survey note is written and the user has accepted the result.
**Move-on gate:** the user accepts the survey and the result; the plan is then ready for the
release, which closes it.

## Carry-overs

- Writing the image to standard output (`--output -`) is not required and is not planned.
- The PNG is deliberately uncompressed stored blocks, as requirement 11 states; no size or speed
  optimisation is planned.
- Jittered sampling and gamma correction are the materials plan's, not this plan's.
- No deferred documentation items; the README and `gates.md` are updated in C1.
