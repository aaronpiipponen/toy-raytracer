## Phase A1r - Hit record constructor and materials (done)

**Status:** done.
**Gate level:** standard.

**Frozen contract:**
- `HitRecord` in `src/hittable.rs` holds `t`, `point`, `normal` and `material: Rc<dyn Material>`, in
  that order, and derives `Clone` only. `normal` is the outward geometric normal passed through
  unchanged. `HitRecord::new(t, point, normal, material)` is the only construction of a record.
  `HittableList::hit` reads the nearest distance through `nearest.as_ref()`, so it never moves the
  record out of the `Option`.
- The face-side arguments and the `front_face`, `u` and `v` fields belong to the geometry plan and
  arrive at the release merge as `PLANS/shared/plan-a-A1+plan-b-B2.md` describes. This branch adds
  none of them.
- `src/material.rs` holds `Material::scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rng) ->
  Option<(Colour, Ray)>`. It returns the attenuation and the scattered ray, or `None` when the ray is
  absorbed. Every material takes the generator as an argument and uses no other randomness source.
- `Lambertian` scatters in the direction of the outward normal plus a random unit vector and
  attenuates by its albedo. `Metal` reflects the unit incident direction about the normal, adds the
  fuzz factor times a random unit vector, absorbs a direction that does not point away from the
  surface, and attenuates by its albedo. The fuzz value is used as given.
- `ray_colour` starts with a white attenuation, follows at most 50 bounces, multiplies the
  attenuation by each scatter's factor, returns black when a material absorbs or the limit runs out,
  and multiplies the sky gradient on a miss. Colours in `render` stay linear.
- `render` owns one `Rng::new(RNG_SEED)` per call, iterates rows from the top and columns from the
  left, and passes the same generator down, so the same options give the same bytes.
- `to_byte` applies gamma 2 before quantizing, and no other place corrects a colour.
- The scene holds a red-brown diffuse centre sphere, a yellow-green diffuse ground sphere and a grey
  metal sphere right of the centre sphere with fuzz 0.3.
- No `#[allow(dead_code)]` remains on `mod rng;` or on `HitRecord::point`, and no source comment
  names a plan. `Vec3::cross` keeps its marker because no non-test build reads it yet.
- Tests pin the diffuse attenuation and above-surface scatter, metal absorption of a direction into
  the surface, gamma quantization of a mid-grey channel to 127, the red-brown centre pixel and the
  blue top row.
- `docs/requirements.md` and `README.md` stay accurate; no `docs/` page changed.

**Evidence:** base `e180476`, tested head `b7af7f8`. `cargo build`, `cargo test` (8 passed, 0
failed), `cargo clippy -- -D warnings`, `cargo fmt --check` and `git diff --check` passed. The
rendered-image conditional gate rendered the 200 by 100 image; the opened conversion showed the sky
gradient, the yellow-green ground, the matte red-brown centre sphere, the grey metal sphere
reflecting the sky, the expected single-sample speckle and no shadows. Test functions added 3, deleted
0.

**Carry-overs:** none in code. Merge facts for the release:
- The command line and PNG writer plan owns `main`, the output path and a sample count on `render`.
  This branch's `main`, `write_ppm`, `render(width, height)` and `ray_colour` are reconciled with
  that work at the release merge; the bounce loop, the seeded generator and gamma 2 in `to_byte`
  are the contract this plan keeps.
- After this phase `hit.normal` is the outward geometric normal. Phase A2's glass material decides
  entry and exit from the ray and this normal. At the release merge the record gains `front_face` and
  a ray-facing `normal`, so that one decision is rewritten to read `hit.front_face`.
