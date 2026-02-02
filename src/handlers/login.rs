use std::sync::Arc;

use axum::{
  extract::{
    State, 
    Json
  },
  http::StatusCode
};

use serde::{
  Deserialize
};

use crate::{
  AppState, 
  utils::{
    api_response,
    AsReturn,
  },
};

use crate::extractors::json::AppJson;

use crate::controllers::{
  login::ctr_login
};


#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Request {
  pub ka: String, 
  pub email: String,
  pub password: String,
}

pub async fn login_handler(
  State(state): State<Arc<AppState>>,
  AppJson(payload): AppJson<Request>,
) -> (StatusCode, Json<crate::utils::ApiResponse>) {  
  if payload.ka != state.ka {
    return api_response(
      "Akses Ditolak, Anda Tidak Punya Wewenang Untuk Ini !!",
      StatusCode::BAD_REQUEST, 
      "error",
      None,
    );
  }
  if payload.email.trim().is_empty() {
    return api_response(
      "Email tidak boleh kosong",
      StatusCode::BAD_REQUEST,
      "error",
      None,
    );
  }
  if payload.password.trim().is_empty() {
    return api_response(
      "Password tidak boleh kosong",
      StatusCode::BAD_REQUEST,
      "error",
      None,
    );
  }

  let rlogin: AsReturn = ctr_login(&state.db_pool, &payload).await;
  if rlogin.code == 1 {    
    api_response(
      &rlogin.message,
      StatusCode::OK,
      &rlogin.status,
      rlogin.data, 
    )
  } else {
    api_response(
      &rlogin.message,
      StatusCode::BAD_REQUEST,
      &rlogin.status,
      None,
    )
  }
  
}