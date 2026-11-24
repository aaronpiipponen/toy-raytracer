# plan-b documentation pass - deferred to the release

Items here are deferred to the release rather than done by a phase.

## Repository map

- `AGENT-INSTRUCTIONS/LOCAL/project.md` - the repository map under "Project sources of truth" still
  lists only the original modules. Once the last of plan-a and plan-b has merged, add `aabb.rs`
  (axis-aligned box), `plane.rs` (infinite plane) and `triangle.rs` (triangle) alongside the existing
  entries, and add `png.rs` for the output plan at the same time. Plan-c already holds the `png.rs`
  half of this same release item; the map is shared by every worktree, so it is updated once, at the
  release, after all three plans' modules exist.
