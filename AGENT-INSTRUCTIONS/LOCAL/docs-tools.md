The toy raytracer has no documentation checker, charts, captures or published pages. It renders an
SVG to a PNG, which the diagram rules and the logo guide need, and checks SVG syntax.

```bash
# Render one SVG to a PNG at a pixel size, outside the repository.
/tmp/opencode/bin/render-svg <svg file> <png file> <width> <height>

xmllint --noout <svg file>
```

Previews go under `/tmp/opencode/`, outside the run.
