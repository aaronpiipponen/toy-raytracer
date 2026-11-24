# Closing survey returns, 2026-11-24

Branch `plan-a`, surveyed range `e180476..46a5c02`. Four independent read-only area surveys and one
cross-check, saved verbatim below before the survey note was written.

## Area 1 - `src/material.rs` and the `Material` trait

### 1. Confirmed findings

**F1 — Speculative guard that hides a real error condition, and drifts from the frozen A2 contract** — *unnecessary complexity / redundant check / drift*
- `src/material.rs:102`: `let parallel = normal * -(1.0 - perpendicular.dot(perpendicular)).abs().sqrt();`
- `refract` is only reached from `src/material.rs:89`, i.e. only when `cannot_refract` was false (`refraction_ratio * sin_theta <= 1.0`). At that call site `|perpendicular|² <= 1.0` by construction, so the radicand `1.0 - |perpendicular|²` is non-negative; `.abs()` can only ever act on a value that should not be negative, i.e. on a bug. It silently converts such a bug into a plausible-looking direction instead of letting the NaN surface, which is the opposite of fail-loudly (§0). It also contradicts the A2 frozen contract, `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A2-glass.md:18` (“No other helper, clamp, validation or random source is added”). Real problem because §0/§3a forbid speculative guards and failing loudly is the house rule; the guard is not required by any requirement. (Severity: low — it cannot currently change output for realistic inputs, since the radicand is provably non-negative at the only call site.)

**F2 — Extra rejection bound not called for by the contract** — *speculative guard (borderline)*
- `src/material.rs:121`: `if len_squared > 1e-12 && len_squared <= 1.0` — the frozen A1 contract, `AGENT-INSTRUCTIONS/PLANS/plan-a/done/A1-diffuse-and-metal-materials.md:18`, describes “rejection sampling of a point in the unit ball”, i.e. the single `<= 1.0` test; the `> 1e-12` lower bound is an added condition.
- Honest judgement: this one is *defensible*, because `p.unit()` on a near-zero `p` would divide by zero; it is a standard idiom rather than tape. I report it only because §0/§3a explicitly treat extra guards as bloat, and it is an unrequested deviation from the frozen helper description. If the team considers divide-by-zero a “realistic data condition”, this is not a defect. (Severity: very low / judgement call.)

No other findings in the module passed the bar.

### 2. Checked and clean

- **Req. 1 (every object has a material; 50-bounce limit; black at limit)** — `src/hittable.rs:15` carries `Rc<dyn Material>`; `src/main.rs:25` `MAX_BOUNCES = 50`, `src/main.rs:32` loop, `src/main.rs:39` absorb → black, `src/main.rs:45` limit → black. Correct.
- **Req. 2 (diffuse: random direction around normal, albedo attenuation)** — `src/material.rs:29-30` `hit.normal + random_unit_vector(rng)`, returns `self.albedo`. Matches A1 (`normal + random_unit_vector` without normalising).
- **Req. 3 (metal: reflect about normal, fuzz, albedo, absorb into surface)** — `src/material.rs:49-55`: unit incoming direction reflected, `random_unit_vector * fuzz` added, `dot(normal) <= 0.0` → `None`, otherwise `self.albedo`. Fuzz is not clamped, but the contract states fuzz is “in 0..=1” as a field invariant, and the scene supplies 0.3 (`src/main.rs:69`); no validation was requested.
- **Req. 4 (glass: Snell, TIR, Schlick, no attenuation)** — `src/material.rs:73-91`: face side derived locally exactly as A2 requires, ratio `1/index` entering / `index` leaving (`:75-79`), TIR test `:83`, `cannot_refract || schlick > rng.next_f64()` with the load-bearing short-circuit (`:84-86`), white attenuation (`:91`). I hand-verified `refract` (`:99-104`), `reflect` (`:95-97`) and `schlick_reflectance` (`:107-110`) against the maths; all correct. Scene uses index 1.5 (`src/main.rs:74`).
- **Req. 6 (seeded generator)** — `scatter` takes `&mut Rng`; `src/main.rs:85` one `Rng::new(SEED)` with `SEED = 7`. No second random source in the module.
- **Simplicity / duplication / dead code** — one trait, three implementations, no duplicate scatter path. All four private helpers (`reflect`, `refract`, `schlick_reflectance`, `random_unit_vector`) are reachable and used; no `#[allow(dead_code)]` and no dead item in `src/material.rs`. `reflect` is shared by metal and glass (good reuse). `refract` recomputes `cos_theta` already computed at `:81`; this is *contract-mandated* (A2 fixes the `refract(unit_direction, normal, ratio)` signature), so not a finding.
- **§0b (records/hashes/receipts)** — none in the module; nothing to flag.
- **§3a (test budget)** — six tests, but they split 3 in A1 (`done/A1…md:47-51`) and 3 in A2 (`done/A2…md:36-40`), each within the per-phase cap of 3. Each test covers a distinct rule from reqs. 2/3/4, which `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md` explicitly requires (“the scattering rules of each material”). No test bloat found. The glass normal-incidence test currently depends on the first `rng.next_f64()` exceeding Schlick’s 0.04, but that is deterministic under the fixed seed and not a defect.
- **Docs / decisions** — module and item doc comments accurate; index 1.5, std-only (`Cargo.toml` has no deps), no `unsafe`, single-threaded. No stale doc in the module.

