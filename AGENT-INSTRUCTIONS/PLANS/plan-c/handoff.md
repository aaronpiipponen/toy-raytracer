# Orchestrator handoff

## Next action

Wait for the user's acceptance of the closing survey, or their named changes. On acceptance, have
the advancer compress Z1 into `done/` and flip the status row; then hand the plan to the release.
Findings are not fixed inside Z1; a fix round is scheduled after the user's direction.

## Current state

C1, C2 and C3 are done at code head `525196b`. Z1's closing survey note is written at
`benchmarks/survey-2026-11-23.md` with ten cross-checked findings and an out-of-scope list; nothing
was fixed. The survey's default-render evidence is in `/tmp/opencode/survey-a.png` (and `-b.png`)
until the user has inspected it. The release deferral list is `docs-pass.md`.

## Learned rules

- `cargo` may not be on `PATH`; use `~/.cargo/bin/cargo` (gates.md).
- Workspace detail must respect `AGENT-INSTRUCTIONS/LOCAL/documentation-pages.md`: a command-line
  change updates the usage in `README.md`.
- Known cross-plan seam: C2 owns `render(width, height, samples)` averaging; the materials plan adds
  jitter to the same function, and plan-a freezes gamma in `to_byte`; the release/coordinator must
  preserve both at merge.
- When the advancer promotes and details the next phase in REVIEW AND ADVANCE, still run the full
  acceptance check on the new phase document before any implementer starts.
- A coordinator hold: make the held phase's `phase_subagent` call so the refusal is recorded, start
  nothing else, tell the user one line, end the turn.

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
