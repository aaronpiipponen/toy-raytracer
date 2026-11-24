# Documentation deferred to the release

Items explicitly deferred to the release instead of being done in a phase. The release resolves
each one and deletes its entry here.

## Hold until the other two plans merge

The release of `plan-c` is finished and `plan-c` is merged into the integration branch `main`.
Every item below is held, not dropped: none can be resolved without the materials plan (`plan-a`)
or the geometry plan (`plan-b`), and each names the release that owns it. `plan-c` closes only after
`plan-a` and `plan-b` merge and their releases clear or accept these items.

- **Owner: the release that merges the last of `plan-a` and `plan-b`.** `AGENT-INSTRUCTIONS/LOCAL/gates.md`
  (rendered-image gate): the fallback sentence "Until the command-line phase lands, `cargo run
  --release > /tmp/opencode/render.ppm` instead." is false on `main` and true only in the `plan-a`
  and `plan-b` worktrees, whose binaries take no options and write PPM to standard output. Delete the
  sentence once no active worktree lacks the command line. The file carries the PNG command only, so
  the same release writes a PPM command beside it, for example `cargo run --release -- --format ppm
  --output /tmp/opencode/render.ppm`.

- **Owner: the release that merges the last of `plan-a` and `plan-b`.** `AGENT-INSTRUCTIONS/LOCAL/project.md`
  (repository map): the `src/` line does not name `png.rs`, the PNG writer the output plan added.
  Add it to the module list in the same style once `src/png.rs` exists in every worktree. It exists
  on `main` and in `plan-c`, and does not exist in `plan-a` or `plan-b`.

- **Owner: `plan-a`'s release.** `src/main.rs` (module list): the comment "Seeded randomness for the
  scattering and sampling the plans add." names future plans in product code, above
  `#[allow(dead_code)] mod rng;`. Remove the comment when the materials work uses the generator and
  the allowance beside it is gone. Never delete the allowance early to make the comment removable.

- **Owner: `plan-a`'s release.** `src/hittable.rs` (the hit record's point field): the comment "The
  origin of the bounce rays the materials plan adds." names a future plan in product code, beside
  `#[allow(dead_code)]` on the field. Remove the comment when the materials work reads the field and
  the allowance beside it is gone. Do not delete the allowance early.

- **Owner: `plan-b`'s release.** `src/vec3.rs` (the cross product): the comment "For the triangle
  normals the geometry plan adds." names a future plan in product code, beside `#[allow(dead_code)]`
  on the method. `plan-b`'s branch already removes the comment and the allowance because its triangles
  call `cross`; the merge of `plan-b` resolves this entry. On `main` the comment and the allowance
  remain until that merge.
