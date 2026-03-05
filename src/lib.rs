use godot::prelude::*;

mod application;
mod domain;
mod presentation;

struct BatalhaNavalExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BatalhaNavalExtension {}
