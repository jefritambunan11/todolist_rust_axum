use std::error::Error;
use axum::{
  extract::rejection::JsonRejection,
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde_json::json;

pub struct AppJson<T>(pub T);

fn extract_field_name(error_str: &str, pattern: &str) -> Option<String> {
  error_str.find(pattern).and_then(|start| {
    let after = &error_str[start + pattern.len()..];
    after.find('`').map(|end| after[..end].to_string())
  })
}

#[axum::async_trait]
impl<S, T> axum::extract::FromRequest<S> for AppJson<T>
where
  T: serde::de::DeserializeOwned,
  S: Send + Sync,
{
  type Rejection = Response;

  async fn from_request(
    req: axum::extract::Request,
    state: &S,
  ) -> Result<Self, Self::Rejection> {
    match axum::Json::<T>::from_request(req, state).await {
      Ok(value) => Ok(Self(value.0)),
      Err(rejection) => {
        let message = match rejection {
          JsonRejection::JsonDataError(err) => {
            let error_str = err.source()
              .map(|s| s.to_string())
              .unwrap_or_else(|| err.to_string());
            
            if error_str.contains("unknown field") {
              extract_field_name(&error_str, "unknown field `")
                .map(|field| format!("Key  '{}' tidak dikenal", field))
                .unwrap_or_else(|| "ada key yang sama sekali tidak dikenal".to_string())
            }
            else if error_str.contains("missing field") {
              extract_field_name(&error_str, "missing field `")
                .map(|field| format!("Key '{}' ini hilang", field))
                .unwrap_or_else(|| "ada key yang dibutuhkan tapi hilang".to_string())
            }
            else {
              format!("Invalid data: {}", error_str)
            }
          }
          JsonRejection::JsonSyntaxError(err) => {
            format!("Invalid JSON syntax: {}", err)
          }
          JsonRejection::MissingJsonContentType(_) => {
            "Content-Type must be application/json".to_string()
          }
          _ => "Invalid request body".to_string(),
        };

        let response = (
          StatusCode::BAD_REQUEST,
          Json(json!({
            "status": "error",
            "message": message,
            "data": null
          })),
        );
        
        Err(response.into_response())
      }
    }
  }
}