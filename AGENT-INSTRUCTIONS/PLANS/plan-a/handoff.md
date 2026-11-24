# Orchestrator handoff

## Next action

Start R1's implementer with `phase_subagent` (`R1`). When it returns, take its receipt into REVIEW AND
ADVANCE for R1; then compress Z1 with the user's recorded acceptance and report the plan's outcome.

## Current state

A1 done (`de1cf31`), A2 done (`a1d640a`), A3 done (`566b11c`); A3r done. Z1's closing survey is written
(`benchmarks/closing-survey-2026-11-24.md` + five returns) and the user ACCEPTED the survey and image at
`566b11c`, choosing: fix the small cleanups and stale records now, accept the test-coverage gaps (no
extra tests). R1 is detailed at `phases/R1-survey-cleanups.md`, passed the orchestrator acceptance check,
and awaits implementation; its render-byte-identical hash is
`5457ef7a788539f5f5b0145e7c20fd178a77f0d184663b83fcd38d299a54665b`. Plan-level fact for the release:
plan-b's `HittableList.objects` is public there, so the merge keeps plan-a's private field with plan-b's
box skip. `docs-pass.md` holds four documentation items plus the answered triangle-material carry-over.

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
- The leader may ask the partner to write its own resolution record via `channel_send` and wait for a
  `channel_notify`; it need not write the partner's file.
- Keep exactly one `active` row. When a resolution makes an earlier phase active, move the previously
  active queued phase to `next`.
- When the user directs one advancer to write both plans' resolution files, it may write plan-b's
  phase document (not its code) per the shared design.
- A coordinator hold lifts only on `channel_close`; close the pair after accepting both resolution
  phase files, before starting the resolution implementer.
- A resolved partner may receive a resolution-bookkeeping phase (documentation-only) that amends the held
  phase document in place; run it, then implement the held phase.
- The branch's `render` doc sentence describes the merged product's progress line (frozen by the shared
  designs); it is deliberately not branch-accurate and must not be edited again at the merge.
- A survey phase runs no implementer: I dispatch the scouts myself, save their returns, write the note,
  and stop for the user's subjective image/survey acceptance.
- After the survey, the user's accepted cleanups run as a small remediation phase; the accepted test
  gaps add no test, and the survey acceptance stands because the fixes change no behaviour.

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
