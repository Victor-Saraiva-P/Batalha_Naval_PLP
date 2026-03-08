extends VBoxContainer

func _ready():
	$start.grab_focus()
	if not $start.pressed.is_connected(_on_start_pressed):
		$start.pressed.connect(_on_start_pressed)
	if not $dinamico.pressed.is_connected(_on_dinamico_pressed):
		$dinamico.pressed.connect(_on_dinamico_pressed)

func _on_start_pressed():
	CampaignState.iniciar_nova_campanha()
	get_tree().change_scene_to_file("res://scenes/modo_campanha.tscn")

func _on_dinamico_pressed():
	CampaignState.iniciar_nova_campanha_dinamica()
	get_tree().change_scene_to_file("res://scenes/modo_campanha.tscn")
