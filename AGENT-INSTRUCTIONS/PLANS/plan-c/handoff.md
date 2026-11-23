# Orchestrator handoff

## Next action

Make the `phase_subagent` call for C3 even though the coordinator hold is armed. When it refuses
with `COORDINATOR HOLD`, tell the user in one line and end the turn. Wait for the coordinator or the
partner orchestrator to resume this session.

## Current state

C1 and C2 are done (`done/C1-png-writer.md`, `done/C2-command-line.md`). C3 is detailed in
`phases/C3-progress-line.md` and passed the orchestrator acceptance check (standard gate, no new
tests, progress line on standard error, README sentence, no image change). A coordinator hold pairs
plan-c C3 with plan-a A1r; I lead the settlement once the channel opens. The release item list is
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
