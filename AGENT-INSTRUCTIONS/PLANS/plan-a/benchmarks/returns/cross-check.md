## Answer

Every area's factual claims check out against the tree and the contracts, with four exceptions: area 1's probability figure for the `1e-12` guard is wrong by ~500× and its "unnecessary" verdict is contestable; area 4's "`#[allow(dead_code)]` remains only on `HitRecord.u`/`.v`" ignores `src/vec3.rs:22`; area 2's statement that A2 "explicitly flagged" A1's face-side lines as carry-over is false (A2 never mentions A1); and area 1's "each of the five tests asserts a requirement-level behaviour" is softened by area 4's own finding that the diffuse test proves only a sign. **No real product defect, data loss, hack, or plan-a-owned plan reference in a permanent file is hidden among the observations.** The only genuine product-code smell is `Rng::new`'s silent `seed.max(1)`; the only genuine document contradiction is the stale `done/A3r` lines (and the same class of staleness in `done/A1:31-32` that area 2 rationalised with a false justification).

## Evidence, per claim

**Area 1 — materials**

- Duplicated `(-v.dot(n)).min(1.0)` at `src/material.rs:79` and `:99`, grazing sine at `:80` vs `1 - perp·perp` at `:101`; "frozen by A2, not a defect" — **CONFIRMED with nuance.** `done/A2-glass.md:22-24` freezes `refract(v, n, eta_ratio)` as self-contained (`min(1.0)`/`max(0.0)` the only guards); it freezes the signature/behaviour, not the literal words "duplicate the cosine", so the judgement is sound.
- `src/material.rs:121` `1e-12` epsilon "unnecessary", cube-volume probability "~1e-12" — **REJECTED as stated.** `material.rs:121` exists, but P(len²≤1e-12) = (4/3·π·(1e-6)³)/8 ≈ **5.2e-10**, not 1e-12. It is not simply removable: `next_f64` can return exactly 0.5 (`u64>>11 == 2^52`), so `p` can be exactly zero and `p.unit()` would be `0/0 = NaN`; the guard prevents a measure-zero NaN. It is an undocumented defensive constant, not an unnecessary branch and not a defect.
- Trait shape `src/material.rs:11`; Lambertian `:28-29`; Metal `:48-53`; Dielectric `:74-87`; helpers `:92,98,106,113` all called; `_ray_in` `:27` — **CONFIRMED.** `grep` shows each helper used once for its job; `schlick` `r0` ratio-symmetric (`state 1/1.5` and `1.5` both give 0.04).
- "five tests, each asserting a requirement-level behaviour" — **CONFIRMED as names/existence** (`material.rs:137,157,178,201,223`) but **overstated for the diffuse test**: it asserts only `dot > 0` (`:147-154`), which area 4 correctly calls unproven randomness.
- Fuzzy-absorption loop deterministic under seed 7 — **CONFIRMED.** Simulating the xorshift64* stream from seed 7, the very first scattered draw is absorbed, so the loop breaks on iteration 1.
- `src/main.rs:1` PPM/stdout, no arg parsing (`:124-127`), defaults 200×100 (`:22-23`) vs requirement 12 — **CONFIRMED** (`docs/requirements.md:37-38` asks 400×200 plus flags).

**Area 2 — hit record and sphere**

- One `HitRecord`, not `Copy`, pub fields; constructor at `src/hittable.rs:26-49` is the only path — **CONFIRMED** (`grep 'HitRecord {'` → only the struct def and `new`; `material.rs:133` is a fixture calling `new`).
- Every field `pub`/mutable, so "only flip" is convention-only; no live violation — **CONFIRMED.** `hittable.rs:11-21`; `grep '\.normal *='` finds no assignment.
- `HittableList` exposes public `objects` (`:60`) and `add()` (`:64-66`), only `add()` used — **CONFIRMED.** `grep` finds `objects` only inside `hittable.rs`; `main.rs` uses `world.add(...)`.
- `src/sphere.rs:73-77,91-95,109-113` identical setup three times — **CONFIRMED.**
- "A1's frozen lines 31-32 … superseded by A2 and **explicitly flagged there as carry-over**" — **REJECTED.** `done/A1:31-32` does say the record "carries no face-side flag" (now false), but `done/A2` never mentions A1 or a carry-over of that line (`grep -n "A1\|supersed\|carry" done/A2-glass.md` → only the grain carry-over at :33). The supersession is implicit; A1's sentence is a second stale frozen record, same class as A3r.
- Three focused sphere tests, no bloat — **CONFIRMED** (`sphere.rs:71,89,107`).

