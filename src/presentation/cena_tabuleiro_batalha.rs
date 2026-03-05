use godot::prelude::*;
use godot::classes::{Node, Node2D, INode2D, TileMap, Label, Input, Sprite2D};
use godot::global::{HorizontalAlignment, VerticalAlignment};

use crate::domain::tabuleiro::BOARD_SIZE;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct CenaTabuleiroBatalha {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for CenaTabuleiroBatalha {
    fn init(base: Base<Node2D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("Cena carregada! Gerando coordenadas visuais...");

        let tilemap_jogador = self.base().get_node_as::<TileMap>("HBoxContainer/AreaJogador/TileMap");
        let tilemap_ia = self.base().get_node_as::<TileMap>("HBoxContainer/AreaIA/TileMap");

        self.gerar_coordenadas(tilemap_jogador);
        self.gerar_coordenadas(tilemap_ia);
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
        let mouse_pos = self.base().get_global_mouse_position();
        
        let tilemap_player = self.base().get_node_as::<TileMap>("HBoxContainer/AreaJogador/TileMap");
        
        let mut cursor = self.base().get_node_as::<Sprite2D>("HBoxContainer/AreaJogador/TileMap/Cursor");

        let local_pos = tilemap_player.to_local(mouse_pos);
        let map_pos = tilemap_player.local_to_map(local_pos);

        if map_pos.x >= 0
            && map_pos.x < BOARD_SIZE as i32
            && map_pos.y >= 0
            && map_pos.y < BOARD_SIZE as i32
        {
            cursor.set_visible(true);
            
            let tamanho_tile = 16.0;
            let pos_x = (map_pos.x as f32) * tamanho_tile;
            let pos_y = (map_pos.y as f32) * tamanho_tile;
            
            cursor.set_position(Vector2::new(pos_x, pos_y));
            
            if input.is_action_just_pressed("clique_esquerdo") {
                godot_print!("Clique resgistrado. Posição válida: [{}, {}]", map_pos.x, map_pos.y);
            }
            
        } else {
            cursor.set_visible(false);
        }
    }
}

#[godot_api]
impl CenaTabuleiroBatalha {
    fn gerar_coordenadas(&self, mut tilemap: Gd<TileMap>) {
    let tamanho_tile = 16.0; 

    // Números (Topo: 1 a 10)
    for i in 0..BOARD_SIZE {
        let mut label = Label::new_alloc();
        let texto = format!("{}", i + 1);
        label.set_text(&texto); 
        
        // --- NOVO: Centralização e Tamanho Fixo ---
        label.set_custom_minimum_size(Vector2::new(tamanho_tile, tamanho_tile));
        label.set_horizontal_alignment(HorizontalAlignment::CENTER);
        label.set_vertical_alignment(VerticalAlignment::CENTER);
        
        // Posicionamos exatamente em cima do quadrado
        label.set_position(Vector2::new((i as f32) * tamanho_tile, -tamanho_tile));
        tilemap.add_child(&label.upcast::<Node>()); 
    }

    // Letras (Esquerda: A até J)
    let letras = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
    for (i, letra) in letras.iter().enumerate() {
        let mut label = Label::new_alloc();
        label.set_text(*letra); 
        
        // --- NOVO: Centralização e Tamanho Fixo ---
        label.set_custom_minimum_size(Vector2::new(tamanho_tile, tamanho_tile));
        label.set_horizontal_alignment(HorizontalAlignment::CENTER);
        label.set_vertical_alignment(VerticalAlignment::CENTER);

        label.set_position(Vector2::new(-tamanho_tile, (i as f32) * tamanho_tile));
        tilemap.add_child(&label.upcast::<Node>());
    }
    }
}
