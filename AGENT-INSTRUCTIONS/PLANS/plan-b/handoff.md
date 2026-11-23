# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in REVIEW AND ADVANCE mode for B3 with the receipt below. It should
complete B3 and produce DETAIL READY for Z1 (closing survey and user acceptance). When Z1 becomes
active, the orchestrator (not the advancer) dispatches the read-only `ops-scout` areas and writes
the dated survey note; the Z1 phase document only defines the areas and the acceptance step.

## Current state

- B1: `done` at `6eb43e09`. B2r: `done` at `5204447`. B2: `done` at `a5c6665`.
- B3: implemented at `2b4e2b7`, pending review/advance. Receipt: base `a5c6665`, head `2b4e2b7`;
  `cargo test aabb` 1 passed, `cargo test hittable` 1 passed, `cargo test` 11 passed, build, clippy
  `-D warnings`, fmt and diff check pass; before/after renders byte-identical with header
  `P3 / 200 100 / 255` and line 7441 `127 255`-family unchanged (`127 127 255`); test budget 2
  added / 0 deleted; review loop 1 round, no finding. User test: not required.
- Z1: `planned`, last phase: closing survey plus user acceptance. It is `minimal` gate; its survey
  is dispatched by the orchestrator, and its survey note is written by the orchestrator from the
  saved scout returns. The plan is not closed or archived by this session.
- No `/workflow-reload` is needed for the `AGENT-INSTRUCTIONS/LOCAL/project.md` edits: agents read
  that file at session start and the plugin commits the workspace root after each round.
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
- The closing survey is orchestrator-dispatched, never inside the implementer or advancer workflow:
  one read-only `ops-scout` per area, then one cross-check `ops-scout`, then the orchestrator writes
  the dated note. It never fixes findings inside that phase.
