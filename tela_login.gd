extends VBoxContainer

@onready var login_input  = $campos/login
@onready var senha_input  = $campos/senha
@onready var erro_label   = $Erro
@onready var btn_entrar   = $botoes/entrar
@onready var btn_cadastro = $botoes/cadastro
@onready var btn_sair     = $botoes/sair

func _ready():
	erro_label.visible = false
	btn_entrar.grab_focus()
	btn_entrar.pressed.connect(_on_entrar_pressed)
	btn_cadastro.pressed.connect(_on_cadastro_pressed)
	btn_sair.pressed.connect(_on_sair_pressed)

func _on_entrar_pressed():
	var login = login_input.text.strip_edges()
	var senha = senha_input.text

	if login.is_empty() or senha.is_empty():
		mostrar_erro("Preencha todos os campos")
		return
	print("Login: ", login)

func _on_cadastro_pressed():
	pass

func _on_sair_pressed():
	get_tree().quit()

func mostrar_erro(mensagem: String):
	erro_label.text = mensagem
	erro_label.visible = true
