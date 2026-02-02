use std::sync::Arc;

use axum::{
  extract::{
    State, 
    Json,
    Path,
  },
  http::StatusCode
};

use serde::{
  Deserialize
};

use crate::{
  AppState, extractors::headers::AuthenticatedUser, utils::{self, api_response}
};

use crate::extractors::json::AppJson;

use crate::controllers::{
  todo::ctr_create_todo, 
  todo::ctr_update_todo, 
  todo::ctr_delete_todo, 
};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Request {  
  pub todo: String,  
  pub date_time: String,  
}

pub async fn todo_create_handler(
  AuthenticatedUser(user): AuthenticatedUser,
  State(state): State<Arc<AppState>>,
  AppJson(payload): AppJson<Request>,
) -> (StatusCode, Json<crate::utils::ApiResponse>) { 
  if payload.todo.trim().is_empty() {
    return api_response(
      "Todo tidak boleh kosong",
      StatusCode::BAD_REQUEST,
      "error",
      None,
    );
  }
  if payload.date_time.trim().is_empty() {
    return api_response(
      "Date Time tidak boleh kosong",
      StatusCode::BAD_REQUEST,
      "error",
      None,
    );
  }

  let re_todo: utils::AsReturn = ctr_create_todo(&state.db_pool, &payload, user.id).await;
  if re_todo.code == 1 { 
    api_response(
      &re_todo.message,
      StatusCode::OK,
      &re_todo.status,
      re_todo.data,            
    )
  } else {
    api_response(
      &re_todo.message,
      StatusCode::BAD_REQUEST,
      &re_todo.status,
      None,
    )
  }  
}

pub async fn todo_update_handler(
  Path(id): Path<i64>, 
  AuthenticatedUser(user): AuthenticatedUser,
  State(state): State<Arc<AppState>>,
  AppJson(payload): AppJson<Request>,
) -> (StatusCode, Json<crate::utils::ApiResponse>) {    
  if payload.todo.trim().is_empty() {
    return api_response(
      "Todo tidak boleh kosong",
      StatusCode::BAD_REQUEST,
      "error",
      None,
    );
  }
  if payload.date_time.trim().is_empty() {
    return api_response(
      "Date Time tidak boleh kosong",
      StatusCode::BAD_REQUEST,
      "error",
      None,
    );
  }
  let re_todo: utils::AsReturn = ctr_update_todo(&state.db_pool, &payload, user.id, id).await;
  if re_todo.code == 1 { 
    api_response(
      &re_todo.message,
      StatusCode::OK,
      &re_todo.status,
      re_todo.data,            
    )
  } else {
    api_response(
      &re_todo.message,
      StatusCode::BAD_REQUEST,
      &re_todo.status,
      None,
    )
  }  
}

pub async fn todo_delete_handler(
  Path(id): Path<i64>, 
  AuthenticatedUser(user): AuthenticatedUser,
  State(state): State<Arc<AppState>>
) -> (StatusCode, Json<crate::utils::ApiResponse>) {    
  let re_todo: utils::AsReturn = ctr_delete_todo(&state.db_pool, user.id, id).await;
  if re_todo.code == 1 { 
    api_response(
      &re_todo.message,
      StatusCode::OK,
      &re_todo.status,
      re_todo.data,            
    )
  } else {
    api_response(
      &re_todo.message,
      StatusCode::BAD_REQUEST,
      &re_todo.status,
      None,
    )
  }  
}

