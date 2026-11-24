## B2 - Planes and triangles

**Status:** done.
**Depends on:** B1 (`done/B1-face-side-and-texture-coordinates.md`): `HitRecord` carries `front_face`,
`u` and `v`, and `HitRecord::new` is the only constructor. The cross-plan shape is settled in
`PLANS/shared/plan-a-A2+plan-b-B2.md`.
**Gate level:** standard.

**Frozen contract:**
- `src/plane.rs` holds `Plane { pub point: Vec3, pub normal: Vec3 }`, where `normal` is a unit vector
  supplied to `Plane::new(point, normal)`. `Plane::hit` reports no hit when the ray direction is
  perpendicular to the normal or `t` falls outside the exclusive `t_min..t_max` interval. It derives
  (u, v) from the hit point's position along a tangent and bitangent pair of the plane, each wrapped
  with `rem_euclid(1.0)` into `0..1`, and builds its record through the six-argument
  `HitRecord::new` with its outward normal.
- `src/triangle.rs` holds `Triangle { pub a: Vec3, pub b: Vec3, pub c: Vec3 }`, counter-clockwise seen
  from its front face. `Triangle::hit` is Möller-Trumbore: it reports no hit when the determinant is
  below `1e-9` in magnitude, when the barycentric `u` lies outside `0..=1`, when `v < 0` or
  `u + v > 1`, or when `t` falls outside the exclusive interval. Its outward normal is
  `edge1.cross(edge2).unit()` and its (u, v) are the barycentric weights of `b` and `c` relative to
  `a`, passed to `HitRecord::new` unchanged.
- Neither shape carries a material on this branch. Both build records through the six-argument
  `HitRecord::new`. The release merge adds `material: Rc<dyn Material>` to `Plane`, `Triangle` and
  both `new` signatures, and passes it to the seven-argument constructor.
- `src/main.rs` declares `mod plane;` with `#[allow(dead_code)]` and `mod triangle;`, imports
  `Triangle`, and the scene holds one triangle with vertices `(0.6, 0.15, -1.0)`,
  `(1.95, 0.15, -1.0)` and `(1.275, 0.92, -1.0)`. The triangle lies in the viewport plane at
  `z = -1`, right of the sphere and wholly against the sky, and `ray_colour` paints it
  `(0.5, 0.5, 1.0)` from its `(0, 0, 1)` normal.
- `Vec3::cross` carries no `#[allow(dead_code)]` and no comment, and both shapes call it.
- The rendered 200 by 100 image shows a flat light-blue triangle with straight edges and a horizontal
  base on the right, wholly in the sky above the ground, while the sphere keeps its shaded normal
  colours and every other pixel is unchanged.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` lists `plane.rs` and `triangle.rs` in the repository map.
  `docs/` and `README.md` are unchanged.

**Proof in the tree:** `src/plane.rs` test
`a_ray_through_the_plane_records_the_hit_and_texture_coordinates_that_wrap`, `src/triangle.rs` test
`a_ray_through_the_triangle_hits_its_front_face_with_barycentric_coordinates`, and nine passing tests
in `cargo test`.
