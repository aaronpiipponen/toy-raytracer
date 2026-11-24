## Phase A1r - Shared hit record settlement

**Status:** active.
**Depends on:** none; records the design settled with the geometry plan in
`AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B1.md`.
**Scope:** record, in this plan's own documents, the settled shared `HitRecord` design so that A1 and
A2 proceed unchanged on this branch and the release merge knows exactly what to combine. The record's
geometry fields (`front_face`, `u`, `v`), the six-argument constructor and `Sphere`'s latitude and
longitude are the geometry plan's branch work and the release merge's; this phase never duplicates
them. It changes no product code, test, `docs/` page or configuration.
**Gate level:** minimal - documentation-only. No runtime behaviour, test, config, schema or
dependency changes; the settlement is verified by re-reading the named documents and the plan status
table, plus `git diff --check`.
**Simplicity statement:** this phase writes plan text and changes nothing else. It adds no code, no
field, no helper, no dependency and no test on this branch. It deliberately does not implement the
geometry plan's fields or constructor, does not re-detail A1, and does not pre-write the merge. The
only product code this plan will own is A1's and A2's, as already detailed.

**Current state (verified):**
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B1.md:63-72` - "plan-a / A1: unchanged as
  detailed. The record keeps its material field and literal construction on this branch." The merge,
  not either branch, combines the two shapes.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/scope.md:1-8` - authorized unit is this plan; branch `plan-a`;
  `Cargo.toml` gains no dependency; other plans' work arrives only through a coordinator resolution
  or the release merge.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md:6` - face sides and texture coordinates are out of
  scope for this plan (the geometry plan owns them).
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md:35-39` - the settled A2 rule: glass derives the face
  side locally on this branch from the outward normal and the ray, and the merge replaces that with
  `hit.front_face`; glass adds no face flag.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md:119-127` - carry-overs already state the merge's job
  and the settled materials for the new shapes.
- `src/hittable.rs:7-14`, `src/sphere.rs:36-43` at `e180476` - the starting record holds `t`,
  `point`, `normal` and `Sphere::hit` builds it literally with the outward normal; no material,
  `front_face` or `u`/`v` exists on this branch. A1 has not been implemented.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A1-diffuse-and-metal-materials.md` - A1's detail already
  matches the shared design's plan-a paragraph: it keeps the material field, builds the record
  literally, adds no face flags and no texture coordinates, and drops `Copy` with the `Rc` field.

**Read first (do not invent):**
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B1.md` - the whole settlement: target merged
  record, one construction path, plan-a's branch shape, A2's local derivation, and the merge's
  obligations. This is the authority for every statement below.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - status table, binding design rules, A1 and A2
  scoped entries, and carry-overs.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A1-diffuse-and-metal-materials.md` - A1's detail; confirm
  it still matches the shared design and leave it byte-identical.
- `AGENT-INSTRUCTIONS/PLANS/plan-b/phases/B1r-shared-hit-record-shape.md` - the partner's side, read
  only to confirm the two branches are consistent. Do not edit it and do not implement it here.
- `src/hittable.rs` and `src/sphere.rs` - current starting shape on this branch; no edit.

**Settlement (the contract this phase records):**

1. **Target merged record.** On the merged branch the record is exactly:

   ```rust
   pub struct HitRecord {
       pub t: f64,
       pub point: Vec3,
       pub normal: Vec3,      // always faces against the ray
       pub front_face: bool,  // true when the ray hit the outside
       pub u: f64,
       pub v: f64,
       pub material: Rc<dyn Material>,
   }
   ```

   It derives `Clone` only. `Copy` is gone with the `Rc` field, and `Debug` is dropped; the
   `Material` trait gains no `Debug` supertrait.

2. **One construction path.** `HitRecord::new(ray, t, point, outward_normal, u, v, material)` sets
   `front_face = ray.direction.dot(outward_normal) < 0.0`, flips `normal` from `outward_normal` when
   the face is back, and records every field. No record literal and no second face-side helper
   survives the merge, including in A1's `src/material.rs` tests.

3. **Merged `Sphere::hit` tail.** After the merge it is the shared design's tail exactly: point, then
   `outward_normal = (point - self.center) / self.radius`, then `theta = (-outward_normal.y).acos()`
   and `phi = (-outward_normal.z).atan2(outward_normal.x) + PI`, then
   `HitRecord::new(ray, t, point, outward_normal, phi / (2.0 * PI), theta / PI, Rc::clone(&self.material))`.

4. **This branch (plan-a) keeps its own shape.** A1 keeps the material field, builds its records
   literally and derives no face flag and no texture coordinates; A2 derives the face side locally:

   ```rust
   let front_face = ray_in.direction.dot(hit.normal) < 0.0;
   let normal = if front_face { hit.normal } else { -hit.normal };
   let cos_theta = -ray_in.direction.unit().dot(normal);
   ```

   Those lines are temporary branch code, never part of the merged shape. The release merge replaces
   them with `hit.front_face` and the facing `hit.normal`.

5. **What A1 may rely on after the merge.** The record still carries the object's material, so
   `HittableList` and the render loop scatter through it unchanged; `hit.normal` faces the incoming
   ray, which is what `Lambertian` and `Metal` are written against. Their comments say "the normal
   facing the ray", not "outward normal".

