# Documentation pass for the release

- `src/vec3.rs:21` - the comment on `cross` says "For the triangle normals the geometry plan adds."
  A permanent code comment must never mention a plan or phase. Not fixed in the materials plan
  because the geometry plan owns that file's use of `cross`; the release clears it after both plans
  land.