### 3. Out-of-scope observations
- `src/vec3.rs:21-29`: `Vec3::cross` is dead code kept behind `#[allow(dead_code)]` with a “triangle normals the geometry plan adds” comment. The geometry absence is expected on this branch and A1 (`done/A1…md`, last doc bullet) records the allowance as intentional, so this is likely correct-as-planned, not a finding — noted only so the closing survey sees it.
- `src/main.rs:126` hardcodes 16 samples in `main`; requirement 5's configurable/default sample count and the CLI belong to the output plan, which the brief says is out of scope. Not investigated further.

### Unknowns
- Whether the team treats the `1e-12` lower bound in `random_unit_vector` as an acceptable divide-by-zero guard or as the banned speculative guard is a judgement call I could not settle from the rules; resolving it needs a maintainer decision, not more evidence.
- I did not run `cargo test` / `clippy` / `fmt` (read-only role); correctness above is from reading the code and hand-checking the maths, and the phase receipts (`done/A2…md`, `Z1` current state) report the gates green at this head.

## Area 2 - `src/hittable.rs` and `src/sphere.rs`

### 1. Confirmed findings

**F1 — stale doc: `t` is described as a distance, but it is a ray parameter.** Category: stale doc.

- `src/hittable.rs:9-10` — "Where a ray hit a surface: **the distance along the ray**, the point, the outward normal and the material of the object."
- `src/camera.rs:31-35` builds `Ray::new(origin, lower_left + horizontal * u + vertical * v - origin)`; the direction is not normalised (`horizontal`/`vertical` from `camera.rs:19-21`), so its length is not 1 except near the viewport centre.
- `src/sphere.rs:29,37-39` solves the quadratic in the ray parameter (`a = ray.direction.dot(ray.direction)`, `t = (-half_b ± root) / a`), so `HitRecord.t` is `t` in `ray.at(t)`, not a distance.
- Only the test at `src/sphere.rs:67` uses a unit direction, which is why "distance" reads plausibly there.

Why it is a real problem: the doc states a unit that the code does not hold. The next consumer of `t` is the geometry work (AABB slab tests and the `t_min` epsilon), where the parameter-vs-distance distinction changes the arithmetic; a reader trusting this line would size bounds and biases incorrectly. The wording is inherited unchanged from the baseline (`git show e1293ef:src/hittable.rs` line 4 has the same sentence), so it is not introduced by this plan, but it is current in the branch's area.

No other confirmed finding in this area.

### 2. Facts checked and found clean

