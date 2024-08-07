use components::{core::Core, item::ITEMS};
use macroquad::prelude::*;
use states::Stage;
use util::{config::{self, write_config}, generators::{generate_name, NameGenerationParams}, language, logger};

mod components;
mod states;
mod util;

#[macroquad::main("Untitled Space Economy")]
async fn main() {
  logger::init(true);

  let mut core = Core::new();
  let mut config = config::read_config();

  log!("Core initialized");

  if config.language.is_empty() {
    log!("Invalid language config, defaulting to English");
    config.language = "en".to_string();

    write_config(&config);
  }

  log!("Loading language file: {}", config.language);

  core.language_file = config.language.clone();
  
  let loaded_language = language::try_load_language(&mut core);
  if loaded_language.is_err() {
    log!("Failed to load language file: {}", core.language_file);

    // TODO do something about this
    // right now it doesn't matter, there isn't any text being used anywhere
  }

  log!("Language file loaded: {}", core.language_file);
  log!("Read {} items", ITEMS.len());

  loop {
    clear_background(BLACK);

    log!("Current stage: {:?}", core.current_stage);

    // Each state handles next_frame() itself
    let result = match core.current_stage {
      Stage::MainMenu => states::main_menu::draw(&mut core).await,
      Stage::ShipSelect => states::ship_select::draw(&mut core).await,
      Stage::SystemView => states::game::draw_system(&mut core).await,
      Stage::PlanetView => states::game::draw_planet(&mut core).await,
      Stage::POIView => states::game::draw_poi(&mut core).await,

      Stage::Error => {
        // We handle this elsewhere
        Ok(())
      },
    };

    if result.is_err() {
      let err = result.err().unwrap();
      log!("Error in main loop: {:?}", err);
      core.errors.push(
        format!("{:?}: {}", err.kind(), err.to_string())
      );
      core.current_stage = Stage::Error;
    }

    if core.current_stage == Stage::Error {
      states::error::draw(&core).await.unwrap();
    }
  }
}
