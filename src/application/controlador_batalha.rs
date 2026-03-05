use godot::classes::{INode2D, InputEvent, InputEventMouseButton, Node2D, TileMapLayer};
use godot::global::MouseButton;
use godot::prelude::*;

use crate::domain::disparo::{ResultadoDisparo, executar_disparo};
use crate::domain::tabuleiro::{BOARD_SIZE, EstadoTabuleiro};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct ControladorBatalha {
    #[allow(dead_code)]
    tabuleiro_jogador: EstadoTabuleiro,
    tabuleiro_inimigo: EstadoTabuleiro,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for ControladorBatalha {
    fn init(base: Base<Node2D>) -> Self {
        let tabuleiro_jogador = EstadoTabuleiro::vazio();
        let mut tabuleiro_inimigo = EstadoTabuleiro::vazio();
        let _ = tabuleiro_inimigo.posicionar_navio(2, 2);

        Self {
            tabuleiro_jogador,
            tabuleiro_inimigo,
            base,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(mouse_event) = event.try_cast::<InputEventMouseButton>() {
            if mouse_event.is_pressed() && mouse_event.get_button_index() == MouseButton::LEFT {
                let click_pos = mouse_event.get_global_position();

                if let Some(mut enemy_map) = self.base().try_get_node_as::<TileMapLayer>("CampoIA")
                {
                    let local_pos = enemy_map.to_local(click_pos);
                    let map_coord = enemy_map.local_to_map(local_pos);

                    if map_coord.x >= 0
                        && map_coord.x < BOARD_SIZE as i32
                        && map_coord.y >= 0
                        && map_coord.y < BOARD_SIZE as i32
                    {
                        let x = map_coord.x as usize;
                        let y = map_coord.y as usize;
                        let retorno_disparo = executar_disparo(&mut self.tabuleiro_inimigo, x, y);

                        godot_print!("{}", retorno_disparo.mensagem);

                        match retorno_disparo.resultado {
                            ResultadoDisparo::Agua => {
                                enemy_map
                                    .set_cell_ex(map_coord)
                                    .source_id(0)
                                    .atlas_coords(Vector2i::new(8, 3))
                                    .done();
                            }
                            ResultadoDisparo::Acerto => {
                                enemy_map
                                    .set_cell_ex(map_coord)
                                    .source_id(0)
                                    .atlas_coords(Vector2i::new(10, 3))
                                    .done();
                            }
                            ResultadoDisparo::JaDisparado => {}
                            ResultadoDisparo::ForaDosLimites => {}
                        }
                    }
                } else {
                    godot_print!("Erro: Nó 'CampoIA' não encontrado!");
                }
            }
        }
    }
}
