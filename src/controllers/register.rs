use sqlx::{
  MySqlPool,
  Row
};
use chrono::{
  // Local,
  Utc, 
  // NaiveDate, 
  // NaiveDateTime
};
use crate::{
  general_tools::{self, hash_password_custom}, handlers::register::Request, utils
};

// pub struct User {
//   pub id: u16, 
//   pub name: String,
//   pub email: String,  
// }


pub async fn ctr_register(pool: &MySqlPool, request: &Request) -> utils::AsReturn {        
  let valid_tables = general_tools::validate_tables(pool, &vec!["user".to_string()]).await;
  if valid_tables.is_empty() {
    return utils::AsReturn {
      code: 0,
      status: "error".to_string(),
      message: "Tabel User Tidak Ditemukan".to_string(),
      data: None,
    };
  }
  let name = request.name.to_string(); 
  let email = request.email.to_string(); 
  let password = hash_password_custom(&request.password.to_string(), 4).unwrap();     
  let now = Utc::now().naive_local(); // atau Local::now().naive_local()
  let created_at = now.format("%Y-%m-%d %H:%M:%S").to_string();
  let updated_at = created_at.to_string();
  let query = "INSERT INTO user (name,email,password,created_at,updated_at) VALUES (?,?,?,?,?)";
  let insert = sqlx::query(&query)
    .bind(&name)
    .bind(&email)
    .bind(&password)
    .bind(&created_at)
    .bind(&updated_at)
    .execute(pool).await;
  let user_id = match insert {
    Ok(result) => result.last_insert_id(),
    Err(e) => {
      eprintln!("Insert error: {}", e);
      return utils::AsReturn {
        code: 0,
        status: "error".to_string(),
        message: "Gagal menambahkan user".to_string(),
        data: None,
      };
    }
  };
  let user_row = sqlx::query("SELECT id, name, password FROM user WHERE id = ?")
    .bind(user_id)
    .fetch_one(pool)
    .await;
  match user_row {
    Ok(row) => {  
      let token = general_tools::generate_token(
        row.get::<i64, _>("id"),
        row.get::<String, _>("name")
      ).unwrap();      
      let data = serde_json::json!({
        // "id": row.get::<i64, _>("id"),
        // "name": row.get::<String, _>("name"),
        // "password": row.get::<String, _>("password"),
        "token": token,    
      });
      utils::AsReturn {
        code: 1,
        status: "success".to_string(),
        message: "User berhasil dibuat".to_string(),
        data: Some(data),
      }
    },
    Err(e) => {
      eprintln!("Fetch error: {}", e);
      utils::AsReturn {
        code: 0,
        status: "error".to_string(),
        message: "User dibuat tapi gagal mengambil data".to_string(),
        data: None,
      }
    }
  }  
}
