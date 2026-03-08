use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::domain::entidades::usuario::Usuario;
use crate::domain::repositorios::repositorio_usuario::RepositorioUsuario;

pub struct UsuarioService<R: RepositorioUsuario> {
    pub repo: R
}

impl<R: RepositorioUsuario> UsuarioService<R> {

    //========================================
    //         SESSÃO DE AUTENTICAÇÃO
    //========================================
    pub fn registrar (
        &mut self,
        nome: String,
        login: String,
        senha: String
    ) -> Result<(), String> {

        if self.repo.achar_por_login(&login).is_some() {
            return Err("login já existe".into())
        }

        let salt = SaltString::generate(&mut OsRng);
        let senha_hash = Argon2::default()
            .hash_password(senha.as_bytes(), &salt)
            .map_err(|e| e.to_string())?
            .to_string();

        let id = self.repo.listar()
            .iter()
            .map(|u| u.id)
            .max()
            .unwrap_or(0) + 1;

        let usuario = Usuario::novo_usuario(id, nome, login, senha_hash);

        self.repo.salvar(usuario)
    }
    pub fn login(
        &self,
        login: &str,
        senha: &str,
    ) -> Result<Usuario, String> {

        let usuario = self.repo
            .achar_por_login(login)
            .ok_or("usuário não encontrado.")?;

        let hash_parsed = PasswordHash::new(&usuario.senha_hash)
            .map_err(|e| e.to_string())?;

        Argon2::default()
            .verify_password(senha.as_bytes(), &hash_parsed)
            .map_err(|_| "Senha inválida".to_string())?;

        Ok(usuario)
    }

    //========================================
    //    SESSÃO DE MANIPULAÇÃO DE USUÁRIO
    //========================================
    pub fn buscar_por_login(&self, login: &str) -> Result<Usuario, String> {
        self.repo
            .achar_por_login(login)
            .ok_or("Usuário não encontrado".to_string())
    }

    pub fn atualizar_nome(&mut self, login: &str, novo_nome: String) -> Result<(), String> {
        let mut usuario = self.repo
            .achar_por_login(login)
            .ok_or("Usuário não encontrado".to_string())?;

        usuario.nome = novo_nome;
        self.repo.atualizar(usuario)
    }

    pub fn atualizar_senha(&mut self, login: &str, senha_atual: &str, nova_senha: String) -> Result<(), String> {
        let usuario = self.repo
            .achar_por_login(login)
            .ok_or("Usuário não encontrado".to_string())?;

        let hash_parsed = PasswordHash::new(&usuario.senha_hash)
            .map_err(|e| e.to_string())?;

        Argon2::default()
            .verify_password(senha_atual.as_bytes(), &hash_parsed)
            .map_err(|_| "Senha atual inválida".to_string())?;

        let salt = SaltString::generate(&mut OsRng);
        let novo_hash = Argon2::default()
            .hash_password(nova_senha.as_bytes(), &salt)
            .map_err(|e| e.to_string())?
            .to_string();

        let mut usuario_atualizado = usuario;
        usuario_atualizado.senha_hash = novo_hash;
        self.repo.atualizar(usuario_atualizado)
    }

    pub fn excluir_conta(&mut self, login: &str, senha: &str) -> Result<(), String> {
        let usuario = self.login(login,senha)?;
        self.repo.excluir(usuario.id)
    }
}


