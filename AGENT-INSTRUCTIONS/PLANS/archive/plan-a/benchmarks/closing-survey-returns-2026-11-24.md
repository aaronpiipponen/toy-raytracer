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

## Cross-check

### Answer

The four returns are substantially accurate on facts and line references, but three of their headline findings do not survive checking. The `.abs()` in `refract` and the `> 1e-12` bound in `random_unit_vector` are not speculative drift: A1/A2 never specify the loop or formula body, and both are the standard floating-point-safe forms of the named helpers, so they should stay. The jitter finding is mis-framed: no sample leaves its own pixel; under the frozen `width - 1` grid it is the outermost pixel cells that overhang the viewport. The real, handful of document/test findings are the stale `HitRecord` "distance" doc (a genuine stale doc, though not clearly owned by this plan), the whole-image render in `the_top_row_is_sky` (literal rule violation, contract-frozen, 0.32 s debug), and `Vec3::cross`'s plan-naming comment (real §4 item 13 violation, but owned by plan-b/B2 and already fixed on `main`). All four changed files and requirements 1–6 are covered by at least one area.

### Contested points settled

1. **`refract` `.abs()` (`src/material.rs:102`) — legitimate, keep.** A2 (`done/A2-glass.md:15-19`) names the helper signature and semantics only; it never specifies the body, and `.abs()` is not a helper, clamp, validation or random source, so there is no contract drift. The call site is `:83-89`, and `cannot_refract` is false there, so mathematically `1 - |perp|² ≥ 0`; but `|perp|²` is evaluated through a different rounding path than `ratio*sin_theta`, so at the Snell/TIR boundary it can be a few ulps above 1. `.abs()` then yields 0 (the tangent direction) instead of `sqrt(-ε) = NaN`. It cannot mask wrong output for valid input because the radicand must be ~0. Area 1's "should be removed / fails loudly" conclusion is rejected.

2. **`random_unit_vector` `len_squared > 1e-12` (`src/material.rs:121`) — legitimate, keep.** A1 (`done/A1-diffuse-and-metal-materials.md:17-19`) says "rejection sampling of a point in the unit ball", but does not state the loop test, so "the single `<= 1.0` test" is an inference the scout invented. Excluding a ball of radius 1e-6 around the origin cannot bias the returned direction (normalisation is scale-invariant and the excluded set has measure ~1e-18), and it prevents `p.unit()` dividing by zero. Not a deviation, not a defect.

3. **Whole-vector comparisons (`src/main.rs:142,147`) — literal rule deviation, but benign and contract-frozen.** `repo-rules-domain.md:37` says "a few pixels of a small image, never a whole image"; these compare the entire 40×20 (800-pixel) vector. However 40×20 is a small image (measured 0.01 s per test), and the invariants proved (byte-identical reproducibility, sample-count dependence) are inherently whole-buffer properties that a few pixels could not prove. A3 (`done/A3-samples-per-pixel.md:40-42`) mandates these exact calls. Low severity, not actionable without changing the contract.

4. **Jitter overhang (`src/main.rs:91-92`) — samples do not leave their pixel; the edge pixel cells overhang the viewport.** With `u = (c + rng − 0.5)/(w−1)`, each draw is within ±0.5/(w−1) of `c/(w−1)`, i.e. centred in its own cell — requirement 5's "jittered inside the pixel" holds. Under the `width − 1` mapping the implied pixel centres are at `c/(w−1)`, so column 0's cell is `[−0.5/199, 0.5/199) ≈ [−0.00251, 0.00251)` and column 199's cell is `[198.5/199, 199.5/199) ≈ [0.99749, 1.00251)`: the *cells* overhang `[0,1]`, not the samples leaving their pixel. Area 3's upper bound `1.00754` is also wrong (that divided by 198). This is baseline `width − 1` mapping, explicitly frozen by A3 `:11-14`.

