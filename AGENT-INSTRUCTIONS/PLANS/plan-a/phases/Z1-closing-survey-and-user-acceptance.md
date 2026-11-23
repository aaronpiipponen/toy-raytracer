## Phase Z1 - Closing survey and user acceptance

**Status:** active.
**Depends on:** `done/A1r-hit-record-constructor-and-materials.md`, `done/A2-glass.md` and
`done/A3-samples-per-pixel.md`, the three frozen contracts of this plan.
**Scope:** one read-only closing survey of every area this plan added or changed, a dated note in
`AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/`, and the user's acceptance of that note. No product
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
- The plan's code head is `dca633c`; `Cargo.toml` has an empty `[dependencies]` table.
- `src/main.rs:22-26` - `WIDTH`, `HEIGHT`, `SAMPLES`, `MAX_BOUNCES` and `RNG_SEED`.
- `src/main.rs:29-48` - `ray_colour`, the 50-bounce loop, the black on absorption or limit, and the
  sky gradient on a miss.
- `src/main.rs:50-73` - `scene()` with four spheres: red-brown diffuse centre, yellow-green diffuse
  ground, grey metal right and glass left.
- `src/main.rs:75-93` - `render(width, height, samples)`, the single seeded generator and the
  jittered sample loop.
- `src/main.rs:95-121` - `to_byte` with gamma 2, `write_ppm` and `main`.
- `src/hittable.rs:9-30` - `HitRecord` with `t`, `point`, `normal` and `material`, and the one
  `HitRecord::new`; `:32-35` - the `Hittable` trait; `:37-60` - `HittableList` and its nearest-hit
  loop.
- `src/sphere.rs:10-24` - `Sphere` with a material and its constructor; `:26-52` - `Sphere::hit`
  with the outward normal and the material on the record.
- `src/material.rs:9-12` - the `Material` trait; `:15-33` - `Lambertian`; `:36-57` - `Metal`;
  `:60-90` - `Dielectric`; `:92-120` - `reflect`, `refract`, `schlick` and `random_unit_vector`.
- `src/rng.rs:1-23` - `Rng`, `Rng::new`, `next_u64` and `next_f64`.
- `src/vec3.rs:21-29` - `Vec3::cross` with the comment "For the triangle normals the geometry plan
  adds." and its `#[allow(dead_code)]` marker.
- Twelve test functions: `src/main.rs` 4, `src/material.rs` 5, `src/rng.rs` 1 and `src/sphere.rs` 2.
  A1r added three, A2 added three and A3 added one, so seven were added across the plan, and none was
  deleted or weakened.
- `AGENT-INSTRUCTIONS/LOCAL/project.md:5` - the repository map line names `main.rs`, `png.rs`,
  `vec3.rs`, `ray.rs`, `camera.rs`, `hittable.rs`, `aabb.rs`, `sphere.rs`, `plane.rs`, `triangle.rs`
  and `rng.rs`.
