## Phase B1r - Face side and texture coordinates [DONE]

Implemented and verified at commit `e5859d4`. Build instructions retired. Later phases consume
these frozen contracts:

- `src/hittable.rs` - `HitRecord` holds `t`, `point`, `normal`, `front_face`, `u`, `v` in that
  field order and derives `Clone, Copy, Debug`. `normal` always points against the ray and
  `front_face` says which side was hit. `front_face`, `u` and `v` each carry
  `#[allow(dead_code)]` because nothing reads them on this branch.
- `src/hittable.rs` - `HitRecord::new(t: f64, point: Vec3, outward_normal: Vec3, ray: &Ray, u: f64,
  v: f64) -> Self` is the one construction path. It sets `front_face =
  outward_normal.dot(ray.direction) < 0.0`, flips the outward normal when the hit is a back face,
  and assigns the texture coordinates unchanged. It validates and clamps nothing. Every shape must
  build its record through it, and later shapes add no second construction path.
- `src/sphere.rs` - `Sphere::hit` computes `outward_normal = (point - self.center) / self.radius`,
  then `theta = (-outward_normal.y).acos()`, `phi = (-outward_normal.z).atan2(outward_normal.x) +
  PI`, `u = phi / (2.0 * PI)` and `v = theta / PI`, all from `std::f64::consts::PI`. `u` and `v`
  lie in 0..=1.
- `Sphere::new(center: Vec3, radius: f64)` is unchanged by this phase; plan-a's merge adds a
  material parameter to it.
- Merge obligation from `shared-plan-a+plan-b.md`: the merged tree changes `HitRecord`'s derive to
  `Clone` only, adds `material: Rc<dyn Material>` as the last field and as the last parameter of
  `HitRecord::new`, and keeps the dead-code allowances until a real reader appears. These belong to
  plan-a's A1r and the coordinator's merge, not to any plan-b phase.
- Carry-over into B1: two source comments still name plans and are removed there, because this
  phase's own rules forbid plan references in source and the rule is about source text, not
  behavior. They are `src/main.rs:6` (`// Seeded randomness for the scattering and sampling the
  plans add.`, above `#[allow(dead_code)] mod rng;`) and `src/vec3.rs:21` (`// For the triangle
  normals the geometry plan adds.`, above `#[allow(dead_code)] pub fn cross`). Both allowances
  stay, because neither item has a reader on this branch.
- `src/main.rs` still colours by `hit.normal`, so the fixed scene's image is unchanged; every hit
  in the fixed scene is a front face, so `normal` is the outward normal there.
