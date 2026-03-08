use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usuario {
    pub id: u64,
    pub nome: String,
    pub login: String,
    pub senha_hash: String,
    pub jogos_totais: usize,
    pub vitorias: usize,
    pub derrotas: usize
}

impl Usuario {

    pub fn novo_usuario (
        id: u64,
        nome: String,
        login: String,
        senha: String
    ) -> Self {

        Self {
            id,
            nome,
            login,
            senha_hash: senha,
            jogos_totais: 0,
            vitorias: 0,
            derrotas: 0
        }
    }

    pub fn registrar_vitoria(&mut self) {
        self.vitorias += 1;
        self.jogos_totais += 1;
    }

    pub fn registrar_derrota(&mut self) {
        self.derrotas += 1;
        self.jogos_totais += 1;
    }

    pub fn taxa_de_vitoria(&self) -> f32 {
        if self.jogos_totais == 0 {
            return 0.0;
        }
        self.vitorias as f32 / self.jogos_totais as f32
    }
}