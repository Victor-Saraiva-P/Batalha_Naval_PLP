use std::fs;
use crate::domain::entidades::usuario::Usuario;
use crate::domain::repositorios::repositorio_usuario::RepositorioUsuario;

pub struct RepositorioUsuarioJson {
    path: String,
    users: Vec<Usuario>
}

impl RepositorioUsuarioJson {

    pub fn new(path: &str) -> Self {
        let data = fs::read_to_string(path)
            .unwrap_or_else(|_| "[]".to_string());

        let users: Vec<Usuario> =
            serde_json::from_str(&data).unwrap_or_default();

        Self {
            path: path.to_string(),
            users
        }
    }

    fn persist(&self) -> Result<(), String> {
        let json =
            serde_json::to_string_pretty(&self.users)
            .map_err(|e| e.to_string())?;

        fs::write(&self.path, json)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl RepositorioUsuario for RepositorioUsuarioJson {

    fn salvar(&mut self, usuario: Usuario) -> Result<(), String> {
        self.users.push(usuario);
        self.persist()
    }

    fn achar_por_login(&self, login: &str) -> Option<Usuario> {
        self.users
            .iter()
            .find(|u| u.login == login)
            .cloned()
    }

    fn achar_por_id(&self, id: u64) -> Option<Usuario> {
        self.users
            .iter()
            .find(|u| u.id == id)
            .cloned()
    }

    fn listar(&self) -> Vec<Usuario> {
        self.users.clone()
    }

}