- `HitRecord`'s four fields (`hittable.rs:12-15`) are each consumed: `t` in nearest selection (`hittable.rs:39`), `point`/`normal` by the materials (`material.rs:29-30`, `50`, `54`, `91`), `material` by the render loop (`main.rs:34`). No unused record field.
- `HittableList::hit` (`hittable.rs:36-45`) is a single nearest-hit loop; `as_ref` + `map_or` reads the running limit from the record found so far, no duplicated distance logic and no second scene path.
- One construction path per object in this area: `Sphere::new` (`sphere.rs:17-23`) and the single record literal (`sphere.rs:45-50`); the material is cloned once per hit (`sphere.rs:49`), matching the settled `Rc<dyn Material>` sharing decision.
- The record's fields and the pre-merge `t/point/normal/material` shape, literal construction, no `front_face`/`u`/`v`, and no derives all match the recorded branch state in `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B1.md` ("plan-a / A1: unchanged as detailed… literal construction on this branch") and `done/A1r-shared-hit-record-settlement.md:19-23`; the doc's "outward normal" wording is the same A1 branch wording the settlement says the merge replaces, so it is not reported as drift.
- `Hittable`/`HittableList` carry no material logic of their own and no randomness; requirement 1's "every object has a material" is met because the only object type (`Sphere`) stores one and puts it on the record (`sphere.rs:13`, `49`, used at `main.rs:34`).
- No dead code in the two files: both imports in each file are used (`hittable.rs:3-7`; `sphere.rs:3-8`), `HittableList::add` is called four times (`main.rs:56,61,66,71`), and `#[derive(Default)]` is used at `main.rs:55`.
- Test purpose/budget (`sphere.rs:54-85`): two tests, one for the near root and outward normal, one for a miss, each with a distinct purpose; the repeated `Lambertian` construction is two lines of unavoidable setup, not copy-pasted logic, so no test bloat under §3a. They do not assert the record's material, but that is a coverage gap rather than bloat and the wiring is a single `Rc::clone` line.
- No unnecessary record, hash or receipt in this area: nothing is persisted, and `t`/`point`/`normal`/`material` are used live only.
- No duplicate material path between `Sphere` and `HittableList`; `HittableList` is agnostic to material and just forwards the record.

### 3. Out-of-scope observation

- `src/vec3.rs:21-22` — `cross` is marked `#[allow(dead_code)]` with the comment "For the triangle normals the geometry plan adds." It is geometry-plan work suspended in this materials branch as dead code, and the comment names a plan in permanent source. Evidence that it is not this plan's change: `git log --oneline -- src/vec3.rs` shows only baseline commit `e1293ef`, and `git log --oneline main..HEAD -- src/vec3.rs` is empty. Reported only; not investigated further.

### Unknowns

- Nothing in the area depends on the merged `HitRecord` shape beyond what the shared settlement documents, so the F1 wording fix has no code-side unknown; whether the release merge already plans to reword that line is not stated in `PLANS/shared/plan-a-A1+plan-b-B1.md`, which the cross-check can confirm.

## Area 3 - `src/main.rs` and `src/rng.rs`

### 1. Confirmed findings

**Finding 1 — low severity, drift from the workspace test rule (test policy).**
`src/main.rs:142` and `src/main.rs:147` compare two *whole-image* pixel vectors:
`assert_eq!(render(40, 20, 4), render(40, 20, 4))` and `assert_ne!(render(40, 20, 1), render(40, 20, 4))`.
`AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md:37` says "Rendering tests check a few pixels of a small image, never a whole image." These two new tests assert over the entire 800-pixel render rather than a few pixels.
Caveat for the orchestrator: the frozen A3 contract (`done/A3-samples-per-pixel.md`, "Tests" section) mandates these exact tests, so this is not implementer drift — it is the phase contract sitting against the local test rule. It is not test bloat by count (2 new ≤ the §3a cap of 3).

**Finding 2 — low severity, drift candidate from requirement 5 / the "jitter centred inside each pixel" decision.**
`src/main.rs:91-92` compute `u = (column + rng.next_f64() - 0.5) / (width - 1)` (and `v` likewise). Because the denominator stays `width - 1`, the jitter from `-0.5` moves the draw to `[(c-0.5)/(w-1), (c+0.5)/(w-1))`, so for the first and last columns/rows half of each edge pixel's samples fall *outside* the 0..1 viewport: at width 200, column 0 gives `u ∈ [-0.00251, 0.00251)` and column 199 gives `u ∈ [0.99749, 1.00754)`. Requirement 5 (`docs/requirements.md:17`) says "jittered inside the pixel", and the settled decision says "jitter centred inside each pixel"; for edge pixels that is only half true. This is a pre-existing half-pixel grid offset (base `e1293ef`/`80d833c` already used the `width - 1` mapping), which `-0.5` inherits; the frozen A3 contract explicitly freezes this formula ("The pixel grid keeps the `width - 1` … denominators"). I report it because it is the one place the code's own claim ("centres every draw inside its own pixel") is not literally true, not as an implementer error.

