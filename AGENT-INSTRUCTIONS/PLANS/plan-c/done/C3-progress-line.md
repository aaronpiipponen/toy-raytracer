## C3 - Progress line

**Status:** done.
**Depends on:** C1, which leaves the resolved settings, the `samples` render seam and the
file-only output path.
**Gate level:** standard.

**Frozen contract:**
- `render(width, height, samples) -> Vec<Colour>` in `src/main.rs` keeps that signature and still
  builds the scene and camera once. When a row finishes it writes `\r<percent>%` to standard
  error, where the percent is `(height - row) * 100 / height` for `row` counting down from
  `height - 1`, so the last row writes `100%`. After the last row it writes one newline, which
  ends the line. The standard error stream is unbuffered, so no flush call and no buffered writer
  are involved.
- The progress line is transient terminal output. No option, verbosity level, timing, terminal
  detection or stored record controls it.
- Standard output stays empty on a successful render, and the output file is the only image
  destination.
- The progress output does not change pixel values, so the same options write the same image
  bytes.
- The `render` doc comment states that the share of finished rows is reported on standard error.
- No test function is added, changed or deleted for the progress line. `tests/cli.rs` asserts a
  non-empty standard error only for rejected input, so a successful render may write to standard
  error.
- `README.md`, `docs/requirements.md`, `AGENT-INSTRUCTIONS/LOCAL/gates.md` and `Cargo.toml` are
  unchanged by this phase; the rendered-image fallback sentence in
  `AGENT-INSTRUCTIONS/LOCAL/gates.md` remains deferred to the release through `docs-pass.md`.

**Proof in the tree:** the progress write sits in the `for row` loop of `render` in `src/main.rs`,
after the `for column` loop; `tests/cli.rs` and the two `src/main.rs` unit tests pass unchanged.
