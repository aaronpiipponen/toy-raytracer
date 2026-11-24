# Area 4 return - tests and documentation (raw ops-scout return)

## CONFIRMED FINDINGS

**1. Missing coverage: "standard output carries nothing on success" is never asserted.**
- Class: missing coverage of public behaviour.
- Evidence: the frozen contract requires it — `AGENT-INSTRUCTIONS/PLANS/plan-c/done/C1-png-writer-and-command-line.md:18` ("Standard output carries nothing on success"), `done/C3-progress-line.md:17`, and binding design rule `full-plan.md:58`.
- Evidence: no test reads `Output::stdout` at all. `grep -n "stdout" tests/cli.rs` → no matches; the three tests only inspect `status`, `stderr` and files (`tests/cli.rs:60-63`, `104-112`, `124-161`).
- This is precisely the regression C3 could introduce (progress accidentally written to stdout instead of stderr) and it would pass every current test. The cap is on *test functions*, not assertions, so adding `assert!(run.stdout.is_empty())` inside test 3 or test 1 costs no extra function.

**2. Missing coverage: the rejected-input test accepts any non-zero exit, but the contract fixes status 2.**
- Class: missing coverage / weak assertion.
- Evidence: contract says "exit with status 2" — `done/C1-png-writer-and-command-line.md:14-15`; `src/main.rs:203` does `exit(2)`.
- Evidence: the test only asserts `!result.status.success()` — `tests/cli.rs:103-107`. An `exit(1)` from a `run` path or a panic would pass. `result.status.code() == Some(2)` is a free assertion in the existing test.

**3. Test helpers duplicate the product's checksum algorithms verbatim, so they cannot verify checksum correctness.**
- Class: duplication (test support re-implements product behaviour, and does so by copy).
- Evidence: `tests/cli.rs:260-272` `crc32` is byte-for-byte the product's `src/png.rs:70-82`; `tests/cli.rs:274-283` `adler32` is `src/png.rs:85-94`.
- Consequence: assertions `tests/cli.rs:186` (per-chunk CRC) and `:255` (Adler-32 trailer) only prove the writer used *the same* algorithm as the copy. A wrong polynomial/modulus copied into both places passes, so requirement 11's "correct CRC-32 and Adler-32 checksums" (`docs/requirements.md:34-36`) and the domain test mandate (`AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md:35-36`) are not independently proven. A known-answer check (a fixed byte string with a published CRC/Adler value) or a genuinely independent decoder would.
- Note: the ~144 lines of PNG/PPM decoding support around this (`tests/cli.rs:164-311`) are otherwise justified — the workspace explicitly requires testing PNG checksums and chunk layout, and comparing pixels requires a decoder — so the size alone is not the finding; the copied checksum code is.

**4. The PNG decoder pins wire details that are not in the contract.**
- Class: test beyond public contract (§3a "What never to test").
- Evidence: `tests/cli.rs:236` asserts the exact zlib header `[0x78, 0x01]`; `tests/cli.rs:210` panics on any chunk other than `IHDR`/`IDAT`/`IEND`. Requirement 11 (`docs/requirements.md:34-36`) and the C1 contract (`done/C1-png-writer-and-command-line.md:23-25`) fix one IDAT of stored blocks, 8-bit RGB, CRC/Adler — not the zlib FLG byte or the absence of ancillary chunks. These assertions would reject an otherwise contract-satisfying writer.

**5. `docs-pass.md` instructs the release to "keep the PPM command", but the only PPM command is the sentence it deletes.**
- Class: stale/incorrect deferral document (drift from design rule 8).
- Evidence: `docs-pass.md:8-9` — "delete it then and keep the PNG command and the PPM command". The one PPM line in `AGENT-INSTRUCTIONS/LOCAL/gates.md:39` is the fallback sentence itself (`cargo run --release > /tmp/opencode/render.ppm`), and it would no longer work once stdout is empty (`full-plan.md:58`); the PNG command at `gates.md:36` is the only other command. Deleting the sentence therefore leaves no PPM command, contrary to "keep ... the PPM command".
- The deferred sentence itself is quoted accurately and the deferral rationale (`docs-pass.md:9-10`, `full-plan.md:64-66`) is correct — this is only the keep/delete wording that the release would mis-follow.

## Verdict on the rest of the area (clean)

- Test budget: exactly **3** new `#[test]` functions, all in `tests/cli.rs` (`:42`, `:92`, `:116`); the two `src/main.rs` tests were adapted, not added (`src/main.rs:216-228`); C3's diff adds no test (`git show 0ad0f2b --stat` → `src/main.rs` only). Within C1's "at most three" (`full-plan.md:126`) and C3's "no test function is added" (`done/C3-progress-line.md:22`).
- No tests of private helpers via module internals; all binary tests go through `env!("CARGO_BIN_EXE_toy-raytracer")` (`tests/cli.rs:11-14`). No exact error-text assertions (`tests/cli.rs:108-111` checks only non-empty stderr), no exact progress-text test (C3 forbids it).
- Rendering tests check 3 pixels of an 8×4 image (`tests/cli.rs:86-89`), matching the workspace rule "Rendering tests check a few pixels ... never a whole image" (`repo-rules-domain.md:37`).
- `README.md:6` run example is current for the merged command line (PNG default via `--output image.png`); the removed paragraph was the plan-workflow framing, so the README is accurate.
- `docs/requirements.md` items 11-13 (`:34-41`) match the implementation: stored-block PNG with CRC-32/Adler-32 and PPM retained (11); the five options, 400×200/16/`render.png`/extension defaults, usage + non-zero exit (12, cf. `src/main.rs:23-28`, `108-184`); progress on stderr (13, cf. `src/main.rs:77-80`).
- No permanent file names a plan or phase: `grep -n "plan-c\|C1\|C3\|phase"` over `README.md`, `docs/requirements.md`, `tests/cli.rs`, `src/main.rs` → no matches.
- `docs-pass.md:6-10` is the only entry and holds exactly one deferred item.

## OUT-OF-SCOPE OBSERVATIONS

- `src/main.rs:8` still carries `#[allow(dead_code)] mod rng;`, which `full-plan.md:118` lists under "Must not ... Silence unused plumbing with `allow(dead_code)`". It predates this plan and belongs to the materials seam; area 1's call, but worth surfacing.
- `render` now writes progress from the unit tests too (`src/main.rs:66-80` called at `:218` and `:225`), so `cargo test` stderr gets `\r…%` noise. C3 explicitly permits stderr on success, so this is not a defect — area 3's observation.
- `README.md` no longer mentions that PPM remains available (`docs/requirements.md:36`), although the README is deliberately minimal and design rule 8 only covers the run example. Not stale, just thin.

## AREA VERDICT

Not clean, but nothing blocking: two real coverage gaps (missing stdout-empty assertion and status-2 assertion, both addable without a new test function), one duplication weakness (copied `crc32`/`adler32` cannot prove checksum correctness), one over-specific decoder assertion (zlib FLG / chunk set), and one imprecise deferral instruction in `docs-pass.md` (the "keep the PPM command" clause). The test count, test layering, README, requirements items 11-13 and the deferred gates.md sentence are otherwise correct.
