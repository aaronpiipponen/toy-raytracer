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
- `src/main.rs` (module list) - the comment "Seeded randomness for the scattering and sampling the
  plans add." names future plans in product code, above `#[allow(dead_code)] mod rng;`. Remove the
  comment at the release **only once the dead-code allowance beside it is gone** (the allowance
  disappears when the materials work actually uses the generator); never delete the allowance early
  to make the comment removable.
- `src/hittable.rs` (the hit record's point field) - the comment "The origin of the bounce rays the
  materials plan adds." names a future plan in product code, beside `#[allow(dead_code)]` on the
  field. Remove the comment at the release **only once the allowance beside it is gone**, for the same
  reason; do not delete the allowance early.
- `src/vec3.rs` (the cross product) - the comment "For the triangle normals the geometry plan adds."
  names a future plan in product code, beside `#[allow(dead_code)]` on the method. Remove the comment
  at the release **only once the allowance beside it is gone**; do not delete the allowance early.
- `README.md` (the run example) - the two commands use bare `cargo`, which does not run verbatim in an
  environment where cargo is not on `PATH` (the workspace gate notes that it may not be). Make the
  README's commands work as written, for example by stating the `PATH` assumption; the run example
  itself is correct.
