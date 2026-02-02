use axum::{
  async_trait,
  extract::{FromRequestParts, Request},
  http::{request::Parts, StatusCode},
  response::{IntoResponse, Response},
  Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use sqlx::MySqlPool; 
use std::sync::Arc;

use crate::{  
  AppState, database, general_tools::{
    self, Claims, SECRET_KEY
  }, models::User, utils
}; 

// Error type khusus untuk auth
#[derive(Debug)]
pub struct AuthError(pub(crate) StatusCode);

impl IntoResponse for AuthError {
  fn into_response(self) -> Response {
    let status = self.0;
    let message = match status {
      StatusCode::UNAUTHORIZED => "Akses Ditolak, Anda Tidak Punya Wewenang Untuk Ini !!",
      StatusCode::INTERNAL_SERVER_ERROR => "Terjadi kesalahan internal pada server",
      _ => "Autentikasi gagal",
    };
        
    let (status, json_body) = utils::api_response(
      message,
      status,
      "error",
      None,
    ); 

    (status, json_body).into_response()
  }
}

pub struct AuthenticatedUser(pub User);

#[async_trait]
impl FromRequestParts<Arc<AppState>> for AuthenticatedUser {
  type Rejection = AuthError;

  async fn from_request_parts(
    parts: &mut axum::http::request::Parts,
    state: &Arc<AppState>,
  ) -> Result<Self, Self::Rejection> {    
    let auth_header = parts
      .headers
      .get("authorization")
      .and_then(|h| h.to_str().ok())
      .ok_or_else(|| AuthError(StatusCode::UNAUTHORIZED))?;

    if !auth_header.starts_with("Bearer ") {
      return Err(AuthError(StatusCode::UNAUTHORIZED));
    }
    let token = &auth_header[7..];
    let token_data = general_tools::validate_token(token).unwrap(); 
    let user_id = token_data.user_id;
    let pool: &MySqlPool = &state.db_pool;
    let user = database::user_auth::get_user_by_id(pool, user_id)
      .await
      .map_err(|_| AuthError(StatusCode::INTERNAL_SERVER_ERROR))?
      .ok_or_else(|| AuthError(StatusCode::UNAUTHORIZED))?;
    Ok(AuthenticatedUser(user)) 
  }
}