### Local environment

The project has no runtime configuration and no secrets. Cargo and its tools live in
`~/.cargo/bin`; when `cargo` is not on `PATH`, call `~/.cargo/bin/cargo`.

### Fast phase gate

For code changes, run named tests covering touched behavior and regressions, then:

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check
git diff --check
```

Documentation-only changes require changed-file reread plus `git diff --check`. Prose a change
writes or edits follows `AGENT-INSTRUCTIONS/BUILDING/DOCUMENTATION-STYLE-GUIDE.md`. Tests follow
REPO-RULES §3a; the receipt states functions added/deleted.

### Experiment gate

This project has no experiments.

### Conditional gates

| Gate             | Run when                                                                 |
| ---------------- | ------------------------------------------------------------------------ |
| Rendered image   | The render loop, a material, the camera or the output format changed     |
| Agent generation | `AGENT-INSTRUCTIONS/SUBAGENTS/**` or agent model assignment changed      |

Rendered-image gate: render a small image and check it opens and looks right.

```bash
cargo run --release -- --width 80 --height 40 --samples 4 --output /tmp/opencode/render.png
```

The command writes `/tmp/opencode/render.png`. Inspect it after the run.

Agent-generation gate:

```bash
python -m json.tool AGENT-INSTRUCTIONS/LOCAL/models.json >/dev/null
python AGENT-INSTRUCTIONS/SUBAGENTS/gen_agents.py --check
```
