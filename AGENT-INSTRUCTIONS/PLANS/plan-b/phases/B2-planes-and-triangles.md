## Phase B2 - Planes and triangles

**Status:** active.
**Depends on:** B1 (`done/B1-face-side-and-texture-coordinates.md`) and the frozen `B1r` contract.
**Scope:** add the two shapes `docs/requirements.md` items 8 and 9 name: an infinite plane and a
triangle, one module each, both building their record through `HitRecord::new`. The fixed scene's
ground becomes a plane and the scene gains a triangle, so the rendered image shows both. B2 also
removes the `#[allow(dead_code)]` on `Vec3::cross`, which both new shapes call. It adds no material,
no bounding box and no texture lookup; B3 owns boxes, and the merge obligation below owns materials.
**Gate level:** standard - two new self-contained hit routines in a single-threaded renderer, with no
durable state, no configuration and no external boundary. The rendered-image conditional gate
applies because the fixed scene changes.

**Simplicity statement:** the phase handles no edge case beyond what the arithmetic produces. A ray
that cannot meet a shape returns `None`; a degenerate triangle or a zero plane normal is not
validated; `(u, v)` are never clamped; a triangle is hit from both sides and the shared constructor
records the face side; the plane's two in-plane axes are derived per hit and never cached. It adds no
epsilon tuning beyond the exact-zero determinant test Möller–Trumbore requires, no back-face culling,
and no second record construction path.

**Merge obligation:** this branch keeps both shapes geometry-only. `Plane` and `Triangle` hold no
material and build their records through the six-argument `HitRecord::new`. The merged tree gives each
shape `pub material: Rc<dyn Material>` as its last field and constructor parameter and passes it as
the seventh `HitRecord::new` argument, per `## Shapes and materials (B2 and A1r)` in
`shared-plan-a+plan-b.md`. The merged `scene()` also differs from this branch's: it shifts the
triangle to `x` 1.6..2.3 at `z = -1.2` so the merged metal sphere does not occlude it. Both changes
are the coordinator's merge step, not this phase's work, and no file on this branch makes them.

**Current state (verified):**
- `src/hittable.rs:6-42` - `HitRecord` holds `t`, `point`, `normal`, `front_face`, `u`, `v`, derives
  `Clone, Copy, Debug`, carries `#[allow(dead_code)]` on `point`, `front_face`, `u` and `v`, and is
  built only by `HitRecord::new(t, point, outward_normal, ray, u, v)`, which sets
  `front_face = outward_normal.dot(ray.direction) < 0.0` and flips the normal for a back face.
- `src/hittable.rs:44-72` - `Hittable` declares `fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) ->
  Option<HitRecord>`; `HittableList::hit` keeps the nearest record and never reads `u`, `v` or
  `front_face`.
- `src/sphere.rs:20-46` - `Sphere::hit` is the shape template: it rejects `t` outside `t_min..t_max`,
  builds the point with `ray.at(t)`, and calls `HitRecord::new` with the outward normal.
- `src/vec3.rs:21-28` - `Vec3::cross(self, other: Vec3) -> Vec3` exists with `#[allow(dead_code)]`
  above it and no caller.
- `src/main.rs:3-17` - the module list is `camera`, `hittable`, `ray`, `#[allow(dead_code)] rng`,
  `sphere`, `vec3`; the imports are `Camera`, `Hittable`, `HittableList`, `Ray`, `Sphere`, `Colour`,
  `Vec3`.
- `src/main.rs:31-36` - `scene()` holds `Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)` and the
  ground stand-in `Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)`, whose top surface is the plane
  `y = -0.5`.
- `src/main.rs:22-29` - `ray_colour` colours a hit by `(hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5`
  and never reads `u`, `v` or `front_face`; the sky is the unaffected background.
- `src/main.rs:81-97` - tests `the_center_pixel_shows_the_sphere_facing_the_camera` and
  `the_top_row_is_sky` render the full scene; both keep passing with the scene below.
- `src/camera.rs:13-36` - the camera sits at the origin with a viewport of height 2 at `z = -1`, so a
  world point `(x, y, z)` appears at `u = (x / -z + aspect) / (2 * aspect)` and
  `v = (y / -z + 1) / 2` for `aspect = 2`.
- `docs/requirements.md:23-30` items 7 to 10 - the face side, texture coordinates for spheres, planes
  and triangles, planes as a point and a normal, triangles as three vertices, and the bounding-box
  obligation that B3 owns.

