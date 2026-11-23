# toy-raytracer

A small, dependency-free Rust raytracer that renders one fixed scene.

```sh
cargo run -- --width 400 --height 200 --samples 16 --output render.png --format png
cargo test
```

The options are:

- `--width N` sets the image width in pixels. The default is 400.
- `--height N` sets the image height in pixels. The default is 200.
- `--samples N` sets the samples per pixel. The default is 16.
- `--output PATH` sets the output file. The default is `render.png`.
- `--format png|ppm` sets the output format. Without it the format comes from the output file's extension.

`cargo run` with no options writes `render.png` at 400 by 200 with 16 samples.

`docs/requirements.md` states what the finished program does.
