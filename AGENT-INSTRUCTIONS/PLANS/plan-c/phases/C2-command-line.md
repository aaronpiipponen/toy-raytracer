## Phase C2 - Command line

**Status:** active.
**Depends on:** `done/C1-png-writer.md`, which froze `png::write_png` and the `render.png` output path.
**Scope:** parse `--width`, `--height`, `--samples`, `--output` and `--format png|ppm` by hand in `src/main.rs`, with the defaults and the usage behavior of `docs/requirements.md` item 12. `main` renders at the parsed size, averages the parsed sample count per pixel, and writes either a PNG through `png::write_png` or a PPM through the existing `write_ppm` to the parsed path. No new dependency, no progress line, no change to the scene, the camera, the materials or the render values.
**Gate level:** standard. The phase changes runtime behavior and the command-line surface, with no durable state, no external input and no trust boundary. Targeted unit tests, the fast gate and the rendered-image conditional gate prove it.

**Simplicity statement:** dimensions and sample counts come from the user and are validated once in `Config::parse`; width and height must be at least 2 because `render` divides by `width - 1` and `height - 1`, and samples must be at least 1. A missing value, a non-numeric value, an unknown option, and an output path whose extension is neither `png` nor `ppm` each produce a clear error message and the usage line, then exit with status 2. The format is derived from the output extension only when `--format` is absent. Duplicate options are allowed and the last one wins. The parser stores no history, no file list and no default beyond the five requirement values. The averaging loop traces the same deterministic camera ray per sample until the materials plan adds jitter; that is the sample-count plumbing this phase owns, not a second shading path.

**Current state (verified):**
- `src/main.rs:1` - the module header `//! Render a fixed scene and write it as a PNG image.`
- `src/main.rs:3-11` - the module list ends `mod png;`, `mod ray;`, `#[allow(dead_code)] mod rng;`, `mod sphere;`, `mod vec3;`.
- `src/main.rs:13-14` - imports `use std::fs::File;` and `use std::io::{self, Write};`.
- `src/main.rs:16-20` - `use camera::Camera;`, `use hittable::{Hittable, HittableList};`, `use ray::Ray;`, `use sphere::Sphere;`, `use vec3::{Colour, Vec3};`.
- `src/main.rs:22-23` - `const WIDTH: usize = 200;` and `const HEIGHT: usize = 100;`.
- `src/main.rs:41-54` - `fn render(width: usize, height: usize) -> Vec<Colour>` builds the scene and camera, iterates rows `(0..height).rev()` and columns `0..width`, and pushes one `ray_colour(&camera.ray(u, v), &world)` per pixel.
- `src/main.rs:56-58` - `fn to_byte(channel: f64) -> u8` returns `(255.999 * channel.clamp(0.0, 1.0)) as u8`.
- `src/main.rs:60-78` - `#[allow(dead_code)] fn write_ppm(out: &mut impl Write, width: usize, height: usize, pixels: &[Colour]) -> io::Result<()>` with the `#[allow(dead_code)]` on line 60.
- `src/main.rs:80-89` - `fn main() -> io::Result<()>` renders `WIDTH`/`HEIGHT`, builds the RGB bytes, and writes `render.png` through `io::BufWriter` and `png::write_png`, then flushes.
- `src/main.rs:91-107` - the test module with `the_center_pixel_shows_the_sphere_facing_the_camera` and `the_top_row_is_sky`, both calling `render(WIDTH, HEIGHT)`.
- `src/png.rs:11` - `pub fn write_png(out: &mut impl Write, width: usize, height: usize, rgb: &[u8]) -> io::Result<()>`.
- `src/vec3.rs:40-45`, `src/vec3.rs:61-66`, `src/vec3.rs:82-83` - `impl Add for Vec3`, `impl Mul<f64> for Vec3`, and `pub type Colour = Vec3;`, the operators the averaging loop uses.
- `src/camera.rs:15` and `src/camera.rs:31` - `Camera::new(aspect_ratio: f64)` and `ray(u: f64, v: f64) -> Ray`.
- `src/hittable.rs:10` - the comment `// The origin of the bounce rays the materials plan adds.` and `src/vec3.rs:21` - the comment `// For the triangle normals the geometry plan adds.`, both carry-overs from C1.
- `README.md:5-12` - the run example is `cargo run` and `cargo test`, and it states `cargo run` writes `render.png`.
- `docs/requirements.md:37-40` - item 12 defines the five options, the defaults and the usage behavior.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the rendered-image gate's option-driven `cargo run --release -- --width 80 --height 40 --samples 4 --output /tmp/opencode/render.png` and the sentence that `cargo run --release` writes `render.png`.
- `Cargo.toml` - `[dependencies]` is empty.

