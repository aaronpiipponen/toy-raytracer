<p align="center"><img src="/docs/brand/logo.svg" alt="toy-raytracer" width="360"></p>

toy-raytracer renders one fixed scene to an image file, written as a PNG or an ASCII PPM.

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

The whole renderer lives in one crate, from the vector arithmetic to the image file. The scene is fixed, so a change to the geometry, a material, the camera or the sampling shows up as a different image, and two images show where the output moved.

Every random value comes from one generator seeded with a fixed value, and the draws happen in a fixed order. The same width, height and sample count therefore give the same pixel colours on every run, so two images can be compared directly. The Rust standard library alone is a hard rule of the design, so the build downloads no crates beyond the toolchain.

## What exists in the project

- [`src/`](/src) holds the renderer: vectors, rays, the camera, the three shapes, the three materials, the random generator, the PNG writer, and the command line, scene and render loop in `main.rs`.
- [`tests/`](/tests) holds the command-line tests, which run the built binary and check formats, pixels, rejected input and exit status.
- [`docs/`](/docs) holds the requirements, the behaviour pages, the render pipeline diagram and the logo.

## Features

### Materials

Three materials give the scene a matte surface, a blurred mirror and a refractive solid, and each one decides how a ray leaves the surface and how much colour the path carries. A diffuse surface scatters in a random direction around the normal and attenuates by its albedo. A metal surface reflects with a blur set by a fuzz factor and absorbs a reflection that points into the surface. A glass surface refracts by its refractive index, reflects totally beyond the critical angle, and otherwise chooses reflection by Schlick's approximation. A surface that absorbs a ray ends that path as black.

Documentation: [materials](/docs/materials.md).

### Geometry

A ray is tested against three shapes, which give the scene curved, flat and polygonal surfaces. Every hit carries the hit point, a normal that points against the ray, the face side, texture coordinates and the material. The scene list tests each object's bounding box before the object and returns the nearest hit. A sphere and a triangle report a box, and an infinite plane is tested on every ray.

Documentation: [geometry](/docs/geometry.md).

### Rendering

A pinhole camera and a bounce loop turn the scene into pixel colours. The camera sits at the origin with a viewport one unit in front of it. Each pixel averages 16 jittered samples by default, and each sample follows a path of at most 50 bounces, where a path that uses all 50 contributes black. A ray that hits nothing takes the sky gradient, and every channel gets gamma 2 correction before it becomes a byte. One seeded generator drives the jitter and every scatter, so the same settings give the same pixel colours.

Documentation: [rendering](/docs/rendering.md).

### Image output

The render writes an 8-bit truecolour PNG or an ASCII PPM from the same pixel colours, so both formats record the same three bytes per pixel. An explicit `--format` selects the format, and without it the output path's extension decides. Without `--format`, an extension other than `.png` or `.ppm` stops the run with an error before it renders.

Documentation: [image output](/docs/image-output.md).

### Command line

Five options set the image width, height, samples per pixel, output path and format, so one binary renders at different sizes and sample counts. An absent option takes its default value, and the defaults are 400 by 200 pixels, 16 samples and `render.png`. The program checks the whole command line before it renders, and a bad option or value prints an error and the usage line with exit status 2. A progress line on standard error counts the rows done.

Documentation: [running the raytracer](/docs/running.md).

## Tech stack

| Layer | Technology | Role |
|---|---|---|
| Language | Rust, edition 2021 | The whole program, from vector arithmetic to the image encoders |
| Dependencies | Rust standard library only | The features the renderer uses, so the build needs no external crate |
| Build and test | Cargo | Builds the binary and runs the unit tests and the command-line tests |

## Architecture

Behaviour lives in `src/main.rs`, which parses the command line, builds the fixed scene, runs the render loop and writes the output file. State moves through the calls as values: the render creates the scene and one seeded generator, and the bounce loop carries the running colour and the current ray. A shape implements the `Hittable` trait in its own module, and a material implements the `Material` trait. A change to the scene is a change to `scene()`.

### Repository layout

- [`src/`](/src) holds the renderer modules, each with its own unit tests.
- [`tests/`](/tests) holds the command-line tests that run the built binary.
- [`docs/`](/docs) holds the documentation pages, the requirements, the diagram in `docs/diagrams/` and the logo in `docs/brand/`.

The [architecture page](/docs/architecture.md) maps every module to its responsibility and names the tests that cover it.

## Data and limits

The program reads no external data and renders one fixed scene of five objects. Every random draw comes from the one seeded generator, so the output depends only on the options.

- A path scatters at most 50 times. A path that scatters on all 50 bounces contributes black.
- The default image is 400 by 200 pixels with 16 samples per pixel.
- The PNG writer stores the scanline data in uncompressed deflate blocks, so a PNG file is larger than its pixel data.
- Rendering runs on one thread.
- Every ray visits the objects in scene order and, for an object with a bounding box, tests the box before the object.

A ray that hits nothing returns the sky gradient blended by the y component of its direction. An output extension other than `.png` or `.ppm`, with no explicit `--format`, prints an error and the usage line and exits before rendering, so it writes no file.

## Run your own copy

Prerequisites:

- The Rust toolchain, edition 2021, with `cargo`.
- `cargo` on `PATH`, or the binary at `~/.cargo/bin/cargo`.

1. Build the release binary.

   ```bash
   cargo build --release
   ```

   The build finishes with no errors and writes the binary under `target/release/`.

2. Render the scene with the default settings.

   ```bash
   cargo run --release
   ```

   The run writes `render.png` in the current directory at 400 by 200 pixels with 16 samples, and a progress line on standard error counts the rows done.

3. Run the tests.

   ```bash
   cargo test
   ```

   Every test passes and the run ends with `test result: ok` for each test target.

The [running page](/docs/running.md) lists every option, its default and the input it rejects.
