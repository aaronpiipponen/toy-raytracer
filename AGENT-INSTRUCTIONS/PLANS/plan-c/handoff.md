# Orchestrator handoff

## Next action

Spawn a fresh `phase-advancer` in `DETAIL ACTIVE` mode for phase C1, then run the acceptance check
on the produced phase document before any implementer starts.

## Current state

Phase C1 (PNG writer) is active and being detailed; no implementation exists. `src/main.rs` renders
a fixed 200x100 scene and writes PPM to standard output via `write_ppm`; `src/png.rs` does not exist
yet. Work happens in the `plan-c` worktree on branch `plan-c`.

## Learned rules

- `cargo` may not be on `PATH`; use `~/.cargo/bin/cargo` (gates.md).
- Rendered-image gate for C1 uses the PPM form until the command-line phase lands.
