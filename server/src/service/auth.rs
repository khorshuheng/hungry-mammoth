use super::error::ServiceError;
use crate::dto::{auth::TokenClaims, user::UserProfile};
use jsonwebtoken::{encode, EncodingKey, Header};

#[derive(Clone)]
pub struct TokenService {
  secret: String,
  token_expiry: u64,
}

impl TokenService {
  pub fn new(secret: String, expiry_time: u64) -> Self {
    Self {
      secret,
      token_expiry: expiry_time,
    }
  }

  pub fn generate_token(&self, user: UserProfile) -> Result<String, ServiceError> {
    let iat = chrono::Utc::now().timestamp();
    let exp = chrono::Utc::now()
      .checked_add_signed(chrono::Duration::seconds(self.token_expiry as i64))
      .unwrap()
      .timestamp();

    let claims = TokenClaims {
      sub: user.uuid,
      email: user.email,
      iat,
      exp,
    };

    let token = encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(self.secret.as_ref()),
    )
    .unwrap();
    Ok(token)
  }

  // pub fn verify_token(&self, token: &str) -> Result<Claims, TokenError> {
  //   let token_data = decode::<Claims>(token, self.secret.as_ref(), &Validation::default())?;
  //   Ok(token_data.claims)
  // }
}