**Read first (do not invent):**
- `src/main.rs` - the whole file: `render`, `to_byte`, `write_ppm`, `main`, the two tests, and the imports this phase extends.
- `src/png.rs:1-22` - the exact `write_png` signature and the RGB byte layout `main` builds.
- `src/vec3.rs` - `Vec3::new`, `Add`, `Mul<f64>`, `Div<f64>` and `Colour`; use the existing operators, do not add a helper.
- `src/camera.rs` - `Camera::new` and `Camera::ray`.
- `docs/requirements.md:32-41` - items 11, 12 and 14.
- `AGENT-INSTRUCTIONS/LOCAL/gates.md` - the fast gate and the rendered-image conditional gate.
- `Cargo.toml` - the empty `[dependencies]` table this phase leaves untouched.

**Build plan:**

1. `src/main.rs:22-23` - change the defaults to the requirement values and add the constant names the parser uses:
   ```rust
   const WIDTH: usize = 400;
   const HEIGHT: usize = 200;
   const DEFAULT_SAMPLES: usize = 16;
   const DEFAULT_OUTPUT: &str = "render.png";
   ```

2. `src/main.rs` - add `use std::path::Path;` beside `use std::fs::File;`. Do not import `std::process`; call `std::process::exit(2)` fully qualified.

3. `src/main.rs` - above `fn main`, add the command-line types and parser:
   ```rust
   #[derive(Clone, Copy, Debug, PartialEq, Eq)]
   enum Format {
       Png,
       Ppm,
   }

   #[derive(Debug, PartialEq)]
   struct Config {
       width: usize,
       height: usize,
       samples: usize,
       output: String,
       format: Format,
   }

   impl Default for Config {
       fn default() -> Self {
           Config {
               width: WIDTH,
               height: HEIGHT,
               samples: DEFAULT_SAMPLES,
               output: DEFAULT_OUTPUT.to_string(),
               format: Format::Png,
           }
       }
   }

   impl Config {
       fn parse(args: impl Iterator<Item = String>) -> Result<Config, String> {
           let mut config = Config::default();
           let mut format: Option<Format> = None;
           let mut args = args;
           while let Some(arg) = args.next() {
               match arg.as_str() {
                   "--width" => config.width = parse_number(args.next(), "--width")?,
                   "--height" => config.height = parse_number(args.next(), "--height")?,
                   "--samples" => config.samples = parse_number(args.next(), "--samples")?,
                   "--output" => config.output = parse_value(args.next(), "--output")?,
                   "--format" => {
                       format = Some(match args.next().as_deref() {
                           Some("png") => Format::Png,
                           Some("ppm") => Format::Ppm,
                           _ => return Err("--format takes png or ppm".to_string()),
                       });
                   }
                   other => return Err(format!("unknown option: {other}")),
               }
           }
           if config.width < 2 {
               return Err("--width must be at least 2".to_string());
           }
           if config.height < 2 {
               return Err("--height must be at least 2".to_string());
           }
           if config.samples < 1 {
               return Err("--samples must be at least 1".to_string());
           }
           config.format = match format {
               Some(format) => format,
               None => match Path::new(&config.output).extension().and_then(|e| e.to_str()) {
                   Some("png") => Format::Png,
                   Some("ppm") => Format::Ppm,
                   _ => return Err("--output must end in .png or .ppm".to_string()),
               },
           };
           Ok(config)
       }
   }

   fn parse_value(value: Option<String>, option: &str) -> Result<String, String> {
       value.ok_or_else(|| format!("{option} needs a value"))
   }

   fn parse_number(value: Option<String>, option: &str) -> Result<usize, String> {
       parse_value(value, option)?
           .parse()
           .map_err(|_| format!("{option} needs a number"))
   }

   fn usage() -> &'static str {
       "usage: toy-raytracer [--width N] [--height N] [--samples N] [--output PATH] [--format png|ppm]"
   }
   ```
   A bare argument that is not an option also falls into `other`, so it prints `unknown option: <arg>`; that is the same usage error the requirement asks for.

