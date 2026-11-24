# Requirements

A raytracer that uses only the Rust standard library and renders one fixed scene to a file with the
same bytes for the same options.

## Materials

1. Every object has a material. A ray that hits an object scatters by that material and the colour
   is the material's attenuation times the colour the scattered ray sees, up to a bounce limit of
   50, and a ray that reaches the limit is black.
2. Diffuse material scatters in a random direction around the surface normal and attenuates by its
   albedo.
3. Metal material reflects about the normal, blurred by a fuzz factor in 0..=1, and attenuates by
   its albedo. A reflection that points into the surface is absorbed.
4. Glass material refracts by its refractive index using Snell's law, reflects on total internal
   reflection, and chooses reflection by Schlick's approximation otherwise. It does not attenuate.
5. Each pixel averages a configurable number of samples (default 16), jittered inside the pixel,
   with gamma 2 correction on output.
6. All randomness comes from the seeded generator in `src/rng.rs`, seeded with a fixed value.

## Geometry

7. Every hit records whether the ray hit the front face, and the normal always points against the
   ray.
8. Every hit records texture coordinates `(u, v)` in 0..=1: spheres by latitude and longitude,
   planes by the hit point's position along two axes in the plane, modulo 1, and triangles by their
   barycentric coordinates.
9. The scene can hold infinite planes (a point and a normal) and triangles (three vertices).
10. Every finite object reports an axis-aligned bounding box, and the scene list tests a ray against
    an object's box before the object. An infinite plane has no box and is always tested.

## Output and command line

11. The image is written as PNG by the program's own writer: one IDAT chunk holding zlib data made
    of stored (uncompressed) deflate blocks, with correct CRC-32 and Adler-32 checksums. PPM is
    available.
12. The command line takes `--width`, `--height`, `--samples`, `--output <path>` and
    `--format png|ppm`. The defaults are 400 by 200, 16 samples, `render.png` and the format from the
    output file's extension. An unknown option or a bad value prints a usage line and exits with a
    non-zero status.
13. While rendering, a progress line on standard error shows the share of rows done.

## Quality

14. `cargo build`, `cargo test`, `cargo clippy -- -D warnings` and `cargo fmt --check` pass.
