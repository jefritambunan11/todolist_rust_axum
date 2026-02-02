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
  utils::{self, api_response},
};

use crate::extractors::json::AppJson;

use crate::controllers::{
  register::ctr_register
};


#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Request {
  pub ka: String, 
  pub name: String,
  pub email: String,
  pub password: String,
}

pub async fn register_handler(
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
  if payload.name.trim().is_empty() {
    return api_response(
      "Name tidak boleh kosong",
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

  let re_register: utils::AsReturn = ctr_register(&state.db_pool, &payload).await;
  if re_register.code == 1 {    
    api_response(
      &re_register.message,
      StatusCode::OK,
      &re_register.status,
      re_register.data,            
    )
  } else {
    api_response(
      &re_register.message,
      StatusCode::BAD_REQUEST,
      &re_register.status,
      None,
    )
  }
  
}