# Documentation pass for the release

Plan `plan-b` stays open until plan-a's release, which merges last. Every item below is on a file
shared by all worktrees, so this release does not edit it. Each item is handed to plan-a's release and
recorded there in `AGENT-INSTRUCTIONS/PLANS/plan-a/docs-pass.md`.

- `AGENT-INSTRUCTIONS/LOCAL/project.md:5` (repository map) - the map lists `material.rs`, which the
  integration tree does not hold until the materials work merges. That line is true only after
  plan-a's merge, so plan-a's release checks it there and does not edit it before.
  Owner: plan-a's release.

- `AGENT-INSTRUCTIONS/LOCAL/project.md:5` (repository map) - the map does not list `src/png.rs` and
  describes `main.rs` as holding the output. The PNG writer merged with the output work.
  Owner: plan-a's release.

- `AGENT-INSTRUCTIONS/LOCAL/repo-rules-domain.md:4-5` (What this repo is) - it says `src/main.rs`
  holds the output. `src/png.rs` writes the image and `main.rs` holds the command line and the render
  loop.
  Owner: plan-a's release.

- `AGENT-INSTRUCTIONS/LOCAL/gates.md` (rendered-image gate) - the fallback sentence "Until the
  command-line phase lands, `cargo run --release > /tmp/opencode/render.ppm` instead." is false in the
  integration tree, which takes the command line. Replace it with a real PPM line beside the PNG line
  (`cargo run --release -- --format ppm --output /tmp/opencode/render.ppm`) and drop the sentence.
  Owner: plan-a's release.

## Release-merge carry-overs (not documentation)

- `src/main.rs` (the scene triangle's material) - the merged product gives the triangle a material,
  and this branch holds no material. The user chose a `Lambertian` with albedo `(0.7, 0.3, 0.3)`, the
  front sphere's red. plan-a's release, which merges last, applies it at the triangle's exact landed
  vertices `(0.6, 0.15, -1.0)`, `(1.95, 0.15, -1.0)` and `(1.275, 0.92, -1.0)`, with no placement
  change.
  Owner: plan-a's release.

Resolved by this branch: the `src/vec3.rs:21` comment "For the triangle normals the geometry plan
adds." and its `#[allow(dead_code)]` are gone from the integration tree, because plan-b owns that
file's use of `cross`. The item is deleted from plan-a's list.
