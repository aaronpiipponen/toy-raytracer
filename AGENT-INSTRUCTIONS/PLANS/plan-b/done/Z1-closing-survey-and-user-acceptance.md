## Z1 - Closing survey and user acceptance

**Status:** done.
**Depends on:** B1, B2 and B3 (`done/B1-face-side-and-texture-coordinates.md`,
`done/B2-planes-and-triangles.md`, `done/B3-bounding-boxes.md`), whose frozen contracts the survey
checked against the tree.
**Gate level:** minimal.

**Frozen contract:**
- The closing survey covered four areas with one read-only scout per area, run in parallel: the hit
  record and the sphere, the plane and the triangle, bounding boxes and the scene list, and the tests
  and documents. One further independent read-only scout cross-checked every claim. The five returns
  are `benchmarks/returns/area-1-hit-record-and-sphere.md`,
  `area-2-planes-and-triangles.md`, `area-3-bounding-boxes.md`, `area-4-tests-and-documents.md` and
  `cross-check.md`, all in the plan folder and kept beside the note.
- The survey record is `benchmarks/closing-survey-2026-11-24.md`. It names the surveyed areas and
  revision, and carries the confirmed findings with `file:line` evidence, the claims the cross-check
  rejected, the out-of-scope observations, the user-review list and the rendered-image evidence.
- Surveyed revision: branch `plan-b`, range `e180476..f85a183`, head `f85a183`, worktree clean. The
  phase changed no product code, test, configuration, `README.md`, `docs/` page or other permanent
  file.
- Survey verdict: no confirmed clear defect in the plan's work. The geometry, the box dispatch and the
  documents match `docs/requirements.md` items 7 to 10, the frozen B1 to B3 contracts and the plan's
  design rules. One stale line in the shared repository map,
  `AGENT-INSTRUCTIONS/LOCAL/project.md:5`, names `material.rs`, which the release merge brings in from
  the materials plan. Several low-severity nits are recorded in the note with no action. No
  remediation round was opened.
- Rendered-image evidence: `~/.cargo/bin/cargo run --release > /tmp/opencode/plan-b-acceptance.ppm`
  at `f85a183` exited 0. The PPM md5 is `13b3bc0a3cf5c41df684e179c1d3e93d`, the same digest the
  bounding-box phase recorded before and after its change. Converted to PNG and opened, the 200 by 100
  image shows the centred sphere shaded by its ray-facing normal, the flat light-blue triangle on the
  right wholly in the sky above the green ground and clear of the sphere and the frame edges, and the
  green ground.
- Test budget: no test function added, changed or deleted. The survey checked the 12 tests against
  REPO-RULES §3a and changed nothing. The fast gate is not applicable. The rendered-image conditional
  gate ran as the survey evidence and for acceptance.
- The survey note and the rendered image were reported to the user, and the user accepted both as the
  finished geometry work. No blocker was named.

**Carry-overs for the release merge:**
- `AGENT-INSTRUCTIONS/LOCAL/project.md:5` lists `material.rs`. The release merge makes the line true
  for the integration tree.
- The scene triangle's material is an open user-owned choice. The release asks the user and records the
  answer. The recommendation is a diffuse material in the scene spheres' family.
- The union design for the pair `plan-a-A3+plan-b-Z1` is
  `PLANS/shared/plan-a-A3+plan-b-Z1.md`. It holds the merge target for `HitRecord`, `HittableList::hit`,
  the shapes and the scene, and the test reconciliation: the scene triangle takes the exact vertices
  `(0.6, 0.15, -1.0)`, `(1.95, 0.15, -1.0)` and `(1.275, 0.92, -1.0)`, and its measured margins of
  2.06 px to the front sphere and 2.5 px to the right edge are valid. The render and progress seam
  belongs to `PLANS/shared/plan-a-A3+plan-c-Z1.md`.
- `docs/requirements.md:29` says "Every object reports an axis-aligned bounding box" ahead of the
  plane exception. The operative item 10 behaviour matches the code. The wording is the user's text and
  no change is proposed.
- The coverage notes in the survey (the box test's `t`-range clipping, `Sphere::bounding_box`,
  `Rng::next_f64`) need no test. Any extra test needs the user's approval under REPO-RULES §3a, and none
  is proposed.
- The other recorded nits (the `HitRecord::point` bounce-origin comment at `src/hittable.rs:12`,
  the duplicated triangle doc phrase at `src/triangle.rs:8` and `:16`) carry no action. The union
  design drops the bounce-origin comment when the scatter path reads `point`.

**Proof in the tree:** `benchmarks/closing-survey-2026-11-24.md` with its five saved returns, and the
12 passing tests at `f85a183` that the B3 receipt records.
