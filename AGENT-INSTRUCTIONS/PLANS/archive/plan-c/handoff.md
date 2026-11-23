# Orchestrator handoff

## Next action

None. The release merged the plan into `main` at `a4b64cf`, cleared `docs-pass.md` and archived the
plan on 2026-11-23.

## Current state

C1, C2 and C3 are done at code head `15bea28`. The release added the command-line failure sentence
to `README.md` and corrected three comments in `src/main.rs`. The fast gate and the rendered-image
gate pass on the merged `main`. The closing survey `benchmarks/survey-2026-11-23.md` was accepted
as-is; the user ordered no fix and no extra test for its findings, and the plan closed without them.

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
