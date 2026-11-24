## Phase A1r3 - Shared scene settlement

**Status:** active.
**Depends on:** none. A1 and A1r2 keep their existing detail, and A1 resumes after this phase.
**Scope:** record the scene composition settled with the geometry plan in
`AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md`: A1's branch shape, the merged scene, the
materials the merge gives the plane and the triangle, the scene list's borrowed nearest record and
the release merge's obligations on plan-a's side. Documentation only.
**Gate level:** minimal - the phase records one settlement in plan documents. It changes no product
code, test, `docs/` page, configuration or dependency.
**Simplicity statement:** this phase records one scene composition and the merge points it touches.
It does not reopen the hit-record settlement of A1r and B1r or the render settlement of A1r2 and
plan-c's C3r. It adds no shape, material, test or `docs/` page, and it does not implement the
geometry plan's branch work. It names only the composition points the merge changes: the ground
object, the material argument on the two new shapes, the four scene materials and the scene list's
record handling and box test.

**Current state (verified):**
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md` - the shared design: the target merged
  `scene()` order, the materials on `Plane` and `Triangle`, the record and scene-list shape, B3's box
  test and the test and documentation state at the merge.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - the A1 row and the scoped A1 entry; A1 is the
  product phase this settlement unblocks, and A2 and A3 remain later product phases on this branch.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A1-diffuse-and-metal-materials.md` - A1's detail: on this
  branch `render(width, height)`, one `Rng::new(SEED)`, `ray_colour` with `sky_colour`, gamma 2 in
  `to_byte`, and the three spheres (centre diffuse, ground diffuse, metal). It is byte-identical
  after this phase.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1r-shared-hit-record-settlement.md` - the merged
  `HitRecord` shape, the one constructor, the material field and the test-record obligation.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1r2-shared-render-settlement.md` - the merged render
  signature, the single generator, the nesting, gamma and the progress ownership.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/done/C3r-shared-render-settlement.md` - the output plan's half of
  the same render settlement, so this phase can point at the contract it must not reopen.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/full-plan.md` and
  `AGENT-INSTRUCTIONS/PLANS/plan-b/phases/B2-planes-and-triangles.md` - B2's branch shape: `Plane`
  and `Triangle` with no material, the six-argument `HitRecord::new`, the ground plane at `y = -0.5`
  and the triangle in the scene, the centre sphere kept, and `Vec3::cross` losing its dead-code
  allowance and purpose comment.
- `src/main.rs:32-37` and `src/hittable.rs:34-42` - the pre-A1 branch tree: the centre sphere and the
  ground sphere in `scene()` and the by-value `nearest.map_or(...)` in `HittableList::hit`. A1
  changes both as detailed; this phase changes neither.
- `src/vec3.rs:21-29` - `Vec3::cross`, which B2 uses and whose comment this phase does not touch.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md` - the only authority for the merged
  scene, the material values and the merge obligations.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - the A1r3 and A1 rows, the A1 scoped entry and the
  carry-overs this phase must stay consistent with.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A1-diffuse-and-metal-materials.md` - every branch symbol
  and scene value named below.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1r-shared-hit-record-settlement.md` - the record shape and
  material field the merge relies on.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1r2-shared-render-settlement.md` - the render shape the
  merge relies on.
- `AGENT-INSTRUCTIONS/PLANS/plan-c/done/C3r-shared-render-settlement.md` - the output plan's half of
  the same render settlement.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/phases/B2-planes-and-triangles.md` - the exact plane and triangle
  constructors, the plane's `(u, v)` mapping, the triangle's vertices and B2's tests.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/full-plan.md` - B2's scoped entry and B3's `Aabb` and box-test
  scoped entry.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/scope.md` - the authorized unit, the branch and the dependency
  fence.

**Build plan:**
1. `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A1r3-shared-scene-settlement.md` (this file) - record
   plan-a's settlement: the branch half in `## Branch shape`, the merge obligations in
   `## Release merge obligations`, and the test and documentation state in
   `## Tests at the merge` and `## Documentation state`.
2. `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - confirm the recorded A1r3 status row (`active`,
   gate level `minimal`, this phase file, exit "the shared scene design is recorded for this branch
   and the merge", move-on gate "plan documents re-read, `git diff --check` clean, no product code
   moved"), the scoped `### A1r3` entry, the A1 row (`planned`, depending on `A1r3`, existing
   `phases/A1-diffuse-and-metal-materials.md`) and A3's dependency on `A1 + A1r2`. Change nothing
   else.
3. No other file. Do not edit `phases/A1-diffuse-and-metal-materials.md`; A1's detail is
   byte-identical after this phase.

**Callers / wiring to update:**
- None: no product symbol, test, configuration key or `docs/` page changes. The only wiring is the
  status row and scoped entry in `full-plan.md`, with A1 resuming behind A1r3.

**Config / schema / migrations:**
- None: no configuration key, migration, generated file or persisted record.
- Retained records and their named consumer/owner: none; the settlement lives in this plan file and
  the shared design only.

**Touched files:**
<!-- touched-files:start -->
- `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A1r3-shared-scene-settlement.md` - changed: records plan-a's branch scene shape and the release merge's obligations on plan-a's side
- Adds: nothing
<!-- touched-files:end -->
**Rules / MUST NOT:**
- Documentation only: change no product code, test, `docs/` page, `Cargo.toml` entry or
  configuration.
