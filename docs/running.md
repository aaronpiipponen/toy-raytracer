# Running the raytracer

This page lists the prerequisites and the commands that build, render, and test the raytracer.

## Prerequisites

- The Rust toolchain, edition 2021, with `cargo`. `Cargo.toml` sets `edition = "2021"` and declares no dependencies, so the build downloads no crates beyond the toolchain.
- `cargo` on `PATH`. When it is not on `PATH`, the binary is at `~/.cargo/bin/cargo`.

The whole command line is one line of options, and each option takes the argument that follows it as its value.

## Build the binary

1. Open a terminal in the repository root, the folder that holds `Cargo.toml`.
2. Run `cargo build --release`. The build finishes with no errors and writes the binary and its build output under `target/release/`.

A debug build works the same way through `cargo build`, under `target/debug/`. The renders in the following sections use the release build.

## Render the scene

1. Run the program with its defaults.

   ```bash
   cargo run --release
   ```

   The render writes `render.png` into the current directory at 400 by 200 pixels with 16 samples per pixel. A progress line on standard error counts the rows done, in the form `5 of 200 rows`, and the last row ends the line. Standard output stays empty.

2. Check the output file.

   ```bash
   ls -l render.png
   ```

   The listing shows `render.png` in the current directory with a size above zero.

The same options on the same build write the same bytes, so a repeated run replaces the file with an identical one.

## Command-line options

The program writes the image to the output file and prints nothing on standard output. The PNG and PPM writers are covered in [image output](/docs/image-output.md).

| Option | Value | Default | Accepted values |
|---|---|---|---|
| `--width` | Width in pixels | 400 | An integer of at least 2 |
| `--height` | Height in pixels | 200 | An integer of at least 2 |
| `--samples` | Samples per pixel | 16 | An integer of at least 1 |
| `--output` | Output file path | `render.png` | A path the process can write |
| `--format` | Output format | From the output path's extension | `png` or `ppm` |

An option and its value form one pair, and the program reads the pairs from left to right. Options that are absent keep their defaults. Under `cargo run`, the two hyphens after the cargo arguments separate them from the program's arguments.

```bash
cargo run --release -- --width 800 --height 400 --samples 64 --output image.png
```

`--format` accepts only the exact text `png` or `ppm`. An explicit `--format` takes precedence over the output path's extension, so a path ending in `.png` with `--format ppm` holds a PPM image. Without `--format`, the extension decides, and `.png` writes PNG while `.ppm` writes PPM.

```bash
cargo run --release -- --format ppm --output image.png
```

```text
usage: toy-raytracer [--width <pixels>] [--height <pixels>] [--samples <count>] [--output <path>] [--format png|ppm]
```

## Rejected input

The program checks the whole command line before it renders anything. Any of the following problems prints an error and the usage line on standard error and exits with status 2, and writes no file.

| Problem | Error message |
|---|---|
| An option the program does not know | `unknown option --name` |
| An option with no argument after it | `--width needs a value` |
| A value that is not a whole number | `--width needs a number, not wide` |
| A count below its minimum | `--samples needs at least 1, not 0` |
| A `--format` value other than `png` or `ppm` | `unknown format jpg` |
| An output extension other than `.png` or `.ppm`, with no `--format` | `cannot choose a format for image.jpg` |

Every numeric value parses as a non-negative whole number, so a negative value or a decimal point fails the number check.

A failure during rendering or during the write of the output file prints the error and exits with status 1. A destination path in a folder that does not exist is such a failure.

## Run the tests

1. Run the test suite.

   ```bash
   cargo test
   ```

   Cargo builds the binary and runs the unit tests in the source modules and the command-line tests in `tests/cli.rs`.

2. Read the summary. Every test passes, and the run ends with `test result: ok` for each test target.

The command-line tests run the built binary and check the default and explicit formats, the rejected input, and the exit status of each case.
