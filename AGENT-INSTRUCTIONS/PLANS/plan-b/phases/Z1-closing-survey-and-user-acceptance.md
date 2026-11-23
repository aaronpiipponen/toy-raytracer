## Phase Z1 - Closing survey and user acceptance

**Status:** active.
**Depends on:** `done/B1-face-side-and-texture-coordinates.md`,
`done/B2r-shared-hit-record-resolution.md`, `done/B2-planes-and-triangles.md` and
`done/B3-bounding-boxes.md`, the four frozen contracts of this plan.
**Scope:** one read-only closing survey of every area this plan added or changed, a dated note in
`AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/`, and the user's acceptance of that note. No product
code, test, configuration or permanent document changes. Findings go to the user and are fixed
outside this phase only when the user directs it.
**Gate level:** minimal - the phase changes no product code, test or configuration, and its one
artifact is a documentation note in the plan folder.
**Simplicity statement:** the survey adds one dated note with evidence for each finding and fixes
nothing. It adds no survey framework, checker, command, test, registry, index or second copy of the
contracts, and it writes no product record or receipt. A change the survey suggests is a user
decision, never work performed inside Z1.

**Execution owner:** the plan orchestrator, not `phase-implementer`. Section 3b of
`AGENT-INSTRUCTIONS/PLANNING/PLAN-AUTHORING-SCOPING.md` assigns the scout dispatch and the note
writing to the orchestrator, and the read-only `ops-scout` agent is available only to it. Do not
hand this phase to `phase_subagent` and do not ask a writer to produce the note.

**Current state (verified):**
- The plan's code head is `2b4e2b7`; `Cargo.toml` has no dependency.
- `src/hittable.rs:7-43` - `HitRecord` with `t`, `point`, `normal`, `front_face`, `u`, `v` and the
  six-parameter `HitRecord::new`. `src/hittable.rs:45-51` - the `Hittable` trait with `hit` and
  `bounding_box`. `src/hittable.rs:53-85` - `HittableList`, `add`, the nearest-hit loop with the box
  guard, and its `None` box.
- `src/sphere.rs:20-49` - `Sphere::hit` with the latitude and longitude texture coordinates and
  `Sphere::bounding_box`.
- `src/plane.rs:22-57` - `Plane::hit`, `plane_axes` and `Plane::bounding_box`.
- `src/triangle.rs:23-67` - `Triangle::hit` and `Triangle::bounding_box`.
- `src/aabb.rs:1-46` - `Aabb { min, max }`, `Aabb::new` and the slab test.
- `src/main.rs:3-13` - the module list with `mod aabb;` first. `src/main.rs:36-46` - `scene()` with
  two spheres and one triangle.
- Eleven test functions: `src/aabb.rs` 1, `src/hittable.rs` 1, `src/main.rs` 2, `src/plane.rs` 1,
  `src/rng.rs` 1, `src/sphere.rs` 3, `src/triangle.rs` 2. B1 added three, B2 added three and B3 added
  two; B2r added none.
- `AGENT-INSTRUCTIONS/LOCAL/project.md:5` - the repository map names every `src/` module, including
  `aabb.rs`.
- `docs/requirements.md:21-30` - items 7 to 10, the geometry requirements this plan implements.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/` does not exist. This plan has no earlier survey note.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANNING/PLAN-AUTHORING-SCOPING.md` section 3b - the binding survey method and
  the closing-survey rule. Follow it exactly and add no step.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/full-plan.md` - the plan scope, the binding design rules and the
  phase rows.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/B1-face-side-and-texture-coordinates.md`,
  `done/B2r-shared-hit-record-resolution.md`, `done/B2-planes-and-triangles.md` and
  `done/B3-bounding-boxes.md` - the frozen contracts the survey checks against.
- `docs/requirements.md:21-30` - items 7 to 10.
- `AGENT-INSTRUCTIONS/LOCAL/project.md`, `AGENT-INSTRUCTIONS/LOCAL/gates.md` and `README.md` - the
  repository map, the gates and the product overview.

**Build plan:**

