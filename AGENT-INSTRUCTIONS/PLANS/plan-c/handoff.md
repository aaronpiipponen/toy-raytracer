# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `DETAIL ACTIVE` mode to re-detail C2 with the README usage
correction.

## Current state

C1 is done (`done/C1-png-writer.md`). C2 is active and detailed in `phases/C2-command-line.md`, but
the document wrongly declares no documentation page changes and forbids touching `README.md`.
`AGENT-INSTRUCTIONS/LOCAL/documentation-pages.md` requires a command-line change to update the
usage in `README.md`, so C2 must include that edit. The rest of C2 passed the acceptance check.

## Learned rules

- `cargo` may not be on `PATH`; use `~/.cargo/bin/cargo` (gates.md).
- Rendered-image gate is the option form once C2 lands.
- Workspace detail must respect `AGENT-INSTRUCTIONS/LOCAL/documentation-pages.md`: a command-line
  change updates the usage in `README.md`.
- Known cross-plan seam: C2 owns `render(width, height, samples)` averaging; the materials plan adds
  jitter to the same function, so the release/coordinator reconciles it at merge.
