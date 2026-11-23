# Orchestrator handoff

## Next action

Run the Z1 closing survey: dispatch four read-only `ops-scout` area scouts in parallel, then one
cross-check scout, then write `benchmarks/survey-2026-11-23.md` and report the user-review list.

## Current state

C1, C2 and C3 are done. Z1 (closing survey and user acceptance) is detailed in
`phases/Z1-closing-survey.md` and passed the orchestrator acceptance check; I execute it myself. The
code head is `525196b`. Findings are reported to the user un-fixed. The release item list is
`AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md`.

## Learned rules

- `cargo` may not be on `PATH`; use `~/.cargo/bin/cargo` (gates.md).
- Workspace detail must respect `AGENT-INSTRUCTIONS/LOCAL/documentation-pages.md`: a command-line
  change updates the usage in `README.md`.
- Known cross-plan seam: C2 owns `render(width, height, samples)` averaging; the materials plan adds
  jitter to the same function, so the release/coordinator reconciles it at merge.
- When the advancer promotes and details the next phase in REVIEW AND ADVANCE, still run the full
  acceptance check on the new phase document before any implementer starts.
- A coordinator hold: make the held phase's `phase_subagent` call so the refusal is recorded, start
  nothing else, tell the user one line, end the turn.
