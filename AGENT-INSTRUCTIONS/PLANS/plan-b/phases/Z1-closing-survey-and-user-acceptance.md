## Phase Z1 - Closing survey and user acceptance

**Status:** active.
**Depends on:** B1, B2 and B3, whose frozen contracts are in
`done/B1-face-side-and-texture-coordinates.md`, `done/B2-planes-and-triangles.md` and
`done/B3-bounding-boxes.md`.
**Scope:** the binding closing survey of every area this plan added or changed, a dated note in the
plan's `benchmarks/`, the acceptance render of the scene, and the user's acceptance. The survey is
read-only. Findings go to the user and are not fixed here. The plan orchestrator runs the survey with
read-only `ops-scout` sessions and writes the note. No `phase-implementer` session runs for this
phase, and no code, test, configuration, README or `docs/` change is made in it.
**Gate level:** minimal. The phase writes one plan-folder note and runs one read-only render, so
`minimal` fits. The rendered-image conditional gate runs as survey evidence and for the user's
acceptance.
**Simplicity statement:** the survey runs once per area, then one cross-check, then one note. It does
not add a report generator, a checklist tool, a findings store or a test. It does not narrow its
scope to a diff-only review, and it fixes nothing. A defect confirmed in the survey goes to a
remediation round or to the user-review list, and a change that needs a whole phase of its own goes
to the user-review list. The plan is neither closed nor archived here: the release takes the plan
after this phase, and archive follows the user's explicit authorization.

**Current state (verified):**
- The plan's changes sit in the branch range `e180476..f85a183`, whose starting tree is the commit
  `e180476`. The range adds `src/aabb.rs`, `src/plane.rs` and `src/triangle.rs`, changes
  `src/hittable.rs`, `src/sphere.rs`, `src/main.rs` and `src/vec3.rs`, and removes the
  `#[allow(dead_code)]` attribute from `Vec3::cross`.
- `src/hittable.rs:10-22` - `#[derive(Clone, Copy, Debug)] pub struct HitRecord { pub t, pub point,
  pub normal, pub front_face, pub u, pub v }`.
- `src/hittable.rs:24-42` - `HitRecord::new(ray: &Ray, t: f64, point: Vec3, outward_normal: Vec3,
  u: f64, v: f64) -> Self`, the only constructor: `front_face` is
  `ray.direction.dot(outward_normal) < 0.0`, and `normal` is the outward normal or its negation so it
  faces against the ray.
- `src/hittable.rs:44-53` - `pub trait Hittable { fn hit(&self, ray: &Ray, t_min: f64, t_max: f64)
  -> Option<HitRecord>; fn bounding_box(&self) -> Option<Aabb> { None } }`.
- `src/hittable.rs:56-65` - `#[derive(Default)] pub struct HittableList { pub objects: Vec<Box<dyn
  Hittable>> }` and `HittableList::add`. `src/hittable.rs:67-83` - `impl Hittable for HittableList`
  whose `hit` walks the objects, computes `let limit = nearest.map_or(t_max, |hit| hit.t);`, skips an
  object whose `bounding_box()` misses with `bounds.hit(ray, t_min, limit)`, and keeps the nearest
  record.
- `src/hittable.rs:85-119` - a `#[cfg(test)] mod tests` holding `Unreachable` and the list test
  `the_list_tests_an_object_without_a_box_and_skips_one_whose_box_the_ray_misses`.
- `src/sphere.rs:21-52` - `impl Hittable for Sphere` with the quadratic `hit` that derives `u` from
  `atan2` and `v` from `acos` on the outward normal, and `bounding_box` returning the center plus and
  minus the radius. `src/sphere.rs:54-99` - four tests.
- `src/plane.rs:7-18` - `Plane { pub point: Vec3, pub normal: Vec3 }` with `Plane::new(point, normal)`.
  `src/plane.rs:20-41` - `Plane::hit` that reports no hit when the direction is perpendicular to the
  normal, wraps (u, v) with `rem_euclid(1.0)`, and takes the trait's default box. `src/plane.rs:43-58`
  - one test.
- `src/triangle.rs:8-20` - `Triangle { pub a, pub b, pub c }` with `Triangle::new(a, b, c)`.
  `src/triangle.rs:22-64` - Möller-Trumbore `hit` and a `bounding_box` holding the corner-wise minimum
  and maximum of the vertices. `src/triangle.rs:66-99` - two tests.
