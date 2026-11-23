## Phase C1 - PNG writer

**Status:** active.
**Depends on:** none.
**Scope:** add `src/png.rs`, an encoder that writes an 8-bit RGB PNG built from a zlib stream of
stored deflate blocks, its CRC-32 and Adler-32 computed in this crate. Point `main` at
`render.png` so a run writes a PNG file instead of PPM on standard output. Everything else about
invocation stays as it is: the fixed scene, the fixed 200 by 100 size, no options, no progress
line. The PPM writer stays in `src/main.rs` and keeps its behavior.
**Gate level:** standard. The phase changes runtime output bytes and adds a module, with no
durable state, no external input, no credentials and no trust boundary. Targeted tests, the fast
gate and the rendered-image conditional gate prove it.
**Simplicity statement:** dimensions and pixel bytes come from the render loop and are never
zero, so the encoder adds no size validation, no empty-input branch and no recovery path. The
encoder always uses stored deflate blocks, the single format `docs/requirements.md` item 11
names, so no compressed-block path, no level choice and no fallback exist. A write error
propagates with `?`. Nothing is deleted: the PPM writer stays for the format option that selects
it, and it carries `#[allow(dead_code)]` because `main` no longer calls it.

**Current state (verified):**
- `src/main.rs:1` - module header `//! Render a fixed scene and write it to standard output as a PPM image.`
- `src/main.rs:3-10` - module list: `camera`, `hittable`, `ray`, `rng`, `sphere`, `vec3`. No `png`.
- `src/main.rs:12` - `use std::io::{self, Write};`.
- `src/main.rs:20-21` - `const WIDTH: usize = 200;` and `const HEIGHT: usize = 100;`.
- `src/main.rs:40-52` - `fn render(width: usize, height: usize) -> Vec<Colour>`, pixels row by row from the top.
- `src/main.rs:54-56` - `fn to_byte(channel: f64) -> u8`, `(255.999 * channel.clamp(0.0, 1.0)) as u8`.
- `src/main.rs:58-75` - `fn write_ppm(out: &mut impl Write, width: usize, height: usize, pixels: &[Colour]) -> io::Result<()>`, P3 header then one line per pixel.
- `src/main.rs:77-80` - `fn main() -> io::Result<()>` renders and calls `write_ppm(&mut io::stdout().lock(), WIDTH, HEIGHT, &pixels)`.
- `src/main.rs:82-98` - `mod tests` with `the_center_pixel_shows_the_sphere_facing_the_camera` and `the_top_row_is_sky`, both calling `render`.
- `src/vec3.rs:82-83` - `/// A colour is a vector of red, green and blue in 0..=1.` and `pub type Colour = Vec3;`.
- `Cargo.toml` - `[dependencies]` is empty.
- `README.md:5-8` - the run example is `cargo run > image.ppm` followed by `cargo test`.
- `.gitignore` - ignores `*.png`, so `render.png` is not committed.

**Read first (do not invent):**
- `src/main.rs` - `to_byte`, `write_ppm`, `main`, the module header and the `WIDTH`/`HEIGHT` constants, because this phase edits each of them.
- `src/vec3.rs:82-83` - `pub type Colour = Vec3;`, the pixel type `main` converts to bytes.
- `docs/requirements.md` items 11 and 14 - the PNG format this phase implements and the quality gate.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast phase gate and the rendered-image conditional gate.
- `Cargo.toml` - the empty `[dependencies]` table this phase leaves untouched.

**Build plan:**
1. `src/png.rs` (create) - module header `//! Encode an image as a PNG file.`, then
   `use std::io::{self, Write};`, then:
   - `const SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];`
   - `const MAX_STORED_BLOCK: usize = 65535;`
   - `pub fn write_png(out: &mut impl Write, width: usize, height: usize, rgb: &[u8]) -> io::Result<()>`
     with a doc comment stating that `rgb` holds `width * height * 3` bytes, three per pixel, row
     by row from the top, and that the PNG is 8-bit RGB with no interlacing. It writes the
     signature, the `IHDR` chunk, one `IDAT` chunk holding `zlib_stream(&scanlines(width, height, rgb))`
     and an empty `IEND` chunk, each through `write_chunk`.
   - `fn write_chunk(out: &mut impl Write, kind: &[u8; 4], data: &[u8]) -> io::Result<()>` writing
     the big-endian data length, the 4 kind bytes, the data, then the big-endian CRC-32 of the
     kind bytes followed by the data bytes.
   - `fn scanlines(width: usize, height: usize, rgb: &[u8]) -> Vec<u8>` returning, per row, one
     filter byte `0` and then `width * 3` pixel bytes, iterating `rgb.chunks_exact(width * 3)`
     and reserving `height * (width * 3 + 1)` bytes.
   - `fn zlib_stream(raw: &[u8]) -> Vec<u8>` returning `0x78, 0x01`, then one stored deflate
     block per `raw.chunks(MAX_STORED_BLOCK)`: the block header byte (`1` for the last block, `0`
     otherwise), the block length as 16-bit little-endian, the bitwise complement of that length
     as 16-bit little-endian, then the block bytes. After the blocks, the big-endian Adler-32 of
     `raw`. The `0x78, 0x01` pair is zlib's 32K window header with no preset dictionary and its
     check bits satisfied.
   - `fn crc32(parts: &[&[u8]]) -> u32` computing CRC-32 (polynomial `0xedb8_8320`) bit by bit:
     start at `0xffff_ffff`, for each byte xor it into the register and shift eight times,
     xoring the polynomial when the low bit is set, then invert the final register. No lookup
     table is stored.
   - `fn adler32(data: &[u8]) -> u32` with `a = 1`, `b = 0`, each byte adding into `a` and then
     `a` into `b`, both taken modulo `65521`, returning `(b << 16) | a`.
   - `write_png` builds `IHDR` as 13 bytes: `width as u32` big-endian, `height as u32`
     big-endian, then `8` (bit depth), `2` (colour type RGB), `0` (deflate), `0` (adaptive
     filtering), `0` (no interlace).
