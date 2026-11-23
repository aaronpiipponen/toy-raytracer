## Phase C1 - PNG writer (done)

**Status:** done.
**Gate level:** standard.

**Frozen contract:**
- `src/png.rs` is the only PNG encoder. `pub fn write_png(out: &mut impl Write, width: usize, height: usize, rgb: &[u8]) -> io::Result<()>` writes `rgb` as an 8-bit RGB PNG with no interlacing. `rgb` holds `width * height * 3` bytes, three per pixel, row by row from the top.
- The file is the 8-byte PNG signature, one `IHDR` chunk (width and height big-endian `u32`, bit depth 8, colour type 2, deflate, adaptive filtering, no interlace), one `IDAT` chunk holding `zlib_stream(&scanlines(width, height, rgb))`, and an empty `IEND` chunk. `write_chunk` writes the big-endian data length, the four kind bytes, the data, then the big-endian CRC-32 of the kind and data.
- `scanlines` prefixes each row with filter byte 0. `zlib_stream` emits `0x78, 0x01`, then one stored deflate block per `raw.chunks(65535)` (last-block flag in the header byte, the 16-bit little-endian length and its complement, then the bytes), then the big-endian Adler-32 of `raw`. `crc32` uses polynomial `0xedb8_8320` bit by bit and `adler32` uses modulo `65521`; neither stores a lookup table.
- `src/main.rs` declares `mod png;` and `main` writes `render.png`: it renders `WIDTH` by `HEIGHT` (200 by 100), converts each `Colour` with `to_byte` into one `Vec<u8>` in row order, opens `render.png` with `File::create`, writes through `io::BufWriter`, and flushes the buffer so a write error reaches the caller.
- `write_ppm` stays in `src/main.rs` with its behavior and carries `#[allow(dead_code)]` because `main` does not call it.
- `Cargo.toml` still has no dependency.
- `README.md` runs `cargo run` and `cargo test`, and states `cargo run` writes `render.png`. `AGENT-INSTRUCTIONS/LOCAL/project.md` lists `png.rs` in the repository map. `AGENT-INSTRUCTIONS/LOCAL/gates.md` states that `cargo run --release` writes `render.png`; the option-driven rendered-image command stays for the command-line phase.

**Evidence:** `cargo build`, `cargo test` (7 passed), `cargo clippy -- -D warnings`, `cargo fmt --check`, `git diff --check` and the rendered-image gate passed at `30f857f`. `cargo run --release` exited 0, printed nothing on standard output and wrote `render.png`; the header decoded as `True 000000c800000064`, `file` reported PNG 200x100 8-bit RGB non-interlaced, and the image showed the sphere over the sky gradient. `render.png` was removed.

**Carry-overs for the next phase (no extra tests):**
- Delete the two plan-naming comments `// The origin of the bounce rays the materials plan adds.` at `src/hittable.rs:10` and `// For the triangle normals the geometry plan adds.` at `src/vec3.rs:21`, keeping their `#[allow(dead_code)]` attributes. Permanent code does not refer to a plan.
