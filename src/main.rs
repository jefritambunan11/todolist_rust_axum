use axum::{ 
  Router, 
  routing::{get, post, put, delete}
};
/*
use sqlx::{
  MySqlPool
};
*/
use std::{
  any::Any, collections::HashMap, fs, process, sync::Arc
  // net::SocketAddr
};
use dotenvy::dotenv;
use anyhow::Result;

// use tokio::net::TcpListener;
//use serde::Deserialize;  // for decode json

pub trait AppStateTrait: Send + Sync {
  fn as_any(&self) -> &dyn Any;
}

impl AppStateTrait for AppState {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

// State aplikasi
#[derive(Clone)]
pub struct AppState {
  pub db_pool: sqlx::MySqlPool,
  pub ka: String,  
}

mod extractors;
mod config;
mod general_tools;
mod database;
mod utils;
mod models;
// mod backup;
// mod restore;
mod handlers {
  pub mod intro; 
  pub mod login; 
  pub mod register; 
  pub mod todo; 
}
mod controllers;

#[tokio::main]
async fn main() -> Result<()> { 
  dotenv().ok();
  let mc = config::load_master_configs();  
  if let Err(err) = fs::create_dir_all(&mc.logs) {
    eprintln!("Gagal Membuat Folder {}: {}", &mc.logs, err);
    process::exit(1);
  }

  let dbc = config::load_db_configs();  
  let mut db: HashMap<String, String> = HashMap::new();
  db.insert(
    String::from("dbUser"), 
    String::from(&dbc.user.to_string())
  );
  db.insert(
    String::from("dbPassword"), 
    String::from(&dbc.pass.to_string())
  );
  db.insert(
    String::from("dbHost"), 
    String::from(&dbc.host.to_string())
  );
  db.insert(
    String::from("dbPort"), 
    String::from(&dbc.port.to_string())
  );
  db.insert(
    String::from("dbName"), 
    String::from(&dbc.name.to_string())
  );

  let db_pool = database::connect::connect(&db)
    .await
    .expect("Gagal koneksi ke database");

  if let Err(e) = sqlx::query("SELECT 'OK' ").execute(&db_pool).await {
    general_tools::log_error_to_file(
      "db_error",
      &format!("Koneksi ke Database gagal: {}", e)
    );
    panic!("Database tidak bisa diakses");
  }

  let app_state = AppState {
    db_pool,
    ka: mc.ka.to_owned()
  };
  let app_state = Arc::new(app_state);

  let app = Router::new()    
    .route("/", get(handlers::intro::intro_handler))
    .route("/api/login", post(handlers::login::login_handler))
    .route("/api/register", post(handlers::register::register_handler))
    .route("/api/todo", post(handlers::todo::todo_create_handler))
    .route("/api/todo/:id", put(handlers::todo::todo_update_handler))
    .route("/api/todo/:id", delete(handlers::todo::todo_delete_handler))
    .with_state(app_state); 



    //.route("/to_backup", post(controllers::to_backup::to_backup_handler)).with_state(app_state)
    //.route("/to_restore", post(controllers::to_restore::to_restore_handler)).with_state(multi_app_state);  

  let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", mc.http_port)).await.unwrap();	
  axum::serve(listener, app).await.unwrap();
  Ok(())  	
}
