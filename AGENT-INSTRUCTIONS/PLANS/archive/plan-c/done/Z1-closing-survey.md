## Phase Z1 - Closing survey and user acceptance (done)

**Status:** done.
**Gate level:** minimal.

**Frozen contract:**
- The closing survey note is `AGENT-INSTRUCTIONS/PLANS/archive/plan-c/benchmarks/survey-2026-11-23.md`. It surveys the four areas this plan added or changed: PNG output, the command line and output selection, the progress line and render loop, and the plan-facing documentation, tests and plan state. It states that no earlier survey note exists.
- The note records 10 findings, F1 to F10, each with `file:line` evidence, impact and a recommendation. One independent read-only scout cross-checked every finding against source, documents and the three done contracts, and marked all 10 `CONFIRMED`. No finding is `WRONG` or `UNCLEAR`.
- The note keeps four out-of-scope observations separate from the plan-c findings and records the checks performed: requirements 11 to 14, the three hard invariants, the test budget, the documentation accuracy and the `docs-pass.md` state.
- The note's user-review list holds every finding, and none is fixed inside the phase.
- `AGENT-INSTRUCTIONS/PLANS/archive/plan-c/docs-pass.md` holds the release-time items: the root `README.md` shape, the missing bad-option sentence, and the licence decision. The licence item records the owner's decision that the repository has no licence file and no licence section.
- No product source, test, configuration, permanent document or gate command changed. `Cargo.toml` still has no dependency, and `git status --porcelain` in the worktree lists no product file.
- The survey adds no framework, checker, command, test, dependency, configuration key, registry, index or receipt. The only file it creates is the note.
- The user accepted the survey results as they stand. No finding is fixed, no extra test is added, no licence file or licence section is added, and the README bad-option sentence is left to the release through `docs-pass.md`.

**Evidence:** the fast gate and the rendered-image evidence at `525196b` were reused, because the revision is unchanged. The survey render at the default options wrote a valid PNG whose header decoded as `True 00000190000000c8` (400 by 200), showed the sphere over the sky gradient, wrote 200 carriage-returned progress updates ending `100%\n` on standard error, wrote nothing to standard output, and was byte-identical to a second run. The temporary survey files are removed. The note was read against the plan scope and the requirements.

**Carry-overs:** the 10 findings and the four out-of-scope observations sit in the note, with their user-review list and recommendations, and are not fixed. The release-time documentation items sit in `docs-pass.md`. Every plan phase is done. The release follows, and the plan is closed only after it finishes and the user authorizes closure.
