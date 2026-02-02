/*
response::{
  IntoResponse, 
  Response
},
*/

use axum::{
  Json,
  http
};

use http::{
  StatusCode
};

pub struct AsReturn {
  pub code: u16, 
  pub status: String,
  pub message: String,
  pub data: Option<serde_json::Value>,
}


#[derive(serde::Serialize)]
pub struct ApiResponse {
  pub message: String,
  pub status: String,
  pub data: Option<serde_json::Value>,
}

pub fn api_response(
  message: &str,
  status_code: StatusCode,
  status: &str,
  data: Option<serde_json::Value>,
) -> (StatusCode, Json<ApiResponse>) {
  (
    status_code,
    Json(ApiResponse {
      message: message.to_string(),
      status: status.to_string(),
      data,
    }),
  )
}
