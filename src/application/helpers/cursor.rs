use godot::classes::{Sprite2D, TileMapLayer};
use godot::prelude::*;

use crate::domain::tabuleiro::BOARD_SIZE;

pub fn controlar_cursor(campo: Gd<TileMapLayer>, mouse_pos: Vector2) {
    let Some(mut cursor) = campo.try_get_node_as::<Sprite2D>("Sprite2D") else {
        return;
    };

    let local_pos = campo.to_local(mouse_pos);
    let map_pos = campo.local_to_map(local_pos);

    if esta_dentro_do_tabuleiro(map_pos) {
        cursor.set_visible(true);
        let cursor_pos = campo.map_to_local(map_pos);
        cursor.set_position(cursor_pos);
    } else {
        cursor.set_visible(false);
    }
}

pub fn esconder_cursor(campo: Gd<TileMapLayer>) {
    if let Some(mut cursor) = campo.try_get_node_as::<Sprite2D>("Sprite2D") {
        cursor.set_visible(false);
    }
}

fn esta_dentro_do_tabuleiro(map_pos: Vector2i) -> bool {
    map_pos.x >= 0 
        && map_pos.x < BOARD_SIZE as i32 
        && map_pos.y >= 0 
        && map_pos.y < BOARD_SIZE as i32
}
