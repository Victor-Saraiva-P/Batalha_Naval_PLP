use crate::presentation::batalha::renderizacao_tabuleiro::atlas_tiles::ATLAS_NAVIO;

pub fn atlas_navio_por_nome(nome_navio: &str) -> (i32, i32) {
    match nome_navio {
        "Porta-Aviões" => ATLAS_NAVIO,
        "Navio de Guerra" => ATLAS_NAVIO,
        "Encouraçado" => ATLAS_NAVIO,
        "Submarino" => ATLAS_NAVIO,
        _ => ATLAS_NAVIO,
    }
}
