use sqlx::{
  MySqlPool, 
  mysql::{
    MySqlPoolOptions
  }
}; 
use std::{
  collections::HashMap
};
use crate::general_tools::{
  log_error_to_file
};

pub async fn connect(c: &HashMap<String, String>) -> Option<MySqlPool> {
  let db_user = c.get("dbUser")?;
  let db_pass = c.get("dbPassword")?;
  let db_host = c.get("dbHost")?;
  let db_port = c.get("dbPort")?;
  let db_name = c.get("dbName")?;
  let db_url = format!(
    "mysql://{}:{}@{}:{}/{}",
    db_user, db_pass, db_host, db_port, db_name
  );
  match MySqlPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await {
      Ok(pool) => Some(pool),
      Err(err) => {
        log_error_to_file("db_error", &err.to_string());
        None
      }
    }
}