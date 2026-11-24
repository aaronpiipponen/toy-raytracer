# C1 - PNG writer and command line (frozen contract)

## Guarantees later phases build on

- `main` resolves the command line once into the settings that one render uses. No later phase parses
  options a second time; new behaviour hangs off the resolved settings.
- The image goes only to the file named by `--output`. Standard output carries no image bytes and no
  status text.
- Defaults are 400 by 200, 16 samples, `render.png`, and the format taken from the output extension.
- An explicit `--format` wins over the extension. `.png` selects PNG, `.ppm` selects PPM, and any
  other extension with no explicit format is a bad value.
- A bad option, a bad value, an unknown format or an unknown extension prints the usage line on
  standard error and exits with status 2. An unwritable output path reports its error on standard
  error and exits with status 1.
- `render(width, height, samples)` returns every pixel colour row by row from the top, averaging that
  many rays per pixel. Nothing jitters the rays, so a given size writes the same bytes at every sample
  count. This signature is the seam the materials plan extends.
- PPM stays reachable through `--format ppm` or a `.ppm` output. `write_ppm` writes to any `Write`
  and keeps its own framing.
- The PNG writer is `src/png.rs`: one `IDAT` chunk of zlib data made of uncompressed stored deflate
  blocks, standard CRC-32 per chunk, Adler-32 in the trailer, 8-bit truecolour, no interlace, filter
  byte 0 per scanline. No dependency, no build script, no real compression.
- `to_byte` maps a channel in 0..=1 to 0 through 255 and both writers use it.
- Rendering stays single-threaded, dependency-free and reproducible.

## Documentation state

- `README.md` shows the real command line: `cargo run --release -- --output image.png`.
- `docs/requirements.md` items 11 and 12 describe this behaviour.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` carries the rendered-image fallback sentence ("until the
  command-line phase lands"). Deleting that sentence is a release item in the plan's `docs-pass.md`,
  because the shared file is true in worktrees whose binary has no command line.

## Test surface

- `tests/cli.rs` proves the binary's public behaviour: PNG pixels match the PPM render of the same
  scene, rejected input exits non-zero and writes no file, and the format follows the extension while
  an explicit `--format` wins.
- `src/main.rs` keeps the two render tests, now calling `render` with the sample count.
