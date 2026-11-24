# Orchestrator handoff

## Next action

Start C3's implementer with `phase_subagent` and the active phase id `C3`. C3's phase document
passed the orchestrator acceptance check after one small fix (the progress-capture command now runs
the built binary, since `cargo run` mixes its own lines into captured standard error).

## Current state

C1 is done (`done/C1-png-writer-and-command-line.md`, tested head `fb4d00c`). C3 (progress line) is
`active` with gate `standard`, detailed in `phases/C3-progress-line.md`; no code for it is
implemented yet. C2 is `not applicable`. Z1 (closing survey) follows once C3 is done. `docs-pass.md`
holds the shared `gates.md` release item; `autonomous-decisions.md` holds the `--format` precedence.
Linked to the coordinator; `workspace.json` names no shared resource.

## Learned rules

- Plan files live in the workspace root and are committed by the workflow plugin: never commit them
  from the worktree.
- Cargo's own "Finished"/"Running" lines share standard error with the program: capture program
  streams from the built binary under `target/release/`, not from `cargo run`.
