# Z1 - Closing survey and user acceptance (frozen contract)

## What the plan's result rests on

- The closing survey covers the four areas this plan owns: the PNG writer, the command line and
  settings, the render loop and progress line, and the documents the plan touched. It is read-only in
  product code.
- Its subject is the plan's product change: `src/png.rs`, the command line and render loop in
  `src/main.rs`, `tests/cli.rs`, and the `README.md` run example. The plan adds nothing else to the
  tree.
- The four area returns are in `PLANS/plan-c/survey-returns-2026-11-24.md`. The survey result is
  `PLANS/plan-c/benchmarks/2026-11-24-closing-survey.md`: one section per area, every finding with
  its evidence, the out-of-scope observations, the cross-check outcome, the acceptance render and the
  accepted gaps.

## Survey result

- The command line and settings area produced no finding: five options, four defaults declared once,
  explicit `--format` over the extension, the usage line with exit 2 on bad input, exit 1 with a
  reported error on an unwritable path, PPM reachable, and nothing on standard output.
- The PNG writer meets requirement 11. Its findings are test strength and cosmetic points.
- The render loop meets the frozen C3 and C3r contract. Its findings are the `--samples` cost of
  averaging one repeated ray before materials consume the generator, and two doc-comment wordings
  inherited from the phase and the requirement. The sample count reaches the render loop, and each
  sample traces a different path once materials use the generator.
- The two wordings stay: `render`'s doc comment describes the averaging the code performs and the
  progress sentence repeats requirement 13's own phrase. Neither is a defect and neither is a release
  item.
- The documents area leaves two records in `PLANS/plan-c/docs-pass.md`: the `project.md` repository
  map omits `src/png.rs`, and the rendered-image gate entry says to write a PPM command rather than
  keep one the file does not have.
- One independent cross-check covers every saved claim. Its mismatched claims are the plan-record
  corrections recorded in the note, and the shared design sketch in
  `PLANS/shared/plan-a-A1+plan-c-C3.md` matches on content. Its five added claims are recorded in the
  note, and two of them are accepted gaps below.

## Accepted gaps

The user accepted these five points as sufficient coverage for a hobby project. None of them is a
release item and none gets follow-up work.

- The test decoder's checksum helpers repeat the writer's algorithms with no known-answer vector
  (`tests/cli.rs:280-303`). The real decoder that opens the acceptance render establishes the bytes
  are a valid PNG.
- The zlib header is compared to its literal value rather than validated as a header
  (`tests/cli.rs:208`). The literal is a valid header.
- The decoder does not assert the end marker is last or that the header precedes the data
  (`tests/cli.rs:198-205`). The writer emits neither defect and a real decoder rejects either.
- Three of thirty-two pixels are compared between the two formats (`tests/cli.rs:82`). Both writers
  share one channel conversion.
- `--width` with no following value has no test (`src/main.rs:130-132`). The other rejected inputs
  exercise the same reporting path, and a test for it would grow the phase's test budget.

## Acceptance render

- Produced through the real command line at the defaults (400 by 200, 16 samples), twice into
  `/tmp/opencode/z1-accept.png` and `/tmp/opencode/z1-accept-2.png`.
- Exit status 0 for both runs, standard output empty, the two files byte-identical by `cmp`, and the
  PNG 400 by 200, 8-bit RGB, non-interlaced at 240283 bytes.
- Standard error holds `1 of 200 rows` through `200 of 200 rows`, each preceded by a carriage
  return, ending with a newline.
- The image shows the sphere over the sky and the ground.

## User acceptance

- The user accepted the survey, the result and the render on 2026-11-24, and accepted the five gaps
  above.
- The release cleans the three plan-naming comments and the README's bare `cargo` line. Both are
  items in `PLANS/plan-c/docs-pass.md`.
- Every plan phase is done or not applicable. The release closes the plan.
