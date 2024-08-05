use crate::components::core::Core;

use super::path::lang_path;

pub fn try_load_language(core: &mut Core) -> Result<(), std::io::Error> {
  let path = lang_path();
  let file = path.join(format!("{}.tr", core.language_file));
  let file = std::fs::File::open(&file)?;
  let reader = std::io::BufReader::new(file);
  let data = serde_json::from_reader(reader)?;
  
  core.language = Some(data);

  Ok(())
}

pub fn get_language_name(code: impl AsRef<str>) -> Result<String, std::io::Error> {
  let path = lang_path();
  let file = path.join(format!("{}.tr", code.as_ref()));
  let file = std::fs::File::open(&file)?;
  let reader = std::io::BufReader::new(file);
  let data: serde_json::Value = serde_json::from_reader(reader)?;

  Ok(data["LANGUAGE"].as_str().unwrap().to_string())
}

pub fn get_language_value(core: &Core, key: impl AsRef<str>) -> Result<String, std::io::Error> {
  if let Some(data) = &core.language {
    let key = key.as_ref();
    
    if let Some(value) = data[key].as_str() {
      return Ok(value.to_string());
    }
  }
  
  Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Key not found"))
}