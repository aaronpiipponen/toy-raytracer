# Closing survey - output plan (2026-11-24)

Read-only survey of everything the output plan added or changed, run as the plan's last phase.
Branch `plan-c`, worktree `plan-c/`, product commits `910a9c5` (PNG writer and command line) and
`e194181` (progress line).

Method: one independent read-only `ops-scout` per area, all four in parallel; then one independent
read-only `ops-scout` cross-checking every claim against the tree; then the orchestrator wrote this
note from the saved returns (`survey-returns-2026-11-24.md`). Nothing was fixed in product code.

Areas surveyed: the PNG writer; the command line and settings; the render loop and progress line;
the documents the plan touched. Findings are recorded with their evidence and left for the user;
this note is not a fix list.

## PNG writer

Confirmed finding, test strength only:

- `tests/cli.rs:280-303` - the test decoder's `crc32` and `adler32` are the same algorithms as
  `src/png.rs:65-89`, so a shared mistake in the polynomial or initial value would pass both sides.
  There is no known-answer vector. The acceptance render is decoded by real software, which covers
  this in practice.
- `tests/cli.rs:208` - the zlib header is compared to the literal `[0x78, 0x01]` instead of being
  validated as a zlib header. The value is valid (`0x7801 % 31 == 0`).
- `tests/cli.rs:198-205` - the decoder stops at `IEND` without checking it is the last chunk and
  without checking that `IHDR` comes before `IDAT`. The writer emits neither defect.
- `tests/cli.rs:82` - the PNG/PPM comparison checks three pixels of thirty-two. Together with the
  shared `to_byte` this is thin, but the acceptance render covers the whole image.

Confirmed, cosmetic:

- `src/png.rs:37` - an empty raw stream would emit no deflate block and an invalid zlib stream. The
  CLI enforces width and height of at least 2, so it is unreachable.
- `src/png.rs:58-61` - the chunk CRC copies type plus data into a fresh buffer, copying the whole
  image-data chunk at defaults. Correct, just avoidable.
- `src/png.rs:18-19,55` - dimensions and lengths are cast to `u32` without a range check. Not
  reachable at these sizes.

Validated clean: one PNG code path, no unused helper, `to_byte` shared by both writers, no
plan-history comment, no dependency, and the writer matches requirement 11.

## Command line and settings

No finding. Verified: the five options, the four defaults declared once, explicit `--format` over
the extension, the usage line with exit 2 on bad input, exit 1 with a reported error on an unwritable
path, PPM reachable, nothing on standard output, one parsing path, every minimum load-bearing, no
swallowed error. The one observation is that an unwritable path is detected after the render
finishes, which the requirements allow.

## Render loop and progress line

Confirmed finding, branch state that the code does not state:

- `src/main.rs:70-75` - the sample loop traces the same ray `samples` times, so it costs `samples`
  times the work for identical bytes (`--samples 1`, `3` and `16` give the same image). This is the
  agreed seam: `--samples` had to reach the render loop, and every sample starts tracing a different
  path as soon as materials consume the generator. The cost until then is real.
- `src/main.rs:60` - the doc comment "averaging `samples` rays per pixel" reads as if the samples
  differ. It is inherited from the phase's own wording.
- `src/main.rs:60` - "Reports the share of rows done" while the line prints a count. It mirrors
  requirement 13's own phrase.

Validated clean: row-major order, one division after the sum, progress after each row with the
terminating newline, one `to_byte` conversion for both writers, one write path, no second sample
guard, and tests that assert only contracted behaviour.

## Documents

Confirmed finding, now recorded for the release:

- `AGENT-INSTRUCTIONS/LOCAL/project.md:5` - the repository map omits `src/png.rs`, which this plan
  added. Because the file is shared by every worktree, the fix belongs to the release; it is now a
  named item in `docs-pass.md`.
- `docs-pass.md` - the recorded release item for the rendered-image gate said to keep a PPM command
  that the file does not have; it now says to write one.

Validated clean: `README.md`, `docs/requirements.md` items 11 to 14, the rendered-image command for
this branch, and the deferred fallback sentence read as the tree behaves.

## Out-of-scope observations

Pre-existing, from the base commit, not introduced by this plan; they belong to the plans that
rewrite those lines or to the release:

