# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` mode for phase C2 with the implementer's
receipt.

## Current state

C2 implementation is at tested head `928af8e` (base `30f857f`), tree clean, one review with no
finding, fast and rendered-image gates passed. Completion, compression to `done/` and promotion of
C3 belong to the advancer. The implementer surfaced a wording mismatch in
`AGENT-INSTRUCTIONS/LOCAL/gates.md` line 39 that the advancer should judge.

## Learned rules

- `cargo` may not be on `PATH`; use `~/.cargo/bin/cargo` (gates.md).
- Workspace detail must respect `AGENT-INSTRUCTIONS/LOCAL/documentation-pages.md`: a command-line
  change updates the usage in `README.md`.
- Known cross-plan seam: C2 owns `render(width, height, samples)` averaging; the materials plan adds
  jitter to the same function, so the release/coordinator reconciles it at merge.
- When the advancer promotes and details the next phase in REVIEW AND ADVANCE, still run the full
  acceptance check on the new phase document before any implementer starts.
