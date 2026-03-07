use godot::classes::{INode2D, Input, InputEvent, InputEventMouseButton, Label, Node2D, TileMapLayer};
use godot::global::MouseButton;
use godot::prelude::*;

use crate::application::fase_posicionamento::FasePosicionamento;
use crate::application::helpers::{conversao_coordenadas, coordenadas, cursor};
use crate::application::gerenciador_turnos::{GerenciadorTurnos, EstadoTurno};
use crate::domain::disparo::ResultadoDisparo;
use crate::domain::jogador::Jogador;
use crate::domain::jogador_ia::JogadorIA;
use crate::presentation::batalha::{
    limpar_preview, render_preview_posicionamento, render_resultado_disparo, render_tabuleiro_jogador,
};

const DELAY_TURNO_IA: f64 = 0.7;

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct ControladorBatalha {
    jogador_humano: Jogador,
    jogador_ia: JogadorIA,
    fase_posicionamento: FasePosicionamento,
    gerenciador_turnos: GerenciadorTurnos,
    tempo_restante_ia: f64,
    tooltip_instrucao: Option<Gd<Label>>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for ControladorBatalha {
    fn init(base: Base<Node2D>) -> Self {
        let total_navios: u32 = crate::domain::tabuleiro::FROTA_PADRAO
            .iter()
            .map(|config| config.quantidade as u32)
            .sum();

        Self {
            jogador_humano: Jogador::novo_humano(),
            jogador_ia: JogadorIA::novo_facil(),
            fase_posicionamento: FasePosicionamento::nova(),
            gerenciador_turnos: GerenciadorTurnos::novo(total_navios),
            tempo_restante_ia: 0.0,
            tooltip_instrucao: None,
            base,
        }
    }

    fn ready(&mut self) {
        if let Some(campo_jogador) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") {
            coordenadas::gerar_coordenadas(campo_jogador);
        }
        if let Some(campo_ia) = self.base().try_get_node_as::<TileMapLayer>("CampoIA") {
            coordenadas::gerar_coordenadas(campo_ia);
        }
    }

    fn process(&mut self, delta: f64) {
        self.atualizar_tooltip_posicionamento();
        
        if self.gerenciador_turnos.estado_atual() == EstadoTurno::PosicionamentoJogador {
            self.atualizar_preview_posicionamento();
            let input = Input::singleton();
            if input.is_action_just_pressed("rotacionar_navio") {
                self.fase_posicionamento.alternar_orientacao();
            }
        } else {
            self.limpar_preview_posicionamento();
        }

        self.atualizar_controle_cursor();

        if self.gerenciador_turnos.estado_atual() == EstadoTurno::TurnoIA {
            self.tempo_restante_ia -= delta;
            if self.tempo_restante_ia <= 0.0 {
                self.executar_turno_ia();
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.gerenciador_turnos.jogo_terminou() {
            return;
        }
        
        if let Ok(mouse_event) = event.try_cast::<InputEventMouseButton>() {
            if !mouse_event.is_pressed() || mouse_event.get_button_index() != MouseButton::LEFT {
                return;
            }
            let click_pos = self.base().get_global_mouse_position();
            
            match self.gerenciador_turnos.estado_atual() {
                EstadoTurno::PosicionamentoJogador => {
                    self.tratar_clique_posicionamento(click_pos);
                }
                EstadoTurno::TurnoJogador => {
                    self.tratar_clique_disparo_jogador(click_pos);
                }
                _ => {}
            }
        }
    }
}

impl ControladorBatalha {
    fn atualizar_tooltip_posicionamento(&mut self) {
        let Some(mut tooltip) = self.tooltip_instrucao.clone() else {
            return;
        };

        if self.gerenciador_turnos.estado_atual() != EstadoTurno::PosicionamentoJogador {
            tooltip.set_visible(false);
            return;
        }

        let Some((nome, tamanho)) = self.fase_posicionamento.navio_atual() else {
            tooltip.set_visible(false);
            return;
        };

        let texto = format!(
            "Posicione: {} ({})\nClique: posicionar | R: rotacionar ({})",
            nome,
            tamanho,
            self.fase_posicionamento.orientacao_texto()
        );

        tooltip.set_text(&texto);
        tooltip.set_visible(true);

        let mouse_pos_global = self.base().get_global_mouse_position();
        let mouse_pos_local = self.base().to_local(mouse_pos_global);
        tooltip.set_position(mouse_pos_local + Vector2::new(14.0, 14.0));
    }

    fn tratar_clique_posicionamento(&mut self, click_pos: Vector2) {
        let Some(player_map) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") else {
            return;
        };

        let Some((x, y, _)) = conversao_coordenadas::clique_para_coordenada(player_map, click_pos) else {
            return;
        };

        match self
            .fase_posicionamento
            .tentar_posicionar_navio(&mut self.jogador_humano, x, y)
        {
            Ok(concluiu) => {
                self.atualizar_visual_meu_campo();
                if concluiu {
                    self.iniciar_fase_batalha();
                }
            }
            Err(_) => {}
        }
    }

    fn iniciar_fase_batalha(&mut self) {
        self.gerenciador_turnos.finalizar_posicionamento_jogador();
        self.jogador_ia
            .jogador_mut()
            .tabuleiro_mut()
            .preencher_aleatoriamente();
        self.limpar_preview_posicionamento();
        self.gerenciador_turnos.iniciar_jogo();
    }

    fn atualizar_preview_posicionamento(&mut self) {
        let Some(player_map) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") else {
            return;
        };
        let Some(mut preview_map) = self
            .base()
            .try_get_node_as::<TileMapLayer>("PreviewPosicionamento")
        else {
            return;
        };

        let mouse_pos = self.base().get_global_mouse_position();
        let Some((x, y, _)) = conversao_coordenadas::clique_para_coordenada(player_map, mouse_pos) else {
            limpar_preview(&mut preview_map);
            return;
        };

        let Some((nome_navio, _)) = self.fase_posicionamento.navio_atual() else {
            limpar_preview(&mut preview_map);
            return;
        };

        let Some(preview) = self
            .fase_posicionamento
            .preview_na_posicao(&self.jogador_humano, x, y)
        else {
            limpar_preview(&mut preview_map);
            return;
        };
        
        render_preview_posicionamento(&mut preview_map, nome_navio, &preview.celulas, preview.valido);
    }

    fn limpar_preview_posicionamento(&mut self) {
        if let Some(mut preview_map) = self
            .base()
            .try_get_node_as::<TileMapLayer>("PreviewPosicionamento")
        {
            limpar_preview(&mut preview_map);
        }
    }

    fn tratar_clique_disparo_jogador(&mut self, click_pos: Vector2) {
        let Some(mut enemy_map) = self.base().try_get_node_as::<TileMapLayer>("CampoIA") else {
            return;
        };

        let Some((x, y, map_coord)) =
            conversao_coordenadas::clique_para_coordenada(enemy_map.clone(), click_pos)
        else {
            return;
        };

        let retorno = self.jogador_ia.receber_disparo(x, y);
        godot_print!("{}", retorno.mensagem);

        render_resultado_disparo(&mut enemy_map, map_coord, &retorno.resultado);

        if retorno.resultado.foi_valido() {
            let acertou = matches!(retorno.resultado, ResultadoDisparo::Acerto | ResultadoDisparo::Afundou(_));
            let afundou = matches!(retorno.resultado, ResultadoDisparo::Afundou(_));
            
            self.gerenciador_turnos.processar_ataque_jogador(acertou, afundou);
            
            if self.jogador_ia.perdeu() {
                return;
            }
            
            if !self.gerenciador_turnos.jogo_terminou() {
                self.tempo_restante_ia = DELAY_TURNO_IA;
            }
        }
    }

    fn executar_turno_ia(&mut self) {
        let Some((x, y)) = self
            .jogador_ia
            .escolher_alvo(self.jogador_humano.tabuleiro())
        else {
            return;
        };

        let retorno = self.jogador_humano.receber_disparo(x, y);
        godot_print!("IA: {}", retorno.mensagem);

        if let Some(mut player_map) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") {
            render_resultado_disparo(
                &mut player_map,
                Vector2i::new(y as i32, x as i32),
                &retorno.resultado,
            );
        }

        let acertou = matches!(retorno.resultado, ResultadoDisparo::Acerto | ResultadoDisparo::Afundou(_));
        let afundou = matches!(retorno.resultado, ResultadoDisparo::Afundou(_));
        
        self.gerenciador_turnos.processar_ataque_ia(acertou, afundou);
    }

    fn atualizar_controle_cursor(&mut self) {
        let mouse_pos = self.base().get_global_mouse_position();
        let estado = self.gerenciador_turnos.estado_atual();

        let (mostrar_jogador, mostrar_ia) = match estado {
            EstadoTurno::PosicionamentoJogador => (true, false),
            EstadoTurno::TurnoJogador => (false, true),
            _ => (false, false),
        };

        if let Some(campo_jogador) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") {
            if mostrar_jogador {
                cursor::controlar_cursor(campo_jogador, mouse_pos);
            } else {
                cursor::esconder_cursor(campo_jogador);
            }
        }

        if let Some(campo_ia) = self.base().try_get_node_as::<TileMapLayer>("CampoIA") {
            if mostrar_ia {
                cursor::controlar_cursor(campo_ia, mouse_pos);
            } else {
                cursor::esconder_cursor(campo_ia);
            }
        }
    }

    fn atualizar_visual_meu_campo(&mut self) {
        if let Some(mut player_map) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") {
            render_tabuleiro_jogador(&mut player_map, self.jogador_humano.tabuleiro());
        }
    }
}


