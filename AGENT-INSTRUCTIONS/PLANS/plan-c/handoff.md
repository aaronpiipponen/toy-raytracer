# Orchestrator handoff

## Next action

Spawn a fresh `phase-implementer` for phase C2 via `phase_subagent`.

## Current state

C1 is done (`done/C1-png-writer.md`). C2 is detailed in `phases/C2-command-line.md`, the README
usage correction is in, and the whole document passed the orchestrator acceptance check. C2 adds the
hand parser, the defaults, PNG/PPM dispatch, `render(width, height, samples)` averaging without
jitter, three tests, two adapted tests, the README usage block and the two carry-over comment
deletions.

## Learned rules

- `cargo` may not be on `PATH`; use `~/.cargo/bin/cargo` (gates.md).
- Workspace detail must respect `AGENT-INSTRUCTIONS/LOCAL/documentation-pages.md`: a command-line
  change updates the usage in `README.md`.
- Known cross-plan seam: C2 owns `render(width, height, samples)` averaging; the materials plan adds
  jitter to the same function, so the release/coordinator reconciles it at merge.
- When the advancer promotes and details the next phase in REVIEW AND ADVANCE, still run the full
  acceptance check on the new phase document before any implementer starts.
