## Phase Z1 - Closing survey and user acceptance

**Status:** next.
**Depends on:** A1 (`done/A1-diffuse-and-metal-materials.md`), A2 (`done/A2-glass.md`) and A3
(`done/A3-samples-per-pixel.md`), whose frozen contracts the survey checks the code against.
**Scope:** the binding closing survey of every area this plan added or changed, a dated note in the
plan's `benchmarks/`, the acceptance render through the branch's current command, and the user's
acceptance of the survey and the image. The survey is read-only. Findings go to the user and are not
fixed here. The plan orchestrator runs the survey with read-only `ops-scout` sessions and writes the
note. No `phase-implementer` session runs for this phase, and no code, test, configuration, README or
`docs/` change is made in it.
**Gate level:** minimal - the phase writes one plan-folder note and runs one read-only render. The
rendered-image conditional gate runs as survey evidence and for the user's acceptance.
**Simplicity statement:** the survey runs once per area, then one cross-check, then one note. It adds
no report generator, checklist tool, findings store, script or test. It does not narrow its scope to a
diff-only review, and it fixes nothing. A confirmed defect goes to a remediation round or the note's
user-review list, and a change needing a whole phase of its own goes to the user-review list. The plan
is neither closed nor archived here: that needs the user's explicit closure authorization after the
survey results are accepted.

**Current state (verified):**
- The plan's changes sit in branch range `e180476..566b11c` on branch `plan-a`, three commits
  (`de1cf31`, `a1d640a`, `566b11c`), worktree clean at head `566b11c`. The
  changed files are `src/material.rs` (new), `src/hittable.rs`, `src/sphere.rs` and `src/main.rs`.
  `README.md`, `docs/requirements.md`, `Cargo.toml` and `AGENT-INSTRUCTIONS/` are unchanged in the
  range, so `e180476` is the base the range uses.
- `src/material.rs:9-12` - the `Material` trait's `scatter(&self, ray_in: &Ray, hit: &HitRecord, rng:
  &mut Rng) -> Option<(Colour, Ray)>`. `Lambertian` at `:15-31`, `Metal` at `:34-56`, `Dielectric` at
  `:60-89`, the private helpers `reflect` `:92`, `refract` `:98`, `schlick` `:106` and
  `random_unit_vector` `:113`.
- `src/hittable.rs:11-50` - the one `HitRecord`, not `Copy`, with `t`, `point`, the ray-facing
  `normal`, `front_face`, `u` and `v` behind `#[allow(dead_code)]`, and `material: Rc<dyn Material>`,
  built only through the seven-argument `HitRecord::new`. `Hittable` at `:52-55`; `HittableList::hit`
  at `:69-80` with `let limit = nearest.as_ref().map_or(t_max, |hit| hit.t);`.
- `src/sphere.rs:11-25` - `Sphere { center, radius, material }` and `Sphere::new`; `hit` at `:27-61`
  computes `(u, v)` by latitude and longitude and builds the record through `HitRecord::new`.
- `src/main.rs:22-26` - `WIDTH` 200, `HEIGHT` 100, `SAMPLES` 16, `SEED` 7, `MAX_BOUNCES` 50.
  `ray_colour` at `:30-47`, `sky_colour` at `:50-53`, `scene` at `:55-78` with the four spheres
  (diffuse red-brown centre, diffuse grey ground, grey metal at `(0.85, -0.25, -1.3)`, glass at
  `(-1.1, 0.0, -1.5)`), `render` at `:82-99`, `to_byte` at `:101-103` with gamma 2, `write_ppm` at
  `:105-122`, `main` at `:124-127`.
- `src/rng.rs:3-23` - `Rng`, `new`, `next_u64` and `next_f64`; the one generator seeded with `SEED` is
  created inside `render` at `src/main.rs:85`.
- The 12 test functions: `src/main.rs:133-147` (`the_top_row_is_sky`,
  `two_renders_with_the_same_options_give_the_same_pixels`, `the_sample_count_changes_the_pixels`),
  `src/material.rs:137-243` (five), `src/sphere.rs:71-116` (three), `src/rng.rs:29-36` (one). There is
  no `tests/` directory on this branch; the binary tests belong to the output plan.
- `docs/requirements.md:6-19` - items 1 to 6 state the material, sampling and reproducibility
  behaviour this plan implements. `README.md:1-10` shows `cargo run > image.ppm` and `cargo test`.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/docs-pass.md` - four documentation items for the release: the
  `AGENT-INSTRUCTIONS/LOCAL/gates.md` rendered-image fallback sentence; `LOCAL/project.md` gaining
  `src/png.rs` and an accurate `main.rs` description; `LOCAL/project.md`'s `src/material.rs` line; and
  the `LOCAL/repo-rules-domain.md` "What this repo is" paragraph. It also carries one release-merge
  carry-over, the scene triangle's material. All are read only here.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md` and
  `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-b-Z1.md` - the two agreed merge designs. The branch's
  `render` doc sentence deliberately describes the merged product's progress line and must not be
  edited again at the merge; that is settled, not a finding.