5. **`HitRecord` doc "distance along the ray" (`src/hittable.rs:9-10`) — genuinely stale.** `Ray::at` is `origin + direction*t` (`src/ray.rs:16-18`), the camera direction is non-unit except at the viewport centre (`src/camera.rs:19-21,31-35`), and `Sphere::hit` solves for the parameter `t` with `a = direction·direction` (`src/sphere.rs:29,37-40`). So `t` is a parameter, not a distance — confirmed. Caveat: the sentence was edited by this plan's A1 commit `a1358ff`, and A1r (`done/A1r-shared-hit-record-settlement.md:32-33`) assigns only the normal wording to the merge, not the "distance" word; so this is a genuine stale doc that no phase explicitly owns.

6. **`the_top_row_is_sky` (`src/main.rs:135-138`) — literal violation, contract-frozen, measured 0.32 s.** It renders 200×100×16 = 320,000 samples to assert one pixel (`pixels[0]`), so it is a whole production-size image, contrary to `repo-rules-domain.md:37`. `cargo test` here: 12 passed, suite `finished in 0.32s`; this test alone is `0.32s`, the two 40×20 render tests are `0.01s` each. A3 `:21-22` and A1r2's merge obligation freeze `render(WIDTH, HEIGHT, 16)`, so fixing it needs a contract change, not implementer remediation. In release builds the cost would be far smaller, but I did not measure that.

7. **`Vec3::cross` (`src/vec3.rs:21-29`) — real §4 item 13 violation, owned by plan-b/B2, out of plan-a scope.** The comment and code came from baseline `e1293ef` (`git blame`), `main..HEAD -- src/vec3.rs` is empty, and A1 (`done/A1-diffuse-and-metal-materials.md`) and A1r3 (`done/A1r3-shared-scene-settlement.md:44-45`) both assign removal to the geometry plan/B2. Correction to Area 4: `66097bd` fixed *this exact line* to "For the triangle normals." on `main`/`plan-b`, and `281dda8` fixed a different comment in `src/main.rs`; neither is an ancestor of HEAD. So it "survives" only because plan-a forked at `e180476`, and the release merge already resolves it.

### Claim-by-claim

Status: ✅ confirmed · ❌ rejected · ✏️ corrected.

#### Area 1 — `src/material.rs`

| # | Claim | Status | Evidence |
|---|---|---|---|
|1|`:102` is `normal * -(1.0 - perp.dot(perp)).abs().sqrt()`|✅|`src/material.rs:102`|
|2|`refract` only reached from `:89` when `cannot_refract` false|✅|`src/material.rs:83-89`|
|3|Radicand is provably non-negative, so `.abs()` only masks bugs|✏️|Math yes, FP no: different rounding of `perp.dot(perp)` vs `ratio*sin_theta`; `.abs()` is the canonical RTIOW form|
|4|It drifts from A2 `:18` ("No other helper, clamp, validation or random source")|❌|`done/A2-glass.md:15-19` names signature only; `.abs()` is none of those|
|5|It should be removed / fail loudly|❌|Guard legitimate at the Snell boundary; no realistic wrong output|
|6|`:121` contains `len_squared > 1e-12 && len_squared <= 1.0`|✅|`src/material.rs:121`|
|7|A1 `:18` prescribes the single `<= 1.0` test|❌|`done/A1-diffuse-and-metal-materials.md:17-19` states only "rejection sampling of a point in the unit ball"|
|8|`1e-12` is an unrequested deviation / speculative guard|✏️|Standard divide-by-zero guard, distribution-neutral; not a deviation|
|9|Scout's "defensible, not a defect" judgement|✅|Same conclusion as #8|
|10|Req 1: every object has a material; 50-bounce; black at limit|✅|`src/hittable.rs:15`, `src/main.rs:25,32,39,45`|
|11|Req 2 diffuse: `normal + random_unit_vector`, albedo|✅|`src/material.rs:29-30`|
|12|Req 3 metal: reflect about unit incoming, fuzz, into-surface absorbed, albedo|✅|`src/material.rs:49-55`; fuzz unclamped but field invariant/scene 0.3 (`src/main.rs:69`)|
|13|Req 4 glass: Snell, TIR, Schlick, white attenuation|✅|`src/material.rs:73-91`; `refract` 99-104, `reflect` 95-97, `schlick` 107-110 math correct; index 1.5 `src/main.rs:74`|
|14|Req 6: `scatter` takes `&mut Rng`, one `Rng`/`SEED`|✅|`src/material.rs:12`; `src/main.rs:24,85`|
|15|Simplicity: one trait, three impls, no duplicate path, all four helpers used|✅|`src/material.rs` entire; no `allow(dead_code)` in it|
|16|§0b: no records/hashes|✅|No persistence in the module|
|17|§3a: six tests, 3 A1 + 3 A2, each ≤3; normal-incidence test depends on first draw > 0.04|✅|`src/material.rs:133,153,173,187,207,229`; first draw from seed 7 = 0.8202 > 0.04|
|18|Docs accurate, index 1.5, std-only, no `unsafe`, single-threaded|✅|`Cargo.toml` empty deps; `grep unsafe` clean|
|19|`src/main.rs:126` hardcodes 16; req 5 config/CLI is the output plan's|✅|`src/main.rs:126`; `full-plan.md:6-7` out of scope|
|20|Out-of-scope: `Vec3::cross` dead code kept intentionally|✅|See contested #7|
|21|A1's doc note: `refract` recomputes `cos_theta` but A2 fixes the signature|✅|`done/A2-glass.md:15-17`; `src/material.rs:81,100`|

