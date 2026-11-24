## Phase A2 - Glass [DONE]

Implemented and verified. Build instructions retired. Later phases consume these
frozen contracts:

- `src/hittable.rs` holds one `HitRecord`, not `Copy`, with `t`, `point`, the ray-facing `normal`,
  `front_face`, `u`, `v` and `material: Rc<dyn Material>`. `normal` always faces against the incoming
  ray.
- `HitRecord::new(ray, t, point, outward_normal, u, v, material)` is the only construction path. It
  sets `front_face = ray.direction.dot(outward_normal) < 0.0` and flips `outward_normal` when the ray
  hits a back face. Nothing else flips a normal.
- `u` and `v` are in 0..=1 and are marked `#[allow(dead_code)]` until a texture consumer exists.
  Spheres fill them by latitude and longitude from the outward normal on both roots.
- `Sphere::hit` computes the outward normal, then `theta = (-outward_normal.y).acos()`,
  `phi = (-outward_normal.z).atan2(outward_normal.x) + PI`, `u = phi / (2.0 * PI)` and
  `v = theta / PI`, and builds the record through `HitRecord::new`.
- `src/material.rs` owns `Dielectric { refractive_index: f64 }` and `Dielectric::new`. It reads
  entering and leaving from `hit.front_face` and the ray-facing `hit.normal`. It reflects on total
  internal reflection (`eta_ratio * sin_theta > 1.0`) or when one `rng.next_f64()` draw is below
  `schlick(cos_theta, eta_ratio)`, and refracts otherwise. It returns unit attenuation and never
  `None`. `refractive_index` is not clamped or validated.
- `refract` and `schlick` are private helpers in `src/material.rs`. `refract` derives the direction
  from the unit incoming direction and the ray-facing normal. The `min(1.0)` on the cosine and the
  `max(0.0)` under the square root are the only guards.
- The scene holds a glass sphere of refractive index 1.5 at `(-1.1, 0.0, -1.5)` with radius 0.5,
  beside the diffuse centre sphere, the diffuse ground sphere and the metal sphere. The bounce
  limit, gamma, the PPM writer and `Cargo.toml` are unchanged.
- Tests: `a_glass_ray_inside_reflects_on_total_internal_reflection` and
  `a_glass_ray_bends_by_snell_when_it_enters` in `src/material.rs`, and
  `a_ray_from_inside_hits_the_back_face` in `src/sphere.rs`. `a_ray_through_the_center_hits_the_near_surface`
  asserts the face side and coordinates. The metal absorption test uses a fuzz factor of 1 at a
  grazing angle, because an unfuzzed reflection about a ray-facing normal always points away.
- Known carry-over: at one sample per pixel the image carries visible grain, and a pixel whose only
  metal-fuzz sample is absorbed renders black. A3's per-pixel averaging smooths both.
