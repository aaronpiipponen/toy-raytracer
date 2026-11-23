## Phase A2 - Glass

**Status:** active.
**Depends on:** A1r. Its frozen contract is `done/A1r-hit-record-constructor-and-materials.md`: the `Material` trait, the `Rc<dyn Material>` on the record, the scatter-based bounce loop, the one seeded generator and gamma 2 output.
**Scope:** the `Dielectric` material in `src/material.rs` (Snell's law refraction, total internal reflection, Schlick's approximation) and one glass sphere in the scene in `src/main.rs`. The material decides entry and exit itself from the ray and the outward geometric normal. Samples per pixel, face-side fields on the record, textures, new shapes, the PNG writer, the command line and progress output stay out of this phase.
**Gate level:** standard. The phase changes the shading path of the one command the product has, with no durable state, no external boundary and no security surface; targeted tests plus the fast gate and the rendered-image conditional gate fit the risk.
**Simplicity statement:** the refractive index is used as given, with no range check, and a value at or below zero produces the direction it produces. There is no dispersion, absorption, tint, roughness or medium tracking. `Dielectric` always scatters and never returns `None`. The only numerical guards are the clamp of the cosine at `1.0` and the radicand floored at zero in `refract`, which keep the grazing and total-internal-reflection paths out of `NaN`; total internal reflection is decided before `refract` is called. No legacy path is added and none is deleted.

**Current state (verified):**
- `src/material.rs:7` - `pub trait Material { fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)>; }`, with `Lambertian` and `Metal` implementations at lines 22 and 43.
- `src/material.rs:57` - `fn reflect(direction: Vec3, normal: Vec3) -> Vec3` returns `direction - normal * (2.0 * direction.dot(normal))`; it is private and already used by `Metal`.
- `src/material.rs:61` - `fn random_unit_vector(rng: &mut Rng) -> Vec3`; private, used by both existing materials. Glass uses neither this nor a random direction, only the generator's scalar draw.
- `src/material.rs:75` - `#[cfg(test)] mod tests` holds the two material tests and imports `std::rc::Rc`.
- `src/main.rs:16` - `use material::{Lambertian, Metal};`.
- `src/main.rs:61-65` - the scene's third `world.add(Box::new(Sphere::new(...)))` adds the metal sphere at `(1.1, -0.1, -1.0)` with radius `0.4` and `Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3))`.
- `src/main.rs:49` - `fn scene() -> HittableList` is the only scene construction; `render` and `ray_colour` need no change because they call `hit.material.scatter` already.
- `src/hittable.rs:19` - `HitRecord::new(t, point, normal, material)` is the only construction; `normal` is the outward geometric normal, passed through unchanged.
- `src/sphere.rs:45` - `Sphere::hit` builds `HitRecord::new(t, point, (point - self.center) / self.radius, Rc::clone(&self.material))`; the normal is outward and unit length.
- `src/rng.rs:20` - `fn next_f64(&mut self) -> f64` returns a value in `0..1`; `Rng::new(1)` yields about `0.280835` as its first draw.
- `src/vec3.rs` - `Vec3::new`, `dot`, `length`, `unit`, `Neg`, `Mul<f64>`, `Div<f64>`; there is no `length_squared`. This phase adds none.
- `src/material.rs` has no `Dielectric`, `refract` or `schlick`.

