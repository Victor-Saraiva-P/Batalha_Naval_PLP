use godot::prelude::*;
use crate::application::services::usuario_service::UsuarioService;
use crate::infrastructure::repositorio_usuario_json::RepositorioUsuarioJson;
use crate::domain::entidades::usuario::Usuario;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct UsuarioNode {
    service: UsuarioService<RepositorioUsuarioJson>,
    base: Base<Node>
}

fn usuario_para_dict(u : Usuario) -> Dictionary<GString, Variant> {
    let mut dict = Dictionary::new();
    dict.set("id", u.id as i64);
    dict.set("nome", &GString::from(&u.nome));
    dict.set("login", &GString::from(&u.login));
    dict.set("jogos_totais", u.jogos_totais as i64);
    dict.set("vitorias", u.vitorias as i64);
    dict.set("derrotas", u.derrotas as i64);
    dict.set("taxa_de_vitoria", u.taxa_de_vitoria() as f64);
    dict
}

#[godot_api]
impl INode for UsuarioNode {
    fn init(base: Base<Node>) -> Self {
        Self{
            service: UsuarioService {
                repo: RepositorioUsuarioJson::new("dados/usuarios.json")
            },
            base
        }
    }
}

#[godot_api]
impl UsuarioNode {

    #[func]
    pub fn registrar(&mut self, nome: GString, login: GString, senha: GString) -> bool {
        self.service
            .registrar(nome.to_string(), login.to_string(), senha.to_string())
            .is_ok()
    }

    #[func]
    pub fn login(&self, login: GString, senha: GString) -> Dictionary<GString, Variant> {
        match self.service.login(&login.to_string(), &senha.to_string()) {
            Ok(usuario) => usuario_para_dict(usuario),
            Err(_) => Dictionary::new()
        }
    }

    #[func]
    pub fn buscar_por_login(&self, login: GString) -> Dictionary<GString, Variant> {
        match self.service.buscar_por_login(&login.to_string()) {
            Ok(usuario) => usuario_para_dict(usuario),
            Err(_) => Dictionary::new()
        }
    }

    #[func]
    pub fn atualizar_nome(&mut self, login: GString, novo_nome: GString) -> bool {
        self.service
            .atualizar_nome(&login.to_string(), novo_nome.to_string())
            .is_ok()
    }

    #[func]
    pub fn atualizar_senha(&mut self, login: GString, senha_atual: GString, nova_senha: GString) -> bool {
        self.service
            .atualizar_senha(&login.to_string(), &senha_atual.to_string(), nova_senha.to_string())
            .is_ok()
    }

    #[func]
    pub fn excluir_conta(&mut self, login: GString, senha: GString) -> bool {
        self.service
            .excluir_conta(&login.to_string(), &senha.to_string())
            .is_ok()
    }
}