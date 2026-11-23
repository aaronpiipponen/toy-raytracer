## Phase B1 - Face side and texture coordinates [DONE]

Implemented and verified at commit `a729a17`. Build instructions retired. Later phases consume
these frozen contracts:

- Requirements 7 and 8 are satisfied for spheres. `HitRecord::new` records the face side and forces
  the normal against the ray, and `Sphere::hit` fills `(u, v)` from latitude and longitude. The
  sphere tests prove both, so a later shape copies the construction path instead of re-deriving it.
- `src/sphere.rs` - `a_ray_through_the_center_hits_the_near_surface` asserts `hit.t` near `1.5`,
  `hit.front_face`, and `hit.normal == Vec3::new(0.0, 0.0, 1.0)`.
  `a_ray_inside_the_sphere_hits_the_back_face_with_the_normal_against_it` asserts `!hit.front_face`
  and the flipped normal, so both face sides are proven.
  `a_sphere_hit_reports_latitude_and_longitude_texture_coordinates` asserts `u` near `0.25` and `v`
  near `0.5` for the ray through the centre toward `-z`.
- No source comment and no test name in `src/` names a plan or a phase. This holds for every later
  phase.
- `src/hittable.rs` is unchanged by this phase and still holds the `B1r` contract exactly:
  `HitRecord` with `t`, `point`, `normal`, `front_face`, `u`, `v` in that order, deriving
  `Clone, Copy, Debug`, with `#[allow(dead_code)]` on `point`, `front_face`, `u` and `v`, and
  `HitRecord::new(t, point, outward_normal, ray, u, v)` as the one construction path.
- `src/vec3.rs` - `Vec3::cross` exists, carries `#[allow(dead_code)]` because nothing calls it on
  this branch, and has no comment above it. `src/main.rs` - `mod rng;` carries
  `#[allow(dead_code)]` with no comment above it.
- Carry-over into B2: a second and third shape each build their record through `HitRecord::new`;
  the `#[allow(dead_code)]` on `front_face`, `u`, `v` and `Vec3::cross` stays until a non-test
  production reader exists, which the render loop does not provide while it colours by `normal`.
- Merge obligation from `shared-plan-a+plan-b.md` stays with plan-a's A1r and the coordinator's
  merge: `HitRecord` ends as `Clone` only with `material: Rc<dyn Material>` as its last field and
  last constructor parameter, and `Sphere::new` gains a material parameter. No plan-b phase performs
  it.
