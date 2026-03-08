extends Node2D

const CAMPANHA_SCENE_PATH := "res://scenes/modo_campanha.tscn"
const MENU_SCENE_PATH := "res://MenuPrincipal.tscn"
const VITORIA_SCENE_PATH := "res://scenes/tela_vitoria.tscn"
const DERROTA_SCENE_PATH := "res://scenes/tela_derrota.tscn"

@onready var controlador: Node = $ControladorBatalha

func _ready() -> void:
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

	if vitoria:
		CampaignState.registrar_vitoria()

		if CampaignState.vitorias >= 3 or CampaignState.campanha_concluida:
			get_tree().change_scene_to_file(VITORIA_SCENE_PATH)
		else:
			get_tree().change_scene_to_file(CAMPANHA_SCENE_PATH)
		return

	CampaignState.registrar_derrota()
	get_tree().change_scene_to_file(DERROTA_SCENE_PATH)