- `src/aabb.rs:1-61` - `Aabb { pub min, pub max }`, `Aabb::new`, the slab `hit` that treats an exact
  zero ray component as parallel to its slab, and one test.
- `src/main.rs:3-13` - the module list, with `mod aabb;` first. `src/main.rs:27-34` - `ray_colour`
  calls `world.hit(ray, 0.001, f64::INFINITY)` and tests no box. `src/main.rs:36-46` - `scene` holds
  two spheres and one triangle with vertices `(0.6, 0.15, -1.0)`, `(1.95, 0.15, -1.0)` and
  `(1.275, 0.92, -1.0)`. `src/main.rs:48-61` - `render`. `src/main.rs:91-107` - two scene tests.
- `src/vec3.rs:5-36` - `Vec3 { pub x, pub y, pub z }` with `new`, `dot`, `cross`, `length` and `unit`,
  and no component-index or component-wise min/max helper. `src/vec3.rs:38-81` - `Add`, `Sub`, `Neg`,
  `Mul<f64>`, `Mul<Vec3>`, `Div<f64>` and the `Colour` alias.
- `cargo test` holds 12 test functions: `src/main.rs` two, `src/rng.rs` one, `src/sphere.rs` four,
  `src/plane.rs` one, `src/triangle.rs` two, `src/aabb.rs` one, `src/hittable.rs` one.
- `docs/requirements.md:23-30` - items 7 to 10 state the face side and normal against the ray, the
  (u, v) sources for the three shapes, planes and triangles in the scene, and the box report, the
  list test and the boxless plane.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` line 5 lists `aabb.rs`, `plane.rs` and `triangle.rs` in the
  repository map. `README.md` shows `cargo run > image.ppm` and `cargo test`.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` holds the rendered-image gate command and its fallback line for
  the branch without the command line: `cargo run --release > /tmp/opencode/render.ppm`.
- The plan folder has no `benchmarks/` directory and no earlier survey note to consult. The survey
  method is the binding one in `AGENT-INSTRUCTIONS/PLANNING/PLAN-AUTHORING-SCOPING.md` §3b: one
  read-only `ops-scout` per area in parallel, then one read-only `ops-scout` cross-checking every
  finding, then the orchestrator writes the note from the saved returns.
