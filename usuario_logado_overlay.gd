extends CanvasLayer

const CURRENT_USER_PATH := "res://dados_autenticacao/usuario_logado.txt"
const LOGIN_SCENE_PATH := "res://TelaLogin.tscn"

var _label: Label

func _ready() -> void:
	_label = Label.new()
	_label.name = "UsuarioLogadoLabel"
	_label.anchor_left = 1.0
	_label.anchor_top = 1.0
	_label.anchor_right = 1.0
	_label.anchor_bottom = 1.0
	_label.offset_left = -250.0
	_label.offset_top = -28.0
	_label.offset_right = -4.0
	_label.offset_bottom = -2.0
	_label.horizontal_alignment = HORIZONTAL_ALIGNMENT_RIGHT
	_label.scale = Vector2(0.75, 0.75)
	_label.add_theme_constant_override("outline_size", 8)
	_label.add_theme_color_override("font_outline_color", Color(0.12, 0.36, 0.92, 1.0))
	add_child(_label)

	var timer := Timer.new()
	timer.wait_time = 0.5
	timer.one_shot = false
	timer.autostart = true
	timer.timeout.connect(_atualizar_texto_usuario)
	add_child(timer)

	_atualizar_texto_usuario()

func _atualizar_texto_usuario() -> void:
	var current_scene := get_tree().current_scene
	if current_scene != null and current_scene.scene_file_path == LOGIN_SCENE_PATH:
		_label.visible = false
		return

	_label.visible = true
	var usuario := _ler_usuario_logado()
	if usuario.is_empty():
		_label.text = "Usuario: -"
	else:
		_label.text = "Usuario: %s" % usuario

func _ler_usuario_logado() -> String:
	if not FileAccess.file_exists(CURRENT_USER_PATH):
		return ""

	var file := FileAccess.open(CURRENT_USER_PATH, FileAccess.READ)
	if file == null:
		return ""

	var usuario := file.get_as_text().strip_edges()
	file.close()
	return usuario