- `AGENT-INSTRUCTIONS/PLANNING/PLAN-AUTHORING-SCOPING.md` §3b - the binding survey method: one
  read-only `ops-scout` per area in parallel, then ONE read-only `ops-scout` cross-checking all
  findings, then the orchestrator writes the note. No per-area checker.
- `AGENT-INSTRUCTIONS/SUBAGENTS/README.md:91-96` - `ops-scout` is the general-purpose read-only scout
  with a full read-only shell; it never writes, repairs or operates.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANNING/PLAN-AUTHORING-SCOPING.md` §3b - the survey method, the scout scope
  rules, the note's finding classes and the closing rule. Binding.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - the status table, the binding design rules and the
  carry-overs.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1-diffuse-and-metal-materials.md`,
  `done/A2-glass.md`, `done/A3-samples-per-pixel.md` and `done/A3r-samples-resolution.md` - the frozen
  contracts the survey checks the tree against.
- `src/material.rs`, `src/hittable.rs`, `src/sphere.rs`, `src/main.rs`, `src/rng.rs`, `src/vec3.rs`,
  `src/camera.rs`, `src/ray.rs` - the changed tree and the types it uses.
- `docs/requirements.md` items 1 to 6 and `README.md` - the required behaviour and the product
  overview. Both are read only here.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/docs-pass.md` - the deferred documentation items, read only.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md`,
  `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-b-Z1.md` - the agreed merge contracts the survey
  reports drift against, if any.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the gate commands, including the rendered-image gate and its
  pre-command-line fallback that this branch still uses.
- `AGENT-INSTRUCTIONS/SUBAGENTS/README.md` - the `ops-scout` description and its read-only boundary.

**Build plan:**
1. Create the directories `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/` and
   `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/returns/`. The note is
   `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/closing-survey-<date>.md`, dated the day the survey
   runs, for example `closing-survey-2026-11-24.md`. Each scout's unchanged return is saved beside it
   with these exact names: `benchmarks/returns/area-1-materials.md`,
   `area-2-hit-record-and-sphere.md`, `area-3-render-loop-generator-and-scene.md`,
   `area-4-tests-and-documents.md` and `cross-check.md`. Never rewrite a return after saving it.
2. The orchestrator dispatches one independent read-only `ops-scout` per area below, all launched
   together so they run in parallel. Each dispatch prompt names its area and range `e180476..566b11c`,
   tells the scout it may read whatever context outside the area it needs to judge it, tells it to
   report an out-of-scope problem as an out-of-scope observation with the evidence it already has and
   not to investigate it further, and does not restrict it to the diff:
   - **Area 1, the materials.** `src/material.rs` against requirements 1 to 4 and the frozen A1 and A2
     contracts: the trait shape, `Lambertian` scattering around the normal, `Metal` reflection with
     fuzz and absorption, `Dielectric` entering/leaving from `front_face` and the ray-facing normal,
     Snell, total internal reflection, Schlick, and unit attenuation. Look for unnecessary complexity,
     duplication, dead code and redundant checks.
   - **Area 2, the hit record and the sphere.** `src/hittable.rs` and `src/sphere.rs` against the A1
     and A2 contracts: one `HitRecord`, the seven-argument constructor as the only path, the normal
     flipped only there, the material shared as `Rc`, the `(u, v)` formula and its `allow(dead_code)`,
     and the nearest-hit borrow in `HittableList::hit`.
   - **Area 3, the render loop, the generator and the scene.** `src/main.rs` and `src/rng.rs` against
     requirements 1, 5 and 6 and the A3 contract: the bounce loop and the 50-bounce black result, the
     gamma-2 `to_byte`, the jittered per-pixel average with one seeded generator created before both
     loops and no reseeding, the four-sphere scene, and that every random draw comes from `Rng`. Look
     for a second render path, duplicate checks, defensive re-checks and anything that breaks
     reproducibility.
   - **Area 4, tests and documentation.** The 12 test functions, `docs/requirements.md` items 1 to 6,
     `README.md` and `AGENT-INSTRUCTIONS/PLANS/plan-a/docs-pass.md`: test necessity and focus against
     REPO-RULES §3a (one test function = one behavior, no packing), no fake product behaviour, current
     and accurate README and requirements text, and nothing stale. Drift from the two shared designs
     and from the user's decisions is reported here. The four `docs-pass.md` documentation items and
     its one release-merge carry-over are known and not findings, but a `docs-pass.md` item the branch
     has already made obsolete or wrong is reported. Also read the code and doc comments in the plan's
     changed files and in `src/vec3.rs` for plan or phase references and for wording that no longer
     describes the code; report each with `file:line` evidence. `src/vec3.rs:21` still carries a comment
     naming "the geometry plan", which a permanent comment must not say; the integration tree already
     drops it, so report it as an out-of-scope observation, not a plan-a defect.
