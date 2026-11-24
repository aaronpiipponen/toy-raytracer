# A1r3 - Shared scene settlement (frozen contract)

## Guarantees later phases build on

- The shared scene design settled with the geometry plan lives in
  `PLANS/shared/plan-a-A1+plan-b-B2.md`. This branch records it and implements no part of it.
- A1's branch work is unchanged: `render(width, height)` with its two arguments, one
  `Rng::new(SEED)` inside `render`, the 50-bounce material `ray_colour` with `sky_colour`, gamma 2 in
  `to_byte`, and `scene()` holding the red-brown diffuse centre sphere, the mid-grey diffuse ground
  sphere and the grey metal sphere with fuzz 0.3. Nothing in this settlement adds or changes code on
  the branch.
- On plan-a's side the release merge:
  - replaces the ground sphere line with B2's ground plane
    `Plane::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), material)` carrying a
    `Lambertian` mid-grey `(0.8, 0.8, 0.8)`, and keeps the metal sphere
    `Metal::new(Colour::new(0.8, 0.8, 0.8), 0.3)` at `Vec3::new(0.85, -0.25, -1.3)` with radius
    `0.25`;
  - gives `Plane` and `Triangle` a `material: Rc<dyn Material>` field taken as the constructor's last
    argument, `Plane::new(point, normal, material)` and `Triangle::new(a, b, c, material)`, and
    passes `Rc::clone(&self.material)` last to the one `HitRecord::new(...)` constructor. No second
    construction path is added. B2's normal normalisation and the plane's texture mapping are
    unchanged by the extra argument;
  - composes `scene()` in the order centre sphere, ground plane, triangle, metal sphere, with the
    four materials: centre `Lambertian` `(0.7, 0.3, 0.3)`; ground plane `Lambertian`
    `(0.8, 0.8, 0.8)`; triangle `Lambertian` `(0.35, 0.5, 0.25)`; metal sphere `Metal`
    `(0.8, 0.8, 0.8)` with fuzz `0.3`;
  - keeps the borrow-based nearest record in `HittableList::hit`,
    `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);`, and adds B3's box test before each
    object's `hit`. The chosen hit does not change;
  - keeps the render settlement of A1r2 and plan-c's C3r: A1's material loop lands inside the output
    plan's `render(width, height, samples)`, with the single `Rng`, linear averaging, the single
    gamma in `to_byte` and the progress calls. This settlement adds no render change.
- At the merge every `Plane` and `Triangle` construction site, including B2's own tests, passes a
  material; the tests may share one matte value.
- A1's deletion of `the_center_pixel_shows_the_sphere_facing_the_camera` stands. The top row stays
  sky, so `the_top_row_is_sky` is unchanged. B2's plane and triangle tests keep their geometry
  assertions and gain a material argument, B3's box tests keep their assertions, and no test
  asserts sample-count invariance in this pair.
- A2's glass sphere is added on top of this composition; this design does not name it.

## Documentation state

- No `docs/` page change: `docs/requirements.md` item 1 states every object has a material, item 9
  that the scene holds planes and triangles, and item 10 the box test. B2 deletes the `Vec3::cross`
  dead-code allowance and the purpose comment above it, so no note about either remains.
- Neither branch's permanent files name this settlement; the release merge is bound by
  `PLANS/shared/plan-a-A1+plan-b-B2.md`.
- This phase changed plan documents only: no product code, test, `docs/` page, `Cargo.toml` or
  configuration. Its verification was the documentation-only gate: the changed plan documents were
  re-read and `git diff --check` was clean.