2. `src/main.rs` - add `mod png;` to the module list in alphabetical position (after
   `mod hittable;`). Change the module header to `//! Render a fixed scene and write it as a PNG image.`
3. `src/main.rs`, `fn main` - before -> after:
   `fn main() -> io::Result<()> { let pixels = render(WIDTH, HEIGHT); write_ppm(&mut io::stdout().lock(), WIDTH, HEIGHT, &pixels) }`
   becomes a function that renders, converts each pixel with `to_byte` into one `Vec<u8>` of
   `WIDTH * HEIGHT * 3` bytes in row order, opens `render.png` with `std::fs::File::create`, and
   writes through `io::BufWriter::new` with `png::write_png(&mut out, WIDTH, HEIGHT, &rgb)`.
   Signature stays `fn main() -> io::Result<()>`; add `use std::fs::File;` beside the `std::io`
   import. The conversion is one expression over `pixels`, for example
   `pixels.iter().flat_map(|p| [to_byte(p.x), to_byte(p.y), to_byte(p.z)]).collect()`; do not add
   a helper function for it.
4. `src/main.rs`, `fn write_ppm` - keep the body as it is and add `#[allow(dead_code)]` directly
   above it, because `main` stops calling it until the format option selects it. Write no
   comment about why it is unused.
5. `src/png.rs`, tests - add `#[cfg(test)] mod tests` with the two tests in the `Tests` section.
6. `README.md` - keep the file's present shape and change only the run example: the code fence
   becomes `cargo run` followed by `cargo test`, and the sentence before `docs/requirements.md`
   states that `cargo run` writes `render.png`. `AGENT-INSTRUCTIONS/LOCAL/documentation-pages.md`
   names `README.md` as the only page that carries the usage, and the guide named there gives the
   README no other page set to update. Load the `readme-guide` skill before editing and keep the
   documentation style wording.
7. `AGENT-INSTRUCTIONS/LOCAL/project.md` - extend the repository map line in
   `## Project sources of truth` with `png.rs` (the PNG encoding). Load the `agent-instructions`
   skill before editing this file, because the edit is outside `PLANS/`.

**Callers / wiring to update:**
- `src/main.rs:77` - `fn main` is the only caller of `write_ppm` and the only new caller of
  `png::write_png`; it changes as step 3.
- `src/main.rs:3-10` - the module list gains `mod png;`, the only wiring point for the new module.
- `src/main.rs:64-72` - `write_ppm` keeps calling `to_byte`; `to_byte` stays private in `main.rs`
  and gains one new call site in `main`.

**Config / schema / migrations:**
- None. No configuration key, no schema, no migration, no generated data file. `Cargo.toml` is
  unchanged.
- Retained records and their named consumer/owner: none. The encoder writes the image file and
  keeps no other record.

