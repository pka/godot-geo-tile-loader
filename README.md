# godot-geo-tile-loader

A Rust-based Godot 4 extension, using the [gdextension crate](https://github.com/godot-rust/gdextension), 
for loading Mapbox Vector Tiles.


## Installation

Download the latest build artifact `godot-geo-tile-loader-plugin` from 
https://github.com/pka/godot-geo-tile-loader/actions and unzip it in the base 
directory of your Godot project.


## Usage

See the GDScript files in `test` as an example.


## Plugin developement

First, make sure to follow the setup instructions for [the gdextension crate](https://github.com/godot-rust/gdextension)
`cd rust`

`cargo build`

Or for a release build:

`cargo build --release`