**Area 3 — render loop, generator, scene**

- Seed `7` hardcoded in `material.rs:146,166,187,211,233` and `rng.rs:31-32`, not `SEED` (`main.rs:25`) — **CONFIRMED as fact.** The implied remedy is contestable: a unit fixture legitimately pins its own seed rather than tracking a product constant, so this is style, not drift — and it cannot silently break anything, since a seed change only alters the fixture's input draws, and the metal-TIR/diffuse assertions are seed-robust (sign/absorption).
- `src/rng.rs:9` `seed.max(1)` collapses `Rng::new(0)` onto `Rng::new(1)`; no product caller passes 0 — **CONFIRMED.** `SEED = 7` is the only `Rng::new` outside tests (`main.rs:85`).
- `main.rs:80-81` progress-line sentence absent on branch, sanctioned by A3 — **CONFIRMED.** `done/A3-samples-per-pixel.md:18-21` freezes exactly that sentence and says it describes the merged product; `shared/plan-a-A3+plan-c-Z1.md:38-42,60` owns the edit.
- Bounce loop `MAX_BOUNCES`/black (`:26,33,46`, absorption `:40`); `sky_colour` `:50` used once at `:43`; gamma `to_byte` `:101-103` on all channels `:116-118` — **CONFIRMED.**
- One `Rng::new(SEED)` at `:85`, `u` before `v` at `:91-92`, `pixel / samples` at `:95`, row loop reversed `:87` — **CONFIRMED**; matches `done/A3:6-11`.
- `SAMPLES` at `:24` used only by `main` `:125`; no test names `WIDTH/HEIGHT/SAMPLES` (tests use literals 200,100,16 and 20,10) — **CONFIRMED.**
- Four spheres `:57-76`; `grep` finds no `SystemTime|Instant|getrandom|urandom|std::env|thread_rng|rand::`; `Cargo.lock` holds one package with no dependency entries — **CONFIRMED.** ("dependency count = 1" really means one package, zero dependencies.)
- `mod rng`'s `#[allow(dead_code)]` and the "plans add" comment were removed in `e180476..566b11c` — **CONFIRMED** (`git show e180476:src/main.rs` lines 6-8 vs head).
- Test budget "2 added, 0 deleted, 1 adapted" for A3 — **CONFIRMED.** `a1d640a` had 1 `main.rs` test, head has 3 (`:133,139,144`), and A3 touched only `main.rs` (`git diff --stat a1d640a..566b11c` → `src/main.rs` alone).
- "runtime 0.35 s" — **UNVERIFIED** (read-only; tests not run).

**Area 4 — tests and documents**

