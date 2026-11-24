# Closing survey returns (2026-11-24)

Verbatim returns of the closing survey's read-only scouts, kept as the saved finding set for
`benchmarks/2026-11-24-closing-survey.md`. Plan-c, branch `plan-c`.

## Area 1 - PNG writer

Read-only closing survey, one area of the output plan's work. Worktree:
/home/opc/projects/toy-raytracer-runs/run003/plan-c (branch plan-c). Do not edit, run tests, or
change anything; read and report.

Area: the PNG writer.

### Answer

The writer is correct and meets requirement 11 as written: std-only, one `IDAT`, zlib header +
stored (uncompressed) deflate blocks (`BFINAL`/`BTYPE=00` framing), CRC-32 per chunk, Adler-32
trailer, 8-bit truecolour filter 0 with no interlace. I re-derived both checksums in Python against
`zlib` for lengths 0/1/13/1000/70000 and they match exactly, and `0x78 0x01` passes the zlib FCHECK
modulus, so the byte stream is a structurally valid PNG. `tests/cli.rs`'s `decode_png` does
independently parse the bytes (chunk CRCs, single-IDAT, stored-block LEN/NLEN, filter bytes,
Adler-32) and does not call the writer's helpers. Remaining points are test-strength and cosmetic,
not defects.

### Findings

- `tests/cli.rs:280-303` - `crc32`/`adler32` are copied verbatim from `src/png.rs:65-89`, so although
  the decoder does not reuse the writer's code, a shared algorithmic mistake (wrong polynomial/init)
  passes both sides; there is no known-answer vector. A one-line constant check
  (`CRC32("IEND") == 0xAE426082`, `adler32("abc") == 0x024D0127`, both confirmed against `zlib`)
  would pin the algorithm itself. Not a bug; the cheap missing guard.
- `tests/cli.rs:208` - asserts the zlib header equals the literal `[0x78, 0x01]` rather than checking
  it is a valid zlib header (CM=8, FCHECK mod 31, FDICT=0). It mirrors the writer's constant instead
  of validating it.
- `tests/cli.rs:198-205` - `IEND` handling `break`s without asserting it is the last chunk
  (`position == bytes.len()`) and without asserting `IHDR` precedes `IDAT`; trailing bytes or a second
  `IHDR` would pass. Low risk because the writer emits neither.
- `tests/cli.rs:82-87` - the PNG/PPM comparison checks only 3 of 32 pixels;
  `done/C1-png-writer-and-command-line.md:36` says "PNG pixels match the PPM render", which overstates
  the coverage slightly. Oracle is weak but the shared `to_byte` makes equality trivial anyway.
- `src/png.rs:37-46` - for an empty `raw` (height 0) the while loop emits no deflate block, producing
  an invalid zlib stream; unreachable from the CLI (`parse_number` minimum 2 in `src/main.rs:120`),
  but `write_png` is `pub` and would silently write a broken PNG. Cosmetic per the hobby-simplicity
  rule.
- `src/png.rs:54-61` - `write_chunk` rebuilds `kind + data` in a fresh `Vec` purely for the CRC; for
  the IDAT this copies the whole stream (~240 KB at defaults). Correct and simple, just an avoidable
  copy.
- `src/png.rs:18-19` - `width as u32` / `height as u32` silently truncate above `u32::MAX` instead of
  erroring; unreachable realistically.

Clean aspects checked: no dead code or unused helpers (`write_png`/`write_chunk`/`crc32`/`adler32`
all used via `src/main.rs:175`); no second PNG code path; `to_byte` is shared by both writers rather
than duplicated (`src/png.rs:28-30`, `src/main.rs:98-100`); no linear pixels/pixel framing
duplication between writers; no stale or plan-history comments in `src/png.rs` (all four doc lines
are accurate); `Cargo.toml` has an empty `[dependencies]`, so "no external crate" holds;
`docs/requirements.md:34-36` matches the implementation exactly.

### Unknowns

- I did not run the binary or an external PNG decoder (read-only survey), so "opens in an image
  viewer" is established by structure and checksum verification, not by opening a file. The Z1
  acceptance step covers that.
- `pixels.len() < width*height` would panic in `src/png.rs:27`; the only caller (`src/main.rs:175`)
  passes `render`'s exactly-sized vector, so I treated it as a caller invariant rather than a
  finding.

### Out-of-scope observations

None.

## Area 2 - Command line and settings

Read-only closing survey, one area of the output plan's work.

### Answer

