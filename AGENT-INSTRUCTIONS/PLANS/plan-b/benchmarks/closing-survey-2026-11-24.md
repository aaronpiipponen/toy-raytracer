# Closing survey - geometry plan

Surveyed revision: branch `plan-b`, range `e180476..f85a183`, head `f85a183`. Worktree clean,
`git status --porcelain` empty. Date: 2026-11-24.

Method: four read-only area scouts ran in parallel; their unchanged returns are in
`benchmarks/returns/area-1-hit-record-and-sphere.md`, `area-2-planes-and-triangles.md`,
`area-3-bounding-boxes.md` and `area-4-tests-and-documents.md`. One independent read-only scout then
cross-checked every claim; its unchanged return is `benchmarks/returns/cross-check.md`. The plan
orchestrator wrote this note from those returns and from its own reading. Nothing was changed in this
phase: no product code, test, configuration or permanent document.

Areas surveyed: (1) the hit record, the face side, texture coordinates and the sphere; (2) the plane,
the triangle and the scene triangle; (3) bounding boxes and the scene list; (4) tests and documents.

Result: no confirmed clear defect in this plan's work. One shared-tree documentation item and several
low-severity nits are recorded below; none needs a remediation round.

## Confirmed findings

1. **Shared repository map lists a module this branch does not have** - class: stale document
   (shared instruction tree). `AGENT-INSTRUCTIONS/LOCAL/project.md:5` lists `material.rs` ("the
   `Material` trait and the diffuse and metal materials"), but `plan-b/src/` holds no such file
   (`ls src/`: `aabb, camera, hittable, main, plane, ray, rng, sphere, triangle, vec3`). The file is a
   regular file inside the shared `AGENT-INSTRUCTIONS` directory, which is a symlink shared by every
   plan worktree; `plan-a/src/material.rs` exists on the materials branch. Owner: the release merge,
   which brings the materials work and that module into the integration branch. Changing the line
   from `plan-b` would make the shared map wrong for the materials worktree, so no change is made
   here.
2. **`HitRecord::point` names a consumer that is not on this branch** - class: stale comment (minor).
   `src/hittable.rs:12` reads `// The origin of the bounce rays.` and nothing on this branch scatters
   (`hit.point` is never read; cross-check claim 1). The field itself is required: requirement 1
   scatters from the hit point, so the point is the product's next consumer, and the field is already
   carried behind `#[allow(dead_code)]` (`src/hittable.rs:13-14`). Recorded, no action; a later reader
   should read it as intent for the scattering work, not as a description of present code.
3. **Duplicated doc phrase** - class: duplication (nit). "counter-clockwise seen from its front face"
   appears in both the `Triangle` struct doc (`src/triangle.rs:8`) and `Triangle::new`'s doc
   (`src/triangle.rs:16`). No behavioural effect.
4. **Requirement wording is looser than the code** - class: stale document (nit, user-owned text).
   `docs/requirements.md:29` says "Every object reports an axis-aligned bounding box" before the
   plane exception; the scene list also takes the default "no box" (`src/hittable.rs:50-52`,
   `67-83`), which is harmless because the scene is a flat list of shapes. Operative item-10
   behaviour matches the code. The requirements are the user's text; no change is proposed here.
5. **Unused-by-design code on this branch** - class: dead code (cross-plan scaffolding, recorded not
   fixed). `Rng`/`next_f64` (`src/rng.rs:20-22`, requirement 6) has no product caller and no test of
   its own (`rng.rs:29-36` tests `next_u64` only); `mod plane` carries `#[allow(dead_code)]` because
   the scene holds no plane (`src/main.rs:6-7`), which the frozen B2 contract specifies; and
   `HitRecord::point` is written by all three shapes (`src/sphere.rs:39`, `src/plane.rs:30`,
   `src/triangle.rs:46`) but never read. All three are consumed by the materials and output work
   that follows this plan.
6. **Coverage observations, no test proposed** - class: test coverage note. The box test's
   `t`-range clipping is unexercised (`src/aabb.rs:58` runs with `near/far = 4/6` inside
   `0.001..∞`); `Sphere::bounding_box` (`src/sphere.rs:48-51`) has no direct test; `Rng::next_f64`
   (`src/rng.rs:20-22`) has neither caller nor test. The cross-check rejected the area claim that the
   zero-direction branch is untested: both rays in `src/aabb.rs:53-60` take it (`:27-32`). Adding a
   test would need the user's approval under REPO-RULES §3a; none is proposed, because the culling
   change is already proven by the byte-identical render and the list-dispatch test.
7. **Pre-existing render-test cost** - class: test cost (not introduced by this plan). Both scene
   tests call `render(WIDTH, HEIGHT)`, the full 200x100 image, and each renders it again
   (`src/main.rs:97,104`). Present since the starting commit; cheap at this size; recorded only.
8. **Tangent-hit tie-break** - class: requirement ambiguity (measure-zero). At a ray exactly tangent
   to the sphere, `direction.dot(outward_normal) == 0`, so `front_face` is false and the stored normal
   is inward (`src/hittable.rs:27`, `src/sphere.rs:28-38`). Requirement 7 does not say which side a
   zero-dot hit belongs to. No visible effect at this render size; recorded so the contract is not
   silently reinterpreted.
9. **Scene triangle margins are tight but valid** - class: observation. Its nearest vertex is 2.06
   pixels from the front sphere's silhouette and its right vertex 2.5 pixels from the frame edge
   (cross-check claim 10); fully inside the image, fully against sky, base above the ground horizon.
   No action.

## Rejected claims (do not reopen)

- "The `Aabb` zero-direction branch is untested." Rejected: both rays in the one `Aabb` test take that
  branch (`src/aabb.rs:56-58` with `:27-32`).
- "`project.md` is a symlink." Rejected: it is a regular file; the `AGENT-INSTRUCTIONS` directory is
  the symlink.
- "B2's deletion of the `mod rng` comment was unintended scope creep." Rejected as drift: B2's phase
  text explicitly required removing that comment ("no code comment refers to a plan. Add no comment in
  its place"). The commit message simply does not mention it.
- "B2's 'nine passing tests' is stale." Rejected: nine at `0bce6d1`; the three extra tests come from
  the later bounding-box commit.
- "`HitRecord::point` is dead and therefore removable." Rejected: requirement 1 makes the hit point
  the origin of scatter rays, and the field is the documented contract for that work.

## Out-of-scope observations (other plans)

- `src/main.rs:1` and `:86-89` write PPM only and take no arguments; `docs/requirements.md` items 11
  to 13 (PNG, command line, samples, progress line) belong to the output plan.
- The shared design `PLANS/shared/plan-a-A2+plan-b-B2.md` leaves one user-owned choice to the release:
  which material the scene triangle gets (recommendation: the same diffuse material as the spheres).

## Rendered-image evidence

Command run at the surveyed head `f85a183`:
`~/.cargo/bin/cargo run --release > /tmp/opencode/plan-b-acceptance.ppm` (exit 0), converted with the
PPM-to-PNG snippet to `/tmp/opencode/plan-b-acceptance.png` and opened. The PPM's md5 is
`13b3bc0a3cf5c41df684e179c1d3e93d`, the same digest the bounding-box phase recorded before and after
its change, so the culling change altered no byte. The opened 200x100 image shows: the centred sphere
shaded by its ray-facing normal (blues and violets), the flat light triangle on the right, wholly in
the sky above the ground and clear of the sphere and the frame edges, and the green ground. That is
the behaviour requirements 7 to 10 describe: the face side and facing normal, texture coordinates for
all three shapes, planes and triangles available and a triangle in the scene, and a box report with
the list testing boxes first and the boxless plane always tested.

## User-review list

1. The shared repository map line naming `material.rs` (finding 1): recommendation is to leave it to
   the release merge, which makes the line true; nothing in this plan changes it.
2. Requirement 10's "Every object reports a bounding box" wording (finding 4): optional wording
   change, and the requirements are the user's.
3. The release-time material choice for the scene triangle (out-of-scope item 2).
4. Any extra tests for the coverage notes (finding 6) would need the user's approval under the test
   budget; none is proposed.

## Verdict

Plan `plan-b`'s own work has no confirmed clear defect: the geometry, the dispatch and the documents
match requirements 7 to 10, the frozen contracts and the plan's design rules, the render is
byte-identical to the tested image, and no remediable drift from the user's decisions was found. The
plan is ready for the user's acceptance of this note and the rendered scene; closure and the release
are not this phase's work.
