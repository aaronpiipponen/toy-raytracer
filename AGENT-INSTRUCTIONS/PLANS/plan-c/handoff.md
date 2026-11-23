# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `REVIEW AND ADVANCE` mode for phase C3 with the implementer's
receipt.

## Current state

C3 implementation is at tested head `525196b` (base `928af8e`), review returned no problem, fast and
rendered-image gates passed, the image is byte-identical to the base. Completion, compression to
`done/` and promotion of Z1 (closing survey and user acceptance) belong to the advancer. The release
item list is `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md`.

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
