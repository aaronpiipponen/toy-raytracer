# Z1 - Closing survey and user acceptance (frozen contract)

## Guarantees later phases build on

- The closing survey covered every product change this plan made on branch `plan-b`, base `e180476`
  to head `c25007c`: `src/aabb.rs`, `src/hittable.rs`, `src/main.rs`, `src/plane.rs`,
  `src/sphere.rs`, `src/triangle.rs` and `src/vec3.rs`. Every changed path and requirements items 7
  to 10 are covered by at least one survey area.
- The finding set is the six verbatim read-only `ops-scout` area returns saved in
  `AGENT-INSTRUCTIONS/PLANS/plan-b/survey-returns-2026-11-24.md`, checked claim by claim by one
  independent read-only `ops-scout`. The dated result is
  `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/2026-11-24-closing-survey.md`, which holds the
  coverage table, the confirmed clean facts, the numbered findings with `path:line` evidence, the
  out-of-scope observations, the cross-check outcome with its rejected, corrected and added claims,
  the acceptance render and the user decision.
- The survey confirmed this branch matches all three frozen contracts (`done/B1r`, `done/B2`,
  `done/B3`) and the shared designs, with no product defect: one `HitRecord` construction path, the
  face side and against-ray normal, the three `(u, v)` mappings in range, two-sided plane and
  triangle, the box test ahead of each object with the chosen record unchanged, and the contracted
  scene and single PPM output path.
- The accepted gaps are exactly the note's findings 1 to 8: the four forward-looking `dead_code`
  allowances on `HitRecord` (expected to become live at the release merge), the unobserved
  sphere-pole `acos` and box float boundaries, the `pub` shape fields that bypass `Plane::new`
  ordering only outside the tree, box tests thinner than the B3 prose implies while the suite count
  stays in budget, requirements item 6 naming `src/rng.rs`, plus two release-bookkeeping items.
  No remediation round follows, and no finding was fixed in this phase.
- The release is left to reconcile the bookkeeping items: `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md`
  still assigns the already-resolved `main.rs` rng comment and `hittable.rs` point comment to
  plan-a's release, and the repository map in `AGENT-INSTRUCTIONS/LOCAL/project.md` still omits
  `aabb.rs`, `plane.rs` and `triangle.rs`. The map is the sole item in
  `AGENT-INSTRUCTIONS/PLANS/plan-b/docs-pass.md` and is updated once, at the release.
- The release merge's obligations are unchanged from
  `done/B2r-shared-scene-settlement.md` and the release clause of `done/B3-bounding-boxes.md`: the
  box test and every `bounding_box` method stay as written, the borrowed nearest record replaces the
  by-value form, the material arguments arrive at the merge, and the merged scene is the centre
  sphere, the ground plane, the triangle and the metal sphere with the four named materials.

## Acceptance

- The acceptance render ran from the worktree root with the branch's release binary at `c25007c`:
  two runs exited 0, `cmp` found no difference, the header is `P3` then `200 100`, and the inspected
  image shows the sky gradient, the normal-shaded centre sphere, the flat light-green ground and the
  light-blue triangle beside and above the sphere, matching requirements items 7 to 10.
- The user read the note and opened the rendered image, and accepted the survey and the scene on
  2026-11-24. The note's **User decision** section records the acceptance date and the accepted
  gaps. No defect was named and no remediation follows.

## Test surface

- No test function was added, changed or deleted; the suite stays at 13, and B1r, B2 and B3 each
  stayed within the phase maximum of three.

## Documentation state

- No product file, test, `docs/` page, `README.md`, `Cargo.toml` or `AGENT-INSTRUCTIONS/LOCAL/` file
  changed in this phase. This phase added the saved returns and the dated note as plan records and
  changed nothing that the branch's contracts already state.
