## Phase B2r - Shared scene settlement

**Status:** active.
**Depends on:** none. B2 keeps its existing detail, and B2 resumes after this phase.
**Scope:** record the scene composition settled with the materials plan in
`AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md`: B2's branch shape, the merged scene, the
material argument the merge gives the plane and the triangle, the scene list's borrowed nearest
record and B3's box test, and the release merge's obligations on plan-b's side. Documentation only.
**Gate level:** minimal - the phase records one settlement in plan documents. It changes no product
code, test, `docs/` page, configuration or dependency.
**Simplicity statement:** this phase records one scene composition and the merge points it touches.
It does not reopen the hit-record settlement of A1r and B1r or the render settlement of A1r2 and C3r,
does not add a shape, material, test or `docs/` page, and does not implement the materials plan's
branch work. It names only the composition points the merge changes: the material argument on
`Plane` and `Triangle`, the four scene materials, the scene list's borrowed record and B3's box test.

**Current state (verified):**
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md` - the shared design: the target merged
  `scene()` order, the materials on `Plane` and `Triangle`, the record and scene-list shape, B3's box
  test and the test and documentation state at the merge.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/full-plan.md` - the B2 and B3 rows and scoped entries; B2 is the
  plane and triangle phase and B3 owns `Aabb` and `bounding_box`.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/phases/B2-planes-and-triangles.md` - B2's detail: `Plane` and
  `Triangle` with no material, the ground plane and triangle in `scene()`, the centre sphere kept,
  the six-argument `HitRecord::new`, `Vec3::cross` losing its dead-code allowance and purpose
  comment, and the three tests. It is byte-identical after this phase.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/B1r-shared-hit-record-shape.md` - the six-argument
  `HitRecord::new` that sets the face side and the ray-facing normal, and the derived `Copy`.
- `src/hittable.rs:9-42` - the current tree: the six-argument `HitRecord::new` and the
  `Clone, Copy, Debug` derives that hold with no material field.
- `src/hittable.rs:66` - `let limit = nearest.map_or(t_max, |hit| hit.t);` in `HittableList::hit`,
  the by-value form that copies the record.
- `src/main.rs:29-34` - `scene()` with the centre sphere and the ground sphere, before B2 replaces
  the ground sphere with the plane and adds the triangle.
- `src/vec3.rs:22-29` - `Vec3::cross` with its `// For the triangle normals.` comment and
  `#[allow(dead_code)]`, which B2 deletes.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A1-diffuse-and-metal-materials.md` - the materials plan's
  branch shape: the material field on `HitRecord`, `Rc<dyn Material>`, the bounce loop and the
  material scene values this settlement composes with.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md` - the only authority for the merged
  scene, the material values and the merge obligations.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/full-plan.md` - the B2r and B2 rows, the B2 and B3 scoped entries
  and the binding design rules.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/phases/B2-planes-and-triangles.md` - every branch symbol, value
  and test named below.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/B1r-shared-hit-record-shape.md` - the constructor and the
  record shape the merge relies on.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A1-diffuse-and-metal-materials.md` - the material field,
  `Lambertian`, `Metal` and the scene values the merge brings.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1r-shared-hit-record-settlement.md` - the merged record
  with `material: Rc<dyn Material>`, `Clone` only, and the one `HitRecord::new` path.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/scope.md` - the authorized unit, the branch and the dependency
  fence.

**Build plan:**
1. `AGENT-INSTRUCTIONS/PLANS/plan-b/phases/B2r-shared-scene-settlement.md` (this file) - record
   plan-b's settlement: the branch half in `## Branch shape`, the merge obligations in
   `## Release merge obligations`, and the test and documentation state in
   `## Tests at the merge` and `## Documentation state`.
2. No other file in plan-b's folder. Plan-b's orchestrator owns `full-plan.md`, `scope.md`, `done/`
   and the status rows; this phase writes only this file. Do not edit
   `phases/B2-planes-and-triangles.md`; B2's detail is byte-identical after this phase.

**Callers / wiring to update:**
- None: no product symbol, test, configuration key or `docs/` page changes. The settlement is
  recorded here, and B2 resumes behind it with its existing detail.

**Config / schema / migrations:**
- None: no configuration key, migration, generated file or persisted record.
- Retained records and their named consumer/owner: none; the settlement lives in this plan file and
  the shared design only.

**Touched files:**
<!-- touched-files:start -->
- `AGENT-INSTRUCTIONS/PLANS/plan-b/phases/B2r-shared-scene-settlement.md` - created: records plan-b's branch scene shape and the release merge's obligations on plan-b's side
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Documentation only: change no product code, test, `docs/` page, `Cargo.toml` entry or
  configuration.
- Do not change `phases/B2-planes-and-triangles.md`; its detail stays byte-identical. Do not
  re-detail B2 or B3 here.
- Do not edit plan-b's `full-plan.md`, `scope.md`, `done/` or status rows; plan-b's orchestrator owns
  its plan state.
- Do not reopen the hit-record settlement of A1r and B1r or the render settlement of A1r2 and C3r.
  Reference them; do not restate a competing contract.
