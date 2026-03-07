use godot::prelude::*;
use godot::classes::{Control, IControl, VBoxContainer, Label, PackedScene, ResourceLoader, TextureRect};

pub struct RegistroRanking {
    pub nome_login: String,
    pub pontuacao: i32,
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct CenaRanking {
    base: Base<Control>,
}

#[godot_api]
impl IControl for CenaRanking {
    fn init(base: Base<Control>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let dados_falsos = vec![
            RegistroRanking { nome_login: "Vinicinho_mete_bala".to_string(), pontuacao: 10000 },
            RegistroRanking { nome_login: "Xi Jinping".to_string(), pontuacao: 8564 },
            RegistroRanking { nome_login: "Sivaldo Albino".to_string(), pontuacao: 5564 },
            RegistroRanking { nome_login: "Jorge".to_string(), pontuacao: 1564 },
        ];

        self.popular_lista(dados_falsos);
    }
}

#[godot_api]
impl CenaRanking {
    fn popular_lista(&mut self, dados: Vec<RegistroRanking>) {
        let mut lista_container = self.base().get_node_as::<VBoxContainer>("VBoxContainer");

        let mut resource_loader = ResourceLoader::singleton();
        let cena_linha = resource_loader
            .load("res://scenes/linha_jogador.tscn")
            .unwrap()
            .cast::<PackedScene>();

        for (index, jogador) in dados.into_iter().enumerate() {
            let mut nova_linha = cena_linha.instantiate().unwrap();

            let mut label_nome = nova_linha.get_node_as::<Label>("nick");
            label_nome.set_text(&jogador.nome_login);

            let mut label_pontuacao = nova_linha.get_node_as::<Label>("pontuacao");
            let texto_pontuacao = format!("{}", jogador.pontuacao);
            label_pontuacao.set_text(&texto_pontuacao);

            let mut icone_trofeu = nova_linha.get_node_as::<TextureRect>("trofeu");
            if index == 0 {
                icone_trofeu.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 1.0));
            } else {
                icone_trofeu.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 0.0));
            }

            lista_container.add_child(&nova_linha);
        }
    }
}