4. `src/main.rs:41-54` - change `render` to take the sample count and average it per pixel:
   ```rust
   fn render(width: usize, height: usize, samples: usize) -> Vec<Colour> {
       let world = scene();
       let camera = Camera::new(width as f64 / height as f64);
       let mut pixels = Vec::with_capacity(width * height);
       for row in (0..height).rev() {
           for column in 0..width {
               let u = column as f64 / (width - 1) as f64;
               let v = row as f64 / (height - 1) as f64;
               let ray = camera.ray(u, v);
               let mut colour = Colour::new(0.0, 0.0, 0.0);
               for _ in 0..samples {
                   colour = colour + ray_colour(&ray, &world);
               }
               pixels.push(colour / samples as f64);
           }
       }
       pixels
   }
   ```
   Keep the existing rows-top-to-bottom order and the `(width - 1)`/`(height - 1)` mapping so the image is the same scene at the new default size.

5. `src/main.rs:60` - delete the `#[allow(dead_code)]` above `write_ppm`; `main` calls it again for the PPM format. Change nothing inside the function.

6. `src/main.rs:80-89` - replace `fn main` with:
   ```rust
   fn main() -> io::Result<()> {
       let config = match Config::parse(std::env::args().skip(1)) {
           Ok(config) => config,
           Err(message) => {
               eprintln!("{message}");
               eprintln!("{}", usage());
               std::process::exit(2);
           }
       };
       let pixels = render(config.width, config.height, config.samples);
       let mut out = io::BufWriter::new(File::create(&config.output)?);
       match config.format {
           Format::Png => {
               let rgb: Vec<u8> = pixels
                   .iter()
                   .flat_map(|p| [to_byte(p.x), to_byte(p.y), to_byte(p.z)])
                   .collect();
               png::write_png(&mut out, config.width, config.height, &rgb)?;
           }
           Format::Ppm => write_ppm(&mut out, config.width, config.height, &pixels)?,
       }
       out.flush()
   }
   ```
   `write_ppm` and `write_png` share the one `BufWriter`, and the single `out.flush()` reports a buffered write error from either format.

7. `src/main.rs:91-107` - adapt the two existing tests to the new signature: both calls become `render(WIDTH, HEIGHT, 1)`. Their assertions are unchanged.

8. `src/main.rs` - add the three tests in the `Tests` section to the existing `mod tests`.

9. Carry-overs from `done/C1-png-writer.md` - delete the comment `// The origin of the bounce rays the materials plan adds.` at `src/hittable.rs:10` and the comment `// For the triangle normals the geometry plan adds.` at `src/vec3.rs:21`. Keep both `#[allow(dead_code)]` attributes. Delete the comment lines only and change nothing else in either file.

10. Docs - no page edit. `README.md` stays accurate because the default output is still `render.png` and `cargo run` still writes it. `docs/requirements.md` item 12 already states every option, default and the usage behavior. `AGENT-INSTRUCTIONS/LOCAL/project.md` already names the command line in its `main.rs` description. `AGENT-INSTRUCTIONS/LOCAL/gates.md` already holds the option-driven rendered-image command, which works after this phase. Read all four and leave them as written; report in the receipt that they are unchanged and accurate.