3. The orchestrator dispatches ONE independent read-only `ops-scout` that cross-checks every finding
   from every area, claim by claim, against the code and documents with evidence. It returns each claim
   as confirmed or rejected with `file:line` evidence. There is no per-area checker.
4. Rendered-image gate on the current head, run by the orchestrator. This branch predates the output
   plan's command line, so it uses the gate's fallback form; capture the PPM and convert it:
   ```bash
   ~/.cargo/bin/cargo run --release > /tmp/opencode/plan-a-acceptance.ppm
   ```
   Then convert the PPM to a PNG for the image reader:
   ```bash
   python3 - <<'PY'
   import struct, zlib

   with open("/tmp/opencode/plan-a-acceptance.ppm") as f:
       assert f.readline().strip() == "P3"
       width, height = map(int, f.readline().split())
       assert f.readline().strip() == "255"
       data = [int(v) for v in f.read().split()]

   raw = b"".join(
       b"\x00" + bytes(data[(r * width + c) * 3 + k] for c in range(width) for k in range(3))
       for r in range(height)
   )

   def chunk(tag, body):
       return struct.pack(">I", len(body)) + tag + body + struct.pack(">I", zlib.crc32(tag + body))

   png = (b"\x89PNG\r\n\x1a\n"
          + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0))
          + chunk(b"IDAT", zlib.compress(raw))
          + chunk(b"IEND", b""))
   with open("/tmp/opencode/plan-a-acceptance.png", "wb") as f:
       f.write(png)
   PY
   ```
   Open `/tmp/opencode/plan-a-acceptance.png` with the image reader and confirm it shows the glass
   sphere on the left refracting and inverting the background, the diffuse red-brown centre sphere, the
   diffuse grey ground and the grey metal sphere on the right, with soft sphere edges, no visible
   salt-and-pepper grain in the sky, and no flat `NaN` grey or pure-black pixels. Keep the PPM and PNG
   until the user answers in the acceptance step, then remove both.
5. The orchestrator writes the dated note from the saved returns and its own reading. The note states
   the surveyed areas and the revision, then lists:
   - confirmed findings with `file:line` evidence and their class: unnecessary complexity, duplication,
     dead code, a redundant check or record, test bloat, a stale document, or drift from the plan's
     design rules, the frozen contracts or the user's decisions;
   - the claims the cross-check rejected, so a later reader does not reopen them;
   - out-of-scope observations with the evidence the area scout had;
   - a user-review list for anything that needs a phase of its own or a user-owned decision;
   - the rendered-image evidence from step 4: the command run, the four spheres seen, the soft edges
     and the absent grain and black pixels.
   The note carries no fix.
6. Report the note and the render to the user: the confirmed findings, the rejected claims, the
   out-of-scope observations and the user-review list, pointing at
   `/tmp/opencode/plan-a-acceptance.png`. Fix nothing in this phase.
7. On the user's acceptance, compress this file into `done/Z1-closing-survey.md` (the survey outcome,
   the note and returns paths, the recorded revision, the render evidence and the user's answer) and
   flip the `full-plan.md` status row to `done`. Never archive the plan in this phase.

**Callers / wiring to update:**
- None. The phase changes no code, test, configuration or published document.

**Config / schema / migrations:**
- No configuration key, schema or migration.
- Retained record and its consumer: the dated note in `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/`
  and its saved returns in `benchmarks/returns/` are the records this phase exists to produce. Their
  consumers are the user's acceptance decision, the release merge's review of `docs-pass.md` and the
  two shared designs, and any later remediation round. They are the only copies and live in the plan
  folder. The acceptance PPM and PNG are transient evidence under `/tmp/opencode/` and are removed
  once the user answers.

