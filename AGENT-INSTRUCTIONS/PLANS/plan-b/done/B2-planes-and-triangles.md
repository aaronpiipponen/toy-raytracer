## Phase B2 - Planes and triangles (done)

**Status:** done.
**Gate level:** standard.

**Frozen contract:**
- `src/plane.rs` holds `pub struct Plane { pub point: Vec3, pub normal: Vec3 }` and
  `Plane::new(point: Vec3, normal: Vec3) -> Plane`. `normal` is unit length and is never
  normalized nor divided by `normal.dot(normal)`.
- `Plane::hit` rejects `denominator == 0.0` and a `t` outside `t_min..t_max`, and passes `u` and
  `v` as `point.dot(axis).rem_euclid(1.0)`. The axes come from `plane_axes(normal)`:
  `u_axis = normal.cross(helper).unit()` where `helper` is the world axis with the smallest
  absolute component of `normal`, and `v_axis = normal.cross(u_axis)`. For the scene normal
  `(0, 0, 1)` the axes are `(0, 1, 0)` and `(-1, 0, 0)`.
- `src/triangle.rs` holds `pub struct Triangle { pub a: Vec3, pub b: Vec3, pub c: Vec3 }` and
  `Triangle::new(a: Vec3, b: Vec3, c: Vec3) -> Triangle`. `Triangle::hit` is Möller-Trumbore
  without culling: it rejects `determinant.abs() < 1e-8`, a `u` outside `0..=1`, `v < 0.0` or
  `u + v > 1.0`, and a `t` outside `t_min..t_max`. `u` is the weight of `b` and `v` the weight of
  `c`. The outward normal is `(b - a).cross(c - a).unit()`.
- The `u` range check is written `!(0.0..=1.0).contains(&u)`. Clippy `manual_range_contains`
  rejects the two-comparison form, and the behaviour is identical for every non-NaN `u`.
- Both shapes pass the six-parameter `HitRecord::new`; neither normalizes its own normal nor flips
  the returned one.
- `src/main.rs` declares `#[allow(dead_code)] mod plane;` and `mod triangle;`, imports
  `triangle::Triangle`, and `scene()` adds one `Triangle` after the two spheres with vertices
  `(-3.5, 0.0, -2.0)`, `(-1.5, 0.0, -2.0)` and `(-2.5, 1.5, -2.0)`. No plane is in the scene.
- `Vec3::cross` keeps its signature, has a caller, and carries neither `#[allow(dead_code)]` nor a
  comment naming it.
- The comments that named a plan at `src/hittable.rs:11` and `src/main.rs:6` are deleted and their
  `#[allow(dead_code)]` attributes stay.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` names `plane.rs` and `triangle.rs` in the repository map.

**Evidence:** `cargo test plane` 1 passed, `cargo test triangle` 2 passed, `cargo test` 9 passed,
`cargo build`, `cargo clippy -- -D warnings`, `cargo fmt --check` and `git diff --check` passed at
`a5c6665`. The rendered 200 by 100 PPM changes in 925 pixels of the upper-left sky, every one now
`127 127 255`, with line 7441 changing from `182 211 255` to `127 127 255` and the header
unchanged. Three test functions were added and none changed or deleted.
