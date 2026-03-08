use godot::prelude::*;

mod application;
mod domain;
mod presentation;
mod infrastructure;

struct BatalhaNavalExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BatalhaNavalExtension {}