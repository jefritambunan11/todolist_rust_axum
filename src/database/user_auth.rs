use sqlx::{
  MySql, MySqlPool 
  
}; 
use dotenvy::dotenv;
use crate::models::User;

pub async fn get_user_by_id(pool: &MySqlPool, id: i64) -> Result<Option<User>, 
sqlx::Error> {    
  dotenv().ok();
  sqlx::query_as::<MySql, User>("SELECT id, email, name FROM user WHERE id = ?")
  .bind(id)
  .fetch_optional(pool)
  .await
}