**In-area observation (pre-existing base code, not introduced by this plan).**
`src/rng.rs:9` silently maps seed 0 to 1 via `seed.max(1)`. That is a silent fallback rather than a loud failure, but it guards the genuine xorshift64 zero fixed point and has no effect with the fixed `SEED = 7`; `src/rng.rs` is unchanged from base `e1293ef`. Not triggered, so I do not call it a defect of this plan.

### 2. Facts checked and found clean

- Bounce loop: `MAX_BOUNCES = 50` (`main.rs:25`); each hit multiplies the accumulated colour by the material attenuation (`:36`); absorption returns black (`:39`); after 50 scatter events the function returns black (`:32`, `:45`) — requirements 1 and 4 hold.
- Gamma 2 is applied exactly once, in `to_byte` (`main.rs:102-104`), and only at output (`:113-120`); never per sample and not re-applied in the bounce loop.
- Sample loop: row → column → sample → bounce (`:87-96`), sum in linear space starting at `Colour::new(0,0,0)` (`:89`), one division by `samples` after the loop (`:95`) — requirement 5's averaging and default 16 (`:126`) hold; `render` takes the count as a parameter.
- Jitter draw order: exactly two `rng.next_f64()` draws per sample before `ray_colour`, then the trace (`:91-93`), matching the frozen A3 draw order; no guard/clamp for `samples` (as frozen).
- One generator per render: `Rng::new(SEED)` with `SEED: u64 = 7` (`:24`, `:85`), threaded as `&mut` through every trace (`:93`) into material scattering — requirement 6 and the "one seeded generator created once per render" decision hold.
- No other randomness source: `rg` for `rand`, `thread`, `SystemTime`, `Instant` in `src/` returns nothing; the only random calls in the tree are `rng.next_f64()` at `main.rs:91,92` and `material.rs:85,116-118`, all via the passed `&mut Rng`. No `unsafe`.
- Scene (`main.rs:54-77`) has a red-brown diffuse centre sphere, grey diffuse ground, a metal sphere with fuzz 0.3 and a glass sphere with index 1.5 — matte/metal/glass represented as required.
- Determinism: a single generator, fixed row-major consumption and single-threaded float code (`:85-96`); `the_same_settings_render_the_same_pixels` (`:141-143`) exercises it.
- Test budget: 2 new test functions in `main.rs` plus one adapted (`the_top_row_is_sky`), within the §3a cap of 3; purpose is reproducibility and sample-count dependence, not a second implementation.
- REPO-RULES §0b: no records, receipts, hashes or audit state in `main.rs`/`rng.rs`.
- `write_ppm`/`to_byte`/`sky_colour`/`scene`/`ray_colour` are all reachable; no dead code within my two files.

### 3. Out-of-scope observation

- `src/vec3.rs:21-29` carries `cross` as dead code behind `// For the triangle normals the geometry plan adds.` + `#[allow(dead_code)]`. Evidence: `git blame` shows it was introduced in the base commit `e1293ef`, and triangular geometry is the other plan's territory; I did not investigate further. (Same file is outside my area, so this is reported with the evidence already in hand only.)

### Unknowns

- I did not run the render or the tests (read-only), so I cannot empirically confirm byte-identical output or judge the visual impact of the edge-pixel jitter; both would need the rendered-image gate.
- I could not determine whether the user intends the `width - 1` pixel grid to override the literal "inside the pixel" wording; that is a decision for the orchestrator/user.

## Area 4 - tests and documentation

### 1. Confirmed findings

**F1. Permanent code comment names a plan and describes work not on this branch — drift / plan reference in a permanent file.**
- Evidence: `src/vec3.rs:21` `// For the triangle normals the geometry plan adds.` sits above `#[allow(dead_code)] pub fn cross` (`src/vec3.rs:22-29`). `git blame` shows it comes from the base commit `e1293ef` (pre-branch), and `grep` finds no caller of `cross` anywhere in `src/`.
- Why it is real: REPO-RULES §4 item 13 forbids "history or plan ids in permanent files", and the documentation style guide's current-behaviour/only-what-exists rules forbid naming plans or absent features. The comment references a plan and a triangle feature this branch does not contain, so it is both a plan reference and a false statement about the code. (Other plan references of the same kind were already removed in `281dda8` and `66097bd`; this one survives.)

### 2. Checked and clean

