extends VBoxContainer

const DATA_DIR_PATH := "res://dados"
const SESSION_FILE_PATH := DATA_DIR_PATH + "/usuario_atual.json"
const MENU_SCENE_PATH := "res://MenuPrincipal.tscn"

@onready var login_input  = $campos/login
@onready var senha_input  = $campos/senha
@onready var erro_label   = $Erro
@onready var btn_entrar   = $botoes/entrar
@onready var btn_cadastro = $botoes/cadastro
@onready var btn_sair     = $botoes/sair

var usuario_node: Node = null

func _ready():
	erro_label.visible = false
	btn_entrar.grab_focus()
	btn_entrar.pressed.connect(_on_entrar_pressed)
	btn_cadastro.pressed.connect(_on_cadastro_pressed)
	btn_sair.pressed.connect(_on_sair_pressed)

	usuario_node = ClassDB.instantiate("UsuarioNode")
	if usuario_node == null:
		mostrar_erro("Falha ao carregar autenticacao Rust")
		return
	add_child(usuario_node)

	if not garantir_arquivo_sessao():
		mostrar_erro("Falha ao preparar sessao")
		return

	if tem_sessao_valida_rust():
		call_deferred("ir_para_menu_principal")

func _on_entrar_pressed():
	var login = normalizar_login(login_input.text)
	var senha = senha_input.text

	if login.is_empty() or senha.is_empty():
		mostrar_erro("Preencha todos os campos")
		return

	limpar_erro()
	var usuario: Dictionary = usuario_node.call("login", login, senha)
	if usuario.is_empty():
		mostrar_erro("Login ou senha invalidos")
		return

	var login_autenticado := normalizar_login(str(usuario.get("login", login)))
	if not salvar_login_atual(login_autenticado):
		mostrar_erro("Falha ao salvar sessao")
		return

	ir_para_menu_principal()

func _on_cadastro_pressed():
	var login = normalizar_login(login_input.text)
	var senha = senha_input.text

	if login.is_empty() or senha.is_empty():
		mostrar_erro("Preencha todos os campos")
		return

	limpar_erro()
	var cadastro_ok: bool = usuario_node.call("registrar", login, login, senha)
	if not cadastro_ok:
		mostrar_erro("Login ja cadastrado")
		return

	if not salvar_login_atual(login):
		mostrar_erro("Falha ao salvar sessao")
		return

	ir_para_menu_principal()

func _on_sair_pressed():
	get_tree().quit()

func normalizar_login(login: String) -> String:
	return login.strip_edges().to_upper()

func garantir_arquivo_sessao() -> bool:
	var data_dir_name := DATA_DIR_PATH.trim_prefix("res://")
	var root_dir := DirAccess.open("res://")
	if root_dir == null:
		return false

	if not root_dir.dir_exists(data_dir_name):
		var mkdir_error := root_dir.make_dir(data_dir_name)
		if mkdir_error != OK:
			return false

	if FileAccess.file_exists(SESSION_FILE_PATH):
		return true

	var file := FileAccess.open(SESSION_FILE_PATH, FileAccess.WRITE)
	if file == null:
		return false
	file.store_string("{}")
	file.close()
	return true

func ler_sessao() -> Dictionary:
	var file := FileAccess.open(SESSION_FILE_PATH, FileAccess.READ)
	if file == null:
		return {}

	var content := file.get_as_text().strip_edges()
	file.close()
	if content.is_empty():
		return {}

	var parser := JSON.new()
	var err := parser.parse(content)
	if err != OK or typeof(parser.data) != TYPE_DICTIONARY:
		return {}
	return parser.data

func salvar_sessao(data: Dictionary) -> bool:
	var file := FileAccess.open(SESSION_FILE_PATH, FileAccess.WRITE)
	if file == null:
		return false
	file.store_string(JSON.stringify(data))
	file.close()
	return true

func ler_login_atual() -> String:
	var sessao := ler_sessao()
	return normalizar_login(str(sessao.get("login", "")))

func salvar_login_atual(login: String) -> bool:
	return salvar_sessao({"login": normalizar_login(login)})

func limpar_sessao() -> void:
	salvar_sessao({})

func tem_sessao_valida_rust() -> bool:
	var login := ler_login_atual()
	if login.is_empty():
		return false

	var usuario: Dictionary = usuario_node.call("buscar_por_login", login)
	if usuario.is_empty():
		limpar_sessao()
		return false
	return true

func limpar_erro() -> void:
	erro_label.text = ""
	erro_label.visible = false

func ir_para_menu_principal() -> void:
	get_tree().change_scene_to_file(MENU_SCENE_PATH)

func mostrar_erro(mensagem: String):
	erro_label.text = mensagem
	erro_label.visible = true