- Do not change `phases/A1-diffuse-and-metal-materials.md`; its detail stays byte-identical. Do not
  re-detail A1, A2 or A3 here.
- Do not reopen the hit-record settlement of A1r and B1r or the render settlement of A1r2 and
  plan-c's C3r. Reference them; do not restate a competing contract.
- Do not implement, describe as implemented, or claim a product behaviour change for the merged
  scene on this branch. The scene composition below is the release merge's contract.
- Do not edit plan-b's files, including `full-plan.md`, `scope.md`, `done/` and its phase files.
- Do not add tests, a verification receipt or a code safety commit. The evidence is this file and
  the re-read of the plan documents.

## Branch shape

A1's branch work is unchanged as detailed. A1 as detailed keeps `render(width, height)` with its two
arguments, creates one `Rng::new(SEED)` inside `render`, scatters `ray_colour` through `Material`
with the `sky_colour` fallback, applies gamma 2 in `to_byte`, and gives `scene()` three spheres: the
red-brown diffuse centre sphere, the mid-grey diffuse ground sphere and the grey metal sphere with
fuzz 0.3. Nothing in this settlement adds or changes code on the branch.

## Release merge obligations

The release merge combines this branch with the geometry plan's branch. On plan-a's side:

- The ground sphere line becomes B2's ground plane: `Plane::new(Vec3::new(0.0, -0.5, 0.0),
  Vec3::new(0.0, 1.0, 0.0), material)` with a `Lambertian` mid-grey `(0.8, 0.8, 0.8)`.
- The metal sphere stays as A1 gives it: `Metal::new(Colour::new(0.8, 0.8, 0.8), 0.3)` at
  `Vec3::new(0.85, -0.25, -1.3)` with radius `0.25`.
- `Plane` and `Triangle` each gain `material: Rc<dyn Material>` as the constructor's last argument,
  `Plane::new(point, normal, material)` and `Triangle::new(a, b, c, material)`, and each `hit` passes
  `Rc::clone(&self.material)` as the last argument of the one `HitRecord::new(...)` constructor. No
  second construction path is added.
- The merged scene order is the centre sphere, the ground plane, the triangle and the metal sphere.
  Its four materials are exactly: centre `Lambertian` `(0.7, 0.3, 0.3)`; ground plane `Lambertian`
  `(0.8, 0.8, 0.8)`; triangle `Lambertian` `(0.35, 0.5, 0.25)`; metal sphere `Metal`
  `(0.8, 0.8, 0.8)` with fuzz `0.3`.
- `HittableList::hit` keeps the borrow-based nearest record,
  `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);`, and adds B3's box test before each
  object's `hit`. The chosen hit does not change.
- A1's deletion of `the_center_pixel_shows_the_sphere_facing_the_camera` stands. The top row stays
  sky, so `the_top_row_is_sky` is unchanged.
- The render settlement of A1r2 and plan-c's C3r still applies: the merge lands A1's material loop
  inside the output plan's `render(width, height, samples)`, keeps the single `Rng`, the linear
  averaging, the single gamma in `to_byte` and the progress calls. This settlement adds no render
  change.

## Tests at the merge

- B2's plane and triangle tests keep their geometry assertions and only gain a material argument;
  they may share one matte value.
- B3's box tests keep their assertions.
- A1's deletion and the unchanged sky test are as stated above.
- No test asserts sample-count invariance in this pair.

## Documentation state

- No `docs/` page change: `docs/requirements.md` item 1 already says every object has a material,
  item 9 that the scene holds planes and triangles, and item 10 the box test. B2 deletes the
  `Vec3::cross` dead-code allowance and the purpose comment above it, so no note about either
  remains.
- Neither branch's permanent files name this settlement. The release merge is bound by
  `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md`.
- This phase changes plan documents only: no product code, test, `docs/` page, `Cargo.toml` or
  configuration. The verification is the documentation-only gate: the changed plan documents are
  re-read and `git diff --check` is clean.

**Tests:**
- New/changed/deleted: none. No test function is added or changed; the settlement is documentation.
- Fast gate: `git diff --check` at the workspace root plus the changed-file reread. No cargo, test or
  documentation-page gate applies because no code, test or `docs/` page changes.
- Conditional gates: none. The rendered-image gate runs with A1, not here, because this phase renders
  nothing.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required. The documentation-only gate proves acceptance: both plan documents are
  re-read against the shared design and `git diff --check` is clean.
- No render, command or browser step applies because no product code changes.

**Completion checklist (gate):**
- [ ] Gate level `minimal` requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] The shared scene design in
      `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md` is recorded for plan-a's branch half
      and for the release merge's obligations on plan-a's side.
- [ ] A1's detail file is byte-identical and the A1 row is `planned` behind A1r3 with its existing
      phase file.
- [ ] `full-plan.md` carries the A1r3 row and scoped entry, and A3 still depends on `A1 + A1r2`.
- [ ] No product code, test, `docs/` page, dependency or configuration changed; no product behaviour
      is claimed for this branch.
- [ ] The plan documents are re-read for accuracy and `git diff --check` is clean.
- [ ] No verification receipt or code safety commit is invented; the plan documents are the change.
- [ ] On passing evidence: compress this file into
      `done/A1r3-shared-scene-settlement.md` (frozen contract only) and flip the A1r3 status row to
      `done`, then promote A1.

**Exit:** the shared scene design is recorded for this branch and the merge, A1's detail is
unchanged, and A1 resumes with its existing detail.
