use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  pub email: String,  
}