# Architecture

The crate divides its code into modules around rays, the scene, materials and two image encoders, and this page maps the modules and their tests.

## Module map

The crate is a binary. `src/main.rs` declares every other module with `mod`, owns the command line and the render loop, and holds the scene.

| Module | Responsibility |
|---|---|
| `src/main.rs` | Command line, settings, the fixed scene, the render loop, `to_byte`, the PPM encoder and the output dispatch |
| `src/vec3.rs` | `Vec3`, its operators, and the `Colour` alias |
| `src/ray.rs` | `Ray`, with `new` and `at` |
| `src/camera.rs` | `Camera`, the pinhole camera and `Camera::ray` |
| `src/hittable.rs` | `HitRecord`, the `Hittable` trait and `HittableList` |
| `src/material.rs` | The `Material` trait, `Lambertian`, `Metal`, `Dielectric`, and the reflection, refraction and random direction helpers |
| `src/sphere.rs` | `Sphere` and its hit test |
| `src/plane.rs` | `Plane`, the infinite plane, and its hit test |
| `src/triangle.rs` | `Triangle` and its hit test |
| `src/aabb.rs` | `Aabb` and `Aabb::hit` |
| `src/png.rs` | `write_png`, with the CRC-32 and Adler-32 checksums |
| `src/rng.rs` | `Rng`, the seeded xorshift64* generator |

`Ray` stores the direction as given, so `HitRecord::t` is a distance only when the direction has unit length. The metal and glass materials normalise the incoming direction before reflecting or refracting.

## The render pipeline

`main` collects the arguments into a vector and passes it to `parse_args`, which returns a `Settings` with width, height, samples, output path and `Format`. A bad option or value becomes a `String` error that `main` prints with `USAGE` before exiting with status 2. A failed write exits with status 1.

`render` builds the scene with `scene()` and the camera with `Camera::new(width as f64 / height as f64)`, then creates one generator with `Rng::new(SEED)`. It walks the rows from the top and the columns from left to right. Within a pixel, each of the `samples` draws moves `u` and `v` by `rng.next_f64()` inside the pixel cell, asks `Camera::ray` for the ray and adds `ray_colour` to the pixel colour. The pixel stores the sum divided by `samples`. After each row, `render` writes the share of rows done to standard error.

`ray_colour` follows one ray for at most `MAX_BOUNCES` segments, which is 50. `HittableList::hit` returns the nearest `HitRecord` whose `t` lies in `0.001..f64::INFINITY`, and `Material::scatter` returns an attenuation and a scattered ray. The running colour is multiplied by the attenuation and the scattered ray becomes the next segment. A material that returns `None` gives black, and a ray that reaches 50 segments gives black. A ray that hits nothing ends the walk with the running colour multiplied by `sky_colour`.

`run` encodes the finished pixels into a byte buffer. `Format::Png` calls `png::write_png` and `Format::Ppm` calls `write_ppm`. `fs::write` writes the whole buffer to the output path in one call. Both encoders convert a channel with `to_byte`, which clamps to `0..=1` and takes the square root for gamma 2.

## Where to change what

### The scene

`scene()` in `src/main.rs` returns a `HittableList` with five objects: a diffuse sphere, an infinite plane, a triangle, a metal sphere and a glass sphere. Each object is boxed and added with `world.add(Box::new(...))`, and each material is an `Rc<dyn Material>` built beside its object. A different scene is a different body for `scene()`, and `render` takes no scene argument.

### Adding a shape

A shape implements `Hittable`. `hit` returns the nearest `HitRecord` with `t` in the given range, built with `HitRecord::new` so the normal faces against the ray, and `bounding_box` returns the `Aabb` that holds the shape or `None`. `HittableList::hit` tests the box first and skips the shape when the box is missed. A shape that returns `None` from `bounding_box` is tested on every ray, as `Plane` does.

### Adding a material

A material implements `Material::scatter`. It returns `Some((attenuation, ray))` for a scatter and `None` when the surface absorbs the ray. The scattered ray starts at `hit.point`. Randomness arrives as the `&mut Rng` argument, and each of the three materials draws from it. A shape shares its material as an `Rc<dyn Material>`, so one material value can serve several shapes.

### The camera

`Camera::new` fixes the viewport height at 2, places the origin at `(0, 0, 0)` and the viewport one unit along negative z, with the width taken from the aspect ratio. `Camera::ray` maps `u` and `v` in `0..=1` from the lower left through that viewport. A different position or field of view is a change in `src/camera.rs`.

