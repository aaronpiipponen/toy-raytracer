# Documentation deferred to the release

Items explicitly deferred to the release instead of being done in a phase. The release resolves
each one and deletes its entry here.

- `AGENT-INSTRUCTIONS/LOCAL/gates.md` (rendered-image gate) - the fallback sentence "Until the
  command-line phase lands, `cargo run --release > /tmp/opencode/render.ppm` instead." is
  unnecessary once the output plan's command line is merged into the integration branch; delete it
  then and keep the PNG command and the PPM command. Deferred 2026-11-24: the file is shared by every
  worktree, and the sentence is still true in worktrees whose binary has no command line yet.
