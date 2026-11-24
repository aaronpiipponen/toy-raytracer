## Phase C1 - PNG writer and command line

**Status:** active.
**Depends on:** none (the first phase of this plan).
**Scope:** replace the PPM-on-stdout output with one resolved set of command-line settings and an
image file written as PNG or PPM. Add the dependency-free PNG writer. Give `render` the sample
count. Update the README run example. No progress line, no jitter and no scattering.
**Gate level:** standard - bounded runtime behaviour in one binary plus a new output format, with
the rendered-image conditional gate covering the format change. No durable state, no trust
boundary, no migration.
**Simplicity statement:** no `--help`, no `--output -`, no case-insensitive extension matching, no
duplicate-option rejection (the last value wins), no real compression, no ancillary chunks, no alpha,
palette or interlaced output, and no PNG decoder in product code. Every unusable option, value,
format or extension is one clear error on standard error plus a non-zero exit. The phase deletes the
`WIDTH` and `HEIGHT` constants and the stdout PPM call from `main`, and nothing else.

**Current state (verified):**
- `src/main.rs:1` - module header says the program writes the image to standard output as PPM.
- `src/main.rs:20-21` - `const WIDTH: usize = 200;` and `const HEIGHT: usize = 100;`.
- `src/main.rs:40` - `fn render(width: usize, height: usize) -> Vec<Colour>` builds the scene and
  camera, then pushes `ray_colour(&camera.ray(u, v), &world)` for each pixel, row by row from the
  top, with `u = column as f64 / (width - 1) as f64` and `v = row as f64 / (height - 1) as f64`.
- `src/main.rs:54` - `fn to_byte(channel: f64) -> u8` returns `(255.999 * channel.clamp(0.0, 1.0))
  as u8`.
- `src/main.rs:58-75` - `fn write_ppm(out: &mut impl Write, width: usize, height: usize, pixels:
  &[Colour]) -> io::Result<()>` writes `P3`, `{width} {height}`, `255`, then one `R G B` line per
  pixel through `to_byte`.
- `src/main.rs:77-80` - `fn main() -> io::Result<()>` calls `render(WIDTH, HEIGHT)` and
  `write_ppm(&mut io::stdout().lock(), WIDTH, HEIGHT, &pixels)`.
- `src/main.rs:82-98` - tests `the_center_pixel_shows_the_sphere_facing_the_camera` and
  `the_top_row_is_sky`, both calling `render(WIDTH, HEIGHT)`.
- `src/vec3.rs:5-6` - `pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }`;
  `src/vec3.rs:13` - `pub const fn new(x: f64, y: f64, z: f64) -> Self`;
  `src/vec3.rs:83` - `pub type Colour = Vec3;`. `Add` and `Div<f64>` are implemented at lines 40 and
  75, so averaging sums and divides with existing operators.
- `src/camera.rs:15` - `pub fn new(aspect_ratio: f64) -> Self`;
  `src/camera.rs:31` - `pub fn ray(&self, u: f64, v: f64) -> Ray`.
- `Cargo.toml` - package `toy-raytracer`, edition 2021, empty `[dependencies]`. The default binary
  target is `toy-raytracer`, from `src/main.rs`.
