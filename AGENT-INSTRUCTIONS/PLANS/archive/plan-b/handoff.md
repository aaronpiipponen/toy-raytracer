# Orchestrator handoff

## Next action

None. The release merged the plan into `main` at `5b149b7` and archived the plan on 2026-11-23.

## Current state

- B1 `6eb43e09`, B2r `5204447`, B2 `a5c6665`, B3 `2b4e2b7`; Z1 done (survey note + user acceptance).
- Every `full-plan.md` row reads `done`; `done/` holds the five frozen contracts.
- The release documentation pass corrected four comments in `src/aabb.rs`, `src/hittable.rs` and
  `src/plane.rs` at `3f6ef46`: the closed box interval, the two function doc comments that opened
  with a noun phrase, and the ambiguous `HitRecord::new` wording. It changed no code.
- The merge conflicted only in `src/main.rs` (both branches' module lists) and `src/vec3.rs` (the
  `#[allow(dead_code)]` line), both covered by `PLANS/shared/plan-a-A1+plan-b-B2.md`. `src/hittable.rs`
  merged cleanly. No conflict needed a choice the shared design does not make.
- The merged `main` passes `cargo build`, `cargo test` (16 passed), `cargo clippy -- -D warnings`,
  `cargo fmt --check` and `git diff --check`. The rendered-image gate passes: the 80 by 40 PNG opens
  and shows the sky, ground, sphere and triangle, and the 200 by 100 PPM keeps the survey's pixel
  probes (line 4 `165 201 255`, line 7441 `127 127 255`, line 10104 `129 126 255`).
- Settled user decisions: accept the survey as it stands; do not add the F9/F10 coverage tests.
- No `docs-pass.md` exists because this plan deferred no page work to the release.
- Autonomous mode on; standing authorization for phase commits exists.

## Learned rules

- Detail the rendered-image gate as a before/after render comparison (`cmp`), never a remembered
  pixel value, when the claim is that the image did not change. A specific pixel probe is acceptable
  only when a diff cannot prove the new behaviour (for example, that a newly added object renders).
- A coordinator hold stops only the held phase's implementer start; a running advancer may still
  detail it, and unrelated phases stay blocked too once the hold is armed.
- Deleting comments that name a plan or a later stage (REPO-RULES §4.13) is legitimate work inside a
  phase that already touches the file; keep it to the comment lines only.
- A `Rc<dyn Trait>` field kills `Copy` and `Debug` on a derived struct; do not add `Debug` as a trait
  supertrait just to keep a derive when no reader prints the value.
- Commits never move between plan branches; parallel plans each implement their own side on their own
  branch and the release merges both into `main`, reconciling textual conflicts by the shared design.
- A phase file that still contains `<angle-bracket placeholders>` is a template, not a detailed
  phase; do not accept it or close a channel on it.
- The partner orchestrator cannot message the leader on its own; when it says it is waiting, re-read
  its phase file rather than waiting for a message.
- Clippy `-D warnings` rejects a literal `x < a || x > b`; a phase that spells it out will be
  implemented as `!(a..=b).contains(&x)`. Do not treat that equivalence as drift.
- The closing survey is orchestrator-dispatched, never inside the implementer or advancer workflow:
  one read-only `ops-scout` per area, then one cross-check `ops-scout`, then the orchestrator writes
  the dated note. It never fixes findings inside that phase.

## Learned rules

- Detail the rendered-image gate as a before/after render comparison (`cmp`), never a remembered
  pixel value, when the claim is that the image did not change. A specific pixel probe is acceptable
  only when a diff cannot prove the new behaviour (for example, that a newly added object renders).
- A coordinator hold stops only the held phase's implementer start; a running advancer may still
  detail it, and unrelated phases stay blocked too once the hold is armed.
- Deleting comments that name a plan or a later stage (REPO-RULES §4.13) is legitimate work inside a
  phase that already touches the file; keep it to the comment lines only.
- A `Rc<dyn Trait>` field kills `Copy` and `Debug` on a derived struct; do not add `Debug` as a trait
  supertrait just to keep a derive when no reader prints the value.
- Commits never move between plan branches; parallel plans each implement their own side on their own
  branch and the release merges both into `main`, reconciling textual conflicts by the shared design.
- A phase file that still contains `<angle-bracket placeholders>` is a template, not a detailed
  phase; do not accept it or close a channel on it.
- The partner orchestrator cannot message the leader on its own; when it says it is waiting, re-read
  its phase file rather than waiting for a message.
- Clippy `-D warnings` rejects a literal `x < a || x > b`; a phase that spells it out will be
  implemented as `!(a..=b).contains(&x)`. Do not treat that equivalence as drift.
- The closing survey is orchestrator-dispatched, never inside the implementer or advancer workflow:
  one read-only `ops-scout` per area, then one cross-check `ops-scout`, then the orchestrator writes
  the dated note. It never fixes findings inside that phase.
