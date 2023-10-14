# godot-geo-tile-loader

A Rust-based Godot 4 extension, using [the gdextension crate](https://github.com/godot-rust/gdextension), for loading Mapbox Vector Tiles.

# Installation

Create a directory `addons/geo-tile-loader`

Download the latest build artifact from https://github.com/pka/godot-geo-tile-loader/actions

Unpack zip file in `addons/geo-tile-loader`.

Create a file called `GeoTileLoader.gdextension` next to it that looks something like this:

```
[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1

[libraries]
linux.x86_64 = "libgodot_mvt.so"
windows.x86_64 = "godot_mvt.dll"
macos = "libgodot_mvt.dylib"
```

# Building

First, make sure to follow the setup instructions for [the gdextension crate](https://github.com/godot-rust/gdextension). At the time of this writing, that includes setting the `GODOT4_BIN` environment variable to point to your Godot4 binary.
Then...

`cd rust`

`cargo build`

Or for a release build:

`cargo build --release`


After building, copy `godot_mvt.dll` (or `.dylib` or `.so`) from `./target/release/` into addons directory.