- `done/A3r-samples-resolution.md:8` "sits at `planned`" and `:42-43` "A3 remains `planned`" are now false — **CONFIRMED.** `full-plan.md:20` reads A3 `done`; `done/A3-samples-per-pixel.md` exists; `phases/` holds only `Z1-closing-survey.md`.
- `shared/plan-a-A3+plan-c-Z1.md:22,28` use `colour`; `done/A3:11` and `src/main.rs:89,95` use `pixel` — **CONFIRMED.** Shared uses `let mut colour`/`colour / samples as f64`; code and contract say `pixel`.
- `main.rs:141` `assert_eq!` and `:146` `assert_ne!` compare whole 20×10 vectors, against "Rendering tests check a few pixels of a small image, never a whole image" (`LOCAL/repo-rules-domain.md:36`); the sample-count test proves the argument is read, not that averaging happens — **CONFIRMED.** `assert_ne!` passes for any code that merely consumes `samples`; the frozen A3 tests are `done/A3:22-25`.
- `material.rs:147-154` diffuse test proves only `dot > 0`, a constant direction would pass — **CONFIRMED.**
- `sphere.rs:84-86` and `:101-104` pack `(u,v)`/face assertions; `material.rs:209-210` and `:231-232` re-assert `front_face`/normal — **CONFIRMED as lines; "already proven at the hit layer" is a fair description, though those two material assertions also serve as preconditions (inside-glass / entering) for the scenario, so they are not purely redundant.** The dropped plan-b test is at `shared/plan-a-A3+plan-b-Z1.md:99-101` (not exactly 100-101 for the whole statement, but that is the paragraph).
- Fuzz/Schlick coverage gap — **CONFIRMED.** `material.rs:164` uses fuzz 0.0 and `:185` fuzz 1.0; no non-zero fuzz that stays mirrored. `:82`'s Schlick-reflect arm is never taken: the TIR test short-circuits on `eta_ratio·sin_theta > 1.0`, and the Snell test's first draw 0.8202 exceeds `schlick(0.8, 1/1.5) ≈ 0.0403`, so it refracts. A1 added 3 tests at `de1cf31` (material 0→3) and A2 added 3 (`a1d640a`: material 3→5, sphere 2→3 vs `de1cf31`), so both phases did use the 3-function cap.
- "`#[allow(dead_code)]` remains only on `HitRecord.u`/`.v` (`hittable.rs:17-20`)" — **REJECTED literally.** There are three: `hittable.rs:17`, `hittable.rs:19`, and `vec3.rs:22`. The `vec3` one is base code outside the area, but the sentence as written is false.
- `docs/requirements.md:8-19` matches the branch — **CONFIRMED for items 1-4, 6 and the 16 default**; item 5's "configurable number of samples" is not true on this branch (it is plan-c's at merge). Minor overreach.
- `README.md:6` `cargo run > image.ppm` accurate here — **CONFIRMED** (`README.md:6`; `write_ppm` to stdout).
- A1-era "the plans add" comments gone — **CONFIRMED** (`e180476` `main.rs:6` and `hittable.rs:10`; absent at head).
- `src/vec3.rs:21` names "the geometry plan"; integration tree drops it — **CONFIRMED.** Present since `e1293ef`, untouched by plan-a; `diff main/src/vec3.rs plan-a/src/vec3.rs` shows exactly those two added lines (`:21` comment, `:22` allow).
- `README.md` post-merge default gap; `docs-pass.md` does not list it — **CONFIRMED.** `main/README.md:6` is already `cargo run --release -- --output image.png`, and the merge takes plan-c's README (`done/A3r:25`), so this is a release-merge note, not a branch defect.
- `rng.rs:29-36` overlaps the render determinism test and predates the range (`git diff e180476..566b11c -- src/rng.rs` empty) — **CONFIRMED.**
- `main.rs:1` header accurate now, replaced at merge per `shared:84-85` — **CONFIRMED.**

**Plan and repository documents (asserted across areas)**

- `LOCAL/project.md:5` lists `aabb.rs`, `plane.rs`, `triangle.rs`, `material.rs`; those four are absent/present differently per branch — **CONFIRMED as text, but the interpretation "stale" is wrong for the merged product.** `main/src/` contains `aabb.rs`, `plane.rs`, `triangle.rs` and no `material.rs`; `plan-a/src/` contains `material.rs` and none of the three. The map is a union of the two trees, and `docs-pass.md:15-19` already owns re-checking the `material.rs` line after merge. It is a branch-mismatch, not a false permanent statement, and it is already a known `docs-pass` item.
- `gates.md:39` fallback sentence still true here — **CONFIRMED** (branch has no command line).

## Is a real defect hidden?

No broken behaviour, data loss or hack. Ranked, the only substantive items are:

1. **`src/rng.rs:9` `seed.max(1)`** — a genuine silent-substitution smell that contradicts the "fail loudly, no silent fallback" rule; unreachable from product code (`SEED = 7`), zero behavioural impact. This is the closest thing to a real product-code defect.
2. **`done/A3r-samples-resolution.md:8,42-43`** — a "done"/frozen plan record that now contradicts the status table and the file's own location. Plan-folder record, not a product doc.
3. **`done/A1-diffuse-and-metal-materials.md:31-32`** — also literally false at head ("no face-side flag", "outward normal"); area 2 waved this away with a non-existent A2 flag. Same stale-record class, missed by the returns.
4. `shared …Z1:22,28` vs `done/A3:11` variable-name split — will force the merge to contradict one document's spelling; naming only.
5. Test-coverage items (diffuse randomness, non-zero fuzz, Schlick arm, whole-image assertions) — real gaps, but capped phases and user-review items, not defects.

No plan-a-owned plan reference exists in a permanent product file: the only one, `src/vec3.rs:21`, is base-commit code the integration tree already dropped.

## Unknowns

- `cargo build/test/clippy/fmt` and the 0.35 s runtime claim were not run (read-only), so requirement 14 and area 3's timing remain unverified.
- I did not inspect `plan-b`/`plan-c` branches beyond confirming `main/src/vec3.rs`, `main/README.md` and `the_center_pixel` test locations, so post-merge claims about their files rest on the shared designs and `main`'s tree, not those branches' heads.
