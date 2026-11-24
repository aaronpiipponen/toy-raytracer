# Command line

A run is configured by five options that select the image size, the sample count, the output file and the image format.

## Usage

```text
usage: toy-raytracer [--width <pixels>] [--height <pixels>] [--samples <count>] [--output <path>] [--format png|ppm]
```

A run with no arguments uses the defaults in the table and writes `render.png`.

## Options

| Option | Value | Default | Effect |
| --- | --- | --- | --- |
| `--width` | whole number, at least 2 | `400` | image width in pixels |
| `--height` | whole number, at least 2 | `200` | image height in pixels |
| `--samples` | whole number, at least 1 | `16` | jittered rays averaged per pixel |
| `--output` | file path | `render.png` | path the image is written to |
| `--format` | `png` or `ppm` | the output file's extension | image format |

Each option takes the argument after it as its value, so a value that starts with `--` is read as a value. When an option appears more than once, the last value wins.

The scene is fixed, and the options select the image size, the sample count and the output destination. See [Rendering](/docs/rendering.md) for the scene and the camera, and [Materials](/docs/materials.md) for the materials it uses.

## Format selection

An explicit `--format` value wins over the output file's extension. Without `--format`, the program reads the extension of the output path and accepts the exact text `png` or `ppm`, and the comparison is case-sensitive. Any other extension, or no extension, ends the run with an error before an image is written, so a name such as `render.jpg` needs `--format`.

## Rejected input

A run fails on an unrecognised option, an option with no value after it, a number that does not parse, a dimension below 2, a sample count below 1, a format other than `png` or `ppm`, and an output name without a `png` or `ppm` extension. The program prints one message and the usage line on standard error, exits with status 2, and writes no image file, because it resolves the whole command line before rendering starts.

| Input | Message |
| --- | --- |
| unknown option | `error: unknown option "--nonsense"` |
| missing value | `error: --width needs a value` |
| width that is not a whole number | `error: --width: "wide" is not a whole number` |
| width below 2 | `error: --width: "1" is less than 2` |
| height below 2 | `error: --height: "1" is less than 2` |
| samples that are not a whole number | `error: --samples: "many" is not a whole number` |
| samples below 1 | `error: --samples: "0" is less than 1` |
| format other than `png` or `ppm` | `error: --format: unrecognised format "gif"` |
| output name without a `png` or `ppm` extension | `error: render.jpg: output name has no png or ppm extension and no --format is given` |

## Standard output and progress

On success the program writes nothing to standard output and exits with status 0.

While rendering, standard error carries one progress line. After each finished row the line is rewritten in place, prefixed by a carriage return, with the percentage of finished rows. The percentage is a whole number, truncated, and the final update is followed by a newline. A height of 4 rows shows `25%`, `50%`, `75%` and `100%` in turn.

A failure to write the output file prints the operating system's message after `error: ` on standard error and exits with status 1. The program renders the image and encodes it in memory first, so this error comes after the progress line and covers only the file write.

## Reproducibility

All randomness comes from one generator seeded with a fixed value, and the scene and the camera are fixed, so two runs with the same options write the same bytes. The [requirements](/docs/requirements.md) state this guarantee.

## Example run

```bash
cargo run -- --width 8 --height 4 --samples 1 --output render.png
```

The command renders an 8 by 4 image with one sample per pixel. It writes four progress updates to standard error, writes nothing to standard output, writes `render.png` and exits with status 0.

The bytes of the file are described in [Image output](/docs/image-output.md).
