use serde::{Deserialize, Serialize};
use super::path::config_path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub language: String,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      language: "en".to_string(),
    }
  }
}

pub fn read_config() -> Config {
  let path = config_path();
  if !path.exists() {
    write_config(&Config::default());
  }

  let file = std::fs::File::open(&path).unwrap();
  serde_json::from_reader(file).unwrap()
}

pub fn write_config(config: &Config) {
  let path = config_path();
  let file = std::fs::File::create(&path).unwrap();
  serde_json::to_writer(file, config).unwrap();
}

