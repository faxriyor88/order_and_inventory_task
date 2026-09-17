use crate::auth::jwt::create_token;
use crate::auth::middleware::AuthError;
use crate::auth::password::verify_password;
use crate::repositories::user_repo::UserRepository;
use jsonwebtoken::EncodingKey;

pub struct AuthService {
    encoding_key: EncodingKey,
    user_repo: UserRepository,
}

impl AuthService {
    pub fn new(encoding_key: EncodingKey, user_repo: UserRepository) -> Self {
        Self {
            encoding_key,
            user_repo,
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<String, AuthError> {
        let user = self
            .user_repo
            .find_by_username(username)
            .await
            .map_err(|_| AuthError::InvalidCredentials)?
            .ok_or(AuthError::InvalidCredentials)?;

        if !verify_password(password, &user.password_hash).unwrap_or(false) {
            return Err(AuthError::InvalidCredentials);
        }

        create_token(user.id, &self.encoding_key).map_err(|_| AuthError::TokenCreation)
    }
}
