use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::domain::entidades::usuario::Usuario;
use crate::domain::repositorios::repositorio_usuario::RepositorioUsuario;

pub struct AuthService<R: RepositorioUsuario> {
    pub repo: R
}

impl<R: RepositorioUsuario> AuthService<R> {

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
}