- `docs/requirements.md:6-19` - items 1 to 6, the materials, sampling and reproducibility contract.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/` does not exist. This plan has no earlier survey note.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANNING/PLAN-AUTHORING-SCOPING.md` section 3b - the binding survey method and
  the closing-survey rule. Follow it exactly and add no step.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - the plan scope, the binding design rules and the
  phase rows.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1r-hit-record-constructor-and-materials.md`,
  `done/A2-glass.md` and `done/A3-samples-per-pixel.md` - the frozen contracts the survey checks
  against.
- `docs/requirements.md:6-19` and item 14 - the behaviour this plan implements and the gates.
- `AGENT-INSTRUCTIONS/LOCAL/project.md`, `AGENT-INSTRUCTIONS/LOCAL/gates.md` and `README.md` - the
  repository map, the gates and the product overview.

**Build plan:**

1. Survey the ten areas below, one read-only `ops-scout` per area, dispatched in parallel. Each
   prompt names its area, the contract it is judged against, the finding classes and the evidence
   format, and does not restrict the scout to the diff.
   - **Material trait and the three materials:** `src/material.rs:9-120`, `random_unit_vector`
     included. Judged against requirements 1 to 4 and REPO-RULES §0.
   - **Hit record, constructor and scene list:** `src/hittable.rs:9-60`, plus every `HitRecord::new`
     call in `src/sphere.rs`. Judged against requirement 1 and the A1r frozen contract.
   - **Sphere and material wiring:** `src/sphere.rs:10-52`. Judged against requirement 1 and the
     A1r frozen contract.
   - **Scene of four spheres:** `src/main.rs:50-73`. Judged against requirement 1 and the A2 frozen
     contract.
   - **Scatter loop and bounce limit:** `src/main.rs:29-48`. Judged against requirement 1.
   - **Samples per pixel:** `src/main.rs:75-93`. Judged against requirement 5 and the A3 frozen
     contract.
   - **Gamma correction and PPM output:** `src/main.rs:95-121`. Judged against requirement 5.
   - **Seeded randomness and reproducibility:** `src/rng.rs:1-37` and every generator call in
     `src/material.rs` and `src/main.rs`. Judged against requirement 6 and the hard invariant that all
     randomness comes from `src/rng.rs` with a fixed seed.
   - **Tests across the plan:** the test modules of `src/main.rs`, `src/material.rs`, `src/rng.rs`
     and `src/sphere.rs`, all twelve functions, their names and their assertions. Judged against
     REPO-RULES §3a and requirement 14.
   - **Permanent documentation and the repository map:** `AGENT-INSTRUCTIONS/LOCAL/project.md:5`,
     `docs/requirements.md:6-19` and item 14, `README.md`, and the dead-code placeholder at
     `src/vec3.rs:21-29`. Judged against the current tree, the requirements and REPO-RULES §0b.
2. Give every scout the same finding classes: unnecessary complexity, duplication, dead code,
   redundant checks or records, test bloat, stale documentation, and drift from the user's decisions.
   Each finding carries `file:line` evidence and its class. An out-of-scope observation is reported
   with the evidence already in hand and is not investigated further.
3. After all ten returns arrive, dispatch one further read-only `ops-scout` that cross-checks every
   finding together against the source, the documents and the three done contracts, and marks each
   `CONFIRMED`, `WRONG` or `UNCLEAR` with evidence.
4. The orchestrator writes `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/survey-2026-11-23.md` itself
   from the saved returns. The note holds, in this order: the date, the plan scope, the ten areas,
   the statement that no earlier survey note exists; one row per real finding with an id, class,
   `file:line` evidence, impact and recommendation; the out-of-scope observations kept separate; the
   cross-check table; the checks performed, including the rendered-image results and the path of the
   image kept for the user; and the user-review list. Drop every `WRONG` finding and put every
   `UNCLEAR` finding on the user-review list. No finding without `file:line` evidence.
5. Record in the note the checks the survey performs:
   - requirements 1 to 4: cite the frozen contract evidence in `done/A1r-...md` and
     `done/A2-...md` and confirm the current source matches; do not rerun their tests;
   - requirement 5: the A3 contract holds in `src/main.rs:75-97`, the default is 16 and `to_byte` is
     the only place that applies gamma 2;
   - requirement 6 and the hard invariants: `Cargo.toml` has no dependency, all randomness comes from
     the seeded `src/rng.rs`, `src/` uses no `unsafe` and no thread type, and two identical renders
     write identical bytes;
   - requirement 14 and the fast gate: the receipt at `dca633c` reports `cargo build`, `cargo test`
     (12 passed), `cargo clippy -- -D warnings`, `cargo fmt --check` and `git diff --check`, so do not
     rerun them on the unchanged revision;
   - the test budget: seven functions added across A1r, A2 and A3, at most three per phase, and no
     deleted function or weakened assertion;
   - documentation accuracy: record whether the repository map names every module in `src/`, and
     whether any permanent document or code comment names a plan. Each failure is a finding with
     `file:line` evidence and goes on the user-review list like every other finding.
6. Render the scene twice for the comparison and the reproducibility check, from `~/.cargo/bin/cargo`
   when `cargo` is not on `PATH`, then check the files in the same shell:
   ```bash
   ~/.cargo/bin/cargo run --release > /tmp/opencode/survey-a.ppm
   ~/.cargo/bin/cargo run --release > /tmp/opencode/survey-b.ppm
   cmp /tmp/opencode/survey-a.ppm /tmp/opencode/survey-b.ppm
   wc -l /tmp/opencode/survey-a.ppm
   sed -n '1,3p' /tmp/opencode/survey-a.ppm
   ```
   `cmp` exits 0, `wc -l` prints `20003`, and the three lines print `P3`, `200 100` and `255`. The
   PPM is ASCII text, so `sed` reads it line by line.
7. Convert the kept render to PNG with this throwaway script in `/tmp/opencode`, never in the
   repository, and open the PNG with the image reader:
   ```python
   import sys, zlib, struct
   data = open(sys.argv[1], "rb").read().split()
   w, h = int(data[1]), int(data[2])
   vals = [int(v) for v in data[4:]]
   raw = b"".join(b"\x00" + bytes(vals[r * w * 3:(r + 1) * w * 3]) for r in range(h))
   def chunk(t, b):
       return struct.pack(">I", len(b)) + t + b + struct.pack(">I", zlib.crc32(t + b) & 0xFFFFFFFF)
   png = (b"\x89PNG\r\n\x1a\n"
          + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0))
          + chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b""))
   open(sys.argv[2], "wb").write(png)
   ```
   Run it as
   `python3 /tmp/opencode/ppm2png.py /tmp/opencode/survey-a.ppm /tmp/opencode/survey-a.png`.
   Record in the note that the opened image shows the sky gradient, the yellow-green ground and the
   four spheres with their materials at 200 by 100, with smooth edges. Keep `/tmp/opencode/survey-a.ppm`
   and `/tmp/opencode/survey-a.png` for the user's acceptance, remove `/tmp/opencode/survey-b.ppm`
   after the comparison, and remove the kept files after the user replies.
8. Report to the user: the note path, the finding summary by class, the cross-check result and the
   user-review list. State plainly that no finding was fixed inside Z1.
9. On the user's acceptance, the advancer compresses this file into `done/Z1-closing-survey.md` and
   flips the `full-plan.md` row to `done`, repointing its `Phase file` cell to the done file. If the
   user names changes, return their direction to the orchestrator for a later survey or remediation
   round under section 3b. Acceptance alone does not archive the plan: closure needs the user's
   explicit authorization (`AGENT-INSTRUCTIONS/PLANS/README.md`).

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
- `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/survey-2026-11-23.md` - created: the closing survey note with cross-checked findings, the checks performed and the user-review list.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - changed after acceptance: the advancer flips the Z1 row to done and repoints its `Phase file` cell to `done/Z1-closing-survey.md`.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/Z1-closing-survey.md` - created after acceptance: the advancer compresses this phase into the frozen contract.
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
- Do not rerun the fast gate on the unchanged `dca633c` revision. Do not rerun any phase's rendered
  image; the survey's own two renders are the ones Z1 owns.
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
- Fast gate: not run on the unchanged `dca633c` revision, whose receipt already reports every fast
  command. The note itself is re-read for accuracy and its paths are checked.