### Defaults and validation

`DEFAULT_WIDTH`, `DEFAULT_HEIGHT`, `DEFAULT_SAMPLES`, `DEFAULT_OUTPUT`, `SEED` and `MAX_BOUNCES` are constants in `src/main.rs`. `parse_number` enforces a minimum of 2 for `--width` and `--height` and 1 for `--samples`. `resolve_format` takes an explicit `--format` over the output extension, and rejects an extension that is neither `png` nor `ppm`.

### The image encoders

`png::write_png` writes an 8-bit truecolour PNG without interlacing. It writes one `IDAT` chunk whose zlib stream holds stored deflate blocks, with the CRC-32 of each chunk and the Adler-32 trailer. `write_ppm` writes an ASCII `P3` image. `to_byte` in `src/main.rs` is the shared channel conversion for both, so a gamma change belongs there.

### The seed

`SEED` in `src/main.rs` is 7, and `Rng::new` takes a `u64`. `Rng::new` stores `seed.max(1)`, so the xorshift state is never zero, and `next_u64` advances that state. A different seed changes every jitter offset and every material draw.

## Tests

### Unit tests in each module

- `src/aabb.rs`: a ray hits the box it crosses and misses one beside it.
- `src/hittable.rs`: the scene list hits a plane that reports no box.
- `src/material.rs`: a diffuse material returns its albedo and scatters away from the surface, a metal with no fuzz reflects about the normal, a metal reflection that points into the surface is absorbed, a glass ray at normal incidence passes through, a glass ray entering at an angle bends toward the normal, and a glass ray leaving beyond the critical angle reflects internally.
- `src/plane.rs`: a ray hits the plane and its texture coordinates wrap into `0..=1`.
- `src/rng.rs`: the same seed gives the same sequence.
- `src/sphere.rs`: a ray through the centre hits the near surface, a ray from inside the sphere hits the back face with `front_face` false, the texture coordinates come from latitude and longitude, and a ray beside the sphere misses.
- `src/triangle.rs`: a ray through the triangle hits it with barycentric coordinates, a ray beside the triangle misses, and the box holds the three vertices.
- `src/main.rs`: the top row is sky, the same settings render the same pixels, and the sample count changes the pixel colours.

A change to a hit test or a scatter updates the test in its own module. `cargo test` runs the unit tests and the integration tests together.

### Integration tests

`tests/cli.rs` runs the built binary through `CARGO_BIN_EXE_toy-raytracer` and writes into a temporary directory that `Scratch` removes when the test ends.

- `writes_a_png_whose_pixels_match_the_ppm_render` renders eight by four pixels with one sample to a PNG, decodes it with the test's own `decode_png`, renders the same size to a PPM, and compares pixels at indexes 0, 20 and 31. `decode_png` also checks each chunk CRC-32, the single `IDAT` chunk, the `IHDR` fields, the scanline filter byte and the zlib Adler-32. The test asserts that standard output is empty and standard error is not.
- `rejects_an_unknown_option_and_an_unparsable_value_with_a_non_zero_exit` passes `--nonsense`, `--width wide` and `--samples 0`. Each attempt exits non-zero, writes a message to standard error and leaves no output file.
- `selects_the_format_from_the_output_extension_and_lets_an_explicit_format_win` checks the PNG signature for a `.png` path, `P3` for a `.ppm` path, `P3` for an explicit `--format ppm` on a `.png` path, and a non-zero exit for a `.jpg` path with no file written.

## Hard invariants

- `Cargo.toml` has an empty `[dependencies]` section, so the crate builds from the standard library alone.
- Every random draw comes from `Rng` in `src/rng.rs`. `render` creates the one generator with `SEED` and passes it to the jitter and to every `scatter` call.
- Rendering runs on one thread. The render loop is the nested loop over rows and columns in `render`, and shared materials are `Rc<dyn Material>` rather than `Arc`.
- `src/` and `tests/` contain no `unsafe` block.

The requirements behind these rules are on [requirements](/docs/requirements.md). The behaviour a change must preserve is described on [rendering](/docs/rendering.md), [geometry](/docs/geometry.md), [materials](/docs/materials.md), [image output](/docs/image-output.md) and [running](/docs/running.md).

## Gates

A change is complete when these four commands pass from the repository root.

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```
