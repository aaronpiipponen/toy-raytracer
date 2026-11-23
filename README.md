# toy-raytracer

A small, dependency-free Rust raytracer. It is the test project for a parallel plan workflow: three
plans (materials, geometry, output) start from this state on their own branches and run at the
same time.

```sh
cargo run > image.ppm
cargo test
```

`docs/requirements.md` states what the finished program does.
