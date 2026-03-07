use godot::classes::RandomNumberGenerator;
use godot::prelude::*;

use crate::domain::disparo::RetornoDisparo;
use crate::domain::jogador::Jogador;
use crate::domain::tabuleiro::{Celula, EstadoTabuleiro, BOARD_SIZE};

pub trait EstrategiaIA {
    fn escolher_alvo(&mut self, tabuleiro_inimigo: &EstadoTabuleiro) -> Option<(usize, usize)>;
}

pub struct EstrategiaFacil;

impl EstrategiaIA for EstrategiaFacil {
    fn escolher_alvo(&mut self, tabuleiro_inimigo: &EstadoTabuleiro) -> Option<(usize, usize)> {
        let mut alvos_disponiveis = Vec::new();

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if let Some(celula) = tabuleiro_inimigo.valor_celula(x, y) {
                    if matches!(celula, Celula::Vazio | Celula::Ocupado(_)) {
                        alvos_disponiveis.push((x, y));
                    }
                }
            }
        }

        if alvos_disponiveis.is_empty() {
            return None;
        }

        let mut rng = RandomNumberGenerator::new_gd();
        rng.randomize();
        let idx = rng.randi_range(0, (alvos_disponiveis.len() - 1) as i32) as usize;
        Some(alvos_disponiveis[idx])
    }
}

pub struct JogadorIA {
    jogador: Jogador,
    estrategia: Box<dyn EstrategiaIA>,
}

impl JogadorIA {
    pub fn novo(estrategia: Box<dyn EstrategiaIA>) -> Self {
        Self {
            jogador: Jogador::novo_ia(),
            estrategia,
        }
    }

    pub fn novo_facil() -> Self {
        Self::novo(Box::new(EstrategiaFacil))
    }

    pub fn jogador_mut(&mut self) -> &mut Jogador {
        &mut self.jogador
    }

    pub fn escolher_alvo(&mut self, tabuleiro_inimigo: &EstadoTabuleiro) -> Option<(usize, usize)> {
        self.estrategia.escolher_alvo(tabuleiro_inimigo)
    }

    pub fn receber_disparo(&mut self, x: usize, y: usize) -> RetornoDisparo {
        self.jogador.receber_disparo(x, y)
    }

    pub fn perdeu(&self) -> bool {
        self.jogador.perdeu()
    }
}
