## Phase B1 - Face side and texture coordinates (done)

**Status:** done.
**Gate level:** standard.

**Frozen contract:**
- `HitRecord` in `src/hittable.rs` holds `t: f64`, `point: Vec3`, `normal: Vec3`, `front_face: bool`,
  `u: f64`, `v: f64`. It gained no other field. Derive list superseded: `PLANS/shared/plan-a-A1+plan-b-B2.md`
  settles `Clone` only plus a `material: Rc<dyn Material>` field and a seventh constructor parameter,
  so later detailing reads that design, not this line.
- `HitRecord::new(t: f64, point: Vec3, u: f64, v: f64, ray: &Ray, outward_normal: Vec3) -> HitRecord`
  is the only construction of a record. It sets `front_face = ray.direction.dot(outward_normal) < 0.0`
  and makes `normal` the outward normal or its negation, so `normal` always points against the ray.
  `outward_normal` is unit length and points away from the surface.
- Every shape calls `HitRecord::new`; no struct literal builds a record.
- `Sphere::hit` keeps its quadratic solve and the `t_min`/`t_max` rejection, and passes texture
  coordinates taken from the unit outward normal `n`: `u = 0.5 + n.z.atan2(n.x) / (2 * PI)` and
  `v = 0.5 - n.y.asin() / PI`, both in 0..=1.
- `HittableList::hit` copies whole records and compares only `hit.t`. Superseded by the shared design:
  once the record carries a material it is no longer `Copy`, so the list must read the distance
  through `nearest.as_ref()`.
- `src/main.rs` reads `hit.normal` and renders the same image as before the phase.

**Evidence:** `cargo build`, `cargo test` (6 passed), `cargo clippy -- -D warnings`,
`cargo fmt --check` and `git diff --check` passed at `6eb43e0`; the rendered PPM was
byte-identical to the pre-phase render.

**Carry-overs for the next phase (no extra tests):**
- `src/hittable.rs:11` and `src/main.rs:6` carry comments that name a plan (`// The origin of the
  bounce rays the materials plan adds.`, `// Seeded randomness for the scattering and sampling the
  plans add.`). Delete both comments and keep their `#[allow(dead_code)]` attributes; permanent code does
  not refer to a plan.
