# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in REVIEW AND ADVANCE mode for phase B1, with the receipt below. On
its handoff, run the acceptance check on B2's detail before any implementer.

## Current state

B1 is implemented and pending review/advance; tested head `6eb43e09`. Receipt: base `e180476`,
head `6eb43e09`; `cargo build`, `cargo test` (6 passed), `cargo clippy -- -D warnings`,
`cargo fmt --check`, `git diff --check` all pass; rendered-image gate passed as a byte-identical
before/after `cmp`; test budget 1 added / 0 deleted. Implementer review loop: 1 round, no real
finding. User test: not required. No `USER DECISION REQUIRED`, no blocker. Next phases: B2 (planes
and triangles), B3 (bounding boxes), Z1 (closing survey and user acceptance). `scope.md` is final.
Autonomous mode is on; standing authorization for phase commits exists. The plan is linked to the
coordinator.

**Coordinator hold:** `HOLD ARMED (pair plan-a-A1+plan-b-B2)` - phase B2 overlaps plan-a's A1 and its
implementer start will be refused. B2 detailing may proceed; the implementer start must not be
retried or routed around. Wait for a coordinator `CHANNEL OPEN` / resolution message.

## Learned rules

- Detail the rendered-image gate as a before/after render comparison (`cmp` of two PPM files), never
  as a hardcoded expected pixel value or line number.
- A coordinator hold stops only the held phase's implementer start; a running advancer may still
  detail it, and unrelated phases stay blocked too once the hold is armed.
