# Orchestrator handoff

## Next action

Run `phase-advancer` in REVIEW AND ADVANCE for phase A2 (base `de1cf31`, tested head `a1d640a`).
Then reach A3's implementer start and make the `phase_subagent` (`A3`) call so the armed
`plan-a-A3+plan-c-Z1` hold records the stop; plan-c leads that settlement.

## Current state

A1 done (`de1cf31`). A2 is implemented and verified at head `a1d640a` (10 tests, review clean) and
awaits review-and-advance. Pair `plan-a-A2+plan-b-B2` closed. A3 is `next` but held by pair
`plan-a-A3+plan-c-Z1` (plan-c leads; A3's sample loop overlaps plan-c C3's `render` and progress
write). `docs-pass.md` holds one item for the release.

## Learned rules

- A coordinator hold (`COORDINATOR HOLD`) stops every phase start, including unrelated ones. Make the
  held phase's `phase_subagent` call once so the stop is recorded, then start nothing until the
  coordinator or partner messages.
- A hold stops only the held phase's implementer; advancers and the review-and-advance of a different,
  unheld phase may still run before reaching the held phase's start.
- As pair leader, use `channel_send` to agree implementation detail only; a product-behavior choice
  (such as the scene triangle's material) is the user's and goes in the shared design as an open
  decision, not a settled one. As partner, answer channel messages from known plan facts and start
  nothing until `RESOLVED`.
- Keep exactly one `active` row. When a resolution makes an earlier phase active, move the previously
  active queued phase to `next`.
- When the user directs one advancer to write both plans' resolution files, it may write plan-b's
  phase document (not its code) per the shared design.
- A coordinator hold lifts only on `channel_close`; close the pair after accepting both resolution
  phase files, before starting the resolution implementer.

## Learned rules
