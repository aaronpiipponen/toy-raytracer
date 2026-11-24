# Documentation pass for the release

- `AGENT-INSTRUCTIONS/LOCAL/gates.md` (rendered-image gate) - the fallback sentence "Until the
  command-line phase lands, `cargo run --release > /tmp/opencode/render.ppm` instead." is
  unnecessary once the output plan's command line is merged into the integration branch; delete it
  then. If the gate is to show both formats, keep a real PPM line beside the PNG line, for example
  `cargo run --release -- --format ppm --output /tmp/opencode/render.ppm`; the fallback sentence is
  not such a line. The integration tree takes the command line now, so the sentence is false there.
  Owner: plan-a's release.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` (repository map) - the `src/` map does not list `src/png.rs`
  and describes `main.rs` as holding the output. Once the output plan is merged, add `png.rs` (the
  PNG writer) and describe `main.rs` as the scene, render loop, command line and output selection.
  The PNG writer is in the integration tree now.
  Owner: plan-a's release.
- `AGENT-INSTRUCTIONS/LOCAL/project.md` (repository map) - the map lists `src/material.rs`, which
  exists only on this branch. Once this branch merges, check that the line names the material module
  the integration tree holds. Handed over by plan-b's closure, which could not make the line true
  from its own worktree.
  Owner: plan-a's release.
- `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md` (What this repo is) - it says `src/main.rs` holds
  "the output" and each other module one concept. Once the output plan is merged, name the PNG
  writer as the output module and `main.rs` as the command line and render loop. Deferred
  2026-11-24 for the same reason: the file is shared and `src/png.rs` exists only on this branch.
  Owner: plan-a's release.

## Release-merge carry-over (not documentation)

- `src/main.rs` (the scene triangle's material) - the merged product gives the triangle a material.
  Handed over by plan-b's closure, which holds no material on its branch. The user's answer: a
  `Lambertian` with albedo `(0.7, 0.3, 0.3)`, the front sphere's red, so the triangle reads as part of
  the scene. Apply it at the triangle's exact vertices `(0.6, 0.15, -1.0)`, `(1.95, 0.15, -1.0)` and
  `(1.275, 0.92, -1.0)`, with no placement change.
  Owner: plan-a's release.
