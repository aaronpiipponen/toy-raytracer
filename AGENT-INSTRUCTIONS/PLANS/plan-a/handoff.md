# Orchestrator handoff

## Next action

Spawn one fresh `phase-advancer` with DETAIL RESOLUTION for both plans (write plan-a's A2 resolution
and plan-b's B2 resolution from the shared design). Then accept both files, `channel_close` naming
plan-b's B2 file, and start A2's implementer with `phase_subagent` (`A2`).

## Current state

A1 done (head `de1cf31`). Pair `plan-a-A2+plan-b-B2` settled in
`PLANS/shared/plan-a-A2+plan-b-B2.md`; plan-a's A2 resolution `phases/A2-glass.md` exists and passed
an earlier acceptance check. Per the user, one advancer now writes both plans' resolution files.
A3 is `next` (detailed and accepted, queued behind A2). `docs-pass.md` holds one item for the release.

## Learned rules

- A coordinator hold (`COORDINATOR HOLD`) stops every phase start, including unrelated ones. Make the
  held phase's `phase_subagent` call once so the stop is recorded, then start nothing until the
  coordinator or partner messages.
- As pair leader, use `channel_send` to agree implementation detail only; a product-behavior choice
  (such as the scene triangle's material) is the user's and goes in the shared design as an open
  decision, not a settled one.
- Keep exactly one `active` row. When a resolution makes an earlier phase active, move the previously
  active queued phase to `next`.
- When the user directs one advancer to write both plans' resolution files, it may write plan-b's
  phase document (not its code) per the shared design.

## Learned rules