6. **What A2 may rely on after the merge.** `hit.normal` always faces the ray; `hit.front_face` is
   true on entry and false when leaving. Glass decides entering versus leaving from
   `hit.front_face` only, with `cos_theta = -ray_in.direction.unit().dot(hit.normal)`, positive on
   entry, and no outward normal and no second face flag.

7. **The release merge's obligations on plan-a's side.** In the merge itself, not as optional cleanup:
   drop `Copy` and `Debug` from the record and add `front_face`, `u`, `v`; route A1's record literals,
   including its material-test records, through `HitRecord::new(...)` with the outward normal and
   `u`, `v`; adopt the merged `Sphere::hit` tail above; replace A2's two local derivation lines with
   `hit.front_face` and the facing `hit.normal`; give the geometry plan's new shapes a material field
   and pass `Rc::clone(&self.material)` last, with the ground plane mid-grey and the triangle a
   distinct dull green as the carry-over records.

**Build plan:**
1. `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md`, status table - confirm the advancer's edit: the
   `A1r` row sits directly above `A1`, is `active` with `Phase file`
   `phases/A1r-shared-hit-record-settlement.md`, and the `A1` row is `planned`, depends on `A1r`,
   with `Phase file` `-`. Every other row is unchanged. Change nothing else in the table.
2. `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A1r-shared-hit-record-settlement.md` - re-read the
   settlement above line by line against `PLANS/shared/plan-a-A1+plan-b-B1.md`. If any statement
   disagrees with the shared design, stop and report drift to the orchestrator; do not edit product
   code, A1's detail or the shared file to paper over it.
3. `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A1-diffuse-and-metal-materials.md` - confirm it still
   matches items 4 and 5 above and leave it unchanged. This phase does not re-detail A1.
4. No product, test, `docs/` or configuration edit. `git status --short` inside the plan worktree must
   show no changed or new Git-tracked file from this phase; the plan document edits live at the
   workspace root and are committed there by the workflow plugin.

**Callers / wiring to update:**
- None. A1r edits plan documents only; no symbol, caller, import, scene or output path changes.

**Config / schema / migrations:**
- None: no configuration key, migration, generated file or persisted record.
- Retained records and their named consumer/owner: none. `HitRecord` is an in-memory value; the
  settlement is plan text read by A1, A2, Z1 and the release merge, not a stored record.

**Touched files:**
<!-- touched-files:start -->
- `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - changed: status table gains the `A1r` row and moves `A1` to `planned` with no phase file
- `AGENT-INSTRUCTIONS/PLANS/plan-a/phases/A1r-shared-hit-record-settlement.md` - created: the settlement record for this branch and the merge
- Adds: nothing
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not change product code, tests, `docs/`, `README.md`, `Cargo.toml` or the shared design file in
  this phase.
- Do not add `front_face`, `u`, `v`, a constructor, a `PI` import or a face-side helper to this
  branch. Those are the geometry plan's branch work and the release merge's; duplicating them here
  creates the second path this settlement forbids.
- Do not edit, re-detail or renumber `phases/A1-diffuse-and-metal-materials.md`, and do not leave
  A1's row `active` alongside `A1r`.
- Do not edit plan-b's files or commit anything on `plan-b`'s behalf.
- Do not add a dependency, use `unsafe`, or touch `src/`.
- Permanent code comments and `docs/` pages never mention a plan or phase; plan documents may.

**Tests:**
- New/changed/deleted: none. This phase changes no test and REPO-RULES §3a adds none for plan text.
- Fast gate: not applicable to code; run `git diff --check` at the workspace root and re-read the
  changed plan documents for accuracy.
- Conditional gates: none. The rendered image and agent-generation gates are untouched: no render
  loop, material, camera, output or `AGENT-INSTRUCTIONS/SUBAGENTS/` change.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - the documentation-only gate proves acceptance: the settlement matches the shared
  design, the status table has exactly one active row, A1's detail is unchanged and no product code
  moved.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] Every statement in the settlement matches `PLANS/shared/plan-a-A1+plan-b-B1.md`; no contradiction remains.
- [ ] `full-plan.md` has exactly one `active` row (`A1r`), `A1` is `planned` with no phase file, and every other row is unchanged.
- [ ] `phases/A1-diffuse-and-metal-materials.md` is byte-identical to its pre-phase content and still matches the shared design's plan-a paragraph.
- [ ] No product code, test, `docs/` page, `Cargo.toml` or configuration changed; `git status --short` in the plan worktree shows nothing from this phase.
- [ ] `git diff --check` reports nothing at the workspace root; the changed plan documents were re-read.
- [ ] Test budget respected (REPO-RULES §3a); receipt reports 0 test functions added and 0 deleted.
- [ ] No conditional or slow gate applies; none was run or claimed.
- [ ] Safety commit: none is needed, because the phase changes no Git-tracked file in the plan worktree; report that plainly in the receipt instead of inventing a commit.
- [ ] Revision-bound verification receipt exists, naming the unchanged code head `e180476` (or the current unchanged head) and the files re-read; no separate user test was required.
- [ ] On passing evidence: compress this file into `done/A1r-shared-hit-record-settlement.md` (frozen contract only) and flip the `full-plan.md` `A1r` row to `done`; the advancer then resumes A1 with its existing `phases/A1-diffuse-and-metal-materials.md` unchanged.

**Exit:** the shared design is recorded for this branch as frozen contract; A1 and A2 still hold
their existing detail, the status table has `A1r` done and `A1` ready to resume, and no product code
on `plan-a` duplicates the geometry plan's work.
