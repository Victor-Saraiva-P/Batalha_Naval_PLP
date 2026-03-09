extends VBoxContainer

const CURRENT_USER_PATH := "res://dados_autenticacao/usuario_logado.txt"
const LOGIN_SCENE_PATH := "res://TelaLogin.tscn"

func _ready():
	$start.grab_focus()
	if not $start.pressed.is_connected(_on_start_pressed):
		$start.pressed.connect(_on_start_pressed)
	if not $dinamico.pressed.is_connected(_on_dinamico_pressed):
		$dinamico.pressed.connect(_on_dinamico_pressed)
	if not $sair.pressed.is_connected(_on_sair_pressed):
		$sair.pressed.connect(_on_sair_pressed)

func _on_start_pressed():
	CampaignState.iniciar_nova_campanha()
	get_tree().change_scene_to_file("res://scenes/modo_campanha.tscn")

func _on_dinamico_pressed():
	CampaignState.iniciar_nova_campanha_dinamica()
	get_tree().change_scene_to_file("res://scenes/modo_campanha.tscn")

func _on_sair_pressed():
	var file := FileAccess.open(CURRENT_USER_PATH, FileAccess.WRITE)
	if file != null:
		file.store_string("")
		file.close()
	get_tree().change_scene_to_file(LOGIN_SCENE_PATH)
