# Area 1 return - the PNG writer (raw ops-scout return)

## CONFIRMED FINDINGS

**1. Duplication (test code), with a weak oracle — `tests/cli.rs` copies the writer's checksum functions byte-for-byte.**
The test-side `crc32` and `adler32` are the same algorithm as the writer's, token for token (only the `///` doc line differs), so the assertions at `tests/cli.rs:186` and `tests/cli.rs:255` cannot catch a wrong polynomial, wrong init/final value or wrong modulus — they only check that chunk assembly feeds the helpers the right bytes. Evidence:
- `tests/cli.rs:260-272` vs `src/png.rs:70-82` (`crc32`: identical `0xFFFF_FFFF` init, `0xEDB8_8320`, `wrapping_neg` mask, `!crc`).
- `tests/cli.rs:274-283` vs `src/png.rs:85-94` (`adler32`: identical modulus `65_521`, `a`, `b` update, `(b << 16) | a`). A `diff` of the two regions shows no body difference.
- `tests/cli.rs:7` also repeats `src/png.rs:8`'s signature constant (unavoidable — integration tests cannot reach the binary's private module — so this part is fine).
This is low severity and test-only: a fixed known-answer vector (CRC-32 of `IEND` = `0xAE426082`, Adler-32 of `"Wikipedia"` = `0x11E60398`) would remove the tautology at no cost.

**2. Test coverage gap: the multi-block / non-final-block path is never exercised.**
`src/png.rs:45-53`'s loop distinguishes the final block (`blocks.peek().is_none()`) from non-final ones, but the only PNG test renders 8×4 (`tests/cli.rs:47-56`, `--width 8 --height 4`), whose filtered stream is 4 × (1+3·8) = 100 bytes and therefore one stored block. The non-final branch and the 65 535 boundary only run with the real default 400×200 (240 200 raw bytes → 4 blocks), which the tests do not execute. Evidence: `src/png.rs:11,45-53`; `tests/cli.rs:47-60`; `tests/cli.rs:218-222` (the test only asserts total scanline length, not block count).

## Checks that came out clean (no finding)

- **Requirement 11 (`docs/requirements.md:34-36`) is met exactly:** one `IDAT` written once at `src/png.rs:56`; stored-block framing at `src/png.rs:44-54`; per-chunk length/type/data/CRC at `src/png.rs:62-67`; filter byte 0 per scanline at `src/png.rs:34`; IHDR 13 bytes in spec order at `src/png.rs:22-30`; IEND at `src/png.rs:57`; no crate (`Cargo.toml` has an empty `[dependencies]`), only `std::io::Write` (`src/png.rs:3`).
- **The checksums and framing are actually correct, not just self-consistent.** Reproducing `src/png.rs`'s two helpers and the zlib/block construction externally: CRC-32 matches `zlib.crc32` on `IEND`, `IHDR` and `Wikipedia`; Adler-32 matches `zlib.adler32`; and the full byte stream built by the same rules (including a 2-block 76 800-byte case) decompresses cleanly with `zlib.decompress`. The zlib header `0x78 0x01` at `src/png.rs:44` is a legitimately framed header ((0x7801) % 31 == 0).
- **Block length/complement:** `!length` on `u16` at `src/png.rs:51` and `MAX_STORED_BLOCK = 65_535` at `src/png.rs:11` are correct; `BFINAL`/`BTYPE` in a single byte at `src/png.rs:48` is valid because the remaining bits are zero padding.
- **Single live caller:** `write_png` is defined once and called once, at `src/main.rs:190`; no other PNG path exists (`grep` for `IHDR|IDAT|IEND|write_png` outside `src/png.rs` finds only `tests/cli.rs` and that call).
- **No dead code or unnecessary complexity in `src/png.rs`:** all five items are used; there is zero validation or speculative error handling; the module has one concept and opens with a `//!` line (`src/png.rs:1`), satisfying design rule 7.
- **Doc/comment accuracy:** every doc comment matches behaviour (`src/png.rs:10,13,61,69,84`); no drift.
- **Unchecked `u32` casts at `src/png.rs:23-24` are not a defect in practice:** `main.rs:187` allocates `width*height` `Colour`s before the writer runs, so a dimension ≥ 2³² aborts on allocation first and can never reach the cast. Not a finding.

## OUT-OF-SCOPE OBSERVATIONS

- **Stale documentation:** `AGENT-INSTRUCTIONS/LOCAL/project.md:5` still describes the `src/` map without `src/png.rs` (and describes `main.rs` only as scene/render/output/command line). This plan added the module, so the map is out of date. (Area 4's territory; reported with evidence, not investigated further.)
- **Pre-existing `#[allow(dead_code)]`:** `src/main.rs:8` sits before `mod rng;` and predates this plan (present at `e1293ef:7`); the same attribute appears at `src/hittable.rs:11` and `src/vec3.rs:22`. Design rule 4 forbids `allow(dead_code)` to paper over plumbing, but these are pre-existing, not introduced in `e1293ef..0ad0f2b`.

## AREA VERDICT

Clean. The PNG writer satisfies requirement 11 and design rules 1 and 7, is correct at the byte level, has a single caller and no dead code. The only real items are two low-severity test-side notes: the checksum helpers in `tests/cli.rs` are bit-identical copies of the writer's (weak, self-confirming oracle) and the multi-block branch has no test.

NOT VERIFIED: `cargo clippy -- -D warnings` / `cargo fmt --check` / the rendered-image gate, because a scout may not build or run gates.