#### Area 2 — `src/hittable.rs`, `src/sphere.rs`

| # | Claim | Status | Evidence |
|---|---|---|---|
|1|F1: "distance along the ray" is stale; `t` is a parameter|✅|`src/hittable.rs:9-10`; `src/ray.rs:16-18`; `src/camera.rs:31-35`; `src/sphere.rs:29,37-40`|
|2|Camera direction is not normalised|✅|`src/camera.rs:19-21,31-35` (unit only at `u=v=0.5`)|
|3|Sphere solves the quadratic in the parameter|✅|`src/sphere.rs:29,37-40`|
|4|"Only the test at `src/sphere.rs:67` uses a unit direction"|✏️|Both sphere tests use unit directions: `:67` and `:82`|
|5|Wording "inherited unchanged from baseline `e1293ef`"|✏️|Phrase is baseline (`e1293ef:src/hittable.rs:4`), but the sentence was edited by this plan's `a1358ff` (`git diff e180476..46a5c02 -- src/hittable.rs`)|
|6|All four `HitRecord` fields are consumed; no unused field|✅|`src/hittable.rs:12-15,39`; `src/material.rs:29-30,50,54,91`; `src/main.rs:34`|
|7|`HittableList::hit` is one nearest-hit loop with `map_or`|✅|`src/hittable.rs:36-45`|
|8|One construction path per object; material cloned once per hit|✅|`src/sphere.rs:17-23,45-50,49`|
|9|Record shape matches A1 branch settlement; no face flag/u/v/derives|✅|`src/hittable.rs:11-16`; `done/A1r-shared-hit-record-settlement.md:19-23`|
|10|No material logic or randomness in these files; req 1 met via `Sphere`|✅|`src/sphere.rs:13,49`; `src/main.rs:34`|
|11|No dead code; imports used; `add` called four times; `Default` used|✅|`src/hittable.rs:3-7`; `src/sphere.rs:3-8`; `src/main.rs:55,56,61,66,71`|
|12|Two sphere tests, distinct purpose; repeated setup is not bloat; material field untested is a coverage gap|✅|`src/sphere.rs:60-84`; no assertion on `hit.material`|
|13|No records/hashes in the area|✅|Fields are live only|
|14|No duplicate material path between `Sphere` and `HittableList`|✅|`src/hittable.rs:36-45` forwards only|
|15|Out-of-scope: `cross` from baseline, geometry plan territory|✅|`git log --oneline -- src/vec3.rs` → only `e1293ef`|

#### Area 3 — `src/main.rs`, `src/rng.rs`

