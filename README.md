# Fiber: Self-contained LaTeX Editor with Tectonic

Fiber gets the LaTeX editing & in-editor preview done, but without the 7GB+ bloat of TeX live.

## Building
Please make sure `harfbuzz` and `icu4c` dynamic dependencies are installed on your system! You may have to ensure `cargo` recognizes these paths with environment variables (particularly with macOS & homebrew).

Simply run
```
cargo b --release
```
to find the program under `target/release/fiber`. Packaging options for macOS are available with `cargo package`.