- `AGENT-INSTRUCTIONS/SUBAGENTS/README.md` describes `ops-scout` as the general-purpose read-only
  scout with a full read-only shell.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANNING/PLAN-AUTHORING-SCOPING.md` §3b - the survey method, the scout scope
  rules, the finding classes and the closing rule. Binding.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/full-plan.md` - the status table, the binding design rules and the
  carry-overs list.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/B1-face-side-and-texture-coordinates.md`,
  `done/B2-planes-and-triangles.md` and `done/B3-bounding-boxes.md` - the frozen contracts the survey
  checks the code against.
- `src/hittable.rs`, `src/sphere.rs`, `src/plane.rs`, `src/triangle.rs`, `src/aabb.rs`, `src/vec3.rs`
  and `src/main.rs` - the changed tree.
- `docs/requirements.md` items 7 to 10, `README.md` and `AGENT-INSTRUCTIONS/LOCAL/project.md` - the
  stated behaviour, the run instructions and the repository map. All are read only.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image conditional gate.

**Build plan:**
1. Create the directory `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/`. The note is
   `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/closing-survey-<date>.md`, where the date is the day
   the survey runs, for example `closing-survey-2026-11-24.md`.
2. The orchestrator dispatches one independent read-only `ops-scout` per area below, all launched
   together so they run in parallel. Each dispatch prompt names its area, tells the scout it may read
   whatever context outside the area it needs to judge it, tells it to report an out-of-scope problem
   as an out-of-scope observation with the evidence it already has without investigating further, and
   does not restrict it to the diff. The orchestrator saves each return unchanged as
   `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/returns/area-<number>-<subject>.md`, so the note is
   written from a saved return and not from memory.
   - **Area 1, the hit record, the face side, texture coordinates and the sphere.**
     `src/hittable.rs` `HitRecord` and `HitRecord::new`, and `src/sphere.rs`, against requirement 7,
     requirement 8's sphere clause and design rule 1: `HitRecord` is the one record every shape
     returns, one helper sets the normal against the ray and records the face side, `HitRecord::new`
     is the only constructor, and the sphere derives (u, v) from the outward normal's latitude and
     longitude. Look for a second way to build or adjust a record, a redundant normal check,
     duplicated math, dead code and unnecessary complexity.
   - **Area 2, planes and triangles.** `src/plane.rs`, `src/triangle.rs` and the triangle in
     `src/main.rs` `scene`, against requirement 8's plane and triangle clauses, requirement 9 and
     design rule 2: one module per shape, the plane's (u, v) wrap into `0..1`, the triangle's
     barycentric (u, v) and Möller-Trumbore test, and the scene triangle wholly against the sky. Look
     for duplicate paths, unneeded branches, invented fallbacks, dead code and drift from the frozen
     B2 contract.
   - **Area 3, bounding boxes and the scene list.** `src/aabb.rs`, `Hittable::bounding_box`,
     `HittableList::hit` and the two overrides in `src/sphere.rs` and `src/triangle.rs`, against
     requirement 10 and the B3 contract: the slab test, the default `None` for an object of infinite
     extent, the plane and the list on the default, the box test before the object with the same
     limit, and no stored, cached or refitted box. Look for redundant checks, a second culling path,
     a box computed outside the object, dead code and unnecessary complexity.
   - **Area 4, tests and documentation.** The test modules in `src/main.rs`, `src/hittable.rs`,
     `src/sphere.rs`, `src/plane.rs`, `src/triangle.rs`, `src/aabb.rs` and `src/rng.rs`, plus `README.md`,
     `docs/requirements.md` items 7 to 10 and `AGENT-INSTRUCTIONS/LOCAL/project.md`: test necessity
     and focus against REPO-RULES §3a, no fake product behaviour, an accurate README, requirements and
     repository map, and nothing stale. Treat `AGENT-INSTRUCTIONS/LOCAL/gates.md` as
     read only.
3. The orchestrator dispatches ONE independent read-only `ops-scout` that cross-checks every finding
   from every area, claim by claim, against the code and documents with evidence. It returns each
   claim as confirmed or rejected with `file:line` evidence. There is no per-area checker.
4. Rendered-image gate on the current head, run by the orchestrator. Render the default scene from
   the real binary and inspect the converted image:
   ```bash
   ~/.cargo/bin/cargo run --release > /tmp/opencode/plan-b-acceptance.ppm
   python3 /tmp/opencode/ppm2png.py /tmp/opencode/plan-b-acceptance.ppm /tmp/opencode/plan-b-acceptance.png
   ```
   Restrict `ppm2png.py` to the conversion snippet below, saved under `/tmp/opencode/`. Open
   `/tmp/opencode/plan-b-acceptance.png` with the image reader. Confirm the 200 by 100 scene:
   a flat light-blue triangle with straight edges and a horizontal base on the right, wholly in the
   sky above the ground, the centred sphere with shaded normal colours, and the green ground. Compare
   what runs with requirements 7 to 10. Keep `/tmp/opencode/plan-b-acceptance.png` and the PPM until
   the user answers in the acceptance step below.

   ```python
   import struct, sys, zlib
   src, dst = sys.argv[1], sys.argv[2]
   tokens = open(src).read().split()
   assert tokens[0] == "P3", tokens[0]
   w, h, top = int(tokens[1]), int(tokens[2]), int(tokens[3])
   vals = [round(int(v) * 255 / top) for v in tokens[4:]]
   assert len(vals) == w * h * 3, (len(vals), w, h)
   raw = b"".join(b"\x00" + bytes(vals[y * w * 3:(y + 1) * w * 3]) for y in range(h))
   def chunk(k, d):
       return struct.pack(">I", len(d)) + k + d + struct.pack(">I", zlib.crc32(k + d) & 0xFFFFFFFF)
   png = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b"")
   open(dst, "wb").write(png)
   ```
5. The orchestrator writes the dated note from the saved returns and the render. The note states the
   surveyed areas and the surveyed revision, then lists:
   - confirmed findings with `file:line` evidence and their class: unnecessary complexity,
     duplication, dead code, a redundant check or record, test bloat, a stale document, or drift from
     the plan's design rules or the user's decisions;
   - claims the cross-check rejected, so a later reader does not reopen them;
   - out-of-scope observations with the evidence the area scout had;
   - a user-review list for anything that needs a phase of its own or a user-owned decision;
   - the rendered-image evidence from step 4: the command run and what the opened image shows.
   The note holds no fix and no plan id.
6. Report the note and the render to the user with the note's findings, the rejected claims and the
   user-review list, pointing at `/tmp/opencode/plan-b-acceptance.png`. Fix nothing in this phase.
   Clear defects and drift from the design rules go to a remediation round after the survey, and the
   plan is neither closed nor archived here. When the survey confirms a clear defect, Z1 stays active
   and the advancer opens the remediation phase that fixes it, then Z1 runs again for the user's
   acceptance. Remove `/tmp/opencode/plan-b-acceptance.ppm`, `/tmp/opencode/plan-b-acceptance.png`
   and `/tmp/opencode/ppm2png.py` once the user answers.

**Callers / wiring to update:**
- None. The phase changes no code, test, configuration or published document.

**Config / schema / migrations:**
- No configuration key, schema or migration.
- Retained record and its consumer: the dated survey note in
  `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/` is the record this phase exists to produce. Its
  consumers are the user's acceptance decision, the release and any later remediation round. The
  unchanged area returns in `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/returns/` are the scout output
  the note draws on, kept beside it for those same consumers. The note is the only copy of the
  findings and lives in the plan folder. The acceptance PNG and PPM are transient evidence under
  `/tmp/opencode/` and are removed once the user answers the acceptance step.

**Touched files:**
<!-- touched-files:start -->
- `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/closing-survey-<date>.md` - created: the dated
  closing survey note, named with the day the survey runs.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/returns/area-<number>-<subject>.md` - created: the
  unchanged return of each area scout, one file per area.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/phases/Z1-closing-survey-and-user-acceptance.md` - deleted on
  acceptance: compressed into `done/Z1-closing-survey-and-user-acceptance.md`.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/done/Z1-closing-survey-and-user-acceptance.md` - created on
  acceptance: the frozen survey outcome and the user's acceptance, with the note path.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/full-plan.md` - changed: the Z1 status row and its phase file
  path.
