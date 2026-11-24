## C1 - PNG writer and command line

**Status:** done.
**Depends on:** none; the starting tree at `e180476`.
**Gate level:** standard.

**Frozen contract:**
- The binary takes exactly `--width`, `--height`, `--samples`, `--output <path>` and `--format
  png|ppm`. The defaults are 400 by 200, 16 samples and `render.png`, and the format otherwise
  follows the output file's extension. A repeated option takes the last value, and an explicit
  `--format` wins over the extension.
- Width and height must parse as whole numbers of at least 2, samples as a whole number of at least
  1. An unknown option, a missing value, an unparsable or out-of-range value, an unrecognised format
  and a path with no `png` or `ppm` extension and no explicit format each print one error line and
  the usage line on standard error and exit with status 2.
- `parse_args` resolves the arguments into one `Settings` value before rendering. `run` renders once
  and calls exactly one writer, `png::write_png` or `write_ppm`.
- The image goes only to the named output file. Standard output carries nothing on success. A writer
  or path failure prints one error line on standard error and exits with status 1.
- `render(width, height, samples)` builds the scene and camera once and averages `samples`
  identical, unjittered rays per pixel. Every sample count therefore writes the same bytes at
  proportional cost. This signature is the seam the materials plan consumes.
- `src/png.rs::write_png` writes an 8-bit RGB, non-interlaced PNG with one `IDAT` chunk of stored
  deflate blocks of at most 65535 bytes, a CRC-32 per chunk, a filter byte of 0 per scanline and an
  Adler-32 trailer. The writer and its checksums use only the standard library.
- `write_ppm` remains the PPM writer and stays reachable through `--format ppm` or a `.ppm` output.
  `to_byte` is `pub(crate)` so both writers share the channel conversion.
- `Cargo.toml` has no dependency, there is no build script, no `unsafe`, and rendering stays
  single-threaded and reproducible.
- `README.md` shows the real run command. `docs/requirements.md` is unchanged because items 11 and
  12 already state this behaviour. `AGENT-INSTRUCTIONS/LOCAL/gates.md` is unchanged, and its
  fallback sentence is deferred to the release through `docs-pass.md`.

**Proof in the tree:** `tests/cli.rs` tests `writes_a_png_whose_pixels_match_the_ppm_render`,
`rejects_an_unknown_option_and_an_unparsable_value_with_a_non_zero_exit` and
`selects_the_format_from_the_output_extension_and_lets_an_explicit_format_win`; `src/main.rs` tests
`the_center_pixel_shows_the_sphere_facing_the_camera` and `the_top_row_is_sky`.
