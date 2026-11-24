# B2r - Shared scene settlement (frozen contract)

## Guarantees later phases build on

- The shared scene design settled with the materials plan lives in
  `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md`. This branch records it; the materials
  plan's work does not land here.
- B2's branch work is material-free and unchanged as detailed: `Plane` and `Triangle` carry no
  material, every hit goes through the six-argument
  `HitRecord::new(ray, t, point, outward_normal, u, v)`, `scene()` holds the centre sphere, the ground
  plane at `y = -0.5` with normal `(0.0, 1.0, 0.0)`, and the triangle with vertices
  `(-2.0, -0.5, -2.0)`, `(2.0, -0.5, -2.0)` and `(0.0, 1.5, -2.0)`. B2 removes the `Vec3::cross`
  dead-code allowance and purpose comment. `HittableList::hit` copies whole records through the
  by-value `nearest.map_or(...)` and needs no edit on this branch.
- `Plane` keeps its normalisation of the normal (`normal.unit()`) and its `(u, v)` mapping on the two
  axes the dominant normal component does not use, with `rem_euclid(1.0)` unchanged.
- The release merge combines this branch with the materials plan's branch. On plan-b's side:
  - Every `Plane` and `Triangle` construction site passes the object's material as the constructor's
    last argument, `Plane::new(point, normal, material)` and
    `Triangle::new(a, b, c, material)`, including the three constructors in B2's own tests, which may
    share one matte value.
  - Each shape's one `HitRecord::new(...)` call passes `Rc::clone(&self.material)` last. No second
    construction path appears.
  - The merged scene order is the centre sphere, the ground plane, the triangle and the metal sphere.
    Its four materials are exactly: centre `Lambertian` `(0.7, 0.3, 0.3)`, ground plane `Lambertian`
    `(0.8, 0.8, 0.8)`, triangle `Lambertian` `(0.35, 0.5, 0.25)`, metal sphere `Metal`
    `(0.8, 0.8, 0.8)` with fuzz `0.3`.
  - `HittableList::hit` keeps the borrowed nearest record once the record loses `Copy`,
    `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);`, and adds B3's box test before each
    object's `hit`. The chosen hit does not change, and the by-value `map_or` and `Copy` do not
    return.
  - The plane returns no bounding box and is always tested. The triangle's box is built from its
    three vertices without shrinking on the degenerate case B2 already rejects. B3 owns `Aabb` and
    the `bounding_box` method, and both compose with the material-carrying shapes.
  - A1's deletion of `the_center_pixel_shows_the_sphere_facing_the_camera` stands, and
    `the_top_row_is_sky` survives unchanged because the top row stays sky.

## Tests at the merge

- The plane test `a_ray_hits_the_plane_and_its_texture_coordinates_wrap` and the triangle tests
  `a_ray_through_the_triangle_hits_it_with_barycentric_coordinates` and
  `a_ray_beside_the_triangle_misses` keep their geometry assertions and only gain a material
  argument.
- B3's box tests keep their assertions.
- No test asserts sample-count invariance in this pair.

## Documentation state

- No `docs/` page change: `docs/requirements.md` item 1 states every object has a material, item 9
  that the scene holds planes and triangles, and item 10 the box test. B2 deletes the stale
  `Vec3::cross` comment, and nothing remains for the release there.
- No permanent file on this branch names this settlement. The release merge is bound by
  `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md`.
- This phase changed plan documents only: no product code, test, `docs/` page, `Cargo.toml` or
  configuration. Verification was the documentation-only gate: the plan documents were re-read
  against the shared design and `git diff --check` was clean.