- `src/main.rs:7-9` - "Seeded randomness for the scattering and sampling the plans add" plus
  `#[allow(dead_code)] mod rng;`.
- `src/hittable.rs:10-11` - "The origin of the bounce rays the materials plan adds" plus
  `#[allow(dead_code)]` on the point.
- `src/vec3.rs:21-22` - "For the triangle normals the geometry plan adds" plus
  `#[allow(dead_code)]` on the cross product.
- `README.md:6-7` shows bare `cargo`, while the workspace gates note that cargo may not be on `PATH`
  here.

## Corrections made while surveying (plan text and shared records only)

- `docs-pass.md`: the rendered-image item now says to write a PPM command; a second item now records
  the missing `src/png.rs` entry in the repository map.
- `phases/Z1-closing-survey-and-user-acceptance.md`: one line reference corrected to the decoder's
  real range.
- `PLANS/shared/plan-a-A1+plan-c-C3.md`: the merged-render sketch placed the pixel computation
  outside the sample loop, contradicting its own jitter rule; the computation now sits inside the
  sample loop, as the draw order says.

## Cross-check outcome

Twenty-two claims checked against the tree; the cross-check rejected four. Three of those rejections
are the corrections above, which the cross-check read in their corrected form, so the original
finding stood when it was made. The fourth, the shared-design sketch, matches on content and could
not be checked against history because the plan repository is outside the scout's read scope.

The cross-check added five claims:

1. Exit status 2 and the unwritable-path status 1 are not pinned by tests. The requirements state
   only a non-zero exit, so this is a choice of ours, not a contract; not accepted as a defect.
2. The usage line is never asserted; only that standard error is not empty. Asserting its wording is
   forbidden by the test rules, so not a defect.
3. `--width` with no following value is not covered by a test. This is a real, small coverage gap in
   contracted behaviour; it cannot be added now without exceeding this phase's test budget.
4. The explicit-format override is only tested in one direction. The other direction runs the same
   branch, so not a defect.
5. The `gates.md` fallback sentence is called false on this branch. The sentence is conditional
   ("until the command-line phase lands") and does not apply here, while it still applies in the
   three worktrees without a command line. Not accepted as a defect; its deletion stays a release
   item.

## Acceptance render

Command, from the worktree root:

```
~/.cargo/bin/cargo build --release
./target/release/toy-raytracer --output /tmp/opencode/z1-accept.png 2>/tmp/opencode/z1-progress.err
./target/release/toy-raytracer --output /tmp/opencode/z1-accept-2.png
cmp /tmp/opencode/z1-accept.png /tmp/opencode/z1-accept-2.png
```

Observed: exit status 0 for both runs; standard output empty; standard error 3093 bytes holding
`1 of 200 rows` through `200 of 200 rows`, each preceded by a carriage return, ending with a newline;
the two files byte-identical; `file` reports PNG 400 by 200, 8-bit RGB, non-interlaced at 240283
bytes; the image opens showing the sphere over the blue sky and green ground, in the placeholder
normal shading this plan's scope leaves in place.

## Accepted gaps

The user reviewed this survey on 2026-11-24 and accepted the following as sufficient coverage for a
hobby project. No follow-up work is planned for them and none of them is a release item.

- The test decoder's checksum helpers share the writer's algorithms with no known-answer vector
  (`tests/cli.rs:280-303`): the real decoder that opens the acceptance render establishes the bytes
  are a valid PNG.
- The zlib header is compared to its literal value rather than validated as a header
  (`tests/cli.rs:208`): the literal is a valid header.
- The decoder does not assert the end marker is last or that the header precedes the data
  (`tests/cli.rs:198-205`): the writer emits neither defect and the real decoder would reject either.
- Three of thirty-two pixels are compared between the two formats (`tests/cli.rs:82`): both writers
  share one channel conversion, and the acceptance render covers the whole image.
- `--width` with no following value has no test (the missing-value branch at `src/main.rs:130-132`):
  the branch is one line, the other rejected inputs exercise the same reporting path, and the test
  budget would have to grow for it.

## For the user

- Nothing must be fixed before the release.
- Accepted on 2026-11-24: the gaps above, with the reasons given.
- Accepted on 2026-11-24: the release removes the three plan-naming comments and the bare `cargo`
  line; they are on the plan's documentation-pass list.
