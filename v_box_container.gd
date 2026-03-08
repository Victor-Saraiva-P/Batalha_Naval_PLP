extends VBoxContainer

func _ready():
	$start.grab_focus()
	
	if not $start.pressed.is_connected(_on_start_pressed):
		$start.pressed.connect(_on_start_pressed)
		
	if not $ranking.pressed.is_connected(_on_ranking_pressed):
		$ranking.pressed.connect(_on_ranking_pressed)
		
	if not $sair.pressed.is_connected(_on_sair_pressed):
		$sair.pressed.connect(_on_sair_pressed)

func _on_start_pressed():
	CampaignState.iniciar_nova_campanha()
	get_tree().change_scene_to_file("res://scenes/modo_campanha.tscn")

func _on_ranking_pressed():
	get_tree().change_scene_to_file("res://scenes/tela_ranking.tscn")

func _on_sair_pressed():
	get_tree().quit()
