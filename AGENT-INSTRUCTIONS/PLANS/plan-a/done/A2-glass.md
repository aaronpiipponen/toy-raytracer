## Phase A2 - Glass (done)

**Status:** done.
**Gate level:** standard.

**Frozen contract:**
- `src/material.rs` holds `Dielectric { pub refractive_index: f64 }` with
  `pub fn new(refractive_index: f64) -> Self`, and `impl Material for Dielectric` defines `scatter`
  only. It never returns `None` and never attenuates: the factor is `Colour::new(1.0, 1.0, 1.0)`.
- `Dielectric::scatter` decides the face locally: `front_face = ray.direction.dot(hit.normal) < 0.0`,
  and the working normal is the outward normal flipped to point against the ray when the ray leaves.
  The ratio is `1.0 / refractive_index` on entry and `refractive_index` on exit. The record keeps its
  outward normal and gains no field on this branch.
- Total internal reflection is decided as `ratio * sin_theta > 1.0` before refraction may be chosen.
  Otherwise Schlick's approximation, with `cos_theta` clamped at `1.0`, chooses reflection when its
  result exceeds one `rng.next_f64()` draw. The draw is consumed only when refraction is
  geometrically possible.
- `refract` floors its radicand at zero, so a grazing ray cannot produce `NaN`. `reflect` is the
  existing private helper. `schlick` and `refract` are private functions in `src/material.rs`.
- The scene's fourth sphere sits at `(-1.1, -0.1, -1.0)` with radius `0.4` and
  `Rc::new(Dielectric::new(1.5))`. The other three spheres keep their materials.
- Tests pin Snell bending on entry, total internal reflection at 45 degrees inside the glass, and
  Schlick's rise from about `0.04` head on to about `0.65` at 85 degrees. No existing test changed
  its assertion.
- `docs/requirements.md` and `README.md` stay accurate; no `docs/` page changed. `src/material.rs`
  and `src/main.rs` are the only changed source files, and `Cargo.toml` gained no dependency.

**Evidence:** base `b7af7f8`, tested head `d079bb0`. `cargo build`, `cargo test` (11 passed, 0
failed), `cargo clippy -- -D warnings`, `cargo fmt --check` and `git diff --check` passed. The
rendered-image conditional gate opened the 200 by 100 image: the glass sphere to the left of the
centre sphere shows the sky and the ground bent through its interior with a brighter or darker rim,
and the matte and metal spheres keep their materials apart from the single-sample speckle. Test
functions added 3, deleted 0.

**Carry-overs:** none in code. Merge facts for the release:
- At the release merge plan-b's record carries `front_face` and a ray-facing `normal`, so
  `Dielectric::scatter` reads `hit.front_face` and that normal in place of its local dot test and
  flip. The refractive behaviour itself is unchanged.
- The command line and PNG writer plan owns the sample count on `render` and the output path. The
  glass material and the scene's four spheres are the scene this plan keeps.
