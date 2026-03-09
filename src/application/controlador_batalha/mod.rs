use godot::classes::{
    INode2D, Input, InputEvent, InputEventKey, InputEventMouseButton, Label, Node, Node2D,
    TileMapLayer,
};
use godot::global::{Key, MouseButton};
use godot::prelude::*;

use crate::application::fase_posicionamento::FasePosicionamento;
use crate::application::fase_selecao_dificuldade::FaseSelecaoDificuldade;
use crate::application::gerenciador_audio::GerenciadorAudio;
use crate::application::gerenciador_efeito::{posicao_global_tile, GerenciadorEfeito};
use crate::application::gerenciador_interface::GerenciadorInterface;
use crate::application::gerenciador_turnos::{EstadoTurno, GerenciadorTurnos};
use crate::application::helpers::{conversao_coordenadas, coordenadas, cursor};
use crate::domain::disparo::ResultadoDisparo;
use crate::domain::jogador::Jogador;
use crate::domain::jogador_ia::JogadorIA;
use crate::domain::tabuleiro::{Celula, BOARD_SIZE};
use crate::presentation::batalha::{
    limpar_preview, render_navio_afundado, render_preview_posicionamento, render_resultado_disparo,
    render_tabuleiro_jogador,
};

mod internal;

const DELAY_TURNO_IA: f64 = 1.0;

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct ControladorBatalha {
    jogador_humano: Jogador,
    jogador_ia: Option<JogadorIA>,
    fase_posicionamento: FasePosicionamento,
    fase_selecao_dificuldade: FaseSelecaoDificuldade,
    gerenciador_turnos: GerenciadorTurnos,
    gerenciador_interface: GerenciadorInterface,
    gerenciador_audio: GerenciadorAudio,
    gerenciador_efeito: GerenciadorEfeito,
    tempo_restante_ia: f64,
    estado_anterior: EstadoTurno,
    tooltip_instrucao: Option<Gd<Label>>,
    mapa_xray_ia: Option<Gd<TileMapLayer>>,
    mapa_xray_ia_navios: Option<Gd<TileMapLayer>>,
    label_xray_ia: Option<Gd<Label>>,
    mapa_navios_jogador: Option<Gd<TileMapLayer>>,
    mapa_navios_ia: Option<Gd<TileMapLayer>>,
    xray_ativo: bool,
    modo_dinamico: bool,
    navio_selecionado_movimento: Option<usize>,
    movimento_jogador_realizado: bool,
    tiros_jogador_no_tabuleiro_ia: [[bool; BOARD_SIZE]; BOARD_SIZE],
    tiros_ia_no_tabuleiro_jogador: [[bool; BOARD_SIZE]; BOARD_SIZE],
    resultado_final_emitido: bool,
    vitoria_registrada: Option<bool>,
    acertos_seguidos_atual: u32,
    max_acertos_seguidos: u32,
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
            jogador_ia: None,
            fase_posicionamento: FasePosicionamento::nova(),
            fase_selecao_dificuldade: FaseSelecaoDificuldade::nova(),
            gerenciador_turnos: GerenciadorTurnos::novo(total_navios),
            gerenciador_interface: GerenciadorInterface::novo(),
            gerenciador_audio: GerenciadorAudio::novo(),
            gerenciador_efeito: GerenciadorEfeito::novo(),
            tempo_restante_ia: 0.0,
            estado_anterior: EstadoTurno::SelecaoDificuldade,
            tooltip_instrucao: None,
            mapa_xray_ia: None,
            mapa_xray_ia_navios: None,
            label_xray_ia: None,
            mapa_navios_jogador: None,
            mapa_navios_ia: None,
            xray_ativo: false,
            modo_dinamico: false,
            navio_selecionado_movimento: None,
            movimento_jogador_realizado: false,
            tiros_jogador_no_tabuleiro_ia: [[false; BOARD_SIZE]; BOARD_SIZE],
            tiros_ia_no_tabuleiro_jogador: [[false; BOARD_SIZE]; BOARD_SIZE],
            resultado_final_emitido: false,
            vitoria_registrada: None,
            acertos_seguidos_atual: 0,
            max_acertos_seguidos: 0,
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

        self.gerenciador_interface.inicializar(self.base().clone());
        self.inicializar_xray_ia();
        self.inicializar_ship_layers();

        // Inicializar áudio com o nó base
        let node = self.base().clone().upcast::<Node>();
        self.gerenciador_audio.inicializar(&node);

        // Iniciar música e ondas desde o início (planejamento)
        self.gerenciador_audio.tocar_musica_batalha();
        self.gerenciador_audio.tocar_ondas();
    }

    fn process(&mut self, delta: f64) {
        // Atualizar interface primeiro, independente do estado
        self.gerenciador_interface.atualizar(
            self.gerenciador_turnos.estado_atual(),
            self.gerenciador_turnos.rodada_atual(),
        );

        // Controlar visibilidade do label de movimento dinâmico
        if self.modo_dinamico && self.gerenciador_turnos.estado_atual() == EstadoTurno::TurnoJogador
        {
            self.gerenciador_interface
                .mostrar_label_movimento_dinamico();
        } else {
            self.gerenciador_interface
                .esconder_label_movimento_dinamico();
        }

        if self.gerenciador_turnos.estado_atual() == EstadoTurno::SelecaoDificuldade {
            if let Some(campo_jogador) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador")
            {
                cursor::esconder_cursor(campo_jogador);
            }
            if let Some(campo_ia) = self.base().try_get_node_as::<TileMapLayer>("CampoIA") {
                cursor::esconder_cursor(campo_ia);
            }

            return;
        }

        self.atualizar_tooltip_posicionamento();

        if self.gerenciador_turnos.estado_atual() == EstadoTurno::PosicionamentoJogador {
            self.atualizar_preview_posicionamento();
            let input = Input::singleton();
            if input.is_action_just_pressed("rotacionar_navio") {
                self.fase_posicionamento.alternar_orientacao();
            }
        } else if self.modo_dinamico
            && self.gerenciador_turnos.estado_atual() == EstadoTurno::TurnoJogador
            && !self.movimento_jogador_realizado
            && self.navio_selecionado_movimento.is_some()
        {
            self.atualizar_preview_movimento_dinamico();
        } else {
            self.limpar_preview_posicionamento();
        }

        self.atualizar_controle_cursor();
        self.atualizar_xray_ia();

        // Processar delays de som e partículas de fumaça
        self.gerenciador_audio.processar_delays(delta);
        self.gerenciador_efeito.atualizar();

        // Detectar fim de jogo e tocar sons apropriados
        let estado_atual = self.gerenciador_turnos.estado_atual();
        if estado_atual != self.estado_anterior {
            match estado_atual {
                EstadoTurno::PosicionamentoJogador => {
                    godot_print!("Mudou para PosicionamentoJogador - populando container");
                    self.popular_container_navios();
                }
                EstadoTurno::TurnoJogador => {
                    self.movimento_jogador_realizado = false;
                }
                EstadoTurno::TurnoIA => {}
                EstadoTurno::VitoriaJogador => {
                    self.gerenciador_audio.tocar_vitoria();
                    self.emitir_resultado_final(true);
                    self.gerenciador_interface
                        .atualizar(estado_atual, self.gerenciador_turnos.rodada_atual());
                }
                EstadoTurno::VitoriaIA => {
                    self.gerenciador_audio.tocar_derrota();
                    self.emitir_resultado_final(false);
                    self.gerenciador_interface
                        .atualizar(estado_atual, self.gerenciador_turnos.rodada_atual());
                }
                _ => {}
            }
            self.estado_anterior = estado_atual;
        }

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

        if let Ok(key_event) = event.clone().try_cast::<InputEventKey>() {
            if key_event.is_pressed() && !key_event.is_echo() {
                match key_event.get_keycode() {
                    // F1 -> vitória instantânea (debug)
                    Key::F1 => {
                        self.vencer_teste();
                        return;
                    }
                    // F2 -> derrota instantânea (debug)
                    Key::F2 => {
                        self.perder_teste();
                        return;
                    }
                    // F3 -> alternar X-Ray
                    Key::F3 => {
                        self.alternar_xray();
                        return;
                    }
                    _ => {}
                }
            }
        }

        if self.gerenciador_turnos.estado_atual() == EstadoTurno::SelecaoDificuldade {
            if let Ok(key_event) = event.try_cast::<InputEventKey>() {
                if key_event.is_pressed() && !key_event.is_echo() {
                    let keycode = key_event.get_keycode();
                    if let Some(ia) = self.fase_selecao_dificuldade.processar_tecla(keycode) {
                        let mut ia = ia;
                        ia.configurar_modo_dinamico(self.modo_dinamico);
                        self.jogador_ia = Some(ia);
                        self.gerenciador_turnos.confirmar_dificuldade();
                    }
                }
            }
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
                    if self.modo_dinamico && !self.movimento_jogador_realizado {
                        if self.tratar_clique_movimento_jogador(click_pos) {
                            return;
                        }
                    }
                    self.tratar_clique_disparo_jogador(click_pos);
                }
                _ => {}
            }
        }
    }
}