- Do not implement, describe as implemented, or claim a product behaviour change for the merged
  scene on this branch. The scene composition below is the release merge's contract.
- Do not edit plan-a's files.
- Do not add tests, a verification receipt or a code safety commit. The evidence is this file and
  the re-read of the plan documents.

## Branch shape

B2's branch work is unchanged as detailed. On this branch `Plane` and `Triangle` carry no material,
every hit goes through the six-argument `HitRecord::new(ray, t, point, outward_normal, u, v)`, and
`scene()` holds the centre sphere, the ground plane at `y = -0.5` with normal `(0.0, 1.0, 0.0)` and
the triangle with vertices `(-2.0, -0.5, -2.0)`, `(2.0, -0.5, -2.0)` and `(0.0, 1.5, -2.0)`. B2
removes the `Vec3::cross` dead-code allowance and purpose comment. `HittableList::hit` copies whole
records through the by-value `nearest.map_or(...)` and needs no edit on this branch. Nothing in this
settlement adds or changes code on the branch.

## Release merge obligations

The release merge combines this branch with the materials plan's branch. On plan-b's side:

- Every `Plane` and `Triangle` construction site passes a material as the constructor's last
  argument: `Plane::new(point, normal, material)` and `Triangle::new(a, b, c, material)`. That
  includes the three constructors in B2's own tests, which may share one matte value.
- `Plane` keeps B2's normalisation of the normal (`normal.unit()`) and its `(u, v)` mapping on the
  two axes the dominant normal component does not use, with `rem_euclid(1.0)` unchanged. The extra
  argument changes neither.
- The plane returns no bounding box and is always tested. The triangle's box comes from its three
  vertices without shrinking on the degenerate case B2 already rejects. B3 owns `Aabb` and the
  `bounding_box` method, and both compose with the material-carrying shapes.
- `HittableList::hit` keeps the borrowed nearest record once the record carries the material and
  loses `Copy`: `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);`. It adds B3's box test
  before each object's `hit`. The chosen hit does not change, and the by-value `map_or` and `Copy` do
  not return.
- The merged scene order is the centre sphere, the ground plane, the triangle and the metal sphere.
  Its four materials are exactly: centre `Lambertian` `(0.7, 0.3, 0.3)`; ground plane `Lambertian`
  `(0.8, 0.8, 0.8)`; triangle `Lambertian` `(0.35, 0.5, 0.25)`; metal sphere `Metal`
  `(0.8, 0.8, 0.8)` with fuzz `0.3`.
- The plane and the triangle each pass `Rc::clone(&self.material)` as the last argument of the one
  `HitRecord::new(...)` constructor. No second construction path is added.
- A1's deletion of `the_center_pixel_shows_the_sphere_facing_the_camera` stands. The top row stays
  sky, so `the_top_row_is_sky` is unchanged.

## Tests at the merge

- The plane test `a_ray_hits_the_plane_and_its_texture_coordinates_wrap` and the triangle tests
  `a_ray_through_the_triangle_hits_it_with_barycentric_coordinates` and
  `a_ray_beside_the_triangle_misses` keep their geometry assertions and only gain a material
  argument.
- B3's box tests keep their assertions.
- A1's deletion and the unchanged sky test are as stated above.
- No test asserts sample-count invariance in this pair.

## Documentation state

- No `docs/` page change: `docs/requirements.md` item 1 already says every object has a material,
  item 9 that the scene holds planes and triangles, and item 10 the box test. B2 deletes the stale
  `Vec3::cross` comment, and nothing remains for the release there.
- Neither branch's permanent files name this settlement. The release merge is bound by
  `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md`.
- This phase changes plan documents only: no product code, test, `docs/` page, `Cargo.toml` or
  configuration. The verification is the documentation-only gate: the changed plan documents are
  re-read and `git diff --check` is clean.

**Tests:**
- New/changed/deleted: none. No test function is added or changed; the settlement is documentation.
- Fast gate: not applicable: no code, test, configuration or `docs/` page changes.
- Conditional gates: none. The rendered-image gate runs with B2, not here, because this phase renders
  nothing.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required. The documentation-only gate proves acceptance: this plan document is
  re-read against the shared design and `git diff --check` is clean.
- No render, command or browser step applies because no product code changes.

**Completion checklist (gate):**
- [ ] Gate level `minimal` requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] The shared scene design in `PLAN-A1+plan-b-B2` is recorded for plan-b's branch half and for
      the release merge's obligations on plan-b's side.
- [ ] B2's detail file is byte-identical, and plan-b's `full-plan.md`, `scope.md`, `done/` and
      status rows are untouched.
- [ ] No product code, test, `docs/` page, dependency or configuration changed; no product behaviour
      is claimed for this branch.
- [ ] The plan document is re-read for accuracy and `git diff --check` is clean.
- [ ] No verification receipt or code safety commit is invented; the plan document is the change.
- [ ] On passing evidence: compress this file into `done/B2r-shared-scene-settlement.md` (frozen
      contract only) and flip the B2r status row to `done`, then promote B2. Plan-b's orchestrator
      performs the status-row edits.

**Exit:** the shared scene design is recorded for this branch and the merge, B2's detail is
unchanged, and B2 resumes with its existing detail.
