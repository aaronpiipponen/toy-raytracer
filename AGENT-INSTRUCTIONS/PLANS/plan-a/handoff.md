# Orchestrator handoff

## Next action

WAITING ON PARTNER. When plan-b signals its B2 resolution phase file is ready, run the acceptance
check on BOTH resolution files (`plan-a/phases/A2-glass.md` and plan-b's B2 resolution file), call
`channel_close` naming the partner's file, then start A2's implementer with `phase_subagent` (`A2`).

## Current state

A1 done (head `de1cf31`). A2/B2 settled in `PLANS/shared/plan-a-A2+plan-b-B2.md`; A2's resolution
phase `phases/A2-glass.md` passed the orchestrator acceptance check (math and fixtures hand-verified;
3 new tests, at the cap). A3 is `next` (detailed and accepted, queued behind A2). plan-b's B2
resolution file is not yet written. `docs-pass.md` holds one item for the release.

## Learned rules

- A coordinator hold (`COORDINATOR HOLD`) stops every phase start, including unrelated ones. Make the
  held phase's `phase_subagent` call once so the stop is recorded, then start nothing until the
  coordinator or partner messages.
- As pair leader, use `channel_send` to agree implementation detail only; a product-behavior choice
  (such as the scene triangle's material) is the user's and goes in the shared design as an open
  decision, not a settled one.
- Keep exactly one `active` row. When a resolution makes an earlier phase active, move the previously
  active queued phase to `next`.

## Learned rules