**Callers / wiring to update:**
- `src/main.rs:81` - `main` calls `render(config.width, config.height, config.samples)` instead of `render(WIDTH, HEIGHT)`.
- `src/main.rs:97` and `src/main.rs:104` - the two existing tests gain the `1` sample argument.
- `src/main.rs:60` - `write_ppm` loses `#[allow(dead_code)]` and keeps its one caller in `main`'s PPM arm.
- `src/main.rs:3-11` - no module list change.
- `src/png.rs`, `src/camera.rs`, `src/vec3.rs`, `src/sphere.rs`, `src/ray.rs`, `src/hittable.rs` - no signature change; only `src/vec3.rs` and `src/hittable.rs` lose one comment each.
- No other file parses arguments or calls `render`: a repository-wide search finds the three calls this phase updates.

**Config / schema / migrations:**
- None. The five command-line options are runtime arguments, not stored configuration; no schema, migration or generated file is added.
- Retained records and their named consumer/owner: none. `Config` lives for one process run and is read only by `main`.

**Touched files:**
<!-- touched-files:start -->
- `src/main.rs` - changed: default size and sample constants, `Format`/`Config` and the hand parser, `render` takes and averages the sample count, `main` parses and dispatches PNG or PPM to the parsed path, `write_ppm` loses `#[allow(dead_code)]`, two tests adapted, three tests added.
- `src/hittable.rs` - changed: one plan-naming comment deleted above `point`.
- `src/vec3.rs` - changed: one plan-naming comment deleted above `cross`.
- Adds: nothing (no dependency, no config key, no migration, no generated file).
<!-- touched-files:end -->

**Rules / MUST NOT:**
- Do not add a dependency to `Cargo.toml` or call any crate outside `std`. Argument parsing is by hand.
- Do not add a progress line, a usage help flag (`--help`), `-h`, `-v`, subcommands or a configuration file. Those are not in requirement 12.
- Do not change `scene`, `ray_colour`, `Camera`, `to_byte`, `write_ppm`'s body, `png::write_png` or the render values. The scene and the shading are not this phase's.
- Do not add jitter, gamma correction, a random generator, or any scattering change to the averaging loop. The loop traces the camera ray it is given; the materials plan owns the sampling strategy.
- Do not validate anything beyond the four error cases in step 3. Do not reject a duplicate option, an unusually large number, or a path with no parent directory; the filesystem error from `File::create` propagates with `?`.
- Do not write the PNG and the PPM through two different output paths, two buffers, or two `File::create` calls. One `BufWriter` is opened once and one of the two writers fills it.
- Do not add a second render function, a second scene, or a `render_png`/`render_ppm` pair. `render` returns the colours and `main` picks the writer.
- Do not mention plans, phases or future work in source comments, module headers, the README or test names. Comments state what the code does now.
- Do not add `unsafe`, threads or seeded randomness.
- Do not touch `docs/requirements.md`, `README.md`, `AGENT-INSTRUCTIONS/LOCAL/project.md` or `AGENT-INSTRUCTIONS/LOCAL/gates.md`; verify they stay accurate.

