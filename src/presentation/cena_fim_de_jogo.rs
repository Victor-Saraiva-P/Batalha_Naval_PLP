use crate::application::app_paths::MENU_SCENE_PATH;
use godot::classes::{Button, Control, IControl};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct CenaFimDeJogo {
    base: Base<Control>,
}

#[godot_api]
impl IControl for CenaFimDeJogo {
    fn init(base: Base<Control>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let mut btn_sair = self.base().get_node_as::<Button>("botao_sair");
        let callable = self.base().callable("voltar_menu");
        btn_sair.connect("pressed", &callable);
    }
}

#[godot_api]
impl CenaFimDeJogo {
    #[func]
    fn voltar_menu(&mut self) {
        let mut tree = self.base().get_tree();
        tree.change_scene_to_file(MENU_SCENE_PATH);
    }
}