**Read first (do not invent):**
- `docs/requirements.md` - item 4 is the contract of this phase; item 3 fixes the metal shape whose reflection test already exists.
- `src/material.rs` - the whole file: the trait, both existing materials, the private helpers and the test module this phase extends.
- `src/main.rs` - the import list at lines 11 to 20, `scene` at lines 49 to 67, and `ray_colour` at lines 28 to 47, which needs no edit.
- `src/hittable.rs` - `HitRecord` and `HitRecord::new`, whose `normal` this phase orients against the ray locally.
- `src/sphere.rs` - `Sphere::hit`, to confirm the record's normal is outward and unit length.
- `src/rng.rs` - `Rng::new` and `next_f64`; the phase's only randomness.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image conditional gate. The command line is the output plan's work, so the render writes PPM to standard output.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1r-hit-record-constructor-and-materials.md` - the material, bounce, generator and gamma contract this phase builds on.

**Build plan:**

1. `src/material.rs` - after the `impl Metal` block (around line 41), add:
   ```rust
   /// A transparent material that refracts by its refractive index.
   pub struct Dielectric {
       pub refractive_index: f64,
   }

   impl Dielectric {
       /// A transparent material with refractive index `refractive_index`.
       pub fn new(refractive_index: f64) -> Self {
           Dielectric { refractive_index }
       }
   }
   ```
2. `src/material.rs` - after `impl Material for Metal` (around line 55), add:
   ```rust
   impl Material for Dielectric {
       fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Rng) -> Option<(Colour, Ray)> {
           let front_face = ray.direction.dot(hit.normal) < 0.0;
           let normal = if front_face { hit.normal } else { -hit.normal };
           let ratio = if front_face {
               1.0 / self.refractive_index
           } else {
               self.refractive_index
           };
           let unit_direction = ray.direction.unit();
           let cos_theta = (-unit_direction.dot(normal)).min(1.0);
           let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
           let direction = if ratio * sin_theta > 1.0 || schlick(cos_theta, ratio) > rng.next_f64()
           {
               reflect(unit_direction, normal)
           } else {
               refract(unit_direction, normal, ratio)
           };
           Some((Colour::new(1.0, 1.0, 1.0), Ray::new(hit.point, direction)))
       }
   }
   ```
   `front_face` is true when the ray meets the outward face, so `ratio` is `1.0 / refractive_index` on entry and `refractive_index` on exit. `normal` is the outward normal flipped to point against the ray, which every formula below assumes. `ratio * sin_theta > 1.0` is total internal reflection. The generator draw is consumed only when refraction is geometrically possible. The attenuation is white: glass does not attenuate.
3. `src/material.rs` - after `fn reflect` (around line 59), add:
   ```rust
   fn refract(direction: Vec3, normal: Vec3, ratio: f64) -> Vec3 {
       let cos_theta = (-direction).dot(normal).min(1.0);
       let perpendicular = (direction + normal * cos_theta) * ratio;
       let parallel = normal * -(1.0 - perpendicular.dot(perpendicular)).max(0.0).sqrt();
       perpendicular + parallel
   }
   ```
   and
   ```rust
   fn schlick(cosine: f64, ratio: f64) -> f64 {
       let r0 = (1.0 - ratio) / (1.0 + ratio);
       let r0 = r0 * r0;
       r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
   }
   ```
   `direction` and `normal` are unit length and `normal` points against `direction`. `ratio` is the refractive index of the ray's side over that of the far side, so `schlick` returns the reflectance of a glass surface (about `0.04` head on and about `0.65` at 85 degrees for a ratio of `1.5` in either direction).
4. `src/main.rs:16` - `use material::{Lambertian, Metal};` becomes `use material::{Dielectric, Lambertian, Metal};`.
5. `src/main.rs`, `scene` - after the metal sphere's `world.add` at line 65, add a fourth sphere:
   ```rust
   world.add(Box::new(Sphere::new(
       Vec3::new(-1.1, -0.1, -1.0),
       0.4,
       Rc::new(Dielectric::new(1.5)),
   )));
   ```
   It mirrors the metal sphere on the left: its centre is `2.2` left of the metal sphere, so the two spheres do not overlap, their surfaces are more than `0.2` apart, and its bottom at `y = -0.5` meets the ground surface.
6. `src/material.rs`, tests - append the three tests listed under **Tests** to the existing `mod tests`, after the metal test. They use `Dielectric::new(1.5)`, `HitRecord::new`, `Ray::new` and `Rng::new`; no new imports are needed because the module already imports `Rc`, `super::*`, `Ray`, `Rng` and `Vec3`.
7. `src/material.rs`, prose - doc comments follow `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md`. The trait and `scatter` docs stay as they are; the new type, constructor and private helpers follow the wording above. Do not add a comment naming plans, phases or history.
8. No `docs/` page is created or changed. `docs/requirements.md` item 4 already states this behaviour and `README.md` stays accurate; read both before claiming the docs step complete.

**Callers / wiring to update:**
- `src/main.rs:16` - the `material` import gains `Dielectric`.
- `src/main.rs:61-65` - the metal sphere's addition is the insertion point; the glass sphere is added after it.
- No other caller changes. `ray_colour`, `render`, `to_byte`, `write_ppm`, `main`, `Sphere`, `HitRecord`, `HittableList` and the existing tests keep their signatures and assertions.

**Config / schema / migrations:**
- None. No configuration key, no migration, no generated file.
- Retained records and their named consumer/owner: none. The hit record and the material are per-ray working data and nothing is written to a store.

**Touched files:**
<!-- touched-files:start -->
- `src/material.rs` - changed: `Dielectric`, the refraction and Schlick helpers, and their tests.
- `src/main.rs` - changed: the material import and one glass sphere in the scene.
- Adds: nothing (no dependency, no config key, no migration, no generated file).
<!-- touched-files:end -->

**Rules / MUST NOT:**
- `Cargo.toml` gains no dependency. Use `std` only: `Rc`, `f64` maths. No `rand`, no threads.
- Do not add `unsafe`. Rendering stays single-threaded.
- Do not add `front_face`, `u`, `v`, a second `HitRecord` constructor or a struct literal for a record. The face-side fields belong to the geometry plan and arrive at the release merge; this phase computes the face decision locally inside `scatter`.
- Do not add a `Material` method, a second trait, a boxed or borrowed material, or a `Default`; `Dielectric` implements `scatter` only.
- Do not attenuate glass: return `Colour::new(1.0, 1.0, 1.0)`. Do not add tint, absorption, dispersion, roughness or per-surface medium tracking.
- Do not clamp, validate or reject the refractive index; use it as given, like the metal fuzz value.
- Do not reseed or re-create `Rng` inside the material; the generator arrives as an argument and is drawn once per refraction-capable bounce.
- Do not implement samples per pixel, jitter, shadow rays, Russian roulette, textures, new shapes, bounding boxes, or any PNG, command-line or progress work.
- Do not change the existing tests' assertions. The added glass sphere shifts the generator sequence for the pixels after it, so the centre pixel and the metal sphere's pixels change their exact colour and speckle while their materials and the assertions' outcome stay the same. The top row is computed before any glass pixel and keeps its value.
- Do not touch `src/camera.rs`, `src/ray.rs`, `src/rng.rs`, `src/vec3.rs`, `src/hittable.rs` or `src/sphere.rs`. Keep `#[allow(dead_code)]` and its comment on `Vec3::cross`.
- No comment or doc comment this phase writes or edits mentions a plan, a phase or the history of the code. Prose follows `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md`.
- At the release merge, plan-b's record makes `normal` point against the ray and adds `front_face`; the merged material reads `hit.front_face` and that `normal` in place of this branch's local flip and dot test. Do not anticipate that merge here.