1. Survey the nine areas below, one read-only `ops-scout` per area, dispatched in parallel. Each
   prompt names its area, the contract it is judged against, the finding classes and the evidence
   format, and does not restrict the scout to the diff.
   - **Hit record and constructor:** `src/hittable.rs:7-43`, plus every `HitRecord::new` call in
     `src/sphere.rs`, `src/plane.rs` and `src/triangle.rs`. Judged against requirement 7, requirement
     8's ranges, and the binding rule that one helper sets the normal against the ray and records the
     face side.
   - **Scene list and its nearest-hit and box loop:** `src/hittable.rs:53-85` and `ray_colour` at
     `src/main.rs:27-34`. Judged against requirement 10 and the B2r frozen contract.
   - **Sphere texture coordinates:** `src/sphere.rs:20-49`. Judged against requirement 8's latitude
     and longitude clause.
   - **Plane:** `src/plane.rs` in full, including `plane_axes`. Judged against requirements 8 and 10.
   - **Triangle:** `src/triangle.rs` in full. Judged against requirements 8, 9 and 10.
   - **Bounding box and its four implementations:** `src/aabb.rs` in full and the four
     `bounding_box` methods. Judged against requirement 10 and REPO-RULES §0.
   - **Scene's triangle object:** `src/main.rs:3-13` and `:36-46`. Judged against requirement 9.
   - **Tests across the plan:** the test modules of the six files above, all eleven functions, their
     names and their assertions. Judged against REPO-RULES §3a and requirement 14.
   - **Repository map and requirement pages:** `AGENT-INSTRUCTIONS/LOCAL/project.md:5`,
     `docs/requirements.md:21-30` and `README.md`. Judged against the current tree and REPO-RULES
     §0b.
2. Give every scout the same finding classes: unnecessary complexity, duplication, dead code,
   redundant checks or records, test bloat, stale documentation, and drift from the user's decisions.
   Each finding carries `file:line` evidence and its class. An out-of-scope observation is reported
   with the evidence already in hand and is not investigated further.
3. After all nine returns arrive, dispatch one further read-only `ops-scout` that cross-checks every
   finding together against the source, the documents and the four done contracts, and marks each
   `CONFIRMED`, `WRONG` or `UNCLEAR` with evidence.
4. The orchestrator writes `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/survey-2026-11-23.md` itself
   from the saved returns. The note holds, in this order: the date, the plan scope, the nine areas,
   the statement that no earlier survey note exists; one row per real finding with an id, class,
   `file:line` evidence, impact and recommendation; the out-of-scope observations kept separate; the
   cross-check table; the checks performed; and the user-review list. Drop every `WRONG` finding and
   put every `UNCLEAR` finding on the user-review list. No finding without `file:line` evidence.
5. Record in the note the checks the survey performs:
   - requirements 7, 8 and 9: cite the frozen contract evidence in `done/B1-...md` and
     `done/B2-...md` and confirm the current source matches; do not rerun their tests;
   - requirement 10: every `Hittable` implementor reports a box or `None` as `done/B3-...md` states,
     and the scene list tests the box before the object;
   - requirement 14 and the fast gate: the receipt at `2b4e2b7` reports `cargo build`,
     `cargo test` (11 passed), `cargo clippy -- -D warnings`, `cargo fmt --check` and
     `git diff --check`, so do not rerun them on the unchanged revision;
   - the hard invariants: `Cargo.toml` has no dependency, all randomness comes from the seeded
     `src/rng.rs`, `src/` uses no `unsafe` and no thread type, and two identical renders write
     identical bytes;
   - the test budget: eight functions added across B1, B2 and B3, at most three per phase, and no
     changed or deleted function;
   - documentation accuracy: the repository map names every module and no permanent document or code
     comment names a plan.
6. Render the scene once for the comparison and the reproducibility check, from `~/.cargo/bin/cargo`
   when `cargo` is not on `PATH`:
   ```bash
   ~/.cargo/bin/cargo run --release > /tmp/opencode/survey-a.ppm
   ~/.cargo/bin/cargo run --release > /tmp/opencode/survey-b.ppm
   cmp /tmp/opencode/survey-a.ppm /tmp/opencode/survey-b.ppm
   ```
   The header is `P3`, `200 100`, `255`, line 7441 is `127 127 255`, and `cmp` exits 0. Open
   `/tmp/opencode/survey-a.ppm` with the image reader and confirm the two spheres and the triangle
   over the sky. Record all of it in the note and remove both files afterwards.
7. Report to the user: the note path, the finding summary by class, the cross-check result and the
   user-review list. State plainly that no finding was fixed inside Z1.
8. On the user's acceptance, the advancer compresses this file into `done/Z1-closing-survey.md` and
   flips the status row. If the user names changes, return their direction to the orchestrator for a
   later survey or remediation round under section 3b. Acceptance alone does not archive the plan:
   closure needs the user's explicit authorization (`AGENT-INSTRUCTIONS/PLANS/README.md`).

