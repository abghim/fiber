Fiber: Self-contained LaTeX Editor with Tectonic

Under development. I had to add these flags to have cargo recongize external dependencies; my Mac fails to build with vendored versions of the `tectonic` crate's dependencies. Add these to .zshrc (or run once via environment variables).
Install the packages in the first export line with `brew`.
```
export PKG_CONFIG_PATH="$(brew --prefix harfbuzz)/lib/pkgconfig:$(brew --prefix icu4c)/lib/pkgconfig:$(brew --prefix graphite2)/lib/pkgconfig:$(brew --prefix freetype)/lib/pkgconfig:$(brew --prefix libpng)/lib/pkgconfig:${PKG_CONFIG_PATH}"
export CPPFLAGS="-I$(brew --prefix harfbuzz)/include ${CPPFLAGS}"
export C_INCLUDE_PATH="$(brew --prefix harfbuzz)/include:${C_INCLUDE_PATH}"
export CPATH="$(brew --prefix harfbuzz)/include:${CPATH}"
export CFLAGS_aarch64_apple_darwin="-I$(brew --prefix harfbuzz)/include ${CFLAGS_aarch64_apple_darwin}"
export CXXFLAGS="-std=c++17 ${CXXFLAGS}"
```
