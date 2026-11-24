<h1 align="center">toy-raytracer</h1>

<p align="center">
  A small Rust raytracer that three AI plan orchestrators built in parallel, as the test project of
  my-workflow.
</p>

---

**Contents**

- [Why it exists](#why-it-exists)
- [What is in the repository](#what-is-in-the-repository)
- [The runs](#the-runs)
- [Run your own copy](#run-your-own-copy)

## Why it exists

[my-workflow](https://github.com/aaronpiipponen/my-workflow) can run several plan orchestrators at
once. Each one is an AI chat session that owns one plan, and each plan works on its own git branch.
A coordinator watches the plans. When two of them are about to change the same code, it stops both
and has them agree on one design first, so their branches merge without surprises.

To test that, the workflow needs a real project that a run can finish in a few hours and in which
three plans edit the same files. This repository is that project. It is a raytracer described by
the fourteen requirements in `docs/requirements.md`. Three plans extend a small starting program:
one adds materials, one adds geometry and one adds the output. AI agents write every phase.

## What is in the repository

- `src/` holds the raytracer: the camera and ray, the hit record, spheres, planes and triangles,
  three materials, a seeded random generator, a hand-written PNG writer, a PPM writer and the
  command line.
- `docs/` holds `requirements.md`, which states what the program does, and the other pages that
  document it as the latest run wrote them.
- `README-RUN.md` is the program's own README, with its logo, as the latest run left it.
- The runs and the starting state they begin from are branches, listed below.

## The runs

Each run is recorded as a set of branches:

| Run | Code | The run's README |
|---|---|---|
| run001 | [run001/main](https://github.com/aaronpiipponen/toy-raytracer/tree/run001/main) | [README](https://github.com/aaronpiipponen/toy-raytracer/blob/run001/main/README.md) |
| run002 | [run002/main](https://github.com/aaronpiipponen/toy-raytracer/tree/run002/main) | [README](https://github.com/aaronpiipponen/toy-raytracer/blob/run002/main/README.md) |
| run003 | [run003/main](https://github.com/aaronpiipponen/toy-raytracer/tree/run003/main) | [README](https://github.com/aaronpiipponen/toy-raytracer/blob/run003/main/README.md) |
| run004 | [run004/main](https://github.com/aaronpiipponen/toy-raytracer/tree/run004/main) | [README](https://github.com/aaronpiipponen/toy-raytracer/blob/run004/main/README.md) |

`main` holds the latest run's code, this README and `README-RUN.md`.

- **run001** tested the connections with two plain chat sessions instead of real orchestrators:
  linking them to the coordinator, holding both at an overlap and settling it in a channel between
  them.
- **run002** ran three real plan orchestrators and the coordinator. One pair of plans was held and
  settled on one shared design, and three releases merged into `main`.
- **run003** was the full test: three plan orchestrators, three releases and a hotfix release that
  wrote the program's documentation and README. The orchestrators ran in autonomous mode, which
  means they decided inside their plan's scope without waiting for a person, and a driving agent
  answered their questions. The coordinator settled three held pairs, and a lock on a shared
  resource was tested by hand.
- **run004** repeated the full test with `toy/drive.py` starting the sessions, so a person only
  answered their questions. It found four defects in the workflow, which the report on the runs in
  [my-workflow](https://github.com/aaronpiipponen/my-workflow) describes. It ended with three
  releases merged and a documentation hotfix.

Every role in every run used DeepSeek V4.1 Flash on its `low` variant.

### Branches

```text
main              the latest run's code, this README and README-RUN.md
run001/*          the first run, branch by branch
run002/*          the second run, branch by branch
run003/*          the third run, branch by branch
run004/*          the fourth run, branch by branch
seed/*            the starting state every run begins from
```

Each run holds its `main`, `plan-a`, `plan-b`, `plan-c` and `workflow` branches. The `workflow`
branch is the run's workspace root: the three plans, their scoping, the coordinator's state and the
shared designs. The transcripts of the top-level sessions, the coordinator's included, are on `main`
with the project, and each plan worktree's transcripts are on that plan's branch. The `seed/*`
branches hold the same five branches for the start: the project before any plan ran, and the
workspace with the three plans scoped and each plan's first phase detailed and audited.

### How a run works

1. A fresh workspace is cloned from `seed/workflow`, and a bare repository is built from
   `seed/main`, `seed/plan-a`, `seed/plan-b` and `seed/plan-c`, each with its own worktree.
2. A driver starts three plan orchestrators with `/plan`, each in its plan's worktree in
   autonomous mode, and links each to the coordinator. The coordinator watches them. When two plans
   may touch the same code, it starts a conflict investigator and holds both plans before the
   overlapping phase.
3. Each held pair agrees on one shared design. Each plan writes its own resolution phase from it,
   and both plans continue. A plan takes a lock when its work needs a shared resource.
4. The driver starts each plan's release in turn, in the order plan-c, plan-b, plan-a. A release
   merges its branch into the run's `main`, settles conflicts with the shared designs and runs the
   checks. The last release archives the plans.
5. The driver starts a hotfix release in `main`, which writes the program's documentation and its
   README with a logo.
6. The run's branches are recorded as `run<n>/*`, and `main` is rebuilt from the run's `main`
   with this README, and the program README renamed to `README-RUN.md`.

The driver types everything a person would, except the answers to the orchestrators' questions.
The person running the test answers those in each session's chat. The requirements, the toy
harness and the report on the runs live in
[my-workflow](https://github.com/aaronpiipponen/my-workflow).

## Run your own copy

```bash
git clone --branch main https://github.com/aaronpiipponen/toy-raytracer
cd toy-raytracer
cargo run --release
```

The default run writes `render.png` beside the program. The command-line options and their defaults
are in [README-RUN.md](README-RUN.md).
