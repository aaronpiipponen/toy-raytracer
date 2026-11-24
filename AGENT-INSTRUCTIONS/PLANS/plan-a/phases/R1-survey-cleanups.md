## Phase R1 - Survey cleanups

**Status:** active.
**Depends on:** A1 (`done/A1-diffuse-and-metal-materials.md`), A2 (`done/A2-glass.md`), A3
(`done/A3-samples-per-pixel.md`), A3r (`done/A3r-samples-resolution.md`) and the accepted closing
survey `benchmarks/closing-survey-2026-11-24.md` with its five returns.
**Scope:** fix exactly the accepted non-behavioural survey items: the seed-0 silent substitution in
`src/rng.rs`, the duplicate scene-list surface in `src/hittable.rs`, the repeated sphere test setup in
`src/sphere.rs`, two stale sentences in the frozen records `done/A3r-samples-resolution.md` and
`done/A1-diffuse-and-metal-materials.md`, and the `colour`/`pixel` spelling in
`AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md`. Rendering behaviour does not change: the
default 200 by 100, 16-sample render produces byte-identical output.
**Gate level:** standard - two small runtime-neutral code edits and one test-setup edit, plus
plan-folder documentation. The fast gate runs, and the rendered-image conditional gate proves the
bytes are unchanged.
**Simplicity statement:** the seed rule is one `assert!` that raises with a clear message, so the
generator has one authoritative input range instead of a `max(1)` substitution. One scene-list
surface stays (`add()`), so no second way to fill the list exists. The sphere tests share one local
helper. Nothing else is touched: no test function is added (the §3a cap is spent and the user accepted
the coverage gaps), the `1e-12` guard stays, the record fields stay public and mutable, the `pixel`
local name in `src/main.rs` and the frozen `pixel` wording in `done/A3-samples-per-pixel.md` stay, and
no rendering, sampling, material, scene, writer or command-line behaviour changes.

**Current state (verified):**

- `src/rng.rs:8-10` - `pub fn new(seed: u64) -> Self { Rng { state: seed.max(1) } }`. `max(1)` maps
  seed 0 onto seed 1. The only product call is `Rng::new(SEED)` at `src/main.rs:85` with
  `const SEED: u64 = 7;` (`src/main.rs:25`). Tests call it with the literal 7 at `src/material.rs:146,
  166, 187, 211, 233` and `src/rng.rs:31-32`. Nothing passes 0.
- `src/hittable.rs:58-67` - `#[derive(Default)] pub struct HittableList { pub objects: Vec<Box<dyn
  Hittable>> }` with `impl HittableList { pub fn add(&mut self, object: Box<dyn Hittable>) {
  self.objects.push(object); } }`. `HittableList::hit` at `:69-80` iterates `&self.objects`.
- Every construction site already uses `HittableList::default()` plus `add()`: `src/main.rs:56-77`
  (four `world.add(...)` calls). No file reads or assigns `.objects` from outside the module, and no
  struct-literal `HittableList { objects: ... }` exists in the tree.