**Callers / wiring to update:**
- None. No product file calls anything new and no signature changes.
- The note is read by the user; `full-plan.md` and `done/Z1-closing-survey.md` change only in the
  advancer's completion step.

**Config / schema / migrations:**
- None. No configuration key, schema, migration or generated product file is added.
- Retained records and their named consumer/owner: the dated survey note only. Its consumer is the
  user's acceptance decision, and the plan folder's `benchmarks/` directory owns it. No product
  record, receipt or hash is stored.

**Touched files:**
<!-- touched-files:start -->
- `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/survey-2026-11-23.md` - created: the closing survey note with cross-checked findings, the checks performed and the user-review list.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/full-plan.md` - changed after acceptance: the advancer flips the Z1 row to done.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/Z1-closing-survey.md` - created after acceptance: the advancer compresses this phase into the frozen contract.
- Adds: nothing in the project repository (no dependency, configuration key or generated file).
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not edit `src/`, `Cargo.toml`, `README.md`, `docs/requirements.md`, `AGENT-INSTRUCTIONS/LOCAL/**`
  or any permanent document. The only new file is the survey note.
- Do not fix any finding inside Z1, not even a one-line documentation or comment correction.
  Findings go to the user-review list, and a fix needs the user's direction in a later round.
- Do not add a test, gate, checker, command, registry, index file or receipt to run or record the
  survey.
- Do not claim a finding without `file:line` evidence confirmed by the cross-checking scout. Drop
  `WRONG` findings and put `UNCLEAR` ones on the user-review list.
- Do not rerun the fast gate on the unchanged `2b4e2b7` revision. Do not rerun the B3 before/after
  byte comparison; the survey's one render is the one Z1 owns.
- Do not recount a contract in the note as a second source of truth; cite the done contract and the
  current source.
- Do not write history, review narration or plan identifiers into a permanent file, and do not name a
  plan in code or `docs/`.
- Do not move or archive the plan folder without the user's explicit closure authorization.
- Do not add a dependency, `unsafe`, threads or a second randomness source.

**Tests:**
- New/changed/deleted: none. The phase changes no product code, so REPO-RULES §3a allows zero new
  test functions.
- Test budget: 0 new test functions, 0 changed, 0 deleted.
- Fast gate: not run on the unchanged `2b4e2b7` revision, whose receipt already reports every fast
  command. The note itself is re-read for accuracy and its paths are checked.
- Conditional gates: rendered image, run once by the plan orchestrator at 200 by 100 (step 6),
  because the survey compares the scene with the requirements and checks reproducibility.
  Agent generation: not run; no file under `AGENT-INSTRUCTIONS/SUBAGENTS/` changes.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** user.
- The plan's move-on gate is the user's acceptance of the closing survey.
- 1. Open `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/survey-2026-11-23.md` and read the findings,
  the cross-check verdicts and the user-review list.
- 2. Confirm from the note that the plan's rendered scene is the one the user expects: two spheres
  and the triangle over the sky, 200 by 100.
- 3. Reply that the survey results are accepted, or name each change to make. Any change is
  scheduled outside Z1.
- Acceptance alone does not close or archive the plan.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a for `minimal` are satisfied.
- [ ] `benchmarks/survey-2026-11-23.md` exists, is dated `2026-11-23`, and covers the nine areas the
      plan added or changed.
- [ ] Every finding carries `file:line` evidence and a cross-check verdict. No `WRONG` finding
      remains, and every `UNCLEAR` finding is on the user-review list.
- [ ] The note records the requirement checks for items 7 to 10, the three hard invariants, the test
      budget, the documentation accuracy and the statement that no earlier survey note exists.
- [ ] The 200 by 100 render has the header `P3`, `200 100`, `255`, line 7441 `127 127 255`, two runs
      are byte-identical, and the image shows the two spheres and the triangle over the sky.
- [ ] No product source, test, configuration or permanent document changed, and `git status
      --porcelain` in the worktree lists no product file.
- [ ] No test, dependency, configuration key, checker, command or receipt was added.
- [ ] Findings sit only in the note's user-review list and none was fixed inside Z1.
- [ ] The user accepted the survey results, or named the changes to make.
- [ ] On acceptance: the advancer compresses this file into `done/Z1-closing-survey.md` and flips
      the `full-plan.md` row to done. The plan is not archived without the user's explicit
      authorization.

**Exit:** the closing survey note records the plan's nine areas with cross-checked findings and a
user-review list, and the user has accepted the results or named the changes to make.