The command line and settings resolution are **clean**. The five options, the four defaults,
explicit `--format` winning over the extension, the usage line plus exit 2 on bad input, exit 1 with
a reported `io::Error` on an unwritable path, reachable PPM, and image-free stdout all match
requirement 12 and the frozen contract. There is a single parsing path, defaults are declared once,
every validation is load-bearing with no unreachable arm, and no error is swallowed. Nothing here is
more machinery than the job needs.

### Evidence

- `src/main.rs:23-26,108-112` - the four defaults (`400`, `200`, `16`, `render.png`) are each declared
  once and read directly into the locals; no second copy exists (`grep` over `src/` finds the
  constants only here).
- `src/main.rs:119-133` - all five options handled in one `match`; `_` covers unknown options and the
  `("--width"|...,"--format", None)` arm covers a missing value. Both branches reachable, no
  unreachable pattern.
- `src/main.rs:138,160-169` - `resolve_format` returns the explicit `--format` first, else maps
  `.png`/`.ppm`, else errors; called exactly once.
- `src/main.rs:149-157` - `parse_number` is the only minimum-check site; `min 2` guards
  `(width-1)`/`(height-1)` and `min 1` guards the `samples` divisor, so the bounds are required, not
  redundant.
- `src/main.rs:173-178` - both writers target an in-memory `Vec` and only `fs::write` touches the
  path; no `println!` anywhere, progress uses `eprint!`/`eprintln!` (`src/main.rs:77-79`), so stdout
  stays clean.
- `src/main.rs:183-190` - parse error prints `error:` + `USAGE` and `exit(2)`; `src/main.rs:191-194` -
  `run` error prints and `exit(1)`; neither error is caught-and-ignored.
- `src/main.rs:182,107` - the only argument entry point is `env::args().skip(1)` feeding
  `parse_args`; no second parse path (`grep` for `env::args|parse_args` finds only these).
- `tests/cli.rs:52-59,91-106,108-165` - tests assert empty stdout, non-empty stderr, non-zero exit
  with no file on bad input, extension-driven format, and explicit `--format` precedence.
- `docs/requirements.md:37-40` and `done/C1-...md:5-18` - the code matches item 12 and the frozen
  guarantees line for line.
- `README.md:6` - shows the real command line, satisfying the contract's docs state.

Stale comments or plan history **inside the area**: none. `src/main.rs:1`, `:44`, `:60-61`, `:106`,
`:148`, `:159` all describe current behaviour with no forward/plan references.

### Observations outside this area

- `src/main.rs:7-9` - `// Seeded randomness for the scattering and sampling the plans add.` plus
  `#[allow(dead_code)] mod rng;` is plan-history wording around a module nothing in `main.rs` uses;
  `src/rng.rs` was added by the base commit `e1293ef "Add the starting raytracer"`, so this is
  pre-existing scaffolding owned by the materials work, not the output area. Not investigated
  further.

### Unknowns

- None affecting this area. The only unverified runtime detail is that `fs::write` (not a pre-open)
  means an unwritable path is detected only after the full render completes
  (`src/main.rs:171-179`); the contract only requires the error be reported, which it is, so this is
  not treated as a defect.

## Area 3 - Render loop and progress line

Read-only closing survey, one area of the output plan's work.

### Answer

`render` matches the frozen C3/C3r contract: pixels are pushed row-major from the top (`row` runs
`height-1..=0`, `v = row/(height-1)`), `samples` values are summed and divided once in linear space
before any byte conversion, `to_byte` is the single conversion used by both writers, the
`\r{N} of {H} rows` write happens after each row's column loop, a terminating `eprintln!()` follows,
and nothing reaches stdout. What it does **not** do is say that the sample loop is currently a no-op:
with no jitter, all `samples` iterations trace the identical `ray`, so the average is mathematically
just `ray_colour`, costing `samplesx` the tracing work for exactly the same image (verified: PPM at
`--samples 1` and `--samples 16` are byte-identical). That is intended branch state awaiting A3, but
the code carries no note and the doc comment reads as if the samples differ.

### Findings

**In-area**

- `src/main.rs:70-75` - the sample loop traces one pre-built `ray` `samples` times and divides; it is
  a no-op average and multiplies render work by `samples` (default 16 -> ~1.28M traces for 400x200
  instead of 80k), and nothing in the code or its doc comment (`:60-61`) marks this as pre-jitter
  branch state.
- `src/main.rs:60-61` - "averaging `samples` rays per pixel" is misleading on this branch because
  every sample is the same ray; the merged target carries `// A3 jitters these later`
  (`PLANS/shared/plan-a-A1+plan-c-C3.md:19`), which this tree omits.
- `src/main.rs:7-9` - the comment "Seeded randomness for the scattering and sampling **the plans
  add**" leaks plan history into product code, and `mod rng` is dead code here behind
  `#[allow(dead_code)]` (acknowledged branch state in `done/C3r-shared-render-settlement.md`).
