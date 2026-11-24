## Phase C1 - Survey remediation (tests and argument parser)

**Status:** active (remediation - C1 landed and was surveyed; six small defects were confirmed).
**Depends on:** C1's frozen contract in `done/C1-png-writer-and-command-line.md` and the closing
survey note `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/closing-survey-2026-11-24.md`.
**Scope:** the confirmed survey findings that belong to C1's own work, and nothing else: four test
edits (F1 to F4) and one small simplification of `parse_args` (F5). The sixth confirmed finding is
the plan record, which the orchestrator already corrected, so this phase does not edit it (F6 is
verify-only). No product behaviour, message, exit status, default, option or output byte changes.
**Gate level:** standard - the phase edits tests and one argument-parser function whose behaviour must
stay identical, so targeted tests plus the fast gate and the rendered-image gate fit.
**Simplicity statement:** the phase adds no test function (C1 already used the three the budget allows)
and no new helper; it strengthens existing assertions, pins the two test-side checksum helpers with
published known-answer values, stops pinning one value the contract leaves free, and deletes the
duplicated option-name match plus its `unreachable!` arm. It does not add coverage for anything the
survey recorded as intentionally unfixed: the multi-block PNG path, the accepted chunk set, the usage
wording, or the README's missing PPM mention. Those stay as the survey recorded them.

**Current state (verified):**

Implemented and sound (do not rebuild):
- `src/png.rs::write_png` - writes the signature, `IHDR`, one `IDAT` of stored deflate blocks and
  `IEND`, with `crc32` per chunk and `adler32` in the trailer. Unchanged by this phase.
- `src/main.rs` `Format`, `Settings`, `parse_dimension`, `resolve_format`, `run`, `main` - the five
  options, the defaults, the extension rule, the explicit `--format` precedence, the usage line plus
  status 2 on bad input, and status 1 on a writer or path failure. Their behaviour is correct and is
  not changed; only `parse_args`' internal shape changes.
- `tests/cli.rs` tests `writes_a_png_whose_pixels_match_the_ppm_render`,
  `rejects_an_unknown_option_and_an_unparsable_value_with_a_non_zero_exit` and
  `selects_the_format_from_the_output_extension_and_lets_an_explicit_format_win`, plus the helpers
  `run`, `Scratch`, `decode_png`, `inflate_stored`, `crc32`, `adler32` and `read_ppm`. The three
  test functions and every helper stay; the edits below are inside them.

Broken / latent (root causes):
- `tests/cli.rs:60-63` asserts only that the PNG run succeeds. Nothing asserts that standard output
  is empty, although `done/C1-png-writer-and-command-line.md:18` freezes "Standard output carries
  nothing on success".
- `tests/cli.rs:104-107` asserts `!result.status.success()` only. The frozen contract
  (`done/C1-png-writer-and-command-line.md:14-15`) fixes status 2 for every rejected run, and
  `src/main.rs:203` exits 2, so a wrong status would pass today.
- `tests/cli.rs:260-272` `crc32` and `tests/cli.rs:274-283` `adler32` are byte-for-byte copies of
  `src/png.rs:70-82` and `src/png.rs:85-94`. The assertions at `tests/cli.rs:186` and
  `tests/cli.rs:255` therefore compare the writer with itself: a wrong polynomial or modulus copied
  into both copies would pass, leaving requirement 11's "correct CRC-32 and Adler-32" unproven.
- `tests/cli.rs:236` asserts the exact zlib header byte pair `[0x78, 0x01]`. Neither requirement 11
  (`docs/requirements.md:34-36`) nor the frozen contract fixes those bytes.
- `src/main.rs:118-121` rejects anything but the five option names by matching them a first time,
  `src/main.rs:126-147` matches the same five names again to dispatch, and
  `src/main.rs:146` needs `unreachable!` because the second match is not exhaustive on its own. The
  two spellings can drift silently.
