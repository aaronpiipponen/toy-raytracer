<h1 align="center">toy-raytracer</h1>

<p align="center">
  A small Rust raytracer that three AI plan orchestrators built in parallel, as the test project of
  my-workflow.
</p>

---

**Contents**

- [Why it exists](#why-it-exists)
- [What exists in the project](#what-exists-in-the-project)
- [The runs](#the-runs)
- [Branch layout](#branch-layout)
- [How a run works](#how-a-run-works)
- [Run your own copy](#run-your-own-copy)

## Why it exists

[my-workflow](https://github.com/aaronpiipponen/my-workflow) runs several plan orchestrators at
once, each in its own worktree and branch, with a coordinator that stops two plans when they reach
the same code and makes them settle the overlap before it becomes a merge conflict.
The idea needs a real project to test on: small enough that a run finishes in hours, and rich enough
that three plans meet in the same files.

This repository is that project. It is a raytracer described by the fourteen requirements in
`docs/requirements.md`, built by three plans that own materials, geometry and output. The plans
work on a small starting program, and every phase they detail and implement is written by AI
agents.

## What exists in the project

- `src/` holds the raytracer: the camera and ray, the hit record, spheres, planes and triangles,
  three materials, a seeded generator, a hand-written PNG writer, a PPM writer and the command line.
- `docs/requirements.md` states what the program does, and the other pages in `docs/` document
  it as the latest run's documentation hotfix wrote them.
- `README-RUN.md` is the program's own README, with its logo, as the latest run left it.
- The runs and the starting state they begin from are branches, listed below.

## The runs

The first run proved the plumbing: linking, the feed, a hold pair, the channel and its resolution
between two plain chat sessions. The second run ran three real `/plan` orchestrators, the
coordinator, a hold pair settled by one shared design, and three releases merged into `main`.

The third run is the full test: three plan orchestrators, three releases and one hotfix release that
writes the program's documentation and README, all in autonomous mode. The orchestrators took the
decisions inside their plans' scope themselves, the user answered only where a decision had to be
theirs, and every result was accepted as reported. Along the way the coordinator settled three hold
pairs through the channel, and a shared-resource lock was tested by hand. The documentation hotfix
ran on the run's recorded end state after the three releases.

| Run | Code | The run's README |
|---|---|---|
| run001 | [run001/main](https://github.com/aaronpiipponen/toy-raytracer/tree/run001/main) | [README](https://github.com/aaronpiipponen/toy-raytracer/blob/run001/main/README.md) |
| run002 | [run002/main](https://github.com/aaronpiipponen/toy-raytracer/tree/run002/main) | [README](https://github.com/aaronpiipponen/toy-raytracer/blob/run002/main/README.md) |
| run003 | [run003/main](https://github.com/aaronpiipponen/toy-raytracer/tree/run003/main) | [README](https://github.com/aaronpiipponen/toy-raytracer/blob/run003/main/README.md) |
| run004 | [run004/main](https://github.com/aaronpiipponen/toy-raytracer/tree/run004/main) | [README](https://github.com/aaronpiipponen/toy-raytracer/blob/run004/main/README.md) |

`main` holds the latest of them, with this README and the name of its program README
(`README-RUN.md`) as the only changes.

## Branch layout

```text
main              the latest run's code, this README and README-RUN.md
run001/*          the first run, branch by branch
run002/*          the second run, branch by branch
run003/*          the third run, branch by branch
seed/*            the starting state every run begins from
```

Each run group holds the run's `main`, `plan-a`, `plan-b`, `plan-c` and `workflow` branches. The
`workflow` branch is the run's workspace root: the three plans, their scoping, the coordinator's
state and the shared designs. The root sessions' transcripts, the coordinator's included, are on
`main` with the project, and each plan worktree's sessions are on that plan's branch. The `seed/*`
branches hold the same five branches for the start: the project before any plan ran, and the
workspace with the three plans scoped and each plan's first phase detailed and audited.

## How a run works

1. A fresh workspace is cloned from `seed/workflow`, and a bare repository is built from
   `seed/main`, `seed/plan-a`, `seed/plan-b` and `seed/plan-c`, each with its own worktree.
2. A driver starts three plan orchestrators through `/plan`, each in its own plan's worktree with
   autonomous mode enabled, and links each to the coordinator. The coordinator watches them,
   starts a conflict investigator when two plans may touch the same code, and holds both plans
   before the overlapping phase starts.
3. Each held pair agrees on one shared design, each plan writes its own resolution phase from it,
   and both plans continue. Plans take a workspace resource with a lock when their work needs it.
4. The driver starts each plan's release in turn, in the order plan-c, plan-b, plan-a: the release
   merges its branch into the run's `main`, resolves conflicts by the shared designs, runs the
   gates, and the last one archives the plans.
5. The driver then starts a hotfix release in `main`, which writes the program's documentation and
   its README with a logo.
6. The run's branches are recorded as `run<n>/*`, and `main` is rebuilt from the run's `main` with
   this README and the program README renamed to `README-RUN.md`.

The driver does everything a person would type except answering the orchestrators' questions, which
the person running the test answers in each session's chat. The model of every role, the
orchestrators and the coordinator included, comes from the run's
`AGENT-INSTRUCTIONS/LOCAL/models.json`; every recorded run assigned DeepSeek V4.1 Flash `low` to
every role. The requirements, the toy harness and the report on the runs live in
[my-workflow](https://github.com/aaronpiipponen/my-workflow).

## Run your own copy

```bash
git clone --branch main https://github.com/aaronpiipponen/toy-raytracer
cd toy-raytracer
cargo run --release
```

The default run writes `render.png` beside the program. The command-line options and their defaults
are in [README-RUN.md](README-RUN.md).
