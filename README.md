<h1 align="center">toy-raytracer</h1>

<p align="center">
  A Rust raytracer that renders one fixed scene to a PNG or PPM image and declares no dependencies.
</p>

---

**Contents**

- [Why it exists](#why-it-exists)
- [What exists in the project](#what-exists-in-the-project)
- [Features](#features)
- [Tech stack](#tech-stack)
- [Architecture](#architecture)
- [Data and limits](#data-and-limits)
- [Run your own copy](#run-your-own-copy)

## Why it exists

A raytracer follows light backwards from the camera to the surfaces it meets. Reading a complete
renderer in one pass hides the parts that make it work, so this program keeps the camera, the
geometry, the materials and the image writers in separate modules, and each part can be read on its
own.

Four principles shape the program.

| Principle | How the program holds it |
|---|---|
| Standard library only | `Cargo.toml` declares no dependencies, so the build downloads no crates |
| Reproducible bytes | All randomness comes from one xorshift64* generator seeded with 2024, so the same options write the same bytes |
| Single-threaded | The render loop finishes one pixel before it starts the next |
| No unsafe code | No module contains an `unsafe` block |

## What exists in the project

- `README.md` describes the program, its features and how to run a copy.
- `/docs/requirements.md` states what the program does, under headings for materials, geometry,
  output and the command line, and quality.
- `/Cargo.toml` names the crate `toy-raytracer`, sets version `0.1.0` and edition 2021, and declares
  no dependencies.
- `/src/` holds the 12 modules that cast rays through the scene and write the image.

## Features

### Materials

Each object carries one material, and scattering returns the material's attenuation and the
scattered ray. A diffuse material scatters in a random direction around the surface normal and
attenuates by its albedo. A metal material reflects about the normal with a fuzz factor in 0 to 1
and absorbs a reflection that points into the surface. Glass refracts by its refractive index with
Snell's law, reflects on total internal reflection and chooses reflection or refraction by Schlick's
approximation. One scattering interface serves every surface, and a material that reports absorption
makes the ray black.

Requirements: [materials](/docs/requirements.md#materials).

### Shapes and hit data

A ray can hit a sphere, an infinite plane or a triangle. Every hit records the distance along the
ray, the point, the normal, whether the ray met the front face, and texture coordinates `(u, v)` in
0 to 1. The normal always points against the ray. Spheres map latitude and longitude, planes map the
hit position along two axes modulo 1, and triangles use their barycentric coordinates. Each object
reports an axis-aligned bounding box, and the scene list tests that box before the shape, so a ray
that misses the box skips the shape test. An infinite plane reports no box and is tested by every
ray.

Requirements: [geometry](/docs/requirements.md#geometry).

### Samples and gamma correction

`--samples` sets how many rays each pixel averages, and the default is 16. Each sample is jittered
inside the pixel, so edges and soft reflections blend across neighbouring pixels. Output applies
gamma 2 by clamping each channel to 0 to 1, taking its square root and scaling to 255. A linear
value of 0.25 therefore writes as 127.

Requirements: [materials](/docs/requirements.md#materials).

### PNG and PPM output

PNG output uses no external crate. The encoder writes the PNG signature, an `IHDR` chunk, one
`IDAT` chunk and an `IEND` chunk, and every chunk carries a CRC-32. The `IDAT` payload is a zlib
stream built from stored deflate blocks of at most 65535 bytes and closed with an Adler-32 checksum.
Pixels are 8-bit RGB with no interlacing and a filter byte of zero on every row. PPM output writes
the ASCII `P3` form with a maximum value of 255.

Requirements: [output and command line](/docs/requirements.md#output-and-command-line).

### Command line

Five options set the image size, the samples per pixel, the output path and the format. An unknown
option, a missing value or a value outside its range prints the reason and the usage line on
standard error and exits with status 2.

```text
usage: toy-raytracer [--width N] [--height N] [--samples N] [--output PATH] [--format png|ppm]
```

Requirements: [output and command line](/docs/requirements.md#output-and-command-line).

### Progress line

While the image renders, one line on standard error reports the share of rows finished. The line is
rewritten in place with a carriage return after each row, so each finished row raises the
percentage, and the line reaches `100%` after the last row. The line ends with a newline, and the
program writes the image only to the output file and the progress line only to standard error.

Requirements: [output and command line](/docs/requirements.md#output-and-command-line).

## Tech stack

| Layer | Technology | Version | Role |
|---|---|---|---|
| Language | Rust | 2021 edition | Renderer, command line and image writers in one crate |
| Runtime support | Rust standard library | bundled with `rustc` | Vectors, file writing, process exit and formatting |
| PNG encoder | In-tree code in `/src/png.rs` | 0.1.0 | PNG chunks, zlib stored blocks, CRC-32 and Adler-32 |
| Random numbers | In-tree xorshift64* generator in `/src/rng.rs` | 0.1.0 | Seeded sample jitter |

## Architecture

The program is one binary crate whose modules split the work between the camera, the geometry, the
materials and the image writers.

`/src/main.rs` holds the option parser, the fixed scene and the render loop. The render function
builds the scene and the camera once, casts the configured number of jittered rays per pixel,
averages the colours and reports progress after each row. `ray_colour` walks a ray up to the bounce
limit, multiplies the attenuation at every scatter and returns the sky gradient on a miss. The
finished pixels pass through gamma 2 and go to the PNG encoder or the PPM writer.

Geometry lives in `/src/sphere.rs`, `/src/plane.rs` and `/src/triangle.rs`, which share the
`Hittable` trait and the `HitRecord` in `/src/hittable.rs`. Materials live in `/src/material.rs`.
`/src/vec3.rs` supplies vectors and the `Colour` alias, `/src/ray.rs` the ray, `/src/aabb.rs` the
bounding boxes, `/src/camera.rs` the pinhole camera and `/src/rng.rs` the seeded generator. The
program keeps no state between runs and writes one image file.

### Repository layout

- `/src/` holds the crate and its 12 modules, listed in the table below.
- `/docs/` holds `/docs/requirements.md`, which states what the program does.
- `/Cargo.toml` names the crate, its version and its edition.
- `README.md` is this page.

| Module | Holds |
|---|---|
| [`main.rs`](/src/main.rs) | Option parsing, the fixed scene, the render loop and the PPM writer |
| [`aabb.rs`](/src/aabb.rs) | Axis-aligned bounding boxes and the slab test |
| [`camera.rs`](/src/camera.rs) | The pinhole camera at the origin |
| [`hittable.rs`](/src/hittable.rs) | The hit record, the `Hittable` trait and the scene list |
| [`material.rs`](/src/material.rs) | The `Material` trait and the diffuse, metal and glass materials |
| [`plane.rs`](/src/plane.rs) | The infinite plane |
| [`png.rs`](/src/png.rs) | The PNG encoder |
| [`ray.rs`](/src/ray.rs) | The ray and the point at a distance along it |
| [`rng.rs`](/src/rng.rs) | The seeded xorshift64* generator |
| [`sphere.rs`](/src/sphere.rs) | The sphere, its hit test and its texture coordinates |
| [`triangle.rs`](/src/triangle.rs) | The triangle and its barycentric hit test |
| [`vec3.rs`](/src/vec3.rs) | Three-component vectors and the `Colour` alias |

## Data and limits

The scene is fixed and built in `/src/main.rs`. It holds two diffuse spheres, one metal sphere with
fuzz 0.3, one glass sphere with refractive index 1.5 and one triangle, and the camera sits at the
origin looking down the negative z axis. Rendering reads no external data, and the program writes
one image file per run.

| Case | Value | Result |
|---|---|---|
| Bounce limit | 50 | A ray that scatters 50 times without reaching the sky is black |
| Absorbed ray | Scatter returns no ray | The pixel is black |
| Samples | 16 by default, minimum 1 | `--samples` sets the rays averaged per pixel |
| Image size | 400 by 200 by default, minimum 2 per axis | `--width` and `--height` set the pixels |
| Output file | `render.png` by default | `--output` sets the path |
| Format | `--format` when given, otherwise the output extension | An extension other than `.png` or `.ppm` is rejected when `--format` is absent |
| Progress | Standard error | One line reports the share of rows done |
| Bad option or value | Usage line on standard error | The program exits with status 2 |
| Seed | 2024 | The same options write the same bytes |

A ray that hits nothing returns the sky gradient, scaled by the attenuation gathered on its path.

## Run your own copy

Prerequisites:

- `cargo` and `rustc` from a Rust toolchain that supports the 2021 edition.
- A writable working directory for the image file.

No crate download is needed, because the crate declares no dependencies.

1. Build the program.

   ```bash
   cargo build
   ```

   Result: the compiler writes the binary to `target/debug/toy-raytracer`.

2. Render the image with the defaults.

   ```bash
   cargo run
   ```

   Result: the program writes `render.png` at 400 by 200 with 16 samples, and the progress line
   reaches `100%` on standard error.

3. Render with every option given.

   ```bash
   cargo run -- --width 400 --height 200 --samples 16 --output render.png --format png
   ```

   Result: the program rewrites `render.png` with the same bytes as step 2.

4. Read the progress line while the image renders. Each finished row raises the percentage on
   standard error, and the line ends with `100%` and a newline.

5. Run the tests.

   ```bash
   cargo test
   ```

   Result: the crate's unit tests run, and the summary reports 23 passed and 0 failed.

6. Check the lints.

   ```bash
   cargo clippy -- -D warnings
   ```

   Result: the command finishes with no warning and exits with status 0.

7. Check the formatting.

   ```bash
   cargo fmt --check
   ```

   Result: the command prints nothing and exits with status 0.
