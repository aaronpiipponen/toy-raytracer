# Documentation deferred to the release

Items explicitly deferred to the release instead of being done in a phase. The release resolves
each one and deletes its entry here.

- `AGENT-INSTRUCTIONS/LOCAL/gates.md` (rendered-image gate) - the fallback sentence "Until the
  command-line phase lands, `cargo run --release > /tmp/opencode/render.ppm` instead." is
  unnecessary once the output plan's command line is merged into the integration branch; delete it
  then. The file currently has the PNG command only, so the release also writes a PPM command beside
  it, for example `cargo run --release -- --format ppm --output /tmp/opencode/render.ppm`. Deferred
  2026-11-24: the file is shared by every worktree, and the sentence is still true in worktrees whose
  binary has no command line yet.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` (repository map) - the `src/` line does not name `png.rs`, the
  PNG writer the output plan added. Add it to the module list at the release, alongside the existing
  entries and in the same style. Deferred for the same reason: the file is shared by every worktree,
  and worktrees without the output plan's code have no `src/png.rs`.
