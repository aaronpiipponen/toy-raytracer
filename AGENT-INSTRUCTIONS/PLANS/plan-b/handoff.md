# Orchestrator handoff

## Next action

Spawn `phase-implementer` with `phase_subagent` for the active phase B3.

## Current state

- B1: `done` at `6eb43e09`.
- B2r: `done` at `5204447`.
- B2: implemented at `a5c6665`, pending review/advance. Receipt: base `5204447`, head `a5c6665`;
  `cargo test plane` 1 passed, `cargo test triangle` 2 passed, `cargo test` 9 passed, build, clippy
  `-D warnings`, fmt and diff check all pass; rendered image differs from the pre-change PPM and
  line 7441 reads `127 127 255` (triangle) where it was `182 211 255` (sky); test budget 3 added /
  0 deleted; review loop 2 rounds (2 real findings fixed, then none). User test: not required.
- B3: `planned`, next to detail. Bounding boxes: an `Aabb` type, `Hittable::bounding_box` returning
  `Option<Aabb>`, `HittableList` testing a box before its object.
- Z1: `planned`.
- The `AGENT-INSTRUCTIONS/LOCAL/project.md` repository-map edit is in the shared workspace root, not
  in the project commit. No workflow reload is needed: agents read it at session start and the
  plugin commits the root after each round.
- Approved implementer deviations, no action needed: `src/triangle.rs` uses
  `!(0.0..=1.0).contains(&u)` because clippy rejects the literal range comparison; doc comments were
  added to the new public items and a semicolon removed from a doc comment.
- Autonomous mode on; standing authorization for phase commits exists. No open user decision.

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
