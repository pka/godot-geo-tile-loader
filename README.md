# godot-geo-tile-loader

A Rust-based Godot 4 extension, using [the gdextension crate](https://github.com/godot-rust/gdextension), for loading Mapbox Vector Tiles.

# Building

First, make sure to follow the setup instructions for [the gdextension crate](https://github.com/godot-rust/gdextension). At the time of this writing, that includes setting the `GODOT4_BIN` environment variable to point to your Godot4 binary.
Then...

`cd rust`

`cargo build`

Or for a release build:

`cargo build --release`

# Installation

Create a directory `addons/geo-tile-loader`

After building, copy `godot_eqloader.dll` (or `.dylib` or `.so`) from `./target/release/` into addons directory.

Create a file called `GeoTileLoader.gdextension` next to it that looks something like this:

```
[configuration]
entry_symbol = "gdext_rust_init"

[libraries]
linux.release.x86_64 = "res://./libgodot_mvt.so"
windows.release.x86_64 = "res://./godot_mvt.dll"
macos.release = "res://./libgodot_mvt.dylib"
macos.release.arm64 = "res://./libgodot_mvt.dylib"
```
