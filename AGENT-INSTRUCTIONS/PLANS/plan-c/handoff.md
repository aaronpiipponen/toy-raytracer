# Orchestrator handoff

## Next action

Spawn a fresh `phase-implementer` for phase C3 via `phase_subagent`. A hold may return when the
coordinator re-investigates against plan-a's landed A1r; if the start is refused again, follow the
refusal.

## Current state

C1 and C2 are done (`done/C1-png-writer.md`, `done/C2-command-line.md`). C3 is detailed in
`phases/C3-progress-line.md` and passed the orchestrator acceptance check (standard gate, no new
tests, progress line on standard error, README sentence, no image change). The earlier hold against
plan-a A1r was released because A1r's implementer had already started; plan-a now leads that
investigation. The release item list is `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md`.

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