- Adds: the `benchmarks/` directory.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Read-only survey. Do not change product code, tests, `Cargo.toml`, `README.md`,
  `docs/requirements.md`, `AGENT-INSTRUCTIONS/LOCAL/gates.md` or any other permanent file in this
  phase.
- Do not narrow a scout to a diff-only review. Each area scout covers its whole area and reads
  whatever outside context it needs.
- Do not merge areas into one scout, add a per-area checker, or skip the single cross-check of all
  findings. Do not write the note from memory or from an unsaved return, and no implementer or other
  child writes it.
- Every finding in the note carries `file:line` evidence. A rejected claim stays in the note.
- Do not fix anything here, do not open a remediation phase inside Z1, and do not close or archive
  the plan.
- No new dependency, no `unsafe`, no threads, no new randomness, and no change to the scene or the
  camera.
- The note names no plan id and carries no fix. No history or plan id goes into source comments.

**Tests:**
- New/changed/deleted: none. The phase adds no test function, so the budget is 0 added, 0 deleted and
  0 changed. The survey checks the 12 tests against REPO-RULES §3a and may report test bloat in the
  note, but it changes nothing.
- Fast gate: not applicable. No code or test changes, so the B3 receipt's passing evidence at
  `f85a183` stands and is not rerun.
- Documentation-only gate for the note: reread the changed note and the saved returns. The plan folder
  sits in the shared `AGENT-INSTRUCTIONS/` tree outside the worktree repository, so `git diff --check`
  does not cover them.
- Conditional gates: rendered image, required by step 4 as survey evidence and for the user's
  acceptance. Agent generation: not affected.
- Slow gate (>2 minutes): none. The 200 by 100 render finishes well under a minute.

**User test (acceptance):**
- **Executor:** user. Accepting the survey findings and judging how the rendered image looks are
  user-owned judgments that no automated check proves.
- 1. Open `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/closing-survey-<date>.md` and read the
  confirmed findings, the rejected claims, the out-of-scope observations and the user-review list.
- 2. Open `/tmp/opencode/plan-b-acceptance.png` from step 4. It shows the sphere with shaded normal
  colours, the flat light-blue triangle on the right in the sky, and the green ground at 200 by 100.