- `src/main.rs:60` - "Reports the share of rows done" is a slight wording mismatch: the line prints a
  count (`N of H rows`), not a share; this follows the frozen contract and requirement 13's wording,
  so it is cosmetic only.

**Clean (checked)**

- `src/main.rs:62-81` - row-major top-down order, `samples` sum then single `colour / samples`,
  progress inside the row loop after the column loop, `height - row` running 1..=height, terminating
  newline; live run gave 0 stdout bytes and stderr `^M1 of 10 rows...^M10 of 10 rows$`.
- `src/main.rs:83-85` - `to_byte` clamps then scales `255.999`, applied once per channel; both
  writers use it (`src/png.rs:28-30`, `src/main.rs:95-101`), so no second conversion path.
- `src/main.rs:46-51` / `53-58` - `ray_colour` and `scene` are the pre-material branch shapes, used
  once each; no dead parameters or duplicate scene construction.
- `src/main.rs:197-212` - both unit tests assert render output the design promises (centre sphere
  normal `z>0.9`, top-row sky `z>x`) and consume `render(200,100,16)` as the contract states; the
  `the_center_pixel...` test is the one A1 deletes at merge.
- No second output path: `run` (`:171-179`) encodes into one `Vec<u8>` and calls `fs::write` once.
- No redundant sample guard in `render`; `--samples 0` is rejected by the parser (`:121`, minimum 1)
  and width/height minimum 2 keeps `(width-1)`/`(height-1)` non-zero, matching C3r's "nothing in
  render adds a second guard".
- `tests/cli.rs` asserts only contracted behaviour for this area (stdout empty, stderr non-empty) and
  uses `--samples 1`.

### Out-of-scope observations

- `src/hittable.rs:10` - "The origin of the bounce rays the materials plan adds." is the same
  plan-history pattern; `point` is dead code here.
- `src/vec3.rs:21` - "For the triangle normals the geometry plan adds." also names a future plan in
  product code.
- `AGENT-INSTRUCTIONS/PLANS/shared/plan-a-A1+plan-c-C3.md:19-21` - the frozen target snippet keeps
  `let u/v` and `let ray` outside the sample loop while `:19` says A3 jitters them later and the A3
  draw order requires per-sample jitter; the snippet as printed computes one ray per pixel, so A3
  must move u/v/ray inside the loop - a move the snippet does not show. Current `src/main.rs:68-73`
  inherits this shape.

### Unknowns

- Whether a compiler elides the repeated `ray_colour` calls could not be established from source
  alone; a release run's timing would settle the actual cost, but the byte-identical output already
  proves the loop's arithmetic is a no-op.
- I did not run `cargo test`/`clippy`/`fmt` (out of scope for a read-only survey), so gate status is
  unverified.

## Area 4 - Documents

Read-only closing survey, one area of the output plan's work.

### Answer