**Read first (do not invent):**
- `shared-plan-a+plan-b.md`, `## Shapes and materials (B2 and A1r)` - settles the shape and material
  contract: this branch's `Plane` and `Triangle` are geometry-only, build through the six-argument
  `HitRecord::new` and hold no `Rc`; the field, the constructor parameter and the seventh argument
  arrive only at merge. Read it for the merge shape; it changes nothing in this branch's code.
- `src/hittable.rs` - `HitRecord::new` is the only way to build a record; a shape never fills the
  fields itself.
- `src/sphere.rs` - the `hit` shape this phase copies, including the `t_min..t_max` rejection.
- `src/vec3.rs` - `cross`, `dot`, `unit`, `rem_euclid` use of `f64`, and the operator impls the two
  new modules rely on.
- `src/main.rs` - the module list, the imports, `scene()` and the two scene tests.
- `AGENT-INSTRUCTIONS/LOCAL/project.md`, `Workflow slots` - the repository-map line this phase adds
  the two modules to.

**Build plan:**
1. `src/plane.rs` - create the module with the header `//! Planes.` and:
   - `pub struct Plane { pub point: Vec3, pub normal: Vec3 }`;
   - `pub fn new(point: Vec3, normal: Vec3) -> Self` storing `normal: normal.unit()`, doc comment
     "A plane through `point` with `normal`, stored as a unit vector.";
   - a private helper `fn axes(&self) -> (Vec3, Vec3)` returning two unit axes in the plane derived
     from `self.normal`: choose `helper` as the coordinate axis of `normal` with the smallest
     absolute component (`Vec3::new(1.0, 0.0, 0.0)` when `normal.x.abs()` is no larger than the other
     two, `Vec3::new(0.0, 1.0, 0.0)` when only `normal.y.abs()` is, `Vec3::new(0.0, 0.0, 1.0)`
     otherwise), then `u_axis = helper.cross(normal).unit()` and `v_axis = normal.cross(u_axis)`;
     doc comment "Two unit axes in the plane, used for the texture coordinates.";
   - `impl Hittable for Plane` with `fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) ->
     Option<HitRecord>`: compute `let denominator = self.normal.dot(ray.direction);`, then
     `let t = (self.point - ray.origin).dot(self.normal) / denominator;`, reject with
     `if !(t_min..t_max).contains(&t) { return None; }` (an exactly parallel ray yields an infinite or
     NaN `t` here, which this test rejects; a genuine faraway hit keeps its real `t`), and return
     `Some(HitRecord::new(t, ray.at(t), self.normal, ray, u, v))` with
     `u = point.dot(u_axis).rem_euclid(1.0)` and `v = point.dot(v_axis).rem_euclid(1.0)` where
     `(u_axis, v_axis) = self.axes()`.