- Conditional gates: rendered image, run once as a comparison pair by the plan orchestrator at
  200 by 100 (step 6), because the survey compares the scene with the requirements and checks
  reproducibility. Agent generation: not run; no file under `AGENT-INSTRUCTIONS/SUBAGENTS/` changes.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** user.
- The plan's move-on gate is the user's acceptance of the closing survey.
- 1. Open `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/survey-2026-11-23.md` and read the findings,
   the cross-check verdicts and the user-review list.
- 2. Open `/tmp/opencode/survey-a.png` and confirm the four spheres with their materials over the sky
   gradient and the ground at 200 by 100, with smooth edges.
- 3. Reply that the survey results are accepted, or name each change to make. Any change is
   scheduled outside Z1.
- Acceptance alone does not close or archive the plan.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a for `minimal` are satisfied.
- [ ] `benchmarks/survey-2026-11-23.md` exists, is dated `2026-11-23`, and covers the ten areas the
      plan added or changed.
- [ ] Every finding carries `file:line` evidence and a cross-check verdict. No `WRONG` finding
      remains, and every `UNCLEAR` finding is on the user-review list.
- [ ] The note records the requirement checks for items 1 to 6, the three hard invariants, the test
      budget and the statement that no earlier survey note exists.
- [ ] The note records the documentation-accuracy check result, and each documentation failure it
      finds appears as a finding with `file:line` evidence on the user-review list. The check is
      recorded as an observation, not asserted as a fact the note must satisfy.
- [ ] The 200 by 100 render has 20003 lines starting `P3`, `200 100` and `255`, two runs are
      byte-identical, and the opened PNG shows the four spheres with their materials and smooth
      edges.
- [ ] No product source, test, configuration or permanent document changed, and `git status
      --porcelain` in the worktree lists no product file.
- [ ] No test, dependency, configuration key, checker, command or receipt was added.
- [ ] Findings sit only in the note's user-review list and none was fixed inside Z1.
- [ ] The user accepted the survey results, or named the changes to make.
- [ ] On acceptance: the advancer compresses this file into `done/Z1-closing-survey.md`, flips the
      `full-plan.md` row to done and repoints its `Phase file` cell. The plan is not archived without
      the user's explicit authorization.

**Exit:** the closing survey note records the plan's ten areas with cross-checked findings and a
user-review list, and the user has accepted the results or named the changes to make.
