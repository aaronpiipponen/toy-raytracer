# A1 - Diffuse and metal materials (frozen contract)

## Guarantees later phases build on

- `src/material.rs` holds the `Material` trait with
  `fn scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)>`.
  `Some((attenuation, scattered_ray))` is the colour multiplied into the running colour and a ray
  that starts at `hit.point`; `None` means the ray is absorbed. All randomness comes from the
  `Rng` passed in.
- `Lambertian` holds `pub albedo: Colour` and is built with `Lambertian::new(albedo)`. It scatters
  along `hit.normal + random_unit_vector(rng)` without normalising the sum, and attenuates by the
  albedo.
- `Metal` holds `pub albedo: Colour` and `pub fuzz: f64` in 0..=1 and is built with
  `Metal::new(albedo, fuzz)`. It reflects the unit incoming direction about `hit.normal`, adds
  `random_unit_vector(rng) * fuzz`, and returns `None` when the scattered direction points into the
  surface at `hit.normal`.
- The private helpers in `src/material.rs` are `reflect(v, n)` and
  `random_unit_vector(rng)` (rejection sampling of a point in the unit ball, returned as a unit
  vector). There is one random source, the passed `Rng`.
- `HitRecord` in `src/hittable.rs` is
  `pub struct HitRecord { pub t: f64, pub point: Vec3, pub normal: Vec3, pub material: Rc<dyn Material> }`.
  It derives nothing, so it is not `Copy`. `HittableList::hit` reads the running limit with
  `nearest.as_ref().map_or(t_max, |hit| hit.t)`. Records are still built as struct literals on this
  branch; the release merge replaces that with `HitRecord::new(...)` per
  `PLANS/shared/plan-a-A1+plan-b-B1.md`. On this branch `hit.normal` is the outward normal.
- `Sphere` in `src/sphere.rs` holds `pub material: Rc<dyn Material>` and is built with
  `Sphere::new(center, radius, material)`. `Sphere::hit` returns
  `material: Rc::clone(&self.material)`.
- `src/main.rs` renders through `fn ray_colour(ray: &Ray, world: &dyn Hittable, rng: &mut Rng) -> Colour`:
  it starts from white, follows `hit.material.scatter` for `MAX_BOUNCES` (50) and multiplies each
  attenuation, returns `colour * sky_colour(&ray)` when the ray misses, and returns black when the
  bounce limit is reached or a material absorbs. `sky_colour` holds the one sky gradient
  expression. `render` creates one `Rng::new(SEED)` with `SEED = 7` before the pixel loops and
  passes it down; every pixel shares that sequence. `to_byte` applies gamma 2 by
  `(255.999 * channel.clamp(0.0, 1.0).sqrt()) as u8`.
- `scene()` adds, in this order, the centre diffuse sphere
  `Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.3))))`,
  the ground diffuse sphere
  `Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.8))))`
  and the metal sphere
  `Sphere::new(Vec3::new(0.85, -0.25, -1.3), 0.25, Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.3)))`.
  The ground sphere's top is the plane `y = -0.5`.
- `Cargo.toml` still has no dependency, `src/` has no `unsafe`, and rendering stays
  single-threaded.

## Tests

- `src/material.rs` gained
  `a_diffuse_material_keeps_its_albedo_and_scatters_away_from_the_surface`,
  `a_metal_with_no_fuzz_reflects_about_the_normal` and
  `a_metal_reflection_into_the_surface_is_absorbed`.
- `src/main.rs` lost `the_center_pixel_shows_the_sphere_facing_the_camera` because normal colouring
  is gone. `the_top_row_is_sky`, `the_same_seed_gives_the_same_sequence` and both `src/sphere.rs`
  tests keep their assertions, the sphere tests passing a `Lambertian` to `Sphere::new`.
- Budget used: 3 new test functions, 1 deleted. The fast gate and the rendered-image conditional
  gate passed at tested head `4869d5f`, base `e180476`.

## Documentation state

- `docs/requirements.md` and `README.md` are accurate for this branch and need no change: items 1,
  2, 3, 5 and 6 describe the finished behaviour, and glass and sampling remain later phases.
- The repository map in `AGENT-INSTRUCTIONS/LOCAL/project.md` names `material.rs` as the `Material`
  trait and the diffuse and metal materials.
- `src/vec3.rs` keeps `Vec3::cross` with its dead-code allowance and its purpose comment; the
  geometry plan's branch removes both.

## Carry-overs into A2

- `Material::scatter` and the `Lambertian::new` and `Metal::new` constructors carry no doc comments.
  A2 edits `src/material.rs` and adds them, with `scatter` stating the `(attenuation, scattered_ray)`
  order of its tuple.
- `ray_colour`'s doc comment opens with a noun phrase rather than the required imperative sentence.
- `to_byte` applies gamma 2 through `.sqrt()` with no comment naming the correction.

  A2 edits both files and fixes the two comments alongside its own work. No test is added for these.
