# Area 3 return - the render loop, the progress line and the writers' use (raw ops-scout return)

## CONFIRMED FINDINGS

No material defects in this area. Two minor notes, neither blocking:

1. **Duplication (minor, cosmetic, no action needed)** — `parse_args` matches the option-name list twice: once to reject unknowns before consuming a value (`src/main.rs:118-121`) and again to dispatch (`src/main.rs:126-147`), with a third listing in `USAGE` (`src/main.rs:28`). The trailing `_ => unreachable!(...)` (`src/main.rs:146`) is required for exhaustiveness of a `&str` match, not a defensive re-check. The behaviour is correct: an unknown option errors as "unknown option" and a known option without a value errors as "needs a value" (`src/main.rs:122-124`), both exit 2 (`src/main.rs:203`).

2. **Coverage gap (minor, low severity)** — the "standard output stays empty on success" guarantee (design rule 5, `full-plan.md`; C1 contract "Standard output carries nothing on success", `done/C1-png-writer-and-command-line.md`) has no automated evidence: `grep -n "stdout\|stderr" tests/cli.rs` → only `tests/cli.rs:109-110`, an assertion that stderr is non-empty for *rejected* input. A future regression writing to stdout on success would not be caught. The code itself is clean (no `println!`/stdout writer in `src/`, `git grep "stdout\|println!"` → only `eprintln!` at `src/main.rs:80,201,202,207`). Not a violation of C3, which explicitly forbids adding a progress test (`done/C3-progress-line.md`, "No test function is added, changed or deleted"), and C1's mandated test list did not include stdout.

## Points checked and found correct, so not findings

- Progress stream and shape: `eprint!("\r{}%", ...)` per finished row inside the `for row` loop and one `eprintln!()` after it → one overwritten line on standard error, exactly one terminating newline, no newline per row (`src/main.rs:77-80`). Matches C3 verbatim (`(height - row) * 100 / height`, row counting down from `height - 1`).
- Percentage reaches 100 exactly once, at the last row: `row = 0` → `done = height` → `height * 100 / height == 100` (`src/main.rs:77-78`). First row reports `1 * 100 / height` (0 for the 200-row default), i.e. "rows finished", not an off-by-one.
- No flush or buffered writer around progress; no terminal detection, verbosity or timing option (`src/main.rs:78,80`), as frozen.
- Single output path: `run` renders once and selects exactly one writer into a `Vec<u8>`, then one `fs::write` (`src/main.rs:186-194`); PNG and PPM writers exist side by side with no third path; `to_byte` is shared `pub(crate)` (`src/main.rs:84`; `src/png.rs:6`). No unnecessary flush; in-memory `bytes` is needed by `fs::write` and is ~80 KB at defaults.
- Sampling seam: scene and camera built once per render (`src/main.rs:63-64`), `samples` is a real loop and a real divisor (`src/main.rs:72-75`) — used, not parked; signature matches C1. Rays unjittered, as rule 4 requires.
- Reproducibility and sample-count-independence: I re-implemented the exact f64 arithmetic of `render`/`ray_colour`/`Sphere` (including `Vec3::div` being `* (1.0/d)` and `HittableList` nearest-limit ordering) and evaluated all 80,000 pixels at 400×200: **0** byte differences in `to_byte` output for samples ∈ {1..64, 100, 255, 1000}. So the C1 claim "every sample count writes the same bytes" holds in practice today, and no `time`, env or randomness input exists in the loop.
- No image bytes on stdout, no progress into the image file, no `unsafe`, no threads (`git grep "unsafe\|thread::\|spawn\|dbg!" -- src/` → no hits).
- Doc/comment state: `src/main.rs:1` and the `render` doc comment (`src/main.rs:60-61`) match behaviour; README run example `cargo run --release -- --output image.png` (`README.md:6`) is real and correct; the `gates.md` fallback sentence is still present and still deferred to the release per `full-plan.md` rule 8, so it is not drift.
- C3's "no test changed" claim verified: `git show 0ad0f2b` touches only `src/main.rs` (5 insertions, 1 deletion), and the existing tests call `render(200, 100, 16)` unchanged (`src/main.rs:218,225`).

## OUT-OF-SCOPE OBSERVATIONS

- `#[allow(dead_code)]` appears three times, all present at the base revision `e1293ef` (`git grep -n "allow(dead_code)" e1293ef -- src/` → `src/main.rs:7`, `src/hittable.rs:11`, `src/vec3.rs:22`; today `src/main.rs:8-9`, `src/hittable.rs:10-11`, `src/vec3.rs:21-22`). Plan design rule 4 forbids `allow(dead_code)` "to paper over plumbing", but these are the rng/geometry seam placeholders owned by the materials and geometry plans, not this plan's work; `src/rng.rs` is entirely unused today.
- Requirements item 5 (jitter inside the pixel, gamma 2 correction) and item 6 (seeded generator) are unimplemented; this is the explicitly documented intermediate state (rule 4; C1 "Must not: Add jitter, scattering, gamma or any materials work"), not drift. `to_byte` clamps and scales linearly with no gamma (`src/main.rs:84-86`).
- PNG writer internals (stored-block chunking, CRC/Adler) are area 1's; `src/png.rs` is reachable only through `run` (`src/main.rs:190`) and indexes `pixels[row * width + column]` without a length check, which is safe because `render` always returns `width * height` (`src/main.rs:65`).
- Trailing-whitespace/whitespace gate: `git diff --check e1293ef..0ad0f2b` exits 0.

## AREA VERDICT

**Clean.** The render loop, progress line and image-writer selection match requirements item 13 and the frozen C1/C3 contracts and design rules 4-7; the same options give the same bytes and sample counts produce identical output today. Only the two minor notes above (double option-name match; no test pinning empty stdout on success) are worth recording, and neither warrants a code change in this survey.

**NOT VERIFIED:** the fast gate itself (`cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`) and the live rendered-image check with the progress line observed on a terminal — running them means building/running the product, which a read-only scout may not do. The progress behaviour is established from code inspection against the frozen contract instead, and `git diff --check` was run (clean).
