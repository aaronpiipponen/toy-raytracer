# A2 - Glass (frozen contract)

## Guarantees later phases build on

- `src/material.rs` adds `pub struct Dielectric { pub refraction_index: f64 }` with
  `Dielectric::new(refraction_index)`. It is a `Material`; its `scatter` returns
  `Some((Colour::new(1.0, 1.0, 1.0), Ray::new(hit.point, direction)))`, so glass does not
  attenuate or absorb.
- `Dielectric::scatter` works in this order: decide the ray's side, pick the refraction ratio,
  compute `cos_theta` against the normal that faces the ray, compute `sin_theta`, take total
  internal reflection when `refraction_ratio * sin_theta > 1.0`, otherwise reflect when Schlick's
  share at `cos_theta` exceeds one `rng.next_f64()` draw, else refract. The `||` short-circuit is
  load-bearing: the random value is drawn only when the ray can refract out. Surface hits consume
  exactly that one draw.
- The private helpers in `src/material.rs` are `reflect(v, n)`, unchanged, plus
  `refract(unit_direction, normal, refraction_ratio) -> Vec3` and
  `schlick_reflectance(cos_theta, refraction_ratio) -> f64` (Schlick's approximation with
  `r0 = ((1 - ratio) / (1 + ratio))^2`). No other helper, clamp, validation or random source is
  added.
- Face side on this branch is still derived locally: `front_face` is
  `ray_in.direction.dot(hit.normal) < 0.0`, the normal is flipped to face the ray, and
  `refraction_index` divides (entering) or multiplies (leaving). `HitRecord` still has no face
  field, and `hit.normal` on this branch remains the outward normal.
- `scene()` in `src/main.rs` adds, after the metal sphere, the glass sphere
  `Sphere::new(Vec3::new(-0.85, -0.1, -1.3), 0.4, Rc::new(Dielectric::new(1.5)))`; its bottom rests
  on the ground plane `y = -0.5` and it does not overlap the centre sphere. The import is
  `use material::{Dielectric, Lambertian, Metal};`. No other scene line changed.
- The single seeded `Rng`, the 50-bounce loop, gamma 2 and the two diffuse spheres plus the metal
  sphere are unchanged. `Cargo.toml` still has no dependency and `src/` has no `unsafe`.

## Release merge obligation

`PLANS/shared/plan-a-A1+plan-b-B1.md` replaces the two-line local derivation inside
`Dielectric::scatter` with the merged record: read `hit.front_face`, and use `hit.normal`, which
then always faces the ray. Choose the ratio from `front_face` exactly as before. No face flag is
added to this branch now; the merge carries the whole change.

## Tests

- `src/material.rs` gained
  `a_glass_ray_at_normal_incidence_passes_through_the_surface`,
  `a_glass_ray_entering_at_an_angle_bends_toward_the_normal` and
  `a_glass_ray_leaving_beyond_the_critical_angle_reflects_internally`. The second test spells the
  expected `x` as `f64` because the bare literal is otherwise ambiguous for `sqrt`.
- Budget used: 3 new test functions, 0 deleted. The fast gate and the rendered-image conditional
  gate passed at tested head `80d833c`, base `4869d5f`.

## Documentation state

- `docs/requirements.md` and `README.md` are accurate for this branch and need no change: item 4
  describes glass, and sampling remains A3's.
- The repository map in `AGENT-INSTRUCTIONS/LOCAL/project.md` names `material.rs` as the `Material`
  trait and the diffuse, metal and glass materials.
- The A1 doc-comment carry-overs are closed: `Material::scatter`, `Lambertian::new` and
  `Metal::new` have doc comments, `ray_colour` opens with an imperative sentence, and `to_byte`
  names gamma 2 correction.

## Carry-overs into A3

- None. No small finding was carried over from this phase.
