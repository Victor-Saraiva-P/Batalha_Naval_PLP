extends Node2D

const CAMPANHA_SCENE_PATH := "res://scenes/modo_campanha.tscn"
const VITORIA_SCENE_PATH := "res://scenes/ui/tela_vitoria.tscn"
const DERROTA_SCENE_PATH := "res://scenes/ui/tela_derrota.tscn"

@onready var controlador: Node = $ControladorBatalha

func _ready() -> void:
	var modo_dinamico := CampaignState.modo_campanha == "dinamica"
	controlador.call("definir_modo_dinamico", modo_dinamico)

	if not CampaignState.em_campanha:
		return

	if controlador.has_signal("batalha_encerrada"):
		controlador.connect("batalha_encerrada", Callable(self, "_on_batalha_encerrada"))

	_call_forced_campaign_difficulty()

func _call_forced_campaign_difficulty() -> void:
	match CampaignState.vitorias:
		0:
			controlador.call("selecionar_dificuldade_facil")
		1:
			controlador.call("selecionar_dificuldade_media")
		2:
			controlador.call("selecionar_dificuldade_dificil")
		_:
			controlador.call("selecionar_dificuldade_dificil")

func _on_batalha_encerrada(vitoria: bool) -> void:
	if not CampaignState.em_campanha:
		return

	var usuario_node = UsuarioNode.new()
	var login_atual := SessionStore.ler_login_atual()

	if vitoria:
		CampaignState.registrar_vitoria()
		if login_atual != "":
			usuario_node.registrar_vitoria(login_atual)

			var rodadas = controlador.call("obter_rodadas")
			if rodadas <= 20:
				usuario_node.adicionar_conquista(login_atual, "Marinheiro")

			var max_acertos = controlador.call("obter_max_acertos_seguidos")

			if max_acertos >= 7:
				usuario_node.adicionar_conquista(login_atual, "Capitao")

			if max_acertos >= 8:
				usuario_node.adicionar_conquista(login_atual, "Capitao De Mar E Guerra")

			var perdeu_navio = controlador.call("jogador_perdeu_algum_navio")
			if not perdeu_navio:
				usuario_node.adicionar_conquista(login_atual, "Almirante")

		if CampaignState.vitorias >= 3 or CampaignState.campanha_concluida:
			get_tree().call_deferred("change_scene_to_file", VITORIA_SCENE_PATH)
		else:
			get_tree().call_deferred("change_scene_to_file", CAMPANHA_SCENE_PATH)
		return

	CampaignState.registrar_derrota()
	if login_atual != "":
		usuario_node.registrar_derrota(login_atual)

	get_tree().call_deferred("change_scene_to_file", DERROTA_SCENE_PATH)