- Survey finding 6 (the `docs-pass.md` deferral wording) is already corrected in the plan record by
  the orchestrator and is not this phase's work.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANS/plan-c/benchmarks/closing-survey-2026-11-24.md` - the six confirmed
  findings, their evidence and the list recorded as intentionally unfixed. Binding scope.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/done/C1-png-writer-and-command-line.md:13-19` - exit status 2 for
  rejected input and empty standard output on success. These are the contracts the new assertions
  cover.
- `tests/cli.rs` - the whole file; every edit below is inside it.
- `src/main.rs:108-158` - `parse_args`, the function step F5 rewrites.
- `docs/requirements.md:34-36` - requirement 11's checksum and PNG statement, which the known-answer
  values prove.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate commands and the rendered-image gate command.

**Build plan:**

1. **F1 - assert empty standard output on the PNG run** (`tests/cli.rs`).
   In `writes_a_png_whose_pixels_match_the_ppm_render`, directly after the existing PNG success
   assertion (`tests/cli.rs:60-63`), add one assertion:

   ```rust
   assert!(
       png_run.stdout.is_empty(),
       "a successful render writes nothing to standard output: {png_run:?}"
   );
   ```

   Do not add a matching assertion to the PPM run or to the other tests: one assertion on the
   contract's success path is what the finding asks for.

2. **F2 - assert exit status 2 for each rejected run** (`tests/cli.rs`).
   In `rejects_an_unknown_option_and_an_unparsable_value_with_a_non_zero_exit`, replace the
   `!result.status.success()` assertion (`tests/cli.rs:104-107`) with:

   ```rust
   assert_eq!(
       result.status.code(),
       Some(2),
       "exit status 2 for {args:?}: {result:?}"
   );
   ```

   Keep the existing stderr assertion and the "no image file" assertion unchanged. Do not rename the
   test function: it is named in the frozen contract at `done/C1-png-writer-and-command-line.md:34-36`,
   and its remaining scenario is still the rejected-input one.

3. **F3 - pin the test-side checksum helpers with known answers** (`tests/cli.rs`).
   As the first statements of the body of `writes_a_png_whose_pixels_match_the_ppm_render`, before the
   `Scratch` is created, add:

   ```rust
   assert_eq!(crc32(&[b"IEND"]), 0xAE42_6082, "the CRC-32 test helper");
   assert_eq!(
       adler32(b"Wikipedia"),
       0x11E6_0398,
       "the Adler-32 test helper"
   );
   ```

   Values, both published: `0xAE426082` is the CRC-32 of the ASCII bytes `IEND` (the value every PNG
   reader expects on that chunk), and `0x11E60398` is the Adler-32 of `Wikipedia` from the standard
   reference examples. The `crc32` helper takes `&[&[u8]]`; `&[b"IEND"]` coerces the byte-string literal
   to `&[u8]`, so no cast is needed. Do not add a test function and do not move the helpers into
   `src/`.

4. **F4 - stop pinning the zlib header values** (`tests/cli.rs`, `inflate_stored`).
   Replace `tests/cli.rs:236`:

   ```rust
   assert_eq!(&zlib[..2], &[0x78, 0x01], "the zlib header");
   ```

   with a comment and offset that consume the two header bytes without asserting their values:

   ```rust
   // The stream starts with two zlib header bytes whose values the contract does not fix.
   let mut offset = 2;
   ```

   The existing `let mut offset = 2;` on the following line becomes this one; do not add a length or
   value assertion in its place.

5. **F5 - one option match in `parse_args`** (`src/main.rs:107-158`).
   Replace the whole body of the argument loop with one match that resolves the value only inside a
   known-option arm. `parse_args` keeps its signature
   `fn parse_args(args: &[String]) -> Result<Settings, String>` and its result. After the edit the
   function reads exactly:

   ```rust
   /// Resolve the arguments into one set of settings before any rendering starts.
   fn parse_args(args: &[String]) -> Result<Settings, String> {
       let mut width = DEFAULT_WIDTH;
       let mut height = DEFAULT_HEIGHT;
       let mut samples = DEFAULT_SAMPLES;
       let mut output = PathBuf::from(DEFAULT_OUTPUT);
       let mut explicit_format = None;

       let mut index = 0;
       while index < args.len() {
           let option = args[index].as_str();
           let value = || {
               args.get(index + 1)
                   .map(String::as_str)
                   .ok_or_else(|| format!("{option} needs a value"))
           };
           match option {
               "--width" => width = parse_dimension(option, value()?)?,
               "--height" => height = parse_dimension(option, value()?)?,
               "--samples" => {
                   let value = value()?;
                   let count = value
                       .parse::<usize>()
                       .map_err(|_| format!("--samples: {value:?} is not a whole number"))?;
                   if count < 1 {
                       return Err(format!("--samples: {value:?} is less than 1"));
                   }
                   samples = count;
               }
               "--output" => output = PathBuf::from(value()?),
               "--format" => {
                   explicit_format = Some(match value()? {
                       "png" => Format::Png,
                       "ppm" => Format::Ppm,
                       other => return Err(format!("--format: unrecognised format {other:?}")),
                   });
               }
               other => return Err(format!("unknown option {other:?}")),
           }
           index += 2;
       }

       let format = resolve_format(explicit_format, &output)?;
       Ok(Settings {
           width,
           height,
           samples,
           output,
           format,
       })
   }
   ```

   Seven exact message strings stay byte-identical: `unknown option {other:?}`,
   `{option} needs a value`, `--samples: {value:?} is not a whole number`,
   `--samples: {value:?} is less than 1`, `--format: unrecognised format {other:?}`, and the two in
   `parse_dimension` which are untouched. `parse_dimension` and `resolve_format` are not edited. The
   `unreachable!` arm at `src/main.rs:146` and the pre-match option whitelist at
   `src/main.rs:118-121` are deleted, not kept beside the new match.

   Why behaviour is unchanged, so the implementer does not "improve" it: an unknown option is still
   answered before any value lookup because its arm never calls `value`; a known option with no
   following argument still reports `needs a value`; a repeated option still takes the last value;
   `index += 2` still consumes option plus value. The only ordering difference is that `index` now
   advances after the arm succeeds instead of before the value parses, which is unobservable because
   an error returns from the function.

6. **F6 - verify only, do not edit** (`AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md`).
   The survey's sixth finding (the release deferral wording for `AGENT-INSTRUCTIONS/LOCAL/gates.md`)
   was corrected in the plan record by the orchestrator. Read the file once and confirm the release
   item now names the real PPM command; change nothing. Do not edit `docs-pass.md`.

7. **Verification** (`tests/cli.rs` and `src/main.rs` only; no doc page changes).
   Run the fast gate and the rendered-image gate below. No `docs/`, README, `Cargo.toml` or
   `AGENT-INSTRUCTIONS/` file outside the plan folder is edited. Commit the two changed files, then
   the completion step compresses this phase file.

**Callers / wiring to update:**
- None. `parse_args` is private to `src/main.rs` and is called only at `src/main.rs:198`; its
  signature and `Result` type do not change. `Settings`, `Format`, `parse_dimension`, `resolve_format`
  and `run` keep their shapes. `inflate_stored` and the two checksum helpers are test-local and keep
  their signatures; `crc32` keeps its `&[&[u8]]` parameter.

**Config / schema / migrations:**
- No config key, schema or migration. `Cargo.toml` is unchanged and gains no dependency.
- Retained records and their named consumer/owner: none added. The survey note stays the only
  record; this phase adds no receipt, hash or log.

**Touched files:**
<!-- touched-files:start -->
- `tests/cli.rs` - changed: one empty-stdout assertion, status 2 instead of any non-zero, two
  known-answer assertions for the checksum helpers, and the unpinned zlib header.
- `src/main.rs` - changed: `parse_args` matches the five options once, with the duplicate whitelist
  match and the `unreachable!` arm deleted; behaviour and messages identical.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/phases/C1-survey-remediation.md` - unchanged on completion: this
  detailed file stays in `phases/`, as the earlier phases' detailed files do; the frozen copy is the
  `done/` file below.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/done/C1-survey-remediation.md` - created on passing evidence: the
  frozen remediation contract.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/full-plan.md` - changed on passing evidence: the C1 and Z1 table
  rows, the C1 phase file path, and the C1 and Z1 scoped `**Status:**` lines.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Add no test function. `tests/cli.rs` keeps exactly three `#[test]` functions and `src/main.rs`
  exactly two. Every change is an assertion inside an existing test or an edit to an existing helper
  (`REPO-RULES` §3a; C1 used its three).
