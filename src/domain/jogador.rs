use crate::domain::disparo::{executar_disparo, RetornoDisparo};
use crate::domain::tabuleiro::EstadoTabuleiro;

pub struct Jogador {
    tabuleiro: EstadoTabuleiro,
}

impl Jogador {
    pub fn novo_humano() -> Self {
        Self {
            tabuleiro: EstadoTabuleiro::vazio(),
        }
    }

    pub fn novo_ia() -> Self {
        Self {
            tabuleiro: EstadoTabuleiro::vazio(),
        }
    }

    pub fn tabuleiro(&self) -> &EstadoTabuleiro {
        &self.tabuleiro
    }

    pub fn tabuleiro_mut(&mut self) -> &mut EstadoTabuleiro {
        &mut self.tabuleiro
    }

    pub fn receber_disparo(&mut self, x: usize, y: usize) -> RetornoDisparo {
        executar_disparo(&mut self.tabuleiro, x, y)
    }

    pub fn perdeu(&self) -> bool {
        !self.tabuleiro.navios.is_empty() && self.tabuleiro.navios.iter().all(|n| n.esta_afundado())
    }
}
