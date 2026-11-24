# Closing survey - output plan

- **Date:** 2026-11-24.
- **Revision surveyed:** `0ad0f2b` (branch `plan-c`), the whole plan's work as
  `e1293ef..0ad0f2b`: `README.md`, `src/main.rs`, `src/png.rs`, `tests/cli.rs`.
- **Method:** one independent read-only `ops-scout` per area, run in parallel (the PNG writer; the
  command line and settings; the render loop, progress line and writers' use; tests and
  documentation), then one independent read-only `ops-scout` cross-checking every finding claim by
  claim against the code and documents. This note is written by the plan orchestrator from the saved
  returns in `benchmarks/returns/`. Nothing was fixed during the survey.
- **Checks available to the scouts:** static reading, `git diff`, `git grep` and arithmetic. They did
  not build or run the product; the rendered-image evidence below was produced by the orchestrator on
  the same revision.

## Confirmed findings

| # | Area | Finding | Class | Evidence | Recommended handling |
|---|---|---|---|---|---|
| 1 | Tests | No test asserts that standard output is empty on a successful render, although the frozen contract requires it. | missing coverage of a stated contract | `done/C1-png-writer-and-command-line.md:18`, `done/C3-progress-line.md:17`; no `stdout` reference in `tests/cli.rs`; tests inspect only status, stderr and files (`tests/cli.rs:60-63,104-112,124-161`) | small fix: one assertion inside the existing PNG test |
| 2 | Tests | The rejected-input test accepts any non-zero exit; the contract fixes status 2. | missing coverage / weak assertion | `tests/cli.rs:104-107` (`!status.success()`); `src/main.rs:203` exits 2; contract at `done/C1-png-writer-and-command-line.md:14-15` | small fix: assert status 2 in the existing test |
| 3 | Tests | The test-side `crc32` and `adler32` are byte-identical copies of the writer's functions, so the CRC and Adler assertions are tautologies: a wrong polynomial or modulus copied into both places would pass. This leaves requirement 11's "correct CRC-32 and Adler-32" unproven, which this workspace names as a thing to test. | duplication with a weak oracle; missing independent proof | `tests/cli.rs:260-272` vs `src/png.rs:70-82`; `tests/cli.rs:274-283` vs `src/png.rs:85-94`; assertions at `tests/cli.rs:186,255`; mandate at `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md:35-36` | small fix: pin the helpers with published known-answer values (CRC-32 of `IEND` = `0xAE426082`, Adler-32 of `Wikipedia` = `0x11E60398`) inside the existing test |
| 4 | Tests | The PNG decoder asserts the writer's exact zlib header byte pair, which neither requirement 11 nor the frozen contract fixes. | test beyond the contract | `tests/cli.rs:236` asserts `[0x78, 0x01]`; requirement at `docs/requirements.md:34-36` | small fix: consume the two zlib header bytes instead of pinning their values |
| 5 | Command line | `parse_args` writes the five option names twice (a rejection match and a dispatch match) and needs an `unreachable!` arm because of it; the spellings can drift apart. Behaviour is correct today. | duplication / unnecessary complexity | `src/main.rs:119`, `:126-145`, `:146`; third listing in `USAGE` at `:28` | small fix: one match that keeps the "unknown option" answer ahead of the value fetch |
| 6 | Plan record | The release deferral for `AGENT-INSTRUCTIONS/LOCAL/gates.md` says to "keep the PNG command and the PPM command", but the only PPM command in that file is the fallback sentence the deferral deletes. | stale deferral wording | `docs-pass.md:8-9`; `AGENT-INSTRUCTIONS/LOCAL/gates.md:36` (PNG), `:39` (the fallback sentence) | fixed in the plan record by the orchestrator; the release item now gives the real PPM command |

## Findings recorded but not fixed

- **The multi-block stored-deflate path is never exercised by a test.** The 8 by 4 test PNG is 100
  bytes of filtered data and therefore one stored block; the non-final block branch and the 65535
  boundary only run at larger sizes (`src/png.rs:45-53`; `tests/cli.rs:47-60`). The default-size
  acceptance render below does exercise four blocks and opens correctly, and C1's test budget is
  fully used, so no test function is added. Recording only.
- **The PNG test decoder rejects any chunk other than `IHDR`/`IDAT`/`IEND`** (`tests/cli.rs:210`).
  The helper is deliberately documented as supporting exactly this writer's output, and the writer
  emits no ancillary chunk, so this is accepted as intentional rather than fixed.
- **The README no longer mentions that PPM remains available** (`README.md:5-10`), although
  requirement 11 keeps it. The README is minimal and its run example is correct; this is thin, not
  stale. Recorded only.

## Claims the cross-check corrected or rejected

No confirmed finding was rejected. The cross-check corrected these supporting points:

- Two area returns cited design rule 8 at `full-plan.md:58-61` (it is `:64-67`; `:58-61` is rule 5)
  and the fallback sentence at `gates.md:36` (it is `:39`).
- One return cited `src/png.rs:6` for the shared `to_byte` import; it is `:5`.
- One coverage note listed untested usage-line wording as a gap; it is not, because the phase
  forbids testing the wording (`full-plan.md` C1 "Must not").
- One return said the `unreachable!` arm is required for exhaustiveness; that is true only of the
  current two-match shape, and the arm disappears when the option list is matched once.
- One return said the unit tests print progress into `cargo test` output; libtest captures it, so it
  appears only with `--nocapture` or on failure.

## Out-of-scope observations

- `AGENT-INSTRUCTIONS/LOCAL/project.md:5` describes the `src/` map without `src/png.rs`, and
  describes `main.rs` as holding the output. Stale once this branch is merged.
- `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md:4-5` says `src/main.rs` holds "the output" and that
  every other module holds one concept; the PNG writer's arrival makes the first half imprecise.
- `#[allow(dead_code)]` appears in `src/main.rs:8`, `src/hittable.rs:10-11` and `src/vec3.rs:21-22`.
  All three predate this plan (`e1293ef`) and are the materials and geometry seams, not this plan's
  work.

## User-review list

- Nothing here needs a whole phase of its own, and no user-owned product decision is open.
- Optional, for the user to choose: a dedicated test for the multi-block PNG path would be a fourth
  test function in C1, which REPO-RULES §3a forbids without explicit approval.
- The two shared agent documents above (`project.md`, `repo-rules-domain.md`) are recorded for the
  release in `docs-pass.md`, because they are shared by every worktree and `src/png.rs` exists only
  on this branch until the merge.

## Rendered-image evidence (revision `0ad0f2b`)

```bash
~/.cargo/bin/cargo build --release
target/release/toy-raytracer --width 400 --height 200 --samples 16 --output /tmp/opencode/acceptance.png \
  > /tmp/opencode/acceptance.stdout 2> /tmp/opencode/acceptance.stderr
```

- Exit status 0; `/tmp/opencode/acceptance.png` is PNG data, 400 by 200, 8-bit/color RGB,
  non-interlaced, and opens showing the sphere over the blue-to-white sky and the green ground plane.
- Standard output capture: 0 bytes.
- Standard error capture: 783 bytes of row percentages separated by `\r`, ending `100%` plus one
  newline.
- The 400 by 200 default size is the multi-block PNG path (four stored deflate blocks).

## Survey conclusion

The plan's work is correct against requirements 11 to 13 and the frozen contracts, with no material
code defect. Six small defects are confirmed, four in test proof, one small duplication in the
argument parser and one in the plan's own release note; they are handled by a remediation round, and
the survey loop ends there because every change is small and checked. The plan is neither closed nor
archived here; the user's acceptance of this note and of the rendered result is still required.
