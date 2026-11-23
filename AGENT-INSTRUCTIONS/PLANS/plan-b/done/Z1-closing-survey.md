## Phase Z1 - Closing survey and user acceptance (done)

**Status:** done.
**Gate level:** minimal.

**Frozen contract:**
- `AGENT-INSTRUCTIONS/PLANS/plan-b/benchmarks/survey-2026-11-23.md` is the plan's closing survey
  note, dated 2026-11-23. It surveys the ten areas this plan added or changed, with one read-only
  pass per area and one cross-check pass, and it holds the findings with `file:line` evidence and a
  cross-check verdict, the out-of-scope observations, the checks performed and the user-review list.
- The note is the plan's only survey record. No earlier survey note exists for this plan.
- Findings F1 to F13 stand as recorded across `src/` and the plan documents. F9 (nearest-hit
  selection unproven in `src/hittable.rs:66-80`) and F10 (the sphere's and triangle's reported boxes
  unproven at the shape layer) are not fixed and carry no new test. The user accepted the survey
  results and directed that neither is fixed. The other findings are optional tidy-ups or need no
  action.
- The note added no product source, test, configuration or permanent document change. The code head
  is `2b4e2b7` and `git status --porcelain` in the worktree is empty.
- The plan folder is at `AGENT-INSTRUCTIONS/PLANS/plan-b/`. The release closes it, not this
  phase.

**Evidence:** the 200 by 100 render pair at `2b4e2b7` is byte-identical, with `wc -l` 20003, header
`P3`, `200 100`, `255`, line 4 `165 201 255`, line 7441 `127 127 255` and line 10104
`129 126 255`. The fast gate was not rerun on the unchanged revision, whose receipt records
`cargo build`, `cargo test` (11 passed), `cargo clippy -- -D warnings`, `cargo fmt --check` and
`git diff --check` passing.
