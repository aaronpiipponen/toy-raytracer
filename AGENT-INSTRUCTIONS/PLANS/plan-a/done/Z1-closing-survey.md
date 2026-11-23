## Phase Z1 - Closing survey and user acceptance (done)

**Status:** done.
**Gate level:** minimal - no product code, test, configuration or permanent document changed.

**Frozen contract:**
- The closing survey note is `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/survey-2026-11-23.md`, dated
  `2026-11-23` and surveyed at code head `dca633c`. It covers the ten areas this plan added or
  changed, records that no earlier survey note exists, and holds 21 findings `F1` to `F21`, each with
  `file:line` evidence, a class, impact, recommendation and a `CONFIRMED` cross-check verdict. It
  also records the two claims the cross-check rejected, the out-of-scope observations kept separate,
  the checks performed, and the user-review list.
- The survey was read-only. No product source, test, configuration or permanent document changed;
  the worktree lists no product file at `dca633c`, and the plan's code head stayed `dca633c`.
- The note records: requirements 1 to 4 against the frozen contracts and current source; requirement
  5 in `src/main.rs`; requirement 6 and the three hard invariants (no dependency, one seeded
  `src/rng.rs`, no `unsafe` and no thread type); the fast-gate receipt at `dca633c` reused, not
  rerun; the test budget of seven functions added across A1r, A2 and A3, never more than three per
  phase, with twelve test functions in the suite and none deleted or weakened; and the
  documentation-accuracy result.
- Reproducibility and rendered image: two 200 by 100 renders are byte-identical; the image has 20003
  lines beginning `P3`, `200 100` and `255`; the converted PNG was opened and shows the sky gradient,
  the yellow-green ground, the matte red-brown centre sphere, the glass sphere with its bent interior
  and bright rim, and the grey metal sphere reflecting the scene, with smooth edges at 16 samples.
- No finding was fixed inside Z1 and none is scheduled. The user read the note and the rendered
  image, accepted the survey as it stands, scheduled no changes, and directed that the findings stay
  recorded in the note. Any fix needs a later user direction.
- No test, dependency, configuration key, checker, command, registry or receipt was added; new test
  functions 0, deleted 0.
- The plan folder is not archived or closed by this phase; closure belongs to the release and needs
  the user's explicit authorization.

**Evidence:** the survey note at
`AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/survey-2026-11-23.md`; code head `dca633c` with a clean
worktree; the user's acceptance of the note and `/tmp/opencode/survey-a.png`.

**Carry-overs:** the 21 findings stay recorded in the survey note as accepted observations; none is
scheduled work.
