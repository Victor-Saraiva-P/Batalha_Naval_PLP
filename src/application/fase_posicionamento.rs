use crate::domain::jogador::Jogador;
use crate::domain::tabuleiro::{FROTA_PADRAO, BOARD_SIZE};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewPosicionamento {
    pub celulas: Vec<(usize, usize)>,
    pub valido: bool,
}

pub struct FasePosicionamento {
    fila_navios: Vec<(String, usize)>,
    indice_atual: usize,
    orientacao_horizontal: bool,
    modo_edicao: bool,
}

impl FasePosicionamento {
    pub fn nova() -> Self {
        let mut fila_navios = Vec::new();
        for config in FROTA_PADRAO.iter() {
            for _ in 0..config.quantidade {
                fila_navios.push((config.nome.to_string(), config.tamanho));
            }
        }

        Self {
            fila_navios,
            indice_atual: 0,
            orientacao_horizontal: true,
            modo_edicao: false,
        }
    }

    pub fn alternar_orientacao(&mut self) {
        self.orientacao_horizontal = !self.orientacao_horizontal;
    }

    pub fn orientacao_texto(&self) -> &'static str {
        if self.orientacao_horizontal {
            "Horizontal"
        } else {
            "Vertical"
        }
    }

    pub fn navio_atual(&self) -> Option<(&str, usize)> {
        self.fila_navios
            .get(self.indice_atual)
            .map(|(nome, tamanho)| (nome.as_str(), *tamanho))
    }

    fn ajustar_coordenada_para_centro(&self, x: usize, y: usize, tamanho: usize) -> (usize, usize) {
        let offset = tamanho / 2;
        if self.orientacao_horizontal {
            (x, y.saturating_sub(offset))
        } else {
            (x.saturating_sub(offset), y)
        }
    }

    pub fn preview_na_posicao(
        &self,
        jogador: &Jogador,
        x: usize,
        y: usize,
    ) -> Option<PreviewPosicionamento> {
        let (_, tamanho) = self.navio_atual()?;

        let (start_x, start_y) = self.ajustar_coordenada_para_centro(x, y, tamanho);
        let mut celulas = Vec::with_capacity(tamanho);

        for i in 0..tamanho {
            let (cx, cy) = if self.orientacao_horizontal {
                (start_x as i32, start_y as i32 + i as i32)
            } else {
                (start_x as i32 + i as i32, start_y as i32)
            };

            if cx >= 0 && cy >= 0 && cx < BOARD_SIZE as i32 && cy < BOARD_SIZE as i32 {
                celulas.push((cx as usize, cy as usize));
            }
        }

        let valido = celulas.len() == tamanho
            && jogador
                .tabuleiro()
                .pode_posicionar_navio(start_x, start_y, tamanho, self.orientacao_horizontal);

        Some(PreviewPosicionamento { celulas, valido })
    }

    pub fn tentar_posicionar_navio(
        &mut self,
        jogador: &mut Jogador,
        x: usize,
        y: usize,
    ) -> Result<bool, String> {
        let Some((nome, tamanho)) = self.fila_navios.get(self.indice_atual).cloned() else {
            return Ok(true);
        };

        let (start_x, start_y) = self.ajustar_coordenada_para_centro(x, y, tamanho);

        jogador
            .tabuleiro_mut()
            .posicionar_navio(&nome, start_x, start_y, tamanho, self.orientacao_horizontal)?;

        self.indice_atual += 1;
        Ok(self.terminou())
    }

    pub fn terminou(&self) -> bool {
        self.indice_atual >= self.fila_navios.len()
    }

    pub fn todos_posicionados(&self) -> bool {
        // No modo edição, verificar se a fila está vazia
        if self.modo_edicao {
            return self.fila_navios.is_empty();
        }
        // Fora do modo edição, verificar o índice como antes
        self.indice_atual >= self.fila_navios.len()
    }

    pub fn em_modo_edicao(&self) -> bool {
        self.modo_edicao
    }

    pub fn ativar_modo_edicao(&mut self) {
        if self.terminou() {
            self.modo_edicao = true;
            // Limpar a fila - não é mais necessária após posicionamento inicial
            self.fila_navios.clear();
            self.indice_atual = 0;
        }
    }

    pub fn desativar_modo_edicao(&mut self) {
        self.modo_edicao = false;
    }

    pub fn remover_navio(&mut self, nome: &str) -> bool {
        if !self.modo_edicao {
            return false;
        }

        // Encontrar o navio na frota original para pegar o tamanho
        let tamanho_navio = FROTA_PADRAO
            .iter()
            .find(|config| config.nome == nome)
            .map(|config| config.tamanho);

        if let Some(tamanho) = tamanho_navio {
            // Adicionar o navio removido à fila (que deve estar vazia)
            self.fila_navios.push((nome.to_string(), tamanho));
            // Índice já está em 0, pronto para posicionar este navio
            
            self.desativar_modo_edicao();
            return true;
        }

        false
    }
}