- **Test inventory:** exactly 12 test functions, all unit tests under `src/`, no `tests/` directory or suite duplication (`src/main.rs:135,141,146`; `src/rng.rs:30`; `src/sphere.rs:61,76`; `src/material.rs:134,154,174,188,208,230`). A3's "12 passed" claim matches this count.
- **Coverage maps to the domain's required areas:** hit geometry (`src/sphere.rs:61,76`), each material's scattering rules (`src/material.rs:134,154,174,188,208,230`), and small rendered-image behaviour (`src/main.rs:135,141,146`). No test targets a private helper (`reflect`, `refract`, `schlick_reflectance`, `random_unit_vector` are exercised only through `Material::scatter`).
- **Assertions are purposeful, not internal-value repeats:** the metal reflect test pins the full reflected direction (`src/material.rs:168-170`), the angled-glass test derives the expected bent vector from Snell (`src/material.rs:222-226`), and the critical-angle test pins the reflected vector (`src/material.rs:244-246`). These exercise distinct paths, not one value pinned twice.
- **User decisions hold:** glass index `1.5` (`src/main.rs:74`), 16 samples from `main` (`src/main.rs:126`), jitter centred with `- 0.5` inside each pixel (`src/main.rs:91-92`), gamma applied once in `to_byte` (`src/main.rs:102-104`, not per sample), single-threaded, std-only (`Cargo.toml` has an empty `[dependencies]`), no `unsafe` (grep clean).
- **Requirements match code:** items 1-6 are all satisfied — 50-bounce limit and black at limit (`src/main.rs:25,32,45`), Lambertian albedo scatter (`src/material.rs:27-31`), metal fuzz + into-surface absorption (`src/material.rs:47-56`), glass Snell/TIR/Schlick with white attenuation (`src/material.rs:71-93`), samples/jitter/gamma (`src/main.rs:82-99,102-104`), single seeded `Rng` with `SEED = 7` (`src/main.rs:24,85`).
- **README accuracy for this branch:** `cargo run > image.ppm` and `cargo test` match a program that writes P3 PPM to stdout (`src/main.rs:106-128`); "dependency-free" and "one fixed scene" are true. `git diff e180476..46a5c02 -- docs/ README.md` is empty, so this branch introduced no doc drift.
- **Requirements items 7-13 described but absent:** consistent with `docs/requirements.md` being the finished-program spec (README line 10 says so), not a claim about this branch, so not a finding.
- **No plan/phase ids elsewhere:** grep of `src/`, `docs/`, `README.md` for plan names, phase ids, merge/release/branch/future wording returns only `src/vec3.rs:21`.

### 3. Minor observations (low severity, evidence included)

- **Repeated non-attenuation assertion:** the glass "does not attenuate" fact is asserted in all three glass tests (`src/material.rs:201,221,243` → `Vec3::new(1.0,1.0,1.0)`), even though all three go through the single unconditional `Some((Vec3::new(1.0,1.0,1.0), ...))` return at `src/material.rs:91`. Category: minor duplication; one assertion would prove requirement 4's non-attenuation. Cheap, so not harmful.
- **Full-image render to assert one pixel:** `the_top_row_is_sky` calls `render(WIDTH, HEIGHT, 16)` (`src/main.rs:136`), i.e. 200×100×16 = 320,000 samples, to assert a single pixel (`src/main.rs:137`). The domain test exception says "Rendering tests check a few pixels of a small image, never a whole image" (`AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md`), and the other render tests use 40×20 (`src/main.rs:142,147`). Category: test cost/bloat; the plan's A3/A1r2 contract freezes this call, so changing it touches the merge.

### 4. Out-of-scope observations

- **Dead code:** `Vec3::cross` (`src/vec3.rs:23-29`) has no caller on this branch and is kept alive only by `#[allow(dead_code)]` (`src/vec3.rs:22`). It is product code (area 2/3), reported here because it is what the F1 comment describes.
- No remote hosts, workers or runtime config exist; nothing in the ops-scout domain file applies.

### 5. Unknowns

- I did not execute `cargo test` (read-only role), so the "12 passed at `46a5c02`" claim is taken from `A3-samples-per-pixel.md:45`; the count of test functions matches but pass status was not independently run.
- Whether `the_top_row_is_sky` at WIDTH×HEIGHT is a meaningful runtime cost in seconds was not measured.