**Tests:**
- New/changed/deleted, at most three new functions (REPO-RULES §3a):
  - `src/material.rs` - `a_ray_entering_glass_bends_toward_the_normal`: hit by
    `HitRecord::new(0.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Rc::new(Dielectric::new(1.5)))`,
    ray `Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.5, 0.0, -3.0f64.sqrt() / 2.0))` and
    `Rng::new(1)` returns attenuation `Vec3::new(1.0, 1.0, 1.0)` and a direction within `1e-9` of
    `Vec3::new(1.0 / 3.0, 0.0, -8.0f64.sqrt() / 3.0)`: the 30-degree ray enters and bends toward the
    normal by Snell's law. `Rng::new(1)` draws about `0.281` and the reflectance here is about
    `0.04`, so the refraction branch is chosen. If the assertion fails, the refraction or the
    selection is wrong: report the value and fix it, never weaken the assertion.
  - `src/material.rs` - `a_ray_leaving_glass_past_the_critical_angle_reflects`: the same hit, ray
    `Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 1.0))` and `Rng::new(1)` returns a
    direction within `1e-9` of `Vec3::new(0.5f64.sqrt(), 0.0, -0.5f64.sqrt())`. The ray travels
    inside the glass at 45 degrees from the normal, `1.5 * sin(45 degrees) > 1`, so total internal
    reflection sends it back inside. The generator draw is irrelevant on this path.
  - `src/material.rs` - `grazing_incidence_reflects_where_normal_incidence_refracts`: with the same
    hit and a fresh `Rng::new(1)` per call, a head-on ray
    `Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0))` scatters within `1e-9` of
    `Vec3::new(0.0, 0.0, -1.0)`, and an 85-degree ray
    `Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(85.0f64.to_radians().sin(), 0.0, -85.0f64.to_radians().cos()))`
    scatters within `1e-9` of
    `Vec3::new(85.0f64.to_radians().sin(), 0.0, 85.0f64.to_radians().cos())`. Both draw about
    `0.281`; the head-on reflectance of about `0.04` selects refraction and the 85-degree
    reflectance of about `0.65` selects reflection, so the test proves Schlick's approximation makes
    reflection more likely at grazing incidence.
