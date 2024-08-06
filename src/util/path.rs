use std::path::PathBuf;

use dirs;

pub fn is_not_installed() -> bool {
  // Check if we have a "build" folder next to the executable
  let current_exe = std::env::current_exe().unwrap();
  let exe_folder = current_exe.parent().unwrap();
  let build_folder = exe_folder.join("build");

  build_folder.exists()
}

pub fn game_folder() -> PathBuf {
  // For Windows, this is Documents/My Games/[name]
  // For Linux, this is ~/.local/share/[name]
  // For macOS, this is ~/Library/Application Support/[name]
  #[cfg(target_os = "windows")]
  let path = dirs::document_dir().unwrap().join("My Games").join("space_game");
  #[cfg(target_os = "linux")]
  let path = dirs::data_local_dir().unwrap().join("space_game");
  #[cfg(target_os = "macos")]
  let path = dirs::data_local_dir().unwrap().join("space_game");

  if !path.exists() {
    std::fs::create_dir_all(&path).unwrap();
  }

  path
}

pub fn config_path() -> PathBuf {
  game_folder().join("config.json")
}

pub fn lang_path() -> PathBuf {
  let asset_folder = asset_path();
  asset_folder.join("lang")
}

pub fn log_file_path() -> PathBuf {
  game_folder().join("log.txt")
}

pub fn asset_path() -> PathBuf {
  if is_not_installed() {
    let current_folder = std::env::current_dir().unwrap();
    return current_folder.join("assets");
  }

  let current_exe = std::env::current_exe().unwrap();
  let exe_folder = current_exe.parent().unwrap();
  exe_folder.join("assets")
}