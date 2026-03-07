use godot::classes::TileMapLayer;
use godot::prelude::*;

use crate::domain::tabuleiro::BOARD_SIZE;

pub fn clique_para_coordenada(
    mapa: Gd<TileMapLayer>,
    posicao_clique: Vector2,
) -> Option<(usize, usize, Vector2i)> {
    let posicao_local = mapa.to_local(posicao_clique);
    let coordenada_mapa = mapa.local_to_map(posicao_local);

    if !esta_dentro_dos_limites(coordenada_mapa) {
        return None;
    }

    let linha = coordenada_mapa.y as usize;
    let coluna = coordenada_mapa.x as usize;

    Some((linha, coluna, coordenada_mapa))
}

fn esta_dentro_dos_limites(coordenada: Vector2i) -> bool {
    coordenada.x >= 0
        && coordenada.y >= 0
        && coordenada.x < BOARD_SIZE as i32
        && coordenada.y < BOARD_SIZE as i32
}
