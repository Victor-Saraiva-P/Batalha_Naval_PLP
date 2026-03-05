pub const BOARD_SIZE: usize = 10;

pub struct EstadoTabuleiro {
    cells: [[u8; BOARD_SIZE]; BOARD_SIZE],
}

impl EstadoTabuleiro {
    pub fn vazio() -> Self {
        Self {
            cells: [[0; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn posicionar_navio(&mut self, x: usize, y: usize) -> bool {
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            return false;
        }

        self.cells[x][y] = 1;
        true
    }

    pub(crate) fn valor_celula(&self, x: usize, y: usize) -> Option<u8> {
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            return None;
        }

        Some(self.cells[x][y])
    }

    pub(crate) fn definir_celula(&mut self, x: usize, y: usize, valor: u8) -> bool {
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            return false;
        }

        self.cells[x][y] = valor;
        true
    }
}
