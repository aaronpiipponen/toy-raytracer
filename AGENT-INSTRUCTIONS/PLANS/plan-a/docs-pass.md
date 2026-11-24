# Documentation pass for the release

- `src/vec3.rs:21` - the comment on `cross` says "For the triangle normals the geometry plan adds."
  A permanent code comment must never mention a plan or phase. Not fixed in the materials plan
  because the geometry plan owns that file's use of `cross`; the release clears it after both plans
  land.

- `AGENT-INSTRUCTIONS/LOCAL/gates.md` (rendered-image gate) - the fallback sentence "Until the
  command-line phase lands, `cargo run --release > /tmp/opencode/render.ppm` instead." is
  unnecessary once the output plan's command line is merged into the integration branch; delete it
  then. If the gate is to show both formats, keep a real PPM line beside the PNG line, for example
  `cargo run --release -- --format ppm --output /tmp/opencode/render.ppm`; the fallback sentence is
  not such a line. Deferred 2026-11-24: the file is shared by every worktree, and the sentence is
  still true in worktrees whose binary has no command line yet.
  Owner: plan-a's release.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` (repository map) - the `src/` map does not list `src/png.rs`
  and describes `main.rs` as holding the output. Once the output plan is merged, add `png.rs` (the
  PNG writer) and describe `main.rs` as the scene, render loop, command line and output selection.
  Deferred 2026-11-24: the file is shared by every worktree, and `src/png.rs` does not exist there
  until the merge.
  Owner: plan-a's release.
- `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md` (What this repo is) - it says `src/main.rs` holds
  "the output" and each other module one concept. Once the output plan is merged, name the PNG
  writer as the output module and `main.rs` as the command line and render loop. Deferred
  2026-11-24 for the same reason: the file is shared and `src/png.rs` exists only on this branch.
  Owner: plan-a's release.