**Tests:**
- New: `src/main.rs` - `the_defaults_are_400_by_200_with_16_samples`: `Config::parse(std::iter::empty::<String>())` returns `Ok` with `width == 400`, `height == 200`, `samples == 16`, `output == "render.png"` and `format == Format::Png`. This pins the five requirement defaults and the extension-derived default format.
- New: `src/main.rs` - `a_full_command_line_sets_every_option`: `Config::parse(vec!["--width".to_string(), "80".to_string(), "--height".to_string(), "40".to_string(), "--samples".to_string(), "4".to_string(), "--output".to_string(), "scene.ppm".to_string()].into_iter())` returns `Ok` with `width == 80`, `height == 40`, `samples == 4`, `output == "scene.ppm"` and `format == Format::Ppm`. This proves every option parses and the `.ppm` extension selects PPM.
- New: `src/main.rs` - `an_unknown_option_is_rejected`: `Config::parse(vec!["--bogus".to_string()].into_iter())` returns `Err`. The usage text and exit status are proved by the conditional gate below, not by capturing standard error in a unit test.
- Changed existing tests, no new functions: `src/main.rs` - `the_center_pixel_shows_the_sphere_facing_the_camera` and `the_top_row_is_sky` pass `1` as the sample count to `render`; their assertions do not change.
- Test budget: 3 new test functions, within the 3 allowed, with none deleted.
- Fast gate, run in this order: `cargo test`, `cargo build`, `cargo clippy -- -D warnings`, `cargo fmt --check`, `git diff --check`.
- Conditional gates: rendered image, required because the default size, the sample count and the output writer change. The implementer runs, once on the tested revision:
  1. `cargo run --release -- --width 80 --height 40 --samples 4 --output /tmp/opencode/render.png` exits 0, prints nothing on standard output and writes `/tmp/opencode/render.png`. Check it with `python3 -c "d=open('/tmp/opencode/render.png','rb').read();print(d[:8]==bytes([137,80,78,71,13,10,26,10]), d[16:24].hex())"`, which must print `True 0000005000000028`, and open the file with the image reader: the sphere sits over the sky gradient at 80 by 40.
  2. `cargo run --release -- --format ppm --width 40 --height 20 --output /tmp/opencode/render.ppm` exits 0, prints nothing on standard output, and writes a P3 file whose second line is `40 20`. `head -2 /tmp/opencode/render.ppm` must print `P3` and `40 20`.
  3. `cargo run --release -- --bogus` exits with status 2 and prints a line ending in `unknown option: --bogus` and the `usage:` line on standard error, with nothing written. `cargo run --release -- --width 0` likewise exits 2 and prints `--width must be at least 2`.
  4. Remove `/tmp/opencode/render.png` and `/tmp/opencode/render.ppm` after the checks.
- Slow gate (>2 minutes): none.

**User test (acceptance):**
- **Executor:** not required.
- Not required - automated phase gates prove acceptance. The three unit tests prove parsing, defaults and rejection, and step 1 to 3 of the rendered-image gate run the program with options, both formats and a bad option and observe the results.

**Completion checklist (gate):**
- [ ] Gate level requirements from `PLAN-PHASE-DETAILING.md` §3a are satisfied.
- [ ] `Config`/`Format` and the hand parser exist in `src/main.rs`; the five options, the defaults 400 by 200, 16 samples, `render.png` and the extension-derived format match requirement 12.
- [ ] A missing value, a non-numeric value, an unknown option, a width or height below 2, a sample count below 1, and an output path with no `png`/`ppm` extension each print an error and the usage line and exit with status 2.
- [ ] `render(width, height, samples)` averages the sample count and keeps the row order and pixel mapping of the previous `render`.
- [ ] `main` opens the parsed output path once, writes a PNG through `png::write_png` or a PPM through `write_ppm`, and flushes once.
- [ ] `write_ppm` is called by `main`, keeps its body and no longer carries `#[allow(dead_code)]`.
- [ ] `Cargo.toml` still has no dependency.
- [ ] Named tests pass.
- [ ] Test budget respected (REPO-RULES §3a); the receipt reports 3 functions added, 0 deleted and 2 existing tests adapted.
- [ ] Fast phase gate and the rendered-image conditional gate pass, including both formats and the usage exit.
- [ ] `README.md`, `docs/requirements.md`, `AGENT-INSTRUCTIONS/LOCAL/project.md` and `AGENT-INSTRUCTIONS/LOCAL/gates.md` were read and stay accurate; no page refers to a plan.
- [ ] Both carry-over comments are deleted and no comment names a plan or a later phase.
- [ ] Safety commit.
- [ ] Revision-bound verification receipt exists with the rendered-image observations.
- [ ] On passing evidence: compress this file into `done/C2-command-line.md` (frozen contract only) and flip the `full-plan.md` status row to `done`.

**Exit:** `cargo run -- --width W --height H --samples S --output PATH --format png|ppm` renders the fixed scene at the given size and sample count and writes a PNG or a PPM to `PATH`; a bad option or value prints the usage line and exits non-zero; the defaults are 400 by 200, 16 samples and `render.png`.
