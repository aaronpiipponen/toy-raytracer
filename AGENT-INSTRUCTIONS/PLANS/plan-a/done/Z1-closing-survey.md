## Phase Z1 - Closing survey and user acceptance [DONE]

Survey complete and accepted by the user. Build instructions retired. Later work consumes
these frozen facts:

- The survey note is `AGENT-INSTRUCTIONS/PLANS/plan-a/benchmarks/closing-survey-2026-11-24.md`.
  Its five unchanged scout returns are
  `benchmarks/returns/area-1-materials.md`,
  `benchmarks/returns/area-2-hit-record-and-sphere.md`,
  `benchmarks/returns/area-3-render-loop-generator-and-scene.md`,
  `benchmarks/returns/area-4-tests-and-documents.md` and
  `benchmarks/returns/cross-check.md`.
- Surveyed revision: branch `plan-a`, range `e180476..566b11c`, head clean at `566b11c`; four
  independent read-only area scouts in parallel, then one independent read-only cross-check.
- Outcome: requirements 1 to 6 are present and correct, with no broken behaviour, no data loss, no
  hack and no plan reference in a permanent product file. The note's confirmed findings are three
  test-coverage gaps (diffuse randomness unproven, metal fuzz and the Schlick arm untested, two
  whole-image render assertions), five small cleanups and two stale frozen plan records, plus one
  naming split in the merge designs. Rejected claims and out-of-scope observations are recorded in
  the note and stay rejected.
- Rendered-image evidence at `566b11c`: `cargo run --release > /tmp/opencode/plan-a-acceptance.ppm`
  at the 200 by 100, 16-sample defaults, converted to PNG and opened. It shows the glass sphere on
  the left refracting and inverting the background, the diffuse red-brown centre sphere, the diffuse
  grey ground and the grey metal sphere on the right, with soft sphere edges, no salt-and-pepper sky
  grain and no flat `NaN` grey or pure-black pixels.
- User acceptance: the user accepted the survey findings and the rendered image, and chose to fix
  the accepted small cleanups and stale records (done in R1, whose render at `b22d647` is
  byte-identical with sha256
  `5457ef7a788539f5f5b0145e7c20fd178a77f0d184663b83fcd38d299a54665b`) and to accept the
  test-coverage gaps with no extra tests.
- `docs-pass.md` keeps its four deferred documentation items and its release-merge carry-over; the
  survey did not change them.
- The plan is not archived by this phase; archiving needs explicit user closure authorization.
