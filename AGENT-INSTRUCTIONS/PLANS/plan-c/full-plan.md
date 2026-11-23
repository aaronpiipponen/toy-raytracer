# Output Plan

> **Status:** scoped forward plan on branch `plan-c`.
> **Scope:** how the program is run and what it writes: the PNG writer, the command line and the
> progress line (`docs/requirements.md` items 11 to 13).
> **Out of scope:** materials, sampling and scattering (the materials plan); shapes and hit data
> (the geometry plan).
> **Target:** a correct, small hobby raytracer.
>
> **How to use this plan:** expand exactly one phase into an implementation-grade phase from
> current code before coding.

## Phase status

| Phase | Title | Status | Gate level | Depends on | Phase file | Exit state | Move-on gate |
|---|---|---|---|---|---|---|---|
| C1 | PNG writer | done | standard | - | `done/C1-png-writer.md` | the image can be written as a valid PNG | gates green, the PNG opens |
| C2 | Command line | planned | standard | C1 | - | size, samples, output path and format come from options | gates green, a bad option prints usage |
| C3 | Progress line | planned | minimal | C2 | - | rendering shows its progress on standard error | gates green |
| Z1 | Closing survey and user acceptance | planned | minimal | C1 + C2 + C3 | - | the user accepts the result | the user accepts |

## How to implement this plan

Read `docs/requirements.md` and the code a phase touches before detailing it. Never invent a name.
Commit whenever useful. Run the fast gate in `AGENT-INSTRUCTIONS/LOCAL/gates.md`.

## Binding design rules

- No dependency: the PNG writer builds the zlib stream from stored deflate blocks and computes
  CRC-32 and Adler-32 itself.
- Output writing lives in `src/png.rs` and the existing PPM writer; `src/main.rs` only chooses one.

## Starting state

`src/main.rs` renders a fixed 200 by 100 image and writes PPM to standard output through
`write_ppm`. There is no option parsing.

## Scoped phases

### C1 - PNG writer

`src/png.rs` with a writer taking the width, the height and the pixels: signature, `IHDR` (8-bit
RGB), one `IDAT` with zlib-wrapped stored deflate blocks of at most 65535 bytes, `IEND`, each
chunk with its CRC-32. `main` writes `render.png` instead of standard output.

### C2 - Command line

Options `--width`, `--height`, `--samples`, `--output` and `--format png|ppm` parsed by hand in
`src/main.rs`, with the defaults and the usage line of the requirements. `--samples` is passed to
the render loop.

### C3 - Progress line

The render loop reports the share of rows done on standard error, overwriting one line.

### Z1 - Closing survey and user acceptance

Render the scene, compare it with the requirements, and ask the user to accept.

## Carry-overs

- C2: delete the plan-naming comments at `src/hittable.rs:10` and `src/vec3.rs:21` (frozen in
  `done/C1-png-writer.md`), keeping their `#[allow(dead_code)]` attributes.