- `src/sphere.rs:63-116` - three tests, each opening with the identical
  `Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5, Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.8))))`:
  `a_ray_through_the_center_hits_the_near_surface` at `:73-77`,
  `a_ray_from_inside_hits_the_back_face` at `:91-95`, `a_ray_beside_the_sphere_misses` at
  `:109-113`. The test module imports `Rc`, `Lambertian` and `Colour` at `:65-69`.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A3r-samples-resolution.md:8` says
  "`phases/A3-samples-per-pixel.md` is amended in place to that design and sits at `planned`", and
  `:42-43` says "Exit state: A3 remains `planned` with `phases/A3-samples-per-pixel.md` and is
  implemented after the pair returns `RESOLVED`". A3 is `done` and its contract is
  `done/A3-samples-per-pixel.md`; `phases/` holds no A3 file.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1-diffuse-and-metal-materials.md:31-32` says
  "`HitRecord.normal` is the outward normal and the record carries no face-side flag. The face-side
  shape shared with the geometry plan is settled through the coordinator before A2 is detailed."
  A2 landed a ray-facing `normal` and a `front_face` field (`src/hittable.rs:11-49`), and the shared
  shape is `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A2+plan-b-B2.md`.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md:22,26,28` spells the per-pixel accumulator
  `let mut colour`, `colour = colour + ray_colour(...)` and `colour / samples as f64`, while the
  landed code at `src/main.rs:89-95` and the frozen contract at `done/A3-samples-per-pixel.md:10-11`
  use `pixel`. Line 13's doc sentence ("Every pixel's colour") is prose and stays.
- Baseline evidence captured before this detail: at head `566b11c` with a clean worktree,
  `./target/release/toy-raytracer > /tmp/opencode/r1-baseline.ppm` wrote 239842 bytes, sha256
  `5457ef7a788539f5f5b0145e7c20fd178a77f0d184663b83fcd38d299a54665b`, and a second run gave the same
  hash.

**Read first (do not invent):**

- `src/rng.rs` - the whole file; `Rng::new(u64) -> Self` is the only seed entry point.
- `src/hittable.rs` - `HitRecord`, `Hittable`, `HittableList`, its `add` and its `hit`, all in one
  module, so a private `objects` field stays usable by `HittableList::hit` and `add`.
- `src/sphere.rs` - the test module and its three identical fixture blocks.
- `src/main.rs` - `SEED` at `:25`, the `Rng::new(SEED)` call at `:85`, and `scene()` at `:55-78`,
  which is the only list construction site.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/closing-survey-2026-11-24.md` - confirmed findings 4 to
  10 and the user-review list: this phase implements exactly those, and findings 1 to 3 and every
  rejected claim stay untouched.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A3r-samples-resolution.md`,
  `done/A1-diffuse-and-metal-materials.md`, `done/A2-glass.md`, `done/A3-samples-per-pixel.md` - the
  frozen contracts the corrected sentences must agree with.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md` - lines 12-36 hold the merged `render`
  snippet whose three accumulator lines change.

**Build plan:**

1. `src/rng.rs:9`, `Rng::new` - replace `Rng { state: seed.max(1) }` with
   `assert!(seed != 0, "the generator seed must be nonzero");` followed by `Rng { state: seed }`.
   Signature stays `pub fn new(seed: u64) -> Self`. The state sequence for every nonzero seed is
   unchanged, so `SEED = 7` renders the same bytes.
2. `src/hittable.rs:60`, `HittableList.objects` - change `pub objects: Vec<Box<dyn Hittable>>` to
   `objects: Vec<Box<dyn Hittable>>`. Keep `#[derive(Default)]` and keep `pub fn add(&mut self,
   object: Box<dyn Hittable>)` at `:64-66` as the one way to fill the list. `add` pushes into the
   field and `HittableList::hit` reads it, both inside this module, so neither body changes.
3. `src/sphere.rs:63-116`, test module - add one local helper above the first test:
   `fn grey_sphere() -> Sphere` returning the fixed fixture
   `Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5, Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.8))))`.
   Replace each of the three fixture blocks at `:73-77`, `:91-95` and `:109-113` with
   `let sphere = grey_sphere();`. Keep every assertion, ray and test name exactly as it is. The
   imports at `:65-69` stay as they are.
4. `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A3r-samples-resolution.md` - two corrections, keeping the
   record a frozen contract:
   - line 8: replace "`phases/A3-samples-per-pixel.md` is amended in place to that design and sits at
     `planned`. The amendments are:" with "A3's contract is `done/A3-samples-per-pixel.md`, amended to
     that design. The amendments are:".
   - lines 42-43: replace "Exit state: A3 remains `planned` with `phases/A3-samples-per-pixel.md` and
     is implemented after the pair returns `RESOLVED`." with "Exit state: A3 is `done`, with its
     contract at `done/A3-samples-per-pixel.md`."
   Every other line of the record stays as written.
