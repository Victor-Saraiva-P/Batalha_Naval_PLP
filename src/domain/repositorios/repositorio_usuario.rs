use crate::domain::entidades::usuario::Usuario;

pub trait RepositorioUsuario {
    
    fn salvar(&mut self, usuario: Usuario) -> Result<(), String>;
    fn achar_por_login(&self, login: &str) -> Option<Usuario>;
    fn achar_por_id(&self, id: u64) -> Option<Usuario>;
    fn atualizar(&mut self, usuario: Usuario) -> Result<(), String>;
    fn excluir(&mut self, id: u64) -> Result<(), String>;
    fn listar(&self) -> Vec<Usuario>;
}