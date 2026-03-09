use crate::application::app_paths::{MENU_SCENE_PATH, USERS_DATA_RES_PATH, USER_SESSION_RES_PATH};
use crate::application::services::usuario_service::UsuarioService;
use crate::infrastructure::repositorio_usuario_json::RepositorioUsuarioJson;
use godot::classes::{Button, Control, IControl, Label, VBoxContainer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct CenaConquistas {
    base: Base<Control>,
}

#[godot_api]
impl IControl for CenaConquistas {
    fn init(base: Base<Control>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let login = self.obter_login_atual();

        if !login.is_empty() {
            self.exibir_estatisticas(&login);
            let conquistas_desbloqueadas = self.obter_conquistas_usuario(&login);
            self.popular_lista(conquistas_desbloqueadas);
        }

        let mut btn_voltar = self.base().get_node_as::<Button>("botao_voltar");
        let callable = self.base().callable("voltar_menu");
        btn_voltar.connect("pressed", &callable);
    }
}

#[godot_api]
impl CenaConquistas {
    fn obter_login_atual(&self) -> String {
        let caminho = USER_SESSION_RES_PATH;
        if godot::classes::FileAccess::file_exists(caminho) {
            let conteudo = godot::classes::FileAccess::get_file_as_string(caminho).to_string();
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&conteudo) {
                return json
                    .get("login")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
            }
        }
        "".to_string()
    }

    fn exibir_estatisticas(&mut self, login: &str) {
        let mut label_stats = self.base().get_node_as::<Label>("LabelEstatisticas");

        let ps = godot::classes::ProjectSettings::singleton();
        let path_absoluto = ps.globalize_path(USERS_DATA_RES_PATH).to_string();

        let service = UsuarioService {
            repo: RepositorioUsuarioJson::new(&path_absoluto),
        };

        if let Ok(u) = service.buscar_por_login(login) {
            let texto = format!(
                "Partidas: {} | Vitórias: {} | Derrotas: {}\nTaxa de Vitória: {:.1}%",
                u.jogos_totais,
                u.vitorias,
                u.derrotas,
                u.taxa_de_vitoria() * 100.0
            );
            label_stats.set_text(&texto);
        }
    }

    fn obter_conquistas_usuario(&self, login: &str) -> Vec<String> {
        let ps = godot::classes::ProjectSettings::singleton();
        let path_absoluto = ps.globalize_path(USERS_DATA_RES_PATH).to_string();

        let service = UsuarioService {
            repo: RepositorioUsuarioJson::new(&path_absoluto),
        };

        if let Ok(u) = service.buscar_por_login(login) {
            return u
                .conquistas
                .iter()
                .map(|c| {
                    match c {
                        crate::domain::entidades::conquista::Conquista::Almirante => "Almirante",
                        crate::domain::entidades::conquista::Conquista::Capitao => "Capitao",
                        crate::domain::entidades::conquista::Conquista::CapitaoDeMarEGuerra => {
                            "CapitaoDeMarEGuerra"
                        }
                        crate::domain::entidades::conquista::Conquista::Marinheiro => "Marinheiro",
                    }
                    .to_string()
                })
                .collect();
        }
        vec![]
    }

    fn popular_lista(&mut self, desbloqueadas: Vec<String>) {
        let mut lista_container = self.base().get_node_as::<VBoxContainer>("VBoxContainer");

        let todas_conquistas = vec![
            ("Almirante", "Vencer sem perder navios"),
            ("Capitao", "Acertar 7 tiros seguidos"),
            ("CapitaoDeMarEGuerra", "Acertar 8 tiros seguidos"),
            ("Marinheiro", "Vencer em 20 rodadas ou menos"),
        ];

        for (nome, descricao) in todas_conquistas {
            let mut label = Label::new_alloc();
            let mut texto = format!("{} - {}", nome, descricao);

            if desbloqueadas.contains(&nome.to_string()) {
                texto = format!("✅ {}", texto);
                label.set_modulate(Color::from_rgb(1.0, 0.84, 0.0));
            } else {
                texto = format!("🔒 {}", texto);
                label.set_modulate(Color::from_rgb(0.5, 0.5, 0.5));
            }

            label.set_text(&texto);
            lista_container.add_child(&label);
        }
    }

    #[func]
    fn voltar_menu(&mut self) {
        self.base().get_tree().change_scene_to_file(MENU_SCENE_PATH);
    }
}
