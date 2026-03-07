use godot::classes::{Label, Node, TileMapLayer};
use godot::global::{HorizontalAlignment, VerticalAlignment};
use godot::prelude::*;

use crate::domain::tabuleiro::BOARD_SIZE;

const TILE_SIZE: f32 = 16.0;
const FONT_SIZE: i32 = 12;
const OUTLINE_SIZE: i32 = 2;

pub fn gerar_coordenadas(mut tilemap: Gd<TileMapLayer>) {
    criar_labels_numericos(&mut tilemap);
    criar_labels_alfabeticos(&mut tilemap);
}

fn criar_labels_numericos(tilemap: &mut Gd<TileMapLayer>) {
    for i in 0..BOARD_SIZE {
        let texto = format!("{}", i + 1);
        let posicao = Vector2::new((i as f32) * TILE_SIZE, -TILE_SIZE);
        adicionar_label(tilemap, &texto, posicao);
    }
}

fn criar_labels_alfabeticos(tilemap: &mut Gd<TileMapLayer>) {
    let letras = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
    for (i, letra) in letras.iter().enumerate() {
        let posicao = Vector2::new(-TILE_SIZE, (i as f32) * TILE_SIZE);
        adicionar_label(tilemap, letra, posicao);
    }
}

fn adicionar_label(tilemap: &mut Gd<TileMapLayer>, texto: &str, posicao: Vector2) {
    let mut label = Label::new_alloc();
    label.set_text(texto);
    label.set_custom_minimum_size(Vector2::new(TILE_SIZE, TILE_SIZE));
    label.set_horizontal_alignment(HorizontalAlignment::CENTER);
    label.set_vertical_alignment(VerticalAlignment::CENTER);
    
    label.add_theme_color_override("font_color", Color::from_rgb(1.0, 1.0, 1.0));
    label.add_theme_color_override("font_outline_color", Color::from_rgb(0.0, 0.0, 0.0));
    label.add_theme_constant_override("outline_size", OUTLINE_SIZE);
    label.add_theme_font_size_override("font_size", FONT_SIZE);
    
    label.set_position(posicao);
    tilemap.add_child(&label.upcast::<Node>());
}
