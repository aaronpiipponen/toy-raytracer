## Phase B2r - Shared hit record resolution (done)

**Status:** done.
**Gate level:** minimal.

**Frozen contract:**
- `HittableList::hit` in `src/hittable.rs` reads the nearest distance through
  `nearest.as_ref().map_or(t_max, |hit| hit.t)`, so the loop borrows the nearest record and never
  moves it out of `Option<HitRecord>`. The loop compiles with or without `Copy` on the record.
- The loop keeps the nearest record and returns the same `Option<HitRecord>` for the same input: it
  tests every object in list order with `t_min` and the current limit, and `nearest` starts as
  `None`, so an untouched list returns `None`.
- `HitRecord`'s derive list, its six fields and the six-parameter `HitRecord::new` are unchanged on
  this branch. The material field, the `Clone`-only derive and the seventh constructor parameter
  belong to the materials plan; the shared design `PLANS/shared/plan-a-A1+plan-b-B2.md` owns the
  merged shape.
- No other line of any file changed.

**Evidence:** `cargo build` with no warnings, `cargo test` (6 passed), `cargo clippy -- -D warnings`,
`cargo fmt --check` and `git diff --check` passed at `5204447`; `cargo run` produced a byte-identical
PPM before and after the edit.
