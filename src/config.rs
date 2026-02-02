use serde::{
  // Serialize, 
  Deserialize
};
use std::{
  fs
}; 

#[derive(Deserialize, Debug)]
pub struct DbConfig {
  #[serde(rename = "DB_HOST")]
  pub host: String,
  #[serde(rename = "DB_USER")]
  pub user: String,
  #[serde(rename = "DB_PASS")]
  pub pass: String,
  #[serde(rename = "DB_NAME")]
  pub name: String,
  #[serde(rename = "DB_PORT")]
  pub port: String,
}

#[derive(Deserialize, Debug)]
pub struct MasterConfig {
  #[serde(rename = "HTTP_PORT")]
  pub http_port: String,
  #[serde(rename = "KA")]
  pub ka: String,
  #[serde(rename = "LOGS")]
  pub logs: String,  
  #[serde(rename = "RESULTS")]
  pub results: String,  
}

// pub fn load_db_configs() -> Vec<DbConfig> {
//   let data = fs::read_to_string("db_conf.txt")
//     .expect("Gagal membaca db_conf.txt");
//   return serde_json::from_str(&data).expect("Gagal parsing JSON db_conf.txt");
// }

pub fn load_db_configs() -> DbConfig {
  let data = fs::read_to_string("db_conf.txt")
    .expect("Gagal membaca db_conf.txt");
  return serde_json::from_str(&data).expect("Gagal parsing JSON db_conf.txt");
}

pub fn load_master_configs() -> MasterConfig {
  let data = fs::read_to_string("master_config.txt")
    .expect("Gagal membaca master_config.txt");
  return serde_json::from_str(&data).expect("Gagal parsing JSON master_config.txt");
}

/*
pub fn load_tables() -> Vec<String> {
  let data = fs::read_to_string("tables.txt")
    .expect("Gagal membaca tables.txt");
  return serde_json::from_str(&data).expect("Gagal parsing JSON tables"); 
}
*/
