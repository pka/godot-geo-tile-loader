pub mod mvt_commands;
pub mod vector_tile;

use godot::prelude::*;

struct MvtExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MvtExtension {}
