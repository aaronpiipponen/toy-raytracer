# Orchestrator handoff

## Next action

Spawn `phase-implementer` with `phase_subagent` for the active phase B2.

## Current state

- B1: `done` at `6eb43e09`.
- B2r: `done` at `5204447`; contract `done/B2r-shared-hit-record-resolution.md`.
- B2: `active`, detailed and orchestrator-accepted at `phases/B2-planes-and-triangles.md`; not
  implemented. Three new test functions; rendered-image gate compares before/after and probes one
  triangle pixel.
- B3, Z1: `planned`.
- Out-of-scope observation: `AGENT-INSTRUCTIONS/LOCAL/project.md:5` already names `png.rs` and a
  command line, which do not exist in this worktree; that is another plan's forward edit in the
  shared workspace root, not ours to fix. B2's step 7 preserves every existing entry.
- The coordinator pair `plan-a-A1+plan-b-B2` is RESOLVED and the channel is closed. Shared design:
  `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-b-B2.md`, including the seven merge hunks. plan-a's
  A1r passed the same check; plan-a resumes its own loop.
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
