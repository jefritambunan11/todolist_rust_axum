use sqlx::{
  MySqlPool,
  Row, 
  // Column, 
  // MySql, 
  // TypeInfo
};
use serde_json::{
  // Value,
  json, 
};
use crate::{
  general_tools, handlers::login::Request, utils
};

pub async fn ctr_login(pool: &MySqlPool, request: &Request) -> utils::AsReturn {        
  let valid_tables = general_tools::validate_tables(pool, &vec!["user".to_string()]).await;
  if valid_tables.is_empty() {
    return utils::AsReturn {
      code: 0,
      status: "error".to_string(),
      message: "Tabel User Tidak Ditemukan".to_string(),
      data: None,
    };
  }  
  let email = request.email.to_string(); 
  let password = request.password.to_string();   
  let query = format!("SELECT * FROM `{}` WHERE email = ? ", "user");
  let row_result = sqlx::query(&query).bind(&email).fetch_one(pool).await;
  let row = match row_result {
    Ok (row) => {
      eprintln!("Data Ketemu Di Database");
      row
    },
    Err (_) => {
      //eprintln!("Ada error : {}", e);
      return utils::AsReturn {
        code: 0,
        status: "error".to_string(),
        message: "Email Tidak Ditemukan".to_string(),
        data: None,
      };
    }
  };
  let password_hash: String = row.get("password");
  let result = general_tools::verify_password(&password, &password_hash);
  let _check_pass = match result {
    Ok (check_pass) => {
      if check_pass {
        // eprintln!("Password Sama ");            
      }else{        
        return utils::AsReturn {
          code: 0,
          status: "error".to_string(),
          message: "Password Salah".to_string(),
          data: None,
        };
      }
      check_pass
    },
    Err (_) => {
      // buat log untuk ada kesalahan hashing 
      return utils::AsReturn {
        code: 0,
        status: "error".to_string(),
        message: "Terjadi Kesalah Enkripsi".to_string(),
          data: None,
      };
    }
  }; 
  let id: i64 = row.get("id");
  let name: String = row.get("name");
  // let email: String = row.get("email");
  // let password: String = row.get("password");
  let token = general_tools::generate_token(
    id,
    name.to_owned()
  ).unwrap();
  let data = json!({
    // "id": id,
    // "name": name,
    // "email": email,    
    "token": token,    
  });  
  
  utils::AsReturn {
    code: 1,
    status: "success".to_string(),
    message: "User Berhasil Login !".to_string(),
    data: Some(data),    
  }
}