- There is no `src/png.rs` and no `tests/` directory.
- `.gitignore` ignores `/target`, `*.ppm` and `*.png`, so rendered output never enters a commit.
- `README.md:5-8` - the run example is `cargo run > image.ppm` followed by `cargo test`.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md:36` - the rendered-image command is already
  `cargo run --release -- --width 80 --height 40 --samples 4 --output /tmp/opencode/render.png`;
  line 39 still says "Until the command-line phase lands, `cargo run --release >
  /tmp/opencode/render.ppm` instead." This file is shared by every worktree and the phase does not
  edit it; its stale fallback sentence is a release item in the plan's `docs-pass.md`.

**Read first (do not invent):**
- `src/main.rs` - the whole file; every changed signature above is read from it.
- `src/vec3.rs` - `Vec3::new`, `Colour`, `Add` and `Div<f64>`, used by the sample average.
- `src/camera.rs` - `Camera::new(aspect_ratio)` and `Camera::ray(u, v)`, already correct for any
  width and height; do not change them.
- `docs/requirements.md` items 11 and 12 - PNG structure, the five options, the defaults and the
  error rule. Item 12 does not state which of `--format` and the extension wins, so the settled
  precedence (explicit `--format` wins) adds no contradiction and `docs/requirements.md` stays
  unchanged.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the rendered-image gate command this phase runs. The file
  is read only, never edited.
- `README.md` - the run example.
- `Cargo.toml` - confirmation that no dependency may be added.

**Build plan:**

1. `src/png.rs` (new file) - the writer, with this module header as its first line:
   `//! A PNG writer that stores scanlines in uncompressed deflate blocks.`
   - Public entry point:
     `pub fn write_png(out: &mut impl Write, width: usize, height: usize, pixels: &[Colour]) -> io::Result<()>`.
     It imports `std::io::{self, Write}`, `crate::to_byte` and `crate::vec3::Colour`.
   - Signature bytes: `[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]`.
   - `IHDR` data, 13 bytes in this order: `width as u32` big-endian, `height as u32` big-endian,
     bit depth `8`, colour type `2` (truecolour RGB), compression `0`, filter `0`, interlace `0`.
   - One `IDAT` chunk holding the zlib stream. zlib header bytes `0x78, 0x01`. Then stored deflate
     blocks over the raw scanline data: for each block, one byte `0x00` (not final) or `0x01`
     (final), the block length as `u16` little-endian, the ones' complement of that length as
     `u16` little-endian, then the block bytes. Block payloads are at most 65535 bytes. The final
     block ends the stream. The trailer is the Adler-32 of the whole raw scanline data, big-endian.
   - Raw scanline data: for each row from 0 to `height - 1`, push filter byte `0`, then for each
     column push `to_byte(colour.x)`, `to_byte(colour.y)`, `to_byte(colour.z)` of
     `pixels[row * width + column]`. `width` and `height` are at least 2 by the CLI validation, so
     the raw data is never empty and the writer needs no empty-input branch.
   - `IEND` chunk with empty data, written last.
   - Every chunk is written as: data length as `u32` big-endian, the 4 type bytes, the data, then
     the CRC-32 of the type bytes followed by the data, as `u32` big-endian.
   - CRC-32 is the standard reflected CRC with polynomial `0xEDB8_8320`, initial value
     `0xFFFF_FFFF` and final XOR `0xFFFF_FFFF`. Adler-32 uses modulus 65521 and returns
     `(b << 16) | a` with `a = 1` and `b = 0` initially. Both are private functions in this module.
     A per-byte bitwise CRC loop is enough for these image sizes; no table, no crate, no `unsafe`.
   - Do not validate `pixels.len()` against `width * height`; the single caller always passes
     exactly that many colours.