- Do not change any product behaviour, error wording, exit status, default, option, scene, camera,
  sampling or output byte. Reproducibility and empty standard output must still hold.
- Do not edit `src/png.rs`, `README.md`, `docs/requirements.md`, `Cargo.toml`,
  `AGENT-INSTRUCTIONS/LOCAL/gates.md`, `AGENT-INSTRUCTIONS/LOCAL/project.md`,
  `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md`, `docs-pass.md`, or any `AGENT-INSTRUCTIONS/` file
  outside `AGENT-INSTRUCTIONS/PLANS/plan-c/`.
- Do not add coverage the survey recorded as intentionally unfixed: the multi-block PNG path, the
  accepted chunk set, the usage wording, the README's PPM mention. Do not reopen the survey or add a
  finding.
- Do not add a dependency, a build script, `unsafe`, a thread or new randomness. Do not touch the
  frozen `done/` contracts.
- Do not test `parse_args` directly or pin the usage wording; C1's `Must not` still holds. This edit
  is proven through the existing binary tests.
- No test name, error string or usage string is renamed by this phase.

**Tests:**
- New/changed/deleted: `tests/cli.rs` - `writes_a_png_whose_pixels_match_the_ppm_render`: empty
  standard output on the PNG run (F1), and known-answer values for `crc32` and `adler32` (F3).
  `tests/cli.rs` - `rejects_an_unknown_option_and_an_unparsable_value_with_a_non_zero_exit`: each
  rejected run exits 2 (F2). `tests/cli.rs` - `inflate_stored` helper: the zlib header is consumed,
  not asserted (F4). `src/main.rs` - no test edit; the existing binary tests prove F5's behaviour.
  Test functions added 0, deleted 0, changed 0; only assertions inside existing tests and helpers
  change.