5. `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1-diffuse-and-metal-materials.md:31-32` - replace the two
   sentences with: "`HitRecord.normal` faces against the incoming ray and the record carries
   `front_face`. The shared shape with the geometry plan is
   `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A2+plan-b-B2.md`." Every other line of the record stays.
6. `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md:22,26,28` - rename the accumulator in the
   `render` snippet only: `let mut colour = Colour::new(0.0, 0.0, 0.0);` becomes
   `let mut pixel = Colour::new(0.0, 0.0, 0.0);`, `colour = colour + ray_colour(&camera.ray(u, v),
   &world, &mut rng);` becomes `pixel = pixel + ray_colour(&camera.ray(u, v), &world, &mut rng);`, and
   `pixels.push(colour / samples as f64);` becomes `pixels.push(pixel / samples as f64);`. No other
   line of that file changes, and no plan-c file is touched.
7. `AGENT-INSTRUCTIONS/PLANS/plan-a/full-plan.md` - no edit is required. The R1 row is already
   `active` with `Phase file` set to `phases/R1-survey-cleanups.md`, Z1 is `next`, and the implementer
   changes nothing in this file. Flipping R1 to `done` happens on passing evidence per the completion
   checklist.

**Callers / wiring to update:**

- `src/main.rs:85` - `Rng::new(SEED)` needs no change; `SEED` is 7, so the new assertion holds and the
  generator stream is identical.
- `src/main.rs:56-77` - the four `world.add(...)` calls already use the one remaining surface; no
  change.
- No other caller reads `Rng::new` with 0 and no file outside `src/hittable.rs` touches
  `HittableList.objects`, so no import or signature ripples.

**Config / schema / migrations:**

- None.
- Retained records and their named consumer/owner: none added. The corrected frozen records and the
  shared design are plan-folder contracts read by later phases and the release merge; they are not
  product records.

**Touched files:**

<!-- touched-files:start -->
- `src/rng.rs` - changed: `Rng::new` asserts a nonzero seed instead of substituting 1.
- `src/hittable.rs` - changed: `HittableList.objects` becomes private; `add()` stays the one entry.
- `src/sphere.rs` - changed: one local `grey_sphere()` test helper replaces three identical fixture
  blocks; assertions unchanged.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A3r-samples-resolution.md` - changed: two sentences state
  A3 is done with its contract in `done/`.
- `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1-diffuse-and-metal-materials.md` - changed: the record is
  described with the ray-facing normal, `front_face` and the shared shape file.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A3+plan-c-Z1.md` - changed: the `render` snippet names the
  accumulator `pixel`.
- Adds: nothing.
<!-- touched-files:end -->

**Rules / MUST NOT:**

- Do not add, delete or rename a test function; the §3a cap is spent and the user accepted the
  coverage gaps. The sphere change is setup only, with identical assertions.
- Do not change rendering output. The default 200 by 100, 16-sample PPM must be byte-identical to the
  baseline hash, and the sampling loop, `SEED`, `MAX_BOUNCES`, the scene, `to_byte`, `write_ppm` and
  the materials stay untouched.
- Do not add a second way to build the scene list, and do not add an `objects()` accessor, a clear
  method or a constructor for `HittableList`.
- Do not replace the assertion with a clamp, a `max(1)`, a `debug_assert!` or a panic-free fallback.
  The nonzero rule is the one authoritative rule.
- Do not edit the `1e-12` guard in `src/material.rs`, the public record fields, the `pixel` local name
  in `src/main.rs`, `done/A3-samples-per-pixel.md`, `docs/requirements.md`, `README.md`, any
  `AGENT-INSTRUCTIONS/LOCAL/` file or any plan-b or plan-c file.