2. `src/main.rs` - module list and header:
   - Header becomes
     `//! Render a fixed scene and write it to a file as a PNG or PPM image.`
   - Add `mod png;` beside the other `mod` lines. Leave `mod rng;` and its `#[allow(dead_code)]`
     attribute unchanged.
   - Imports become `use std::fs;`, `use std::io::{self, Write};`, `use std::path::{Path, PathBuf};`
     in place of `use std::io::{self, Write};`. `std::env` is used by path, so no extra `use` is
     required, or add `use std::env;` (implementer's choice).

3. `src/main.rs` - defaults and settings, replacing `WIDTH` and `HEIGHT`:
   ```rust
   const DEFAULT_WIDTH: usize = 400;
   const DEFAULT_HEIGHT: usize = 200;
   const DEFAULT_SAMPLES: usize = 16;
   const DEFAULT_OUTPUT: &str = "render.png";

   const USAGE: &str = "usage: toy-raytracer [--width <pixels>] [--height <pixels>] [--samples <count>] [--output <path>] [--format png|ppm]";

   #[derive(Clone, Copy)]
   enum Format {
       Png,
       Ppm,
   }

   struct Settings {
       width: usize,
       height: usize,
       samples: usize,
       output: PathBuf,
       format: Format,
   }
   ```

4. `src/main.rs` - `fn parse_args(args: &[String]) -> Result<Settings, String>`, the whole command
   line resolved once before any rendering:
   - Start from the defaults through the four constants from step 3: `DEFAULT_WIDTH`,
     `DEFAULT_HEIGHT`, `DEFAULT_SAMPLES` and `PathBuf::from(DEFAULT_OUTPUT)`, with no explicit
     format (`None`). Those constants are the only place the default values appear; do not repeat
     the literals in `parse_args`.
   - Walk the arguments in order with an index. The recognized arguments are exactly `--width`,
     `--height`, `--samples`, `--output` and `--format`, each consuming the next argument as its
     value. A missing value, an argument that is not one of these five, and a positional argument
     are all `Err` with a short message naming the offending argument.
   - `--width` and `--height` parse as `usize` and must be at least 2. The render loop maps pixel
     centres with `width - 1` and `height - 1`, so smaller values are bad values. `--samples`
     parses as `usize` and must be at least 1; a pixel averages `samples` rays.
   - `--output` takes the next argument as a `PathBuf`, including an empty string. Nothing is
     validated about the path here; an unwritable path fails loudly in `run`.
   - `--format` accepts exactly `png` or `ppm`. Anything else is `Err`.
   - A repeated option replaces the earlier value; the last value wins.
   - After the loop, resolve the format with the fallback rule and return the `Settings`.

5. `src/main.rs` - `fn resolve_format(explicit: Option<Format>, output: &Path) -> Result<Format, String>`:
   - `Some(format)` returns that format unchanged, so an explicit `--format` wins over the
     extension.
   - Otherwise read `output.extension()`: `"png"` gives `Format::Png`, `"ppm"` gives `Format::Ppm`.
   - No extension, a non-UTF-8 extension, and every other extension are `Err` naming the path, so no
     format is guessed.

6. `src/main.rs` - `fn run(settings: &Settings) -> io::Result<()>`:
   - `let pixels = render(settings.width, settings.height, settings.samples);`
   - Build the image bytes in memory with `let mut bytes = Vec::new();` and `match settings.format`
     calling `png::write_png(&mut bytes, settings.width, settings.height, &pixels)?` for `Png` and
     `write_ppm(&mut bytes, settings.width, settings.height, &pixels)?` for `Ppm`. `Vec<u8>`
     implements `Write`, so `write_ppm` keeps its exact signature.
   - Write once with `fs::write(&settings.output, bytes)`. The returned `io::Result` carries an
     unwritable path, a missing directory or a full disk; it is never swallowed.

7. `src/main.rs` - `fn main()`, replacing `fn main() -> io::Result<()>`:
   - Collect `std::env::args().skip(1).collect::<Vec<String>>()`, call `parse_args`, and on `Err`
     print `error: {message}` then `{USAGE}` to standard error and exit with status 2.
   - On `Ok(settings)`, call `run(&settings)`; on `Err` print `error: {error}` to standard error and
     exit with status 1.
   - `main` returns `()`. No image bytes and no other text reach standard output.

8. `src/main.rs` - `render` gains the sample count:
   - Before: `fn render(width: usize, height: usize) -> Vec<Colour>`
   - After: `fn render(width: usize, height: usize, samples: usize) -> Vec<Colour>`
   - Inside the two loops, build the pixel's ray once (`let ray = camera.ray(u, v);`), start
     `let mut colour = Colour::new(0.0, 0.0, 0.0);`, add `ray_colour(&ray, &world)` in
     `for _ in 0..samples`, and push `colour / samples as f64`. The ray is identical for every
     sample because there is no jitter, so every sample count writes the same bytes at proportional
     cost. Do not add jitter, randomness, gamma or a scattering call. Update the existing doc
     comment to state that each pixel averages `samples` rays.

9. `src/main.rs` - `to_byte` becomes visible to the new module:
   - Before: `fn to_byte(channel: f64) -> u8`
   - After: `pub(crate) fn to_byte(channel: f64) -> u8`, body unchanged. `write_ppm` keeps calling it
     unchanged.

10. `tests/cli.rs` (new file) - the three public-behaviour tests and their helpers, described in
    `Tests` below. First line:
    `//! Command-line behaviour of the binary: output formats, PNG pixels and rejected input.`

11. `README.md` - replace the run example lines with the real command:
    ```sh
    cargo run --release -- --output image.png
    cargo test
    ```
    Leave the rest of the file unchanged.

12. `docs/requirements.md` - unchanged. Items 11 and 12 already describe this behaviour, and the
    precedence rule makes no stated requirement false.

**Callers / wiring to update:**
- `src/main.rs:78` - `main` is the only caller of `render`; it passes the resolved width, height and
  sample count instead of the deleted constants.
- `src/main.rs:79` - `main` stops calling `write_ppm` with `io::stdout()`; `run` calls it with a
  `Vec<u8>` for `--format ppm`, and `png::write_png` for PNG.
- `src/main.rs:88` and `src/main.rs:95` - the two existing tests call `render` with the new third
  argument.
- `src/main.rs:69-71` - `write_ppm` still calls `to_byte`; the visibility change needs no call-site
  edit.
- No other module calls `render`, `write_ppm` or `to_byte` (grep over `*.rs` finds only these).

**Config / schema / migrations:**
- No configuration file, no schema and no migration. The defaults and the five options are constants
  and code in `src/main.rs` only.
- Retained records and their named consumer/owner: none. The phase writes only the image file the
  user names, and keeps no receipt, log or copy of it.

**Touched files:**
<!-- touched-files:start -->
- `src/png.rs` - created: the dependency-free PNG writer, chunk framing, CRC-32 and Adler-32.
- `src/main.rs` - changed: module header, `mod png`, defaults, `Format`, `Settings`, `parse_args`,
  `resolve_format`, `run`, `main`, the `render` sample count and `to_byte` visibility.
- `tests/cli.rs` - created: three binary-level tests with their helpers.
- `README.md` - changed: the run example uses the real command line.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- No dependency, no build script, no bundled compressor and no real deflate compression. CRC-32,
  Adler-32 and the stored-block framing are written in `src/png.rs`.
- No `unsafe`, no threads and no new randomness. The sample loop reuses one ray, so the same options
  still give the same bytes.
- No second output path. `main` resolves once, `run` renders once and calls exactly one writer.
  `write_ppm` is not deleted or copied, and no `--output -` or stdout image path is added.
- No `allow(dead_code)` to park plumbing: `samples` reaches `render` because the loop uses it, and
  every new function and enum variant has a live caller.
- No option beyond the five requirements name, in particular no `--help`, no `--quiet` and no
  `--threads`.
- No jitter, scattering, material, gamma, camera or scene change; the materials plan owns them. Do
  not change `src/camera.rs`, `src/sphere.rs`, `src/hittable.rs`, `src/ray.rs`, `src/vec3.rs` or
  `src/rng.rs`.
- Do not test exact usage wording, error-message text or private parse helpers; exercise the public
  behaviour through the built binary in `tests/cli.rs`.
- Do not add a test dependency. Use only `std::process::Command`, `std::fs`, `std::env::temp_dir`
  and `env!("CARGO_BIN_EXE_toy-raytracer")`.
- Do not assert that standard error is empty on a successful render; the progress line lands on
  standard error in C3.
- No history, plan id or phase id in source comments or the README.
- Do not edit `AGENT-INSTRUCTIONS/LOCAL/gates.md`. Every worktree shares it, and its stale fallback
  sentence is a release item in the plan's `docs-pass.md`.
- Do not touch `docs/requirements.md`, `Cargo.toml`, `Cargo.lock`, `.gitignore` or any file under
  `AGENT-INSTRUCTIONS/`.

**Tests:**
- Changed: `src/main.rs` - `the_center_pixel_shows_the_sphere_facing_the_camera` and
  `the_top_row_is_sky`: call `render(200, 100, 16)` and keep reading the centre pixel and pixel 0
  from the returned pixels. No new assertions.
- New: `tests/cli.rs` - `writes_a_png_whose_pixels_match_the_ppm_render`: run the binary with
  `--width 8 --height 4 --samples 1 --output <dir>/match.png`, assert success, decode the file, and
  compare a few of its RGB triples with the same pixels of a second run using `--format ppm` and the
  same options: the first pixel, the centre pixel and the last pixel of the 8 by 4 image, which
  cover sky and sphere. Do not compare every pixel.
  The decoder asserts the PNG signature, walks the chunks and verifies each chunk CRC-32, requires
  one `IDAT`, checks `IHDR` is 8 by 4, bit depth 8, colour type 2, filter 0, interlace 0, checks the
  zlib header `0x78 0x01`, reads the stored blocks and verifies each `LEN`/`NLEN` pair and the
  final-block flag, matches the Adler-32 trailer, then checks every scanline starts with filter byte
  `0` and has `3 * width` colour bytes. This decoder supports exactly this writer's output and is a
  test helper, not product code.
- New: `tests/cli.rs` - `rejects_an_unknown_option_and_an_unparsable_value_with_a_non_zero_exit`:
  run the binary three times, each with `--output <dir>/rejected.png`: once as
  `--nonsense --output <dir>/rejected.png`, once as `--width wide --output <dir>/rejected.png`, and
  once as `--samples 0 --output <dir>/rejected.png`. Each run exits non-zero, writes something to
  standard error, and creates no `rejected.png`.
- New: `tests/cli.rs` - `selects_the_format_from_the_output_extension_and_lets_an_explicit_format_win`:
  `--output a.png` writes bytes starting with the PNG signature; `--output a.ppm` writes text
  starting with `P3`; `--format ppm --output b.png` writes text starting with `P3`; `--output a.jpg`
  with no `--format` exits non-zero and creates no file.
- Helpers in `tests/cli.rs` (not `#[test]` functions): `run(args: &[&str]) -> std::process::Output`
  around `Command::new(env!("CARGO_BIN_EXE_toy-raytracer"))`, a scratch-directory helper that creates
  `std::env::temp_dir().join(format!("toy-raytracer-{}-{name}", std::process::id()))` and removes it
  at the end of the test, a PNG decoder returning `(u32, u32, Vec<[u8; 3]>)`, and a P3 reader
  returning `Vec<[u8; 3]>`.
- Budget: 3 new test functions and 0 deleted, so 2 changed and 3 added; helper functions are not test
  functions. This is the whole budget; do not add a fourth test for any reason.
- Fast gate, run from the worktree (cargo is not on `PATH` here, so `~/.cargo/bin/cargo`):
  `~/.cargo/bin/cargo test --test cli`, `~/.cargo/bin/cargo test`, `~/.cargo/bin/cargo build`,
  `~/.cargo/bin/cargo clippy -- -D warnings`, `~/.cargo/bin/cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image, required because the render loop and the output format change.
  Run `~/.cargo/bin/cargo run --release -- --width 80 --height 40 --samples 4 --output
  /tmp/opencode/render.png`, open that PNG with the image reader and confirm it shows the sphere and
  the blue sky gradient at 80 by 40. Run the same command with `--output
  /tmp/opencode/render.ppm` and confirm the PPM opens at the same size. Agent generation: not
  affected.
- Slow gate (>2 minutes): none. The default 400 by 200 at 16 samples renders in well under a second
  in release mode.

**User test (acceptance):**
- **Executor:** agent.
- Not a separate manual test: the rendered-image gate above is the acceptance step.
1. Run `~/.cargo/bin/cargo run --release -- --width 80 --height 40 --samples 4 --output
   /tmp/opencode/accept.png`. Expect exit status 0, no output on standard output, and the file
   `/tmp/opencode/accept.png`.
2. Open `/tmp/opencode/accept.png` with the image reader. Expect the same 80 by 40 scene as the
   rendered-image gate: a sphere over a blue-to-white sky gradient, not a corrupt or blank image.
3. Run `~/.cargo/bin/cargo run --release -- --output /tmp/opencode/accept.jpg`. Expect a non-zero
   exit, a usage line on standard error and no `accept.jpg` file.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3 (standard) are satisfied.
- [ ] `cargo run --release -- --output <path>.png` writes a PNG that the image reader opens at the
      requested size, and `--format ppm` and a `.ppm` output still write P3 text.
- [ ] `--width`, `--height`, `--samples`, `--output` and `--format` all take effect, and an explicit
      `--format` wins over the extension.
- [ ] An unknown option, an unparsable or out-of-range value, an unrecognised format and a path
      whose extension is neither `png` nor `ppm` each print the usage line and exit non-zero.
- [ ] `render` averages `samples` identical rays with no jitter, and `Cargo.toml` still has no
      dependency.
- [ ] Named tests pass, including the three in `tests/cli.rs` and the two adapted tests in
      `src/main.rs`.
- [ ] Test budget respected: exactly 3 test functions added, 2 changed, 0 deleted; the receipt
      reports those counts. Any line counts are observations, not gates.
- [ ] Fast phase gate and the rendered-image conditional gate from
      `AGENT-INSTRUCTIONS/BUILDING/VERIFICATION-COMMITS-DEPLOY.md` §1 pass; no slow gate applies.
- [ ] `README.md` shows the real command line and refers to no plan.
- [ ] `AGENT-INSTRUCTIONS/LOCAL/gates.md` is unchanged.
- [ ] Safety commit on branch `plan-c`.
- [ ] Revision-bound verification receipt exists; the agent-run `User test` steps have passing
      evidence.
- [ ] On passing evidence: compress this file into `done/C1-png-writer-and-command-line.md` (frozen
      contract only) and flip the `full-plan.md` status row to `done`.

**Exit:** `cargo run` writes `render.png` at 400 by 200 from the default settings, the five options
are honoured, an explicit `--format` wins over the output extension, PPM still writes through
`--format ppm` or a `.ppm` path, a bad option or value prints a usage line and exits non-zero, and
standard output carries no image bytes.