- 3. Answer with acceptance of the note and the render, or with the findings and observations that
  must be handled before acceptance.
- Expected result: the user accepts the survey note and the rendered image, or names the items that
  block acceptance.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a (minimal) are satisfied.
- [ ] All four areas were surveyed by four separate read-only `ops-scout` sessions, and one further
      independent read-only `ops-scout` cross-checked every finding.
- [ ] The dated note exists in `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/` and every finding and
      rejected claim in it carries `file:line` evidence.
- [ ] The note covers unnecessary complexity, duplication, dead code, redundant checks or records,
      test bloat against REPO-RULES §3a, stale documents and drift from the plan's design rules, and
      it lists the out-of-scope observations and the user-review list.
- [ ] The rendered-image gate ran on the surveyed revision and the opened image shows the B2 and B3
      scene: the triangle in the sky, the sphere with shaded normal colours and the green ground.
- [ ] `docs/requirements.md` items 7 to 10, `README.md` and the repository map in
      `AGENT-INSTRUCTIONS/LOCAL/project.md` describe the behaviour.
- [ ] Test budget respected (REPO-RULES §3a); the phase adds, deletes and changes no test function.
- [ ] No product code, test, configuration or permanent document changed in this phase:
      `git status --porcelain` in the worktree lists no path under `src/`, `Cargo.toml`, `README.md`
      or `docs/`. The plan-folder note and returns are outside the worktree repository.
- [ ] The note and the render were reported to the user, and the user's answer is recorded on
      acceptance.
- [ ] The plan is not closed or archived in this phase.
- [ ] On the user's acceptance and with no confirmed clear defect open: compress this file into
      `done/Z1-closing-survey-and-user-acceptance.md` (the survey outcome, the note path and the
      user's acceptance) and flip the `full-plan.md` status row to `done`.

**Exit:** the dated survey note exists with every finding backed by evidence, the acceptance render
was inspected, and the user accepted the survey and the rendered result. After any confirmed clear
defect is fixed in a remediation round, the plan is ready for the release.

## Resolution record - pair plan-a-A3+plan-b-Z1

The union design in `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-b-Z1.md` is the release-merge
target for `src/hittable.rs`, `src/sphere.rs`, `src/main.rs`, `src/plane.rs` and `src/triangle.rs`.
That file holds the code blocks and the exact signatures, and they are not repeated here. The merge
must not lose these points:

- One `HitRecord` without `Copy`, carrying `material`, `front_face`, `u`, `v` and `point`, with the
  dead-code allow removed from `point`.
- The seven-argument `HitRecord::new`.
- Plan-a's borrowing `HittableList::hit`, carrying plan-b's bounding-box skip with the shared
  nearest-hit limit.
- `Sphere` with both a material and a `bounding_box`.
- `Plane` and `Triangle` with a material, keeping the B2 shapes: `Triangle` keeps its box, and
  `Plane` takes the default `None`.

This branch's B1, B2 and B3 work is the geometry side the release adapts. The scene triangle keeps
the exact vertices `(0.6, 0.15, -1.0)`, `(1.95, 0.15, -1.0)` and `(1.275, 0.92, -1.0)`. Its measured
margins, 2.06 px to the front sphere and 2.5 px to the right edge, are valid, and the merge does not
move or "improve" the placement.

The render and progress seam belongs to the second shared design,
`AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md`, not to this pair. The release takes its
`render`, sample defaults and progress writes from that file.

Test reconciliation for the merge:

- `the_center_pixel_shows_the_sphere_facing_the_camera` is dropped, because material scattering
  replaces normal colouring. The release's rendered-image gate is the recorded proof that the fixed
  scene shows the sphere, and no replacement test is added.
- `the_top_row_is_sky` survives the merge.
- The shape, list and `aabb` tests pass a material after the merge.

The scene triangle's material is an open user-owned decision. The release asks the user and records
the answer. The recommendation is a diffuse material in the scene spheres' family, so the triangle
reads as part of the scene. Neither this branch nor the merge picks it silently.

Z1's read-only closing survey is complete. The note is
`AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/closing-survey-2026-11-24.md` and the scout returns are
in `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/returns/`. The survey fixes nothing. The user's
acceptance of the survey and the branch render remains this phase's gate.