The output/CLI prose in `README.md` and `docs/requirements.md` items 11-14 is true of this tree; the
`gates.md` PNG command is correct for `plan-c`, and the fallback sentence is still true for `main`,
`plan-a` and `plan-b` (all three binaries still write PPM to stdout). The `docs-pass.md` record is
accurate about *why* the sentence is deferred but incomplete about *what* the release must write.
Three product comments still reference plan names and carry `#[allow(dead_code)]` markers (all
pre-date this plan). One workspace doc (`project.md`'s module map) was made stale by C1 and no phase
caught or recorded it.

### Findings

- `AGENT-INSTRUCTIONS/PLANS/plan-c/docs-pass.md:9` - the record says the release should delete the
  fallback and "keep the PNG command and the PPM command", but `gates.md:35-39` has only one PNG
  command plus the fallback sentence; deleting the fallback leaves no PPM command, so the item must
  say to *write* a PPM command (e.g. with `--format ppm`), not "keep" one.
- `AGENT-INSTRUCTIONS/LOCAL/project.md:5` - the repository map lists every `src/` module but omits
  `src/png.rs`, which C1 added (`src/main.rs:5` declares it); this is stale documentation the plan
  made wrong, and `docs-pass.md` does not record it as a shared-file release item.
- `src/main.rs:7-9` - product file carries plan history/unfinished marker: "Seeded randomness for the
  scattering and sampling **the plans** add" followed by `#[allow(dead_code)] mod rng;`. Blame shows
  this predates `plan-c` (commit `e1293ef`), so it was not introduced here, but it is present.
- `src/hittable.rs:10-12` - "The origin of the bounce rays **the materials plan** adds" plus
  `#[allow(dead_code)]` on `point`; plan-history/unused marker, pre-existing (`e1293ef`).
- `src/vec3.rs:21-23` - "For the triangle normals **the geometry plan** adds" plus
  `#[allow(dead_code)]` on `cross`; plan-history/unused marker, pre-existing (`e1293ef`).

No other plan history, phase id, ticket or TODO exists in `src/` or `tests/` (grep for
`TODO|FIXME|unimplemented|plan|phase` returns only the three comments above).

### Clean areas (checked, no finding)

- `README.md:1-10` - "dependency-free ... one fixed scene" true (`Cargo.toml` has an empty
  `[dependencies]`; `src/main.rs:53-58` builds a fixed two-sphere scene);
  `cargo run --release -- --output image.png` is valid (`.png` selects PNG); the tests it advertises
  exist in `tests/cli.rs` and `src/main.rs:197-212`.
- `docs/requirements.md:34-36` (item 11) - `src/png.rs` emits one signature, one `IHDR`, one `IDAT`
  (`png.rs:29-46`), stored deflate blocks of <=65535 bytes (`png.rs:37-45`), per-chunk CRC-32
  (`png.rs:54-78`) and an Adler-32 trailer (`png.rs:48,81-89`); PPM stays reachable through
  `write_ppm` (`main.rs:87-104`) via `--format ppm`/`.ppm`.
- `docs/requirements.md:37-40` (item 12) - defaults are 400x200, 16, `render.png` (`main.rs:23-26`);
  all five options are parsed (`main.rs:107-147`); extension fallback and explicit `--format`
  precedence are in `resolve_format` (`main.rs:160-169`); the built binary answered `--bogus` with the
  usage line and exit 2, and `--width wide` with the usage line (live run of the existing
  `target/release/toy-raytracer`).
- `docs/requirements.md:41` (item 13) - `main.rs:77` writes `\r{n} of {height} rows` to stderr after
  each row and `main.rs:79` terminates it; nothing goes to stdout (`main.rs:171-195`).
- `docs/requirements.md:45` (item 14) - `cargo fmt --check` run via `~/.cargo/bin/cargo` exited 0;
  `cargo build`/`test`/`clippy` were not run (read-only role).
- `AGENT-INSTRUCTIONS/LOCAL/gates.md:36` - the command's options all exist and validate on this branch
  (`--width 80`, `--height 40`, `--samples 4`, `--output ...png`), and `/tmp/opencode` exists; the bare
  `cargo` is covered by the PATH note at `gates.md:5-7`.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md:39` - the fallback is still true where no command line exists:
  `main/src/main.rs:77-79`, `plan-a/src/main.rs:77-79` and `plan-b/src/main.rs:77-79` all render and
  `write_ppm(&mut io::stdout().lock(), ...)`; all four worktrees link the same `AGENT-INSTRUCTIONS`,
  confirming `docs-pass.md:9-10`'s "shared by every worktree" claim.
- `docs-pass.md:6-8,10` - the core claim (sentence unnecessary after merge; still true pre-merge;
  dated and deferred rather than done in a phase) is accurate.
- `src/main.rs:1`, `src/main.rs:60-61` and `src/png.rs:1,8` - the module/function doc comments
  describe exactly what the code does (PNG or PPM file output; row-by-row averaging with stderr
  progress; uncompressed-deflate PNG).

### Out-of-scope observations

- `AGENT-INSTRUCTIONS/PLANS/plan-c/phases/Z1-closing-survey-and-user-acceptance.md:47` cites
  `tests/cli.rs:167-248` for `decode_png`, but the function actually spans `168-249`
  (`tests/cli.rs:168`; `read_ppm` starts at `251`). Phase-doc line drift, not product documentation.
- `README.md:6-7` uses bare `cargo` while `gates.md:5-7` notes cargo may not be on `PATH` in this
  environment; the command fails verbatim here without `~/.cargo/bin`. Pre-existing and
  environment-specific, not introduced by this plan.
- `src/main.rs:199-211` unit tests call `render`, which now writes progress to the test process's
  stderr; harmless, but the test output is noisier than before C3.

### Unknowns

- `docs/requirements.md:45`'s `cargo build`, `cargo test` and `cargo clippy -- -D warnings` could not
  be re-verified: the read-only role forbids builds/tests, and the plan folder contains no saved gate
  output. The existing `target/release/toy-raytracer` (built 02:47, matching commit `e194181`) runs
  correctly.
- Whether `AGENT-INSTRUCTIONS/LOCAL/project.md` is intended to be updated by a phase or only at
  release is unrecorded; `docs-pass.md` lists only `gates.md`, so as it stands the map omission has no
  owner.