#[godot_api]
impl ControladorBatalha {
    #[signal]
    fn batalha_encerrada(vitoria: bool);

    #[func]
    pub fn selecionar_dificuldade_facil(&mut self) {
        godot_print!("selecionar_dificuldade_facil chamado");
        if self.gerenciador_turnos.estado_atual() == EstadoTurno::SelecaoDificuldade {
            if let Some(ia) = self.fase_selecao_dificuldade.processar_selecao(0) {
                let mut ia = ia;
                ia.configurar_modo_dinamico(self.modo_dinamico);
                self.jogador_ia = Some(ia);
                self.gerenciador_turnos.confirmar_dificuldade();
                godot_print!(
                    "Dificuldade confirmada, estado agora: {:?}",
                    self.gerenciador_turnos.estado_atual()
                );
            }
        }
    }

    #[func]
    pub fn selecionar_dificuldade_media(&mut self) {
        godot_print!("selecionar_dificuldade_media chamado");
        if self.gerenciador_turnos.estado_atual() == EstadoTurno::SelecaoDificuldade {
            if let Some(ia) = self.fase_selecao_dificuldade.processar_selecao(1) {
                let mut ia = ia;
                ia.configurar_modo_dinamico(self.modo_dinamico);
                self.jogador_ia = Some(ia);
                self.gerenciador_turnos.confirmar_dificuldade();
                godot_print!(
                    "Dificuldade confirmada, estado agora: {:?}",
                    self.gerenciador_turnos.estado_atual()
                );
            }
        }
    }

