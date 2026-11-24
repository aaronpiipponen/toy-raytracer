# C3 - Progress line (frozen contract)

## Guarantees later phases build on

- `render(width, height, samples) -> Vec<Colour>` reports row progress on standard error. After the
  column loop of each row, it writes `\r` followed by `"{rows done} of {height} rows"`, where rows
  done is `height - row` and runs from 1 to `height`. After the row loop it writes one newline, so
  the finished line is terminated and later output starts on its own line.
- The progress writes happen inside `render`; `run(&Settings)` and `render`'s signature and return
  value are what C1 left. Every render, including the unit tests that call `render` directly,
  writes the line to the process's standard error.
- Standard output stays empty during a successful render, and the image bytes for a given size,
  sample count and format are byte-identical to the render before the line existed. The progress
  line adds no bytes to the image file.
- The line counts rows only. It is independent of the sample count, the ray contents and any future
  bounce or jitter work, and the merged render settlement in
  `done/C3r-shared-render-settlement.md` preserves the two calls unchanged.
- There is no option, configuration key, constant or retained record for the progress line. It is
  transient terminal output and is stored nowhere.

## Documentation state

- `docs/requirements.md` item 13 states the progress line on standard error.
- `README.md` needs no change: its run example is the real command line.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` is unchanged. Its rendered-image fallback sentence is a
  release item recorded in the plan's `docs-pass.md`.

## Test surface

- `tests/cli.rs`, `writes_a_png_whose_pixels_match_the_ppm_render`, asserts after a successful run
  that standard output is empty and standard error is not, alongside its existing PNG and PPM pixel
  comparison. It does not assert the progress wording, the row numbers or the carriage return.
- The two unit tests in `src/main.rs` call `render` with the sample count; the test harness captures
  the progress line they produce.
- Test budget: 0 test functions added, 0 deleted, 1 changed.