- Fast gate: `~/.cargo/bin/cargo test`, `~/.cargo/bin/cargo build`,
  `~/.cargo/bin/cargo clippy -- -D warnings`, `~/.cargo/bin/cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image, required because `src/main.rs` (the parse and run path) changes
  and the output bytes must stay identical; agent generation, not affected.
- Slow gate (>2 minutes): none. The 80 by 40 render at 4 samples finishes in seconds.

Rendered-image gate, run by the implementer, once before and once after the edits:

```bash
~/.cargo/bin/cargo build --release
target/release/toy-raytracer --width 80 --height 40 --samples 4 --output /tmp/opencode/remediation-before.png > /tmp/opencode/before.stdout 2> /tmp/opencode/before.stderr
# apply F1 to F5, then:
~/.cargo/bin/cargo build --release
target/release/toy-raytracer --width 80 --height 40 --samples 4 --output /tmp/opencode/remediation.png > /tmp/opencode/remediation.stdout 2> /tmp/opencode/remediation.stderr
cmp /tmp/opencode/remediation-before.png /tmp/opencode/remediation.png
```

The first render must be taken at the current head `0ad0f2b` before any edit; if the worktree head is
not `0ad0f2b`, stop and report the drift. Then confirm: `cmp` reports no difference; the stdout
captures are both 0 bytes; each stderr capture holds the row shares with `\r` separators and ends
`100%` plus one newline; and `/tmp/opencode/remediation.png` opens in the available image reader
showing the sphere over the blue-to-white sky and the green ground plane (`file` reports PNG 80 by 40,
8-bit RGB). Remove the scratch files under `/tmp/opencode/` afterwards.

**User test (acceptance):**
- **Executor:** not required - the fast gate and the agent-run rendered-image evidence above prove
  this remediation. Nothing it changes is user-visible, and the rendered result's acceptance is Z1's
  step, which this phase re-enables rather than replaces.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a (standard) are satisfied.
- [ ] F1, F2, F3, F4 and F5 are implemented exactly as written; F6 is verified without an edit.
- [ ] `tests/cli.rs` still has exactly three `#[test]` functions and `src/main.rs` exactly two; no
      helper or test was added, deleted or renamed.
- [ ] Every error message, exit status, default, option and output byte is unchanged.
- [ ] Named tests pass; the fast gate commands above pass.
- [ ] The rendered-image gate ran once before and once after: `cmp` reports identical PNG bytes, both
      standard outputs are empty, both progress captures end `100%` plus one newline, and the image
      opens showing the sphere over the sky.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports 0 test functions added and 0
      deleted.
- [ ] No `docs/` page, README, `Cargo.toml`, or `AGENT-INSTRUCTIONS/` file outside
      `AGENT-INSTRUCTIONS/PLANS/plan-c/` changed; `git diff --check` is clean.
- [ ] Safety commit on branch `plan-c` exists; revision-bound verification receipt names the base and
      tested head.
- [ ] On passing evidence: compress this file into `done/C1-survey-remediation.md` (frozen contract
      only; the `phases/` file stays as written, as the earlier phases' detailed files do), set the C1
      row in `full-plan.md` back to `done` with phase file `done/C1-survey-remediation.md`, replace the
      C1 scoped `**Status:**` line with `**Status:** done.`, set the Z1 table row's Status from
      `planned` to `active` with phase file `phases/Z1-closing-survey.md`, and replace the Z1 scoped
      `**Status:**` line with `**Status:** active.` (drop its "waits for the C1 remediation"
      parenthetical, which the completed remediation makes false).

**Exit:** the two checksum helpers are proven independently by published known answers, the PNG test
proves empty standard output, the rejected-input test proves status 2, the decoder no longer pins the
zlib header values, `parse_args` matches its five options once with identical behaviour and messages,
and the rendered image and fast gate are unchanged.