- Do not rewrite or expand the two frozen records beyond the named sentences; they stay contracts, not
  history.
- No dependency, no `unsafe`, no threads, no `Cargo.toml` change.

**Tests:**

- New/changed/deleted: none added, none deleted, three adapted without assertion changes -
  `src/sphere.rs` - `a_ray_through_the_center_hits_the_near_surface`,
  `a_ray_from_inside_hits_the_back_face`, `a_ray_beside_the_sphere_misses`: each takes its sphere from
  the shared local helper. `src/rng.rs` - `the_same_seed_gives_the_same_sequence` is unchanged and
  keeps passing under the assertion.
- Fast gate: `~/.cargo/bin/cargo build`, `~/.cargo/bin/cargo test`, `~/.cargo/bin/cargo clippy --
  -D warnings`, `~/.cargo/bin/cargo fmt --check`, `git diff --check`. Every existing test passes; the
  test count the run reports is an observation for the receipt, not an acceptance gate.
- Conditional gates: rendered image, required. Capture the PPM before the first code edit and again
  after the last one, at the default 200 by 100, 16 samples, and compare:
  ```bash
  ~/.cargo/bin/cargo run --release > /tmp/opencode/r1-before.ppm
  sha256sum /tmp/opencode/r1-before.ppm
  # after the edits
  ~/.cargo/bin/cargo run --release > /tmp/opencode/r1-after.ppm
  cmp /tmp/opencode/r1-before.ppm /tmp/opencode/r1-after.ppm
  sha256sum /tmp/opencode/r1-after.ppm
  ```
  Expected: `cmp` prints nothing and both hashes are
  `5457ef7a788539f5f5b0145e7c20fd178a77f0d184663b83fcd38d299a54665b`. Byte identity to the accepted
  200 by 100 image proves the rendered result, so no separate visual inspection is required. Agent
  generation: not affected.
- Slow gate (>2 minutes): none. The release render of a 200 by 100, 16-sample image finishes well
  under a minute.

**User test (acceptance):**

- **Executor:** not required - automated phase gates prove acceptance.
- The user already accepted the rendered image at `566b11c`; the byte-identical PPM at the new head
  is the same image, and every other item is a plan-folder record correction or a behaviour-neutral
  code cleanup.

**Completion checklist (gate):**

- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a (standard) are satisfied.
- [ ] `Rng::new(0)` panics with the nonzero-seed message and `Rng::new(7)` yields the same sequence as
  before.
- [ ] `HittableList.objects` is private and `add()` is the only way to fill the list; `scene()` still
  builds the four spheres.
- [ ] The three sphere tests use the local helper with unchanged assertions.
- [ ] `done/A3r-samples-resolution.md` and `done/A1-diffuse-and-metal-materials.md` state the true A3
  and A2 contracts; `PLANS/shared/plan-a-A3+plan-c-Z1.md` names the accumulator `pixel`; no plan-c
  file changed.
- [ ] `cmp` shows the default 200 by 100, 16-sample PPM is byte-identical to the pre-change render and
  both sha256 values are `5457ef7a788539f5f5b0145e7c20fd178a77f0d184663b83fcd38d299a54665b`.
- [ ] Every named and existing test passes, with no test function added or deleted; the reported
  test count is recorded as an observation only.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports 0 added, 0 deleted, 3 adapted.
- [ ] Fast phase gate and the rendered-image conditional gate pass.
- [ ] No `docs/` page needs a change, and none is stale or wrong after this phase.
- [ ] Safety commit on branch `plan-a`.
- [ ] Revision-bound verification receipt exists with the base and tested head.
- [ ] On passing evidence: compress this file into `done/R1-survey-cleanups.md` (frozen contract only)
  and flip the `full-plan.md` status row to `done`. Z1 stays `next` for the next advancer round.

**Exit:** the five accepted non-behavioural cleanups are in place, the fast gate passes, and the
default render is byte-identical to the accepted image.