| # | Claim | Status | Evidence |
|---|---|---|---|
|1|`main.rs:142,147` compare whole 40×20 render vectors|✅|`src/main.rs:142,147`|
|2|That drifts from `repo-rules-domain.md:37` "never a whole image"|✅ literal / ✏️ not actionable|`repo-rules-domain.md:37`; 40×20 is small and costs 0.01 s/test; invariant is global|
|3|A3 contract mandates these exact tests|✅|`done/A3-samples-per-pixel.md:40-42`|
|4|F2: `u/v` formula and half of edge-pixel samples land outside `[0,1]`|✅ numerically|`src/main.rs:91-92`; values below|
|5|Column 199 range `[0.99749, 1.00754)`|✏️|Correct is `[198.5/199, 199.5/199) ≈ [0.99749, 1.00251)`|
|6|"The code's own claim 'centres every draw inside its own pixel' is not literally true"|❌|Draws are centred in their own cell; the *cells* overhang `[0,1]` (see contested #4)|
|7|Pre-existing `width − 1` mapping, frozen by A3|✅|`done/A3-samples-per-pixel.md:11-14`|
|8|`rng.rs:9` `seed.max(1)` silent fallback, not triggered; file unchanged from base|✅|`src/rng.rs:9`; `git log --oneline -- src/rng.rs` → only `e1293ef`|
|9|Bounce loop: `MAX_BOUNCES=50`, attenuation, absorb black, limit black|✅|`src/main.rs:25,32,36,39,45`|
|10|Gamma 2 once in `to_byte`, only at output|✅|`src/main.rs:102-104,113-120`|
|11|Sample loop row→column→sample→bounce, linear sum, one divide, default 16|✅|`src/main.rs:87-96,126`|
|12|Two jitter draws per sample before the trace|✅|`src/main.rs:91-93`|
|13|One generator per render, `SEED = 7`, threaded `&mut`|✅|`src/main.rs:24,85,93`|
|14|No other randomness source (crate/API)|✅|Grep of `src/` finds only the passed `Rng`; the literal word "random" appears only in doc prose|
|15|Scene: red-brown diffuse centre, grey diffuse ground, metal fuzz 0.3, glass 1.5|✅|`src/main.rs:54-77`|
|16|Determinism from one generator, fixed order, single-threaded|✅|`src/main.rs:85-96`; `the_same_settings_render_the_same_pixels` passes|
|17|Test budget: 2 new + 1 adapted|✅|`src/main.rs:135,141,146`; `done/A3-samples-per-pixel.md:40`|
|18|§0b: no records/receipts in the area|✅|None|
|19|No dead code in `main.rs`/`rng.rs`|✅|All functions reachable from `main`/tests|
|20|Out-of-scope: `cross` reported with evidence in hand|✅|`src/vec3.rs:21-29`|

#### Area 4 — tests and docs

| # | Claim | Status | Evidence |
|---|---|---|---|
|1|`src/vec3.rs:21` names "the geometry plan", above `#[allow(dead_code)] cross`, no caller, from `e1293ef`|✅|`src/vec3.rs:21-29`; `git blame` → `e1293ef`; `grep` no caller|
|2|REPO-RULES §4 item 13 forbids history/plan ids in permanent files|✅|`AGENT-INSTRUCTIONS/BUILDING/REPO-RULES.md:170`|
|3|"Other plan references removed in `281dda8` and `66097bd`; this one survives"|✏️|`66097bd` fixed this exact line on `main`/`plan-b`; `281dda8` fixed `src/main.rs`'s `mod rng` comment; neither is an ancestor of HEAD (`git merge-base --is-ancestor` → no)|
|4|Exactly 12 test functions; no `tests/` dir|✅|`src/main.rs:135,141,146`; `src/rng.rs:30`; `src/sphere.rs:61,76`; `src/material.rs:134,154,174,188,208,230`; `ls tests` absent|
|5|Coverage maps to hit geometry, material rules, small renders|✅|Same lines; private helpers untested directly|
|6|Assertions purposeful (full reflected vector, Snell-derived vector, critical-angle reflection)|✅|`src/material.rs:168-170,222-226,244-246`|
|7|User decisions: 1.5, 16, `- 0.5`, gamma once, single-threaded, std-only, no `unsafe`|✅|`src/main.rs:74,126,91-92,102-104`; `Cargo.toml`; grep clean|
|8|Requirements 1–6 satisfied|✅ core / ✏️ item 5 user-config|`src/main.rs:25,32,45`; `src/material.rs:27-31,47-56,71-93`; `src/main.rs:82-99,102-104,24,85`; no CLI on this branch (`full-plan.md:6-7`)|
|9|README accurate; doc/README diff empty|✅|`README.md:1-10`; `git diff --stat e180476..46a5c02 -- docs README.md` empty|
|10|Items 7–13 absent is consistent with a finished-program spec|✅|`README.md:10`; `docs/requirements.md:21-41`; geometry/output plan scope|
|11|No plan ids elsewhere in `src/`, `docs/`, `README.md`|✅|Grep for plan/phase/merge/release/branch/future → only `src/vec3.rs:21`|
|12|White attenuation asserted in all three glass tests though returned unconditionally|✅ fact, minor|`src/material.rs:201,221,243,91`|
|13|`the_top_row_is_sky` renders 200×100×16 = 320,000 samples to assert one pixel|✅|`src/main.rs:136-137`; measured 0.32 s debug|
|14|Contract freezes this call|✅|`done/A3-samples-per-pixel.md:21-22`; `done/A1r2-shared-render-settlement.md` merge obligation|
|15|`cross` is product dead code reported here|✅|`src/vec3.rs:22-29`|
|16|No remote hosts/workers/runtime config apply|✅|`AGENT-INSTRUCTIONS/LOCAL/ops-scout.md`|
|17|Unknown: "12 passed at `46a5c02`" not independently run|✅ resolved|I ran `cargo test`: 12 passed, 0 failed|
|18|Unknown: top-row test runtime not measured|✅ resolved|0.32 s debug (suite total 0.32 s); other render tests 0.01 s each|

### Coverage

`git diff --stat e180476..46a5c02` changes exactly four files: `src/hittable.rs`, `src/main.rs`, `src/material.rs`, `src/sphere.rs`. All are covered: material by Area 1; hittable and sphere by Area 2; main by Area 3 (and its tests by Area 4). `src/rng.rs` is unchanged but covered by Area 3 as required. Requirements 1–6 are each covered by at least one area (1: Areas 1/3/4; 2: 1/4; 3: 1/4; 4: 1/3/4; 5: 3/4; 6: 1/3/4). `src/vec3.rs`, `README.md` and `docs/` are unchanged and are covered by Area 4 as out-of-scope/doc checks.

### Claims the areas missed

- **The top-row test does not test the row its name claims.** It asserts only `pixels[0]` (`src/main.rs:137`), the top-left pixel, not "the top row"; the name overstates a one-pixel assertion, and it does so at production size. Evidence `src/main.rs:135-138`.
- **Requirement 5's "configurable number of samples" is not user-facing on this branch.** `render` takes `samples` and `main` passes the literal 16 (`src/main.rs:126`); there is no CLI/`--samples` (out of scope per `full-plan.md:6-7`). Area 4's blanket "items 1–6 are all satisfied" should be limited to the render core.
- **The `t_min = 0.001` self-intersection epsilon (`src/main.rs:33`) is also a ray parameter, not a distance.** With a non-unit camera direction (`src/camera.rs:31-35`), its physical scale varies across the image. This is the same parameter-vs-distance issue as the stale `HitRecord` doc and strengthens Area 2's F1; nobody cited it.
- **The `HitRecord` doc sentence was touched by this plan, not purely inherited.** A1's commit `a1358ff` rewrote the comment (`git diff e180476..46a5c02 -- src/hittable.rs`) but kept "distance along the ray", so Area 2's "inherited unchanged" understates plan-a's contact with that line.
- **Lambertian can in principle produce a zero direction when `random_unit_vector ≈ −normal` (`src/material.rs:29`), and `sky_colour` then calls `.unit()` (`src/main.rs:50`), giving NaN → black.** Probability is effectively zero and no realistic input hits it, so this is explicitly *not* a finding under the hobby-simplicity rule; noted only because the areas did not check it.
- **`the_top_row_is_sky` is the only test whose runtime matters:** it accounts for essentially the whole 0.32 s suite, while the two 40×20 tests are ~0.01 s each. That quantifies Area 4's size concern but also shows the cost is small in absolute terms.
