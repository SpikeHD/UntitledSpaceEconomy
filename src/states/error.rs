use macroquad::prelude::*;

use crate::components::core::Core;

pub async fn draw(core: &Core) -> Result<(), std::io::Error>  {
  loop {
    clear_background(BLUE);

    draw_text("An unrecoverable error occurred:", 10.0, 10.0, 20.0, WHITE);

    // Core.errors is an array, so we should draw all of them
    for (i, error) in core.errors.iter().enumerate() {
      let error_xy = vec2(10.0, 40.0 + (i as f32 * 20.0));
      draw_text(&error, error_xy.x, error_xy.y, 20.0, WHITE);
    }

    next_frame().await
  }

  Ok(())
}