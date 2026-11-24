# plan-a documentation pass - deferred to the release

Items here are deferred to the release rather than done by a phase.

## Repository map

- `AGENT-INSTRUCTIONS/LOCAL/project.md` - the repository map under "Project sources of truth" omits
  `aabb.rs` (axis-aligned box), `plane.rs` (infinite plane), `triangle.rs` (triangle) and `png.rs`
  (PNG writer). Plan-a merges last, so its release adds the four entries alongside the existing ones,
  in the same style, once every module exists on `main`.