**Touched files:**
<!-- touched-files:start -->
- `src/png.rs` - created: the PNG encoder, its CRC-32 and Adler-32 helpers, and its two tests.
- `src/main.rs` - changed: module list, module header, `main` writes `render.png`, `write_ppm` marked unused.
- `README.md` - changed: the run example writes `render.png`.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` - changed: the repository map lists `png.rs`.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add a dependency to `Cargo.toml` or call any crate outside `std`. The PNG, the zlib
  stream, the CRC-32 and the Adler-32 are computed in `src/png.rs`.
- Do not read or write anything outside `render.png`, and do not keep `render.png` in Git (it is
  ignored).
- Do not add command-line parsing, `--width`, `--height`, `--samples`, `--output` or `--format`,
  a progress line, a usage string or an option-driven format choice. Those belong to later
  phases.
- Do not delete, weaken or duplicate `write_ppm`, and do not route PNG through it or PPM through
  `src/png.rs`. There is one writer per format and `main` calls one of them.
- Do not change the render output values, `render`, `ray_colour`, `scene`, `to_byte` or the two
  existing tests.
- Do not add a second compression path, a compression level choice, a lookup table, a retry or a
  fallback for a write error. A write error returns from `main` with `?`.
- Do not validate, clamp or reject dimensions and lengths. `render` supplies the bytes and the
  size.
- Do not restructure `README.md`, add new sections to it, or change `docs/requirements.md`; the
  run example is the only README text this phase changes.
- Do not mention plans, phases or future work in source comments, module headers, the README or
  test names. Comments state what the code does now.
- Single-threaded, no `unsafe`.

**Tests:**
- New/changed/deleted: `src/png.rs`, `a_two_pixel_image_is_a_valid_png`: `write_png(&mut bytes, 2, 1, &[255, 0, 0, 0, 127, 255])` into a `Vec<u8>` and assert the bytes equal the expected 75-byte
  file below. This proves the signature, the `IHDR` values (width 2, height 1, bit depth 8,
  colour type 2, deflate, adaptive filtering, no interlace), the chunk layout, every chunk CRC-32,
  the stored deflate block, the zlib header, the big-endian Adler-32 and the empty `IEND`.
  Expected bytes, one chunk per line group:
  ```text
  0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a,
  0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
  0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01,
  0x08, 0x02, 0x00, 0x00, 0x00, 0x7b, 0x40, 0xe8,
  0xdd, 0x00, 0x00, 0x00, 0x12, 0x49, 0x44, 0x41,
  0x54, 0x78, 0x01, 0x01, 0x07, 0x00, 0xf8, 0xff,
  0x00, 0xff, 0x00, 0x00, 0x00, 0x7f, 0xff, 0x07,
  0xfe, 0x02, 0x7e, 0xa1, 0x8a, 0xbb, 0x7c, 0x00,
  0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae,
  0x42, 0x60, 0x82,
  ```
- New/changed/deleted: `src/png.rs`, `a_tall_image_splits_into_stored_blocks`: write a 200 by 110
  image whose pixel `i` is `(255, 0, 0)` when `i % 3 == 0`, `(0, 255, 0)` when `i % 3 == 1` and
  `(0, 0, 255)` otherwise, into a `Vec<u8>`, then parse the result. Assert the 8 signature bytes,
  an `IHDR` chunk of length 13 with width 200, height 110, bit depth 8 and colour type 2, exactly
  one `IDAT` chunk, and `IEND` as the last chunk with length 0. Inside the `IDAT` data assert the
  zlib header `0x78, 0x01`, then walk the stored blocks (header byte with the last-block flag,
  the 16-bit little-endian length, its complement, then the payload) and assert the block lengths
  are 65535 and 575 and that the concatenated payloads equal the expected scanlines built in the
  test as a filter byte `0` plus that row's pixel bytes per row. Do not compare CRCs or the
  Adler-32 here; the first test proves both.
- Existing tests: unchanged. `the_center_pixel_shows_the_sphere_facing_the_camera` and
  `the_top_row_is_sky` keep working because `render` is untouched.
- Test budget: 2 new test functions, within the 3 allowed, with none deleted.
- Fast gate: `cargo test` (both new tests plus the two existing ones), `cargo build`,
  `cargo clippy -- -D warnings`, `cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image, required because the output format changes. Sole execution
  owner is the implementer, once on the tested revision. Run `cargo run --release` in the
  worktree, confirm it exits 0, writes `render.png` and prints nothing on standard output, then
  inspect the file: `python3 -c "d=open('render.png','rb').read();print(d[:8]==bytes([137,80,78,71,13,10,26,10]), d[16:24].hex())"`
  must print `True 000000c800000064`, and opening `render.png` with the image reader must show the
  sphere over the sky gradient at 200 by 100. Remove `render.png` after the check. The
  command-line form of this gate waits for the phase that adds options.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - automated phase gates prove acceptance. The rendered-image conditional gate
  above already runs the program once and inspects the written file, so a separate acceptance run
  would repeat the same command and the same observation.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied for `standard`.
- [ ] `src/png.rs` exists, `main` writes `render.png`, and `cargo run --release` produces no PPM on
  standard output.
- [ ] The written file is a PNG that an external decoder accepts: the signature, `IHDR`, `IDAT`
  and `IEND` chunks parse and the `IDAT` zlib stream decompresses to the filter-prefixed pixel
  rows.
- [ ] `write_ppm` is still present, unchanged in behavior, and `main` no longer calls it.
- [ ] `Cargo.toml` has no dependency and `git diff` shows no change to it.
- [ ] No option parsing, progress line or format selection is present.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports 2 functions added and 0 deleted.
- [ ] Fast phase gate and the rendered-image conditional gate pass.
- [ ] `README.md` states the run writes `render.png`, and the repository map in
  `AGENT-INSTRUCTIONS/LOCAL/project.md` lists `png.rs`. No named page is stale and none refers to
  a plan.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the rendered-image gate has passing evidence on
  the tested head, including the decoded dimensions and the image inspection.
- [ ] On passing evidence: compress this file into `done/C1-png-writer.md` (frozen contract only)
  and flip the `full-plan.md` status row to `done`.

**Exit:** `cargo run` writes `render.png`, a valid 8-bit RGB PNG of the fixed scene, built without
any dependency, and the PPM writer remains in place for the format option.
