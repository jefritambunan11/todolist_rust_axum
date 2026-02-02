use std::{
  fs::{self, OpenOptions}, io::Write, time::{SystemTime, UNIX_EPOCH}
};
use bcrypt::{
  hash, 
  verify, 
  DEFAULT_COST
};

use serde::{
  Deserialize, 
  Serialize
};
use jsonwebtoken::{
  Algorithm, EncodingKey, Header, Validation, encode, decode, DecodingKey
};
use chrono::{
  DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime
};
use serde_json::Value;
use sqlx::MySqlPool;
use crate::config;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub user_id: i64,
  pub user_name: String,  
  pub exp: u64,
}

pub const SECRET_KEY: &[u8] = b"Tfn#@j0lMq2vHte%*&";

#[allow(dead_code)]
pub fn generate_token(
  user_id: i64, 
  user_name: String  
) -> Result<String, jsonwebtoken::errors::Error> {  
  let exp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() + (3600*24*3); 
  let claims = Claims {
    user_id,
    user_name,    
    exp,
  };
  let token = encode(
    &Header::new(Algorithm::HS256),
    &claims,
    &EncodingKey::from_secret(SECRET_KEY),
  )?;
  Ok(token)
}

#[allow(dead_code)]
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
  let validation = Validation::new(Algorithm::HS256);
  // validation.validate_exp = false; 
  let token_data = decode::<Claims>(
    token,
    &DecodingKey::from_secret(SECRET_KEY),
    &validation,
  )?;
  Ok(token_data.claims)
}


#[allow(dead_code)]
pub fn hash_password(plain: &str) -> Result<String, bcrypt::BcryptError> {  
  hash(plain, DEFAULT_COST)
}

#[allow(dead_code)]
pub fn hash_password_custom(password: &str, cost: u32) -> Result<String, bcrypt::BcryptError> {
  hash(password, cost)
}

#[allow(dead_code)]
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
  verify(password, hash)
}


#[allow(dead_code)]
pub fn convert_js_date_to_local_str(js_date: &str) -> Result<String, chrono::ParseError> { 
  let dt_with_offset: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(js_date)?;
  let naive: NaiveDateTime = dt_with_offset.naive_local();
  Ok(naive.format("%Y-%m-%d %H:%M:%S").to_string())
}

#[allow(dead_code)]
pub async fn validate_tables(pool: &MySqlPool, tables: &[String]) -> Vec<String> {
  let mut valid_tables: Vec<String> = Vec::new();
  for table in tables {
    let query = format!("SHOW TABLES LIKE '{}'", table);
    if let Ok(row) = sqlx::query(&query).fetch_optional(pool).await {
      if row.is_some() {
        valid_tables.push(table.clone());
      } else {
        // Todo: Munculkan log di folder logs, jika table tidak ditemukan
        eprintln!("Tabel tidak ditemukan: {}", table);
      }
    }
  }
  return valid_tables;
}

#[allow(dead_code)]
fn extract_value_from_row(row: &sqlx::mysql::MySqlRow, i: usize) -> Value {
  use sqlx::Row;
  
  if let Ok(v) = row.try_get::<Option<NaiveDate>, _>(i) {
    return v.map(|d| Value::String(d.format("%Y-%m-%d").to_string())).unwrap_or(Value::Null);
  }
  
  if let Ok(v) = row.try_get::<Option<NaiveDateTime>, _>(i) {
    return v.map(|dt| Value::String(dt.format("%Y-%m-%d %H:%M:%S").to_string())).unwrap_or(Value::Null);
  }
  
  if let Ok(v) = row.try_get::<Option<String>, _>(i) {
    return v.map(Value::String).unwrap_or(Value::Null);
  }
  
  if let Ok(v) = row.try_get::<Option<i32>, _>(i) {
    return v.map(|n| Value::Number(n.into())).unwrap_or(Value::Null);
  }
  
  if let Ok(v) = row.try_get::<Option<u32>, _>(i) {
    return v.map(|n| Value::Number(n.into())).unwrap_or(Value::Null);
  }
  
  if let Ok(v) = row.try_get::<Option<i64>, _>(i) {
    return v.map(|n| Value::Number(n.into())).unwrap_or(Value::Null);
  }
  
  if let Ok(v) = row.try_get::<Option<u64>, _>(i) {
    return v.map(|n| Value::Number(serde_json::Number::from(n))).unwrap_or(Value::Null);
  }
  
  if let Ok(v) = row.try_get::<Option<f32>, _>(i) {
    return v
      .and_then(|n| serde_json::Number::from_f64(n as f64))
      .map(Value::Number)
      .unwrap_or(Value::Null);
  }
  
  if let Ok(v) = row.try_get::<Option<f64>, _>(i) {
    return v
      .and_then(|n| serde_json::Number::from_f64(n))
      .map(Value::Number)
      .unwrap_or(Value::Null);
  }
  
  if let Ok(v) = row.try_get::<Option<Vec<u8>>, _>(i) {
    return v.map(|b| Value::String(String::from_utf8_lossy(&b).to_string())).unwrap_or(Value::Null);
  }

  eprintln!("⚠️ Gagal ekstrak kolom ke-{}", i);
  Value::Null
}

#[allow(dead_code)]
pub fn log_error_to_file(filename: &str, msg: &str) {
  let mc = config::load_master_configs();  
  let log_file = format!("{}/{}.log", mc.logs, filename);
  let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(log_file)
    .expect("Failed to open log file");

  let timestamp = Local::now();
  writeln!(file, "[{}] ERROR: {}", timestamp.format("%Y-%m-%d %H:%M:%S"), msg)
    .expect("Failed to write log");
}

#[allow(dead_code)]
fn save_json_to_file(table_name: &str, json_value: &Value) {
  let mc = config::load_master_configs();  
  let path = format!("{}/{}.txt", mc.results, table_name);
  let json_pretty = serde_json::to_string_pretty(json_value).unwrap_or_else(|_| "[]".to_string());
  if let Err(e) = fs::write(&path, json_pretty) {
    eprintln!("Gagal menulis file {}: {}", path, e);
  } else {
    println!("✅ Tersimpan: {}", path);
  }
}