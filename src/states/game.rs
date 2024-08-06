use macroquad::prelude::*;

use crate::components::core::Core;

pub async fn draw(core: &Core) -> Result<(), std::io::Error> {
  let state = core.state.as_ref().unwrap();

  loop {
    clear_background(BLACK);

    // Debug core info on screen
    draw_text(format!("Player name: {}", state.player.name).as_str(), 10.0, 10.0, 16.0, WHITE);
    draw_text(format!("Money: {}", state.player.money).as_str(), 10.0, 20.0, 16.0, WHITE);
    draw_text(format!("Current planet: {}", state.current_planet).as_str(), 10.0, 30.0, 16.0, WHITE);

    next_frame().await
  }



  Ok(())
}