2. `src/triangle.rs` - create the module with the header `//! Triangles.` and:
   - `pub struct Triangle { pub vertex0: Vec3, pub vertex1: Vec3, pub vertex2: Vec3 }`;
   - `pub fn new(vertex0: Vec3, vertex1: Vec3, vertex2: Vec3) -> Self`, doc comment
     "A triangle whose corners are `vertex0`, `vertex1` and `vertex2`.";
   - `impl Hittable for Triangle` with the Möller–Trumbore hit from `vertex0`:
     `edge1 = self.vertex1 - self.vertex0`, `edge2 = self.vertex2 - self.vertex0`,
     `pvec = ray.direction.cross(edge2)`, `determinant = edge1.dot(pvec)`. Return `None` when
     `determinant == 0.0` (the ray lies in the triangle's plane). Otherwise
     `inverse = 1.0 / determinant`, `tvec = ray.origin - self.vertex0`,
     `u = tvec.dot(pvec) * inverse`; return `None` unless `(0.0..=1.0).contains(&u)`.
     `qvec = tvec.cross(edge1)`, `v = ray.direction.dot(qvec) * inverse`; return `None` when
     `v < 0.0 || u + v > 1.0`. `t = edge2.dot(qvec) * inverse`; return `None` unless
     `(t_min..t_max).contains(&t)`. Then
     `Some(HitRecord::new(t, ray.at(t), edge1.cross(edge2).unit(), ray, u, v))`: `u` and `v` are the
     barycentric weights of `vertex1` and `vertex2`, and the geometric normal is unnamed and
     unnormalised until `.unit()`. Both face sides are hit; `HitRecord::new` records the side.
3. `src/vec3.rs` - delete only the `#[allow(dead_code)]` line above `pub fn cross`; the function body
   and every other allowance stay.
4. `src/main.rs` - add `mod plane;` after `mod hittable;` and `mod triangle;` after `mod sphere;`, and
   `use plane::Plane;` and `use triangle::Triangle;` beside the other imports.
5. `src/main.rs`, `scene()` - replace the ground stand-in with a plane through `(0.0, -0.5, 0.0)`
   whose normal is `(0.0, 1.0, 0.0)`, keep `Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)`, and add
   `Triangle::new(Vec3::new(0.5, -0.5, -1.2), Vec3::new(1.2, -0.5, -1.2), Vec3::new(0.85, 0.15, -1.2))`.
   The plane's top surface and the triangle's base are at `y = -0.5`, the sphere's lowest point, and
   the triangle stands to the right of the sphere inside the viewport. This is the only scene change;
   `ray_colour`, the camera, the render loop and the output stay untouched.
6. `src/plane.rs` and `src/triangle.rs`, tests - add the three test functions below, each in its
   module's `#[cfg(test)] mod tests { use super::*; }`.
7. `AGENT-INSTRUCTIONS/LOCAL/project.md`, `Workflow slots` - the repository-map line gains
   "`plane.rs` (infinite planes), `triangle.rs` (triangles)" between `sphere.rs` and `rng.rs`. The file
   is shared by every plan's worktree, so re-read the line immediately before the edit, add only these
   two names, and leave every other module name that is already there untouched. This path is a held
   edit: load the `agent-instructions` skill before editing it, and change only that line.
8. `docs/requirements.md` - no change; items 8 and 9 already state the plane and triangle behavior,
   and the phase only makes the code match them. `README.md` - no change; report drift, do not edit it.

**Callers / wiring to update:**
- `src/main.rs` - the module list, the two imports and `scene()`; nothing else.
- `src/vec3.rs` - `Vec3::cross` keeps its body and gains two production callers, so its dead-code
  allowance goes.
- `src/hittable.rs`, `src/sphere.rs`, `src/camera.rs`, `src/ray.rs`, `src/rng.rs` - unchanged.
- `HittableList` - unchanged; it accepts the two new `Box<dyn Hittable>` values through its existing
  `add`.

**Config / schema / migrations:**
- none.
- Retained records and their named consumer/owner: none.

**Touched files:**
<!-- touched-files:start -->
- `src/plane.rs` - created: the `Plane` type, its two in-plane axes and its `Hittable` hit, plus two tests.
- `src/triangle.rs` - created: the `Triangle` type, Möller–Trumbore and its `Hittable` hit, plus one test.
- `src/main.rs` - changed: two module declarations, two imports, and the scene's ground and triangle.
- `src/vec3.rs` - changed: the dead-code allowance above `Vec3::cross` is deleted; no code change.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` - changed: the repository map gains the two modules.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Build every record through `HitRecord::new`; do not construct a `HitRecord` literal and do not
  change `HitRecord`, its derive, its fields or its allowances.
- Do not add a material, an `Rc<dyn Material>`, an `Aabb`, a `bounding_box` method or a box test on
  this branch; B3 owns boxes and the merge obligation above owns the material field.
- Do not name plan-a's new symbols (`Material`, `Lambertian`, `Metal`, `Rc<dyn Material>`) in this
  branch's source, comments or test names; the merge step adds them.
- Do not read `hit.u`, `hit.v`, `hit.front_face` or `hit.point` in production code, and do not add a
  texture lookup; nothing consumes texture coordinates yet, so their allowances stay.
- Do not clamp `u` or `v`, and do not scale them; the plane uses `f64::rem_euclid(1.0)` and the
  triangle uses the Möller–Trumbore weights unchanged.
- Do not cull a back face or flip the geometric normal in the shape; the constructor decides the
  face side and the normal for both shapes.
- Do not add an epsilon threshold, a near-parallel special case, a zero-normal check or a
  degenerate-triangle check.
- Do not change `src/main.rs`'s colouring, camera, render loop, output, image size or command line.
- Do not add or remove a dependency in `Cargo.toml`; the standard library only.
- Do not add a test function beyond the three below and do not touch the existing sphere, rng or
  scene tests (REPO-RULES §3a).
- Do not name a plan, a phase or a branch in a source comment or a test name.
- Do not edit `README.md`, `docs/requirements.md`, the plan files or `shared-plan-a+plan-b.md`.

**Tests:**
- New: `src/plane.rs` - `a_ray_through_the_plane_reports_a_hit_with_texture_coordinates`: plane at
  `(0.0, 0.0, -2.0)` with normal `(0.0, 0.0, 1.0)`, ray from `(-0.25, -0.5, 0.0)` along
  `(0.0, 0.0, -1.0)`. Assert `hit.t` is 2.0 within `1e-9`, `hit.front_face` is true, `hit.u` is 0.5
  and `hit.v` is 0.75 (the hit point's negative `x` makes `v` wrap through `rem_euclid`).
- New: `src/plane.rs` - `a_ray_parallel_to_the_plane_misses`: same plane, ray from `(0.0, -0.5, 0.0)`
  along `(1.0, 0.0, 0.0)` misses, and a ray starting on the plane at `(0.0, 0.0, -2.0)` along
  `(1.0, 0.0, 0.0)` also misses (the second ray makes `t` a NaN).
- New: `src/triangle.rs` - `a_ray_through_the_triangle_reports_barycentric_coordinates`: triangle with
  vertices `(-0.5, -0.5, -1.0)`, `(0.5, -0.5, -1.0)` and `(0.0, 0.5, -1.0)`, ray from
  `(0.0, -0.25, 0.0)` along `(0.0, 0.0, -1.0)`. Assert `hit.t` is 1.0 within `1e-9`,
  `hit.front_face` is true, `hit.u` is 0.375, `hit.v` is 0.25, and
  `hit.normal == Vec3::new(0.0, 0.0, 1.0)`.
- Changed: none. Deleted: none. The scene change adds no test function; the three tests above are the
  whole budget, and each covers one shape's hit and coordinates.
- Fast gate: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`,
  `git diff --check`, and `grep -rniE "plans?\b|phase" src/` finds nothing. The pattern matches the
  words plan, plans and phase only, so the new `plane`/`Plane` module it looks for is not a hit.
- Conditional gate: rendered image, because `scene()` changed. Until the command-line phase lands,
  render with `cargo run --release > /tmp/opencode/render.ppm` and read the image as the `User test`
  below describes. One execution owner (the implementer), once on the tested head.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** agent.
- 1. Run `cargo run --release > /tmp/opencode/render.ppm` and confirm the file's header reads `P3`,
  `200 100`, `255`.
- 2. Convert the PPM to a PNG without adding anything to the repository, using a throwaway script in
  `/tmp/opencode` (the standard library `zlib` writer is enough), then open the PNG.
- 3. Expected image: the sky gradient is unchanged at the top; the ground is a flat plane meeting the
  horizon in one horizontal line; the sphere sits on the ground at the centre with its usual
  rainbow normal colouring; a triangle with the same flat normal colouring stands to the right of the
  sphere, its base on the ground and its point up.
- 4. Confirm nothing else changed: no black pixels, no speckle along the horizon, and the triangle's
  edges are straight.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied: the three new tests and
  the fast gate pass.
- [ ] `src/plane.rs` and `src/triangle.rs` exist, both build their record only through
  `HitRecord::new`, and neither reads `hit.u`, `hit.v` or `hit.front_face`.
- [ ] `src/vec3.rs` has no `#[allow(dead_code)]` above `Vec3::cross` and the function body is unchanged.
- [ ] `src/main.rs`'s `scene()` holds the plane at `y = -0.5`, the sphere and the triangle named above;
  no other part of `src/main.rs` changed.
- [ ] `HitRecord`, `HitRecord::new`, `Sphere`, `Camera`, `Ray` and `HittableList` are unchanged.
- [ ] The three named tests pass and the seven existing test functions still pass.
- [ ] Test budget respected (REPO-RULES §3a); receipt reports three functions added and zero deleted.
- [ ] Fast phase gate passes; the rendered-image conditional gate ran once and the agent inspected the
  image; no slow gate applies.
- [ ] `AGENT-INSTRUCTIONS/LOCAL/project.md` names `plane.rs` and `triangle.rs`, and no doc or source
  comment names a plan.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists; the User test has passing evidence.
- [ ] On passing evidence: compress this file into `done/B2-planes-and-triangles.md` (frozen contract
  only) and flip the `full-plan.md` row `B2` to `done`.
- [ ] No branch file names plan-a's new symbols (`Material`, `Lambertian`, `Metal`, `Rc<dyn Material>`);
  the merged tree is the coordinator's step, following the merge checklist in
  `shared-plan-a+plan-b.md`.

**Exit:** `src/plane.rs` and `src/triangle.rs` hold the two shapes `docs/requirements.md` items 8 and 9
name, both records come from `HitRecord::new`, the fixed scene's ground is a plane and the scene holds
a triangle, and the rendered image shows the triangle on a flat ground with the sphere unchanged.
