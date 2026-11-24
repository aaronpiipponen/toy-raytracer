# Shared design: plan-a-A1 + plan-b-B2 (`scene()`, shapes' materials, scene list)

Settled between the plan-a and plan-b orchestrators; plan-a leads this pair. Both plans reach this
design on their own branches; commits never move between branches, no branch is merged into another,
and the release merges both into the integration branch and resolves the textual conflicts in
`src/main.rs` and `src/hittable.rs` by the shape below.

## Target merged `scene()`

One composed scene, in this order:

1. Centre sphere: `Lambertian` (0.7, 0.3, 0.3), centre (0.0, 0.0, -1.0), radius 0.5 (plan-a's A1).
2. Ground plane: `Lambertian` mid-grey (0.8, 0.8, 0.8), through (0.0, -0.5, 0.0) with normal
   (0.0, 1.0, 0.0) (plan-b's B2 plane, the user's settled material).
3. Triangle: `Lambertian` dull green (0.35, 0.5, 0.25), vertices (-2.0, -0.5, -2.0),
   (2.0, -0.5, -2.0), (0.0, 1.5, -2.0) (plan-b's B2 triangle, the user's settled material).
4. Metal sphere: `Metal` (0.8, 0.8, 0.8) with fuzz 0.3, centre (0.85, -0.25, -1.3), radius 0.25
   (plan-a's A1; it rests on the plane and stands in front of the triangle).

Any object a plan-a phase adds after A1 (A2's glass sphere) is added on top of this composition; this
design does not name it. The ground sphere A1 gives a matte material on its own branch is replaced by
the ground plane at the merge.

## Materials on the new shapes (merge only)

- `Plane` and `Triangle` each gain a `material: Rc<dyn Material>` field, taken as the constructor's
  last argument: `Plane::new(point, normal, material)` and `Triangle::new(a, b, c, material)`.
- Each `hit` passes `Rc::clone(&self.material)` as the last argument of the one `HitRecord::new(...)`
  constructor settled by the A1+B1 pair. No new record shape, no second construction path.
- `Plane::new` keeps B2's normalisation of the normal; the plane's `(u, v)` mapping and
  `rem_euclid(1.0)` are unchanged by the extra argument.
- At the merge every `Plane`/`Triangle` construction site, including B2's own tests, passes a
  material; the tests may share one matte value.

## Hit record and scene list

- `HitRecord` stays the A1+B1 settlement's shape: `t`, `point`, the normal that faces the ray,
  `front_face`, `u`, `v`, `material: Rc<dyn Material>`, deriving `Clone` only.
- `HittableList::hit` keeps A1's borrow-based nearest record, because the record is no longer `Copy`:
  `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);` and adds B3's box test before each
  object's `hit`. A by-value `map_or` and `Copy` must not return at the merge.
- B3 owns the `Aabb` type and the `bounding_box` method. At the merge every shape implements it,
  including the material-carrying `Plane` and `Triangle`; the plane returns no box and is always
  tested, and the triangle's box is built from its three vertices without shrinking on the degenerate
  case B2 already rejects. The box test runs before `hit` per requirement 10 and must not change the
  chosen hit.

## What each plan does on its own branch

- **plan-a / A1:** unchanged as detailed — `src/material.rs`, the material field on `HitRecord`,
  materials on `Sphere` and the scene's three spheres including the ground sphere, the render loop,
  the seeded generator and gamma.
- **plan-b / B2:** unchanged as detailed — `Plane` and `Triangle` with no material, six-argument
  `HitRecord::new`, the ground sphere replaced by the plane, the triangle added, `Vec3::cross`'s dead
  code allowance and stale comment removed.
- **At the merge:** A1's ground sphere line is replaced by B2's ground plane; the metal sphere stays;
  `Plane` and `Triangle` gain the material field and argument; the scene materials are the four named
  above; B2's test constructors pass a material.
- Both resolution phases are documentation-only; neither branch lands the other's work.

## Tests at the merge

- A1's deletion of `the_center_pixel_shows_the_sphere_facing_the_camera` stands.
- The top row is still sky (the triangle's apex is at y 1.5, z -2, below the top-row ray there), so
  the sky test survives unchanged.
- B2's plane and triangle tests keep their geometry assertions and only gain a material argument;
  B3's box tests keep theirs.
- No test asserts sample-count invariance.

## Documentation state

- No `docs/` page change: requirement 1 already says every object has a material, requirement 9 that
  the scene holds planes and triangles, and requirement 10 the box test.
- B2 deletes the stale "the geometry plan adds" comment on `Vec3::cross`, so plan-a's deferred
  documentation note about that comment is dropped; nothing remains for the release there.
- Neither branch's permanent files name this settlement; the release merge is bound by this file.