    #[func]
    pub fn selecionar_dificuldade_dificil(&mut self) {
        godot_print!("selecionar_dificuldade_dificil chamado");
        if self.gerenciador_turnos.estado_atual() == EstadoTurno::SelecaoDificuldade {
            if let Some(ia) = self.fase_selecao_dificuldade.processar_selecao(2) {
                let mut ia = ia;
                ia.configurar_modo_dinamico(self.modo_dinamico);
                self.jogador_ia = Some(ia);
                self.gerenciador_turnos.confirmar_dificuldade();
                godot_print!(
                    "Dificuldade confirmada, estado agora: {:?}",
                    self.gerenciador_turnos.estado_atual()
                );
            }
        }
    }

    #[func]
    pub fn vencer_teste(&mut self) {
        if self.gerenciador_turnos.jogo_terminou() {
            return;
        }

        self.gerenciador_turnos.forcar_vitoria_jogador();
        self.gerenciador_audio.tocar_vitoria();
        self.emitir_resultado_final(true);
        self.gerenciador_interface.atualizar(
            EstadoTurno::VitoriaJogador,
            self.gerenciador_turnos.rodada_atual(),
        );
        self.estado_anterior = EstadoTurno::VitoriaJogador;
    }

    #[func]
    pub fn perder_teste(&mut self) {
        if self.gerenciador_turnos.jogo_terminou() {
            return;
        }

        self.gerenciador_turnos.forcar_vitoria_ia();
        self.gerenciador_audio.tocar_derrota();
        self.emitir_resultado_final(false);
        self.gerenciador_interface.atualizar(
            EstadoTurno::VitoriaIA,
            self.gerenciador_turnos.rodada_atual(),
        );
        self.estado_anterior = EstadoTurno::VitoriaIA;
    }

    #[func]
    pub fn confirmar_posicionamento(&mut self) {
        if self.fase_posicionamento.em_modo_edicao()
            && self.fase_posicionamento.todos_posicionados()
        {
            self.gerenciador_interface.esconder_botao_confirmar();
            self.iniciar_fase_batalha();
        }
    }

    #[func]
    pub fn continuar(&mut self) {
        if let Some(vitoria) = self.vitoria_registrada {
            self.base_mut()
                .emit_signal("batalha_encerrada", &[vitoria.to_variant()]);
        }
    }

    #[func]
    pub fn definir_modo_dinamico(&mut self, ativo: bool) {
        self.modo_dinamico = ativo;
        if !ativo {
            self.xray_ativo = false;
        }
        self.navio_selecionado_movimento = None;
        self.movimento_jogador_realizado = false;
        if let Some(ref mut ia) = self.jogador_ia {
            ia.configurar_modo_dinamico(ativo);
        }
    }

    #[func]
    pub fn obter_max_acertos_seguidos(&self) -> i32 {
        self.max_acertos_seguidos as i32
    }

    #[func]
    pub fn jogador_perdeu_algum_navio(&self) -> bool {
        self.jogador_humano
            .tabuleiro()
            .navios
            .iter()
            .any(|n| n.esta_afundado())
    }

    #[func]
    pub fn alternar_xray(&mut self) {
        self.xray_ativo = !self.xray_ativo;
        self.atualizar_xray_ia();
    }
    #[func]
    pub fn selecionar_navio_do_container(&mut self, indice: i32) {
        if indice < 0 {
            return;
        }

        if self.fase_posicionamento.selecionar_navio(indice as usize) {
            godot_print!("Navio {} selecionado", indice);
        }
    }

    #[func]
    pub fn obter_rodadas(&self) -> i32 {
        self.gerenciador_turnos.rodada_atual() as i32
    }
}