**Touched files:**
<!-- touched-files:start -->
- `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/closing-survey-2026-11-24.md` - created: the dated
  closing survey note, named with the day the survey runs.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/returns/area-1-materials.md` - created: the materials
  area return, saved unchanged.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/returns/area-2-hit-record-and-sphere.md` - created: the
  hit-record and sphere area return, saved unchanged.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/returns/area-3-render-loop-generator-and-scene.md` -
  created: the render loop, generator and scene area return, saved unchanged.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/returns/area-4-tests-and-documents.md` - created: the
  tests and documents area return, saved unchanged.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/returns/cross-check.md` - created: the single
  cross-check return, saved unchanged.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/Z1-closing-survey.md` - changed: this phase file, only when
  compressing it after acceptance.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/Z1-closing-survey.md` - created on acceptance: the frozen
  survey outcome, the note and returns paths and the user's acceptance.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - changed: the Z1 status row and its phase file path.
- Adds: the `benchmarks/` and `benchmarks/returns/` directories.
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Read-only survey. Do not change product code, tests, `Cargo.toml`, `README.md`,
  `docs/requirements.md`, `AGENT-INSTRUCTIONS/LOCAL/gates.md` or any other permanent file in this
  phase.
- Do not narrow a scout to a diff-only review. Each area scout covers its whole area and reads whatever
  outside context it needs.
- Do not merge areas into one scout, add a per-area checker, or skip the single cross-check of all
  findings. Do not write the note from memory or from an unsaved return, and no implementer or other
  child writes it.
- Every finding in the note carries `file:line` evidence. A rejected claim stays in the note.
- Do not fix anything here, do not open a remediation phase inside Z1, and do not close or archive the
  plan. Archiving needs the user's explicit closure authorization after accepting the survey.
- Do not edit `AGENT-INSTRUCTIONS/LOCAL/gates.md` or `LOCAL/project.md` and
  `LOCAL/repo-rules-domain.md`; their items stay deferred through `docs-pass.md`. Do not add new
  `docs-pass.md` items for things this phase merely observes unless the release must act on them.
- Do not edit the `render` doc sentence: it describes the merged product per the shared design and is
  final. Do not treat that deliberately forward-looking sentence as a finding.
- No new dependency, no `unsafe`, no threads, no new randomness, and no change to the scene or the
  camera.
- The note and returns carry no fix. No history or plan id goes into source comments.

**Tests:**
- New/changed/deleted: none. The phase adds no test function, so the budget is 0 added / 0 deleted /
  0 changed. The survey checks the existing 12 tests against REPO-RULES §3a and may report test bloat
  in the note, but it changes nothing.
- Fast gate: not applicable. No code or test changes, so A3's passing evidence at `566b11c` stands and
  is not rerun.
- Documentation-only gate for the note and returns: reread each written file, and run
  `git diff --check` at the workspace root.
- Conditional gates: rendered image, required by step 4 as survey evidence and for the user's
  acceptance. Agent generation: not affected.
- Slow gate (>2 minutes): none. A 200 by 100, 16-sample release render finishes well under a minute.

**User test (acceptance):**
- **Executor:** user. Accepting the survey findings and judging how the rendered image looks are
  user-owned judgments that no automated check proves.
- 1. Open `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/closing-survey-<date>.md` and read the confirmed
  findings, the rejected claims, the out-of-scope observations and the user-review list.
- 2. Open `/tmp/opencode/plan-a-acceptance.png` from step 4. It shows the glass sphere refracting the
  background on the left, the red-brown centre sphere, the grey ground and the grey metal sphere, with
  soft edges.
- 3. Optionally run `cargo run --release > /tmp/opencode/plan-a-check.ppm` in the worktree and convert
  it to confirm the image matches the one described.
- 4. Answer with acceptance of the note and the render, or with the findings and observations that must
  be handled before acceptance.
- Expected result: the user accepts the survey note and the rendered image, or names the items that
  block acceptance.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a (minimal) are satisfied.
- [ ] All four areas were surveyed by four separate read-only `ops-scout` sessions in parallel, and one
  further independent read-only `ops-scout` cross-checked every finding.
- [ ] The dated note exists in `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/` with its five saved
  returns, and every finding and rejected claim in it carries `file:line` evidence.
- [ ] The note covers unnecessary complexity, duplication, dead code, redundant checks or records, test
  bloat against REPO-RULES §3a, stale documents and drift from the plan's design rules, the frozen
  contracts and the user's decisions, and lists the out-of-scope observations and the user-review list.
- [ ] The rendered-image gate ran on the surveyed revision: the image was opened and inspected with all
  four spheres present, soft edges, no visible grain and no pure-black pixels.
- [ ] `docs/requirements.md` items 1 to 6 and the README still describe the behaviour, and
  `docs-pass.md` still lists its four documentation items and its release-merge carry-over.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports 0 added, 0 deleted and 0 changed.
- [ ] No product code, test, configuration or permanent document changed in this phase, and
  `git diff --check` is clean.
- [ ] The note and the returns are written under the plan folder only. They are not staged or committed
  from the worktree: the workflow plugin commits the workspace root, and plan files never enter a
  worktree commit.
- [ ] The note and the render were reported to the user, and the user's answer is recorded on
  acceptance.
- [ ] The plan is not closed or archived in this phase.
- [ ] On the user's acceptance: compress this file into `done/Z1-closing-survey.md` and flip the
  `full-plan.md` status row to `done`.

**Exit:** the dated survey note exists with every finding backed by evidence, the acceptance render was
inspected, and the user has accepted the survey and the rendered result.
