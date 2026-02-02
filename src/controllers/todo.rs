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
  general_tools::{validate_tables, convert_js_date_to_local_str}, 
  handlers::todo::Request, 
  utils
};

pub async fn ctr_create_todo(pool: &MySqlPool, request: &Request, user_id: i64) -> utils::AsReturn { 
  let valid_tables = validate_tables(pool, &vec!["todo".to_string()]).await;
  if valid_tables.is_empty() {
    return utils::AsReturn {
      code: 0,
      status: "error".to_string(),
      message: "Tabel Todo Tidak Ditemukan".to_string(),
      data: None,
    };
  }
  let todo = request.todo.to_string();    
  let date_time = match convert_js_date_to_local_str(request.date_time.as_str()) {
    Ok(output) => output,
    Err(_) => {      
      return utils::AsReturn {
        code: 0,
        status: "error".to_string(),
        message: "Gagal konvert date time".to_string(),
        data: None,
      };
    },
  };
  let now = Utc::now().naive_local(); // atau Local::now().naive_local()
  let created_at = now.format("%Y-%m-%d %H:%M:%S").to_string();
  let query = "INSERT INTO todo (todo,date_time,user_id,created_at) VALUES (?,?,?,?)";
  let insert = sqlx::query(&query)
    .bind(&todo)
    .bind(&date_time)
    .bind(&user_id)
    .bind(&created_at)    
    .execute(pool).await;
  match insert {
    Ok(_) => {
      return utils::AsReturn {
        code: 1,
        status: "success".to_string(),
        message: "Todo berhasil dibuat".to_string(),
        data: None,
      }
    },
    Err(_) => {      
      return utils::AsReturn {
        code: 0,
        status: "error".to_string(),
        message: "Gagal menambahkan todo".to_string(),
        data: None,
      };
    }
  };    
}

pub async fn ctr_update_todo(pool: &MySqlPool, request: &Request, user_id: i64, id: i64) -> utils::AsReturn { 
  let valid_tables = validate_tables(pool, &vec!["todo".to_string()]).await;
  if valid_tables.is_empty() {
    return utils::AsReturn {
      code: 0,
      status: "error".to_string(),
      message: "Tabel Todo Tidak Ditemukan".to_string(),
      data: None,
    };
  }  
  let query = "SELECT * FROM todo WHERE id = ? ";
  let row_result = sqlx::query(&query).bind(&id).fetch_one(pool).await;
  match row_result {
    Ok (_)  => {}, 
    Err (_) => {      
      return utils::AsReturn {
        code: 0,
        status: "error".to_string(),
        message: "Todo Tidak Ditemukan".to_string(),
        data: None,
      };
    }
  };
  let todo = request.todo.to_string();    
  let date_time = match convert_js_date_to_local_str(request.date_time.as_str()) {
    Ok(output) => output,
    Err(_) => {      
      return utils::AsReturn {
        code: 0,
        status: "error".to_string(),
        message: "Gagal konvert date time".to_string(),
        data: None,
      };
    },
  };
  let now = Utc::now().naive_local(); // atau Local::now().naive_local()
  let updated_at = now.format("%Y-%m-%d %H:%M:%S").to_string();
  let query = "UPDATE todo SET todo = ?, date_time = ?, updated_at = ? WHERE id = ? AND user_id = ?";
  let update = sqlx::query(&query)
    .bind(&todo)
    .bind(&date_time)
    .bind(&updated_at)    
    .bind(&user_id)
    .bind(&id)
    .execute(pool).await;
  match update {
    Ok(_) => {
      return utils::AsReturn {
        code: 1,
        status: "success".to_string(),
        message: "Todo berhasil diupdate".to_string(),
        data: None,
      }
    },
    Err(_) => {      
      return utils::AsReturn {
        code: 0,
        status: "error".to_string(),
        message: "Gagal update todo".to_string(),
        data: None,
      };
    }
  };    
}

pub async fn ctr_delete_todo(pool: &MySqlPool, user_id: i64, id: i64) -> utils::AsReturn { 
  let valid_tables = validate_tables(pool, &vec!["todo".to_string()]).await;
  if valid_tables.is_empty() {
    return utils::AsReturn {
      code: 0,
      status: "error".to_string(),
      message: "Tabel Todo Tidak Ditemukan".to_string(),
      data: None,
    };
  } 
  let query = "SELECT * FROM todo WHERE id = ? ";
  let row_result = sqlx::query(&query).bind(&id).fetch_one(pool).await;
  match row_result {
    Ok (_)  => {}, 
    Err (_) => {      
      return utils::AsReturn {
        code: 0,
        status: "error".to_string(),
        message: "Todo Tidak Ditemukan".to_string(),
        data: None,
      };
    }
  };
  let query = "DELETE FROM todo WHERE id = ? AND user_id = ?";
  let delete = sqlx::query(&query)    
  .bind(&id)
  .bind(&user_id)
    .execute(pool).await;
  match delete {
    Ok(_) => {
      return utils::AsReturn {
        code: 1,
        status: "success".to_string(),
        message: "Todo berhasil dihapus".to_string(),
        data: None,
      }
    },
    Err(_) => {      
      return utils::AsReturn {
        code: 0,
        status: "error".to_string(),
        message: "Gagal hapus todo".to_string(),
        data: None,
      };
    }
  };    
}
