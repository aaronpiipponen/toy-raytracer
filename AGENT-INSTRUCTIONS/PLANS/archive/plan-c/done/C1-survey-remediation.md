## C1 survey remediation of tests and argument parser

**Status:** done.
**Depends on:** C1's contract in `done/C1-png-writer-and-command-line.md` and the closing survey note
`benchmarks/closing-survey-2026-11-24.md`.
**Gate level:** standard.

**Frozen contract:**
- `tests/cli.rs` `writes_a_png_whose_pixels_match_the_ppm_render` asserts that a successful PNG run
  writes nothing to standard output, so the file-only output guarantee has a test.
- `tests/cli.rs` `rejects_an_unknown_option_and_an_unparsable_value_with_a_non_zero_exit` asserts exit
  status 2 for each rejected run. The rejected-input path still prints an error line and the usage
  line on standard error.
- The test-side `crc32` and `adler32` helpers are pinned by published known answers inside the
  existing PNG test, so the chunk CRC and the zlib trailer are checked against an outside value
  rather than against a copy of the writer's own arithmetic. Both helpers stay test-local and keep
  their signatures.
- `tests/cli.rs::inflate_stored` consumes the two zlib header bytes without asserting their values.
  The bytes belong to the writer, and neither requirement 11 nor the C1 contract fixes them.
- `src/main.rs::parse_args` matches the five option names once, in a single `match`, and answers an
  unknown option before any value lookup. `parse_dimension` and `resolve_format` keep their
  signatures, behaviour and messages.
- Every option, default, error message, exit status and output byte of C1's contract holds. The same
  options write byte-identical PNG and PPM files.
- Test functions added 0, deleted 0, changed 0. `tests/cli.rs` holds three tests and `src/main.rs`
  two, so C1's three-test budget is not exceeded and no test is renamed.
- `README.md`, `docs/requirements.md`, `Cargo.toml` and every `AGENT-INSTRUCTIONS/` file outside
  `AGENT-INSTRUCTIONS/PLANS/plan-c/` are unchanged. The release deferrals in `docs-pass.md` are
  unchanged: the `gates.md` fallback sentence, the `project.md` repository map and the
  `repo-rules-domain.md` module description.

**Proof in the tree:** `tests/cli.rs` carries the empty-standard-output assertion in
`writes_a_png_whose_pixels_match_the_ppm_render`, the status-2 assertion in
`rejects_an_unknown_option_and_an_unparsable_value_with_a_non_zero_exit`, the two known-answer
assertions for `crc32` and `adler32`, and the header-consuming `inflate_stored`. `src/main.rs`
`parse_args` has one `match` over the five options with an `other => return Err(...)` arm and no
`unreachable!`.
