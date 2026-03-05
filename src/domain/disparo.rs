use crate::domain::tabuleiro::EstadoTabuleiro;

pub enum ResultadoDisparo {
    Agua,
    Acerto,
    JaDisparado,
    ForaDosLimites,
}

pub struct RetornoDisparo {
    pub resultado: ResultadoDisparo,
    pub mensagem: String,
}

pub fn executar_disparo(
    tabuleiro: &mut EstadoTabuleiro,
    x: usize,
    y: usize,
) -> RetornoDisparo {
    let Some(valor_celula) = tabuleiro.valor_celula(x, y) else {
        return RetornoDisparo {
            resultado: ResultadoDisparo::ForaDosLimites,
            mensagem: format!("Disparo fora dos limites em [{}, {}].", x, y),
        };
    };

    match valor_celula {
        0 => {
            let _ = tabuleiro.definir_celula(x, y, 2);
            RetornoDisparo {
                resultado: ResultadoDisparo::Agua,
                mensagem: format!("Errou! Água em [{}, {}]", x, y),
            }
        }
        1 => {
            let _ = tabuleiro.definir_celula(x, y, 3);
            RetornoDisparo {
                resultado: ResultadoDisparo::Acerto,
                mensagem: format!("FOGO! Navio atingido em [{}, {}]", x, y),
            }
        }
        _ => {
            RetornoDisparo {
                resultado: ResultadoDisparo::JaDisparado,
                mensagem: format!("Você já atirou aqui em [{}, {}]!", x, y),
            }
        }
    }
}
