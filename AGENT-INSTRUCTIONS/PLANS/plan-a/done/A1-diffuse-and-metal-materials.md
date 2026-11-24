## Phase A1 - Diffuse and metal materials [DONE]

Implemented and verified. Build instructions retired. Later phases consume these
frozen contracts:

- `src/material.rs` owns the `Material` trait. `scatter` takes the incoming ray, the hit record and
  the seeded generator, and returns either the attenuation with the scattered ray that starts at the
  hit point, or `None` when the ray is absorbed. The render loop reads only this trait.
- `Lambertian` holds an albedo and scatters in a random direction around the surface normal. It
  never absorbs.
- `Metal` holds an albedo and a fuzz factor in 0..=1. It reflects the unit incoming direction about
  the normal, blurs the result by the fuzz factor, and returns `None` when the blurred direction
  points into the surface. The albedo and the fuzz factor are not clamped or validated.
- `HitRecord` carries `pub material: Rc<dyn Material>` and is not `Copy`. `HittableList::hit` reads
  the nearest distance through a borrow.
- `Sphere` holds `Rc<dyn Material>` and clones it onto every record it returns. `Sphere::new` takes
  the material, so every object in the scene has one.
- `ray_colour` follows scattered rays for at most 50 bounces and multiplies each attenuation into
  the running colour. A miss returns the attenuated sky gradient. An absorbed ray and the 50-bounce
  limit both return black.
- `to_byte` applies gamma 2 by taking the square root of the clamped channel. `write_ppm` writes the
  pixels as ASCII PPM at 200 by 100.
- `render` creates one `Rng` seeded with `SEED` and shares it across every pixel, so the same options
  produce identical bytes.
- The scene holds a diffuse red-brown centre sphere, a diffuse grey ground sphere and a grey metal
  sphere of radius 0.25 at `(0.85, -0.25, -1.3)`.
- Materials are shared as `Rc<dyn Material>` and `Cargo.toml` has no dependency.
- Known carry-over: with one sample per pixel the image carries visible grain, and a pixel whose only
  metal-fuzz sample is absorbed renders black. A3's per-pixel averaging smooths both. A1 does not
  handle sampling.
- `HitRecord.normal` is the outward normal and the record carries no face-side flag. The face-side
  shape shared with the geometry plan is settled through the coordinator before A2 is detailed.
