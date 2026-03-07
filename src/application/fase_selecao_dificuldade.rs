use godot::global::Key;
use godot::prelude::*;
use crate::domain::jogador_ia::JogadorIA;

pub struct FaseSelecaoDificuldade;

impl FaseSelecaoDificuldade {
    pub fn nova() -> Self {
        Self
    }

    pub fn texto_tooltip(&self) -> &'static str {
        "Selecione a dificuldade:\n1 - Fácil\n2 - Médio\n3 - Difícil"
    }

    pub fn processar_tecla(&self, tecla: Key) -> Option<JogadorIA> {
        match tecla {
            Key::KEY_1 => {
                godot_print!("Dificuldade selecionada: Fácil");
                Some(JogadorIA::novo_facil())
            }
            Key::KEY_2 => {
                godot_print!("Dificuldade selecionada: Médio");
                Some(JogadorIA::novo_intermediario())
            }
            Key::KEY_3 => {
                godot_print!("Dificuldade selecionada: Difícil");
                Some(JogadorIA::novo_dificil())
            }
            _ => None,
        }
    }
}
