# Documentation deferred to the release

Items explicitly deferred to the release instead of being done in a phase. The release resolves
each one and deletes its entry here.

All three entries stay open after plan-c's release merge into `main` at `41ab3c2`. Each fix edits a
file in `AGENT-INSTRUCTIONS/LOCAL/`, which the release session is not authorized to change, and each
statement still reads true in a plan worktree whose binary has no command line. The owner of every
entry is plan-a's release, the last merge, when no plan worktree renders without the command line.

- `AGENT-INSTRUCTIONS/LOCAL/gates.md` (rendered-image gate) - the fallback sentence "Until the
  command-line phase lands, `cargo run --release > /tmp/opencode/render.ppm` instead." is
  unnecessary once the output plan's command line is merged into the integration branch; delete it
  then. If the gate is to show both formats, keep a real PPM line beside the PNG line, for example
  `cargo run --release -- --format ppm --output /tmp/opencode/render.ppm`; the fallback sentence is
  not such a line. Deferred 2026-11-24: the file is shared by every worktree, and the sentence is
  still true in worktrees whose binary has no command line yet.
  Owner: plan-a's release. After plan-c's merge the sentence is false in `main`, but plan-a's branch
  still has no command line and its A3 phase renders through the PPM fallback.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` (repository map) - the `src/` map does not list `src/png.rs`
  and describes `main.rs` as holding the output. Once the output plan is merged, add `png.rs` (the
  PNG writer) and describe `main.rs` as the scene, render loop, command line and output selection.
  Deferred 2026-11-24: the file is shared by every worktree, and `src/png.rs` does not exist there
  until the merge.
  Owner: plan-a's release. The map already lists `material.rs`, `plane.rs`, `triangle.rs` and
  `aabb.rs`, so it describes the final tree and one edit must cover every plan, not plan-c alone.
- `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md` (What this repo is) - it says `src/main.rs` holds
  "the output" and each other module one concept. Once the output plan is merged, name the PNG
  writer as the output module and `main.rs` as the command line and render loop. Deferred
  2026-11-24 for the same reason: the file is shared and `src/png.rs` exists only on this branch.
  Owner: plan-a's release. The file is shared by every worktree, and the line must read true in the
  plan worktrees that still hold no `src/png.rs`.
