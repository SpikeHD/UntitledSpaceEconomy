use components::core::Core;
use macroquad::prelude::*;
use states::Stage;
use util::{config, language, logger};

mod components;
mod states;
mod util;

#[macroquad::main("Untitled Space Economy")]
async fn main() {
  logger::init(true);

  let mut core = Core::new();
  let config = config::read_config();

  log!("Core initialized");
  log!("Loading language file: {}", core.language_file);

  core.language_file = config.language.clone();
  
  let loaded_language = language::try_load_language(&mut core);
  if loaded_language.is_err() {
    log!("Failed to load language file: {}", core.language_file);

    // TODO do something about this
    // right now it doesn't matter, there isn't any text being used anywhere
  }

  log!("Language file loaded: {}", core.language_file);

  loop {
    clear_background(BLACK);

    match core.current_state {
      Stage::MainMenu => states::main_menu::draw(&core),
      Stage::ShipSelect => states::ship_select::draw(&core),
      Stage::Game => states::game::draw(&core),
    }

    next_frame().await
  }
}