- Changed existing tests: none.
- Fast gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`,
  `git diff --check`, called as `~/.cargo/bin/cargo` when `cargo` is not on `PATH`. The whole suite
  is the targeted run for this crate (one binary, finishing in well under a second); when one test
  fails, rerun it by node id, for example
  `cargo test material::tests::a_ray_entering_glass_bends_toward_the_normal`.
- Conditional gates: Rendered image, required because the scene and the shading path change. The
  single run is `cargo run --release > /tmp/opencode/a2-render.ppm` (standard output is the image
  because the command line belongs to the output plan); the user test reuses that file and does not
  render a second time. No other conditional gate applies.
- Slow gate (>2 minutes): none. The release render of the fixed 200 by 100 image takes under a
  second.

**User test (acceptance):**
- **Executor:** agent. It runs the commands and opens the image.
- 1. Use the image already written by the rendered-image conditional gate at
  `/tmp/opencode/a2-render.ppm`; do not render a second time.
- 2. Convert it with the throwaway script below (keep it in `/tmp/opencode`, never in the
  repository) and open the resulting PNG with the image reader:
  ```python
  import sys, zlib, struct
  data = open(sys.argv[1], "rb").read().split()
  w, h = int(data[1]), int(data[2])
  vals = [int(v) for v in data[4:]]
  raw = b"".join(b"\x00" + bytes(vals[r * w * 3:(r + 1) * w * 3]) for r in range(h))
  def chunk(t, b):
      return struct.pack(">I", len(b)) + t + b + struct.pack(">I", zlib.crc32(t + b) & 0xFFFFFFFF)
  png = (b"\x89PNG\r\n\x1a\n"
         + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0))
         + chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b""))
  open(sys.argv[2], "wb").write(png)
  ```
  Run it as `python3 /tmp/opencode/ppm2png.py /tmp/opencode/a2-render.ppm /tmp/opencode/a2-render.png`.
- 3. Observe in the opened image: the blue-white gradient sky, the yellow-green ground, the matte
  red-brown centre sphere and the grey metal sphere on the right keep their materials and general
  appearance. Their exact speckle and colour differ from A1r's because the scene and the generator
  sequence changed. A fourth sphere sits to the left of the centre sphere. It is transparent: its interior shows the sky and the
  ground bent and shifted rather than a flat colour, and its rim is brighter or darker than its
  interior where total internal reflection turns the ray back. It does not look like a matte or
  metal surface.
- 4. The image keeps the expected single-sample speckle and neither sphere casts a shadow, as A1r
  left them. A3 removes the speckle, so it is not to be fixed here.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `Dielectric` exists in `src/material.rs` with `pub refractive_index` and
  `pub fn new(refractive_index: f64) -> Self`, and implements `Material::scatter` only.
- [ ] Entry and exit are derived from `ray.direction.dot(hit.normal)`; the outward normal is flipped
  locally and no `front_face`, `u` or `v` field was added.
- [ ] Refraction follows Snell's law, total internal reflection is decided before refraction may be
  chosen, and Schlick's approximation chooses reflection otherwise.
- [ ] Glass returns a white attenuation and never `None`.
- [ ] The scene holds the glass sphere at `(-1.1, -0.1, -1.0)` with radius `0.4` and refractive index
  `1.5`, and the other three spheres keep their materials.
- [ ] `src/material.rs` and `src/main.rs` are the only changed source files; `Cargo.toml` is
  unchanged.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports three test functions added and no
  test function changed or deleted.
- [ ] Fast phase gate and the rendered-image conditional gate pass; the agent viewed the rendered
  image and it matches the **User test** description.
- [ ] `docs/requirements.md` and `README.md` were read and stay accurate; no `docs/` page refers to
  a plan.
- [ ] No dependency, no `unsafe`, no thread, no file outside the touched list.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the agent's rendered-image observation is recorded
  in it.
- [ ] On passing evidence: compress this file into `done/A2-glass.md` (frozen contract only) and flip
  the `full-plan.md` `A2` status row to `done`; `A1r` and `A1` keep their rows.

**Exit:** a ray that meets the glass sphere refracts by Snell's law on entry and exit, reflects by
Schlick's approximation with reflection growing toward grazing incidence, reflects entirely past the
critical angle, and the rendered image shows a transparent glass sphere beside the matte and metal
spheres.
