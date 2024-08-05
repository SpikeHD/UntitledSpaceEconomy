use macroquad::prelude::*;

use crate::components::core::Core;

pub async fn draw(core: &Core) -> Result<(), std::io::Error> {
  next_frame().await;
  Ok(())
}