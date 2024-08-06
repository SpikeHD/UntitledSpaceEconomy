use macroquad::{prelude::*, ui::{root_ui, widgets}};

use crate::components::core::Core;

pub async fn draw_system(core: &mut Core) -> Result<(), std::io::Error> {
  let state = core.state.as_mut().unwrap();
  let mut transition = false;

  loop {
    let mouse_pos = mouse_position();
    
    clear_background(BLACK);

    // Debug core info on screen
    draw_text(format!("Player name: {}", state.player.name).as_str(), 10.0, 10.0, 16.0, WHITE);
    draw_text(format!("Money: {}", state.player.money).as_str(), 10.0, 20.0, 16.0, WHITE);
    draw_text(format!("Current planet: {}", state.current_planet).as_str(), 10.0, 30.0, 16.0, WHITE);
    draw_text("Viewing system", 10.0, 40.0, 16.0, WHITE);
    draw_text(format!("Planets in system: {}", state.planets.len()).as_str(), 10.0, 50.0, 16.0, WHITE);

    // Draw each planet as a circle. It's coordinates are 0-100, a percentage basically
    for (i, planet) in state.planets.iter().enumerate() {
      let x = planet.x as f32 * screen_width() / 100.;
      let y = planet.y as f32 * screen_height() / 100.;

      draw_circle(x, y, 10.0, WHITE);

      if mouse_pos.0 >= x - 10. && mouse_pos.0 <= x + 10. && mouse_pos.1 >= y - 10. && mouse_pos.1 <= y + 10. {
        if is_mouse_button_down(MouseButton::Left) {
          core.current_stage = crate::states::Stage::PlanetView;
          state.current_planet = i as i32;
          
          transition = true
        }

        // Draw the name of the planet to the left of the mouse
        draw_text(planet.name.as_str(), mouse_pos.0 + 10., mouse_pos.1, 16.0, WHITE);
      }
    }

    if transition {
      break;
    }

    next_frame().await
  }

  Ok(())
}

pub async fn draw_planet(core: &mut Core) -> Result<(), std::io::Error> {
  let state = core.state.as_mut().unwrap();
  let mut transition = false;

  loop {
    let mouse_pos = mouse_position();

    clear_background(BLACK);

    // Debug core info on screen
    draw_text(format!("Player name: {}", state.player.name).as_str(), 10.0, 10.0, 16.0, WHITE);
    draw_text(format!("Money: {}", state.player.money).as_str(), 10.0, 20.0, 16.0, WHITE);
    draw_text(format!("Current planet: {}", state.current_planet).as_str(), 10.0, 30.0, 16.0, WHITE);
    draw_text(format!("Planet: {}", state.planets[state.current_planet as usize].name).as_str(), 10.0, 40.0, 16.0, WHITE);
    draw_text(format!("POIs on planet: {}", state.planets[state.current_planet as usize].poi.len()).as_str(), 10.0, 50.0, 16.0, WHITE);

    // Draw crude back button using button and < symbol
    let back_btn = widgets::Button::new("<")
      .position(vec2(0., 0.))
      .size(vec2(20., 20.))
      .ui(&mut *root_ui());

    if back_btn {
      core.current_stage = crate::states::Stage::SystemView;
      break;
    }

    // Draw each POI as a square. It's coordinates are 0-100, a percentage basically
    for (i, poi) in state.planets[state.current_planet as usize].poi.iter().enumerate() {
      let x = poi.x as f32 * screen_width() / 100.;
      let y = poi.y as f32 * screen_height() / 100.;

      draw_rectangle(x - 10., y - 10., 20., 20., WHITE);

      if mouse_pos.0 >= x - 10. && mouse_pos.0 <= x + 10. && mouse_pos.1 >= y - 10. && mouse_pos.1 <= y + 10. {
        if is_mouse_button_down(MouseButton::Left) {
          core.current_stage = crate::states::Stage::POIView;
          state.current_poi = i as i32;
          
          transition = true
        }

        // Draw the name of the POI to the left of the mouse
        draw_text(poi.name.as_str(), mouse_pos.0 + 10., mouse_pos.1, 16.0, WHITE);
      }
    }

    if transition {
      break;
    }

    next_frame().await
  }

  Ok(())
}

pub async fn draw_poi(core: &mut Core) -> Result<(), std::io::Error> {
  let state = core.state.as_ref().unwrap();

  loop {
    clear_background(BLACK);

    // Debug core info on screen
    draw_text(format!("Player name: {}", state.player.name).as_str(), 10.0, 10.0, 16.0, WHITE);
    draw_text(format!("Money: {}", state.player.money).as_str(), 10.0, 20.0, 16.0, WHITE);
    draw_text(format!("Current planet: {}", state.current_planet).as_str(), 10.0, 30.0, 16.0, WHITE);
    draw_text(format!("POI: {}", state.planets[state.current_planet as usize].poi[state.current_poi as usize].name).as_str(), 10.0, 40.0, 16.0, WHITE);

    // Draw crude back button using button and < symbol
    let back_btn = widgets::Button::new("<")
      .position(vec2(0., 0.))
      .size(vec2(20., 20.))
      .ui(&mut *root_ui());

    if back_btn {
      core.current_stage = crate::states::Stage::PlanetView;
      break;
    }

    // Draw some text to the center of the screen showing the name of the POI and some debug info
    let poi = &state.planets[state.current_planet as usize].poi[state.current_poi as usize];

    draw_text(poi.name.as_str(), screen_width() / 2. - 50., screen_height() / 2. - 10., 20.0, WHITE);
    draw_text(format!("Demand: {}", poi.demand.len()).as_str(), screen_width() / 2. - 50., screen_height() / 2., 16.0, WHITE);
    draw_text(format!("Inventory: {}", poi.inventory.len()).as_str(), screen_width() / 2. - 50., screen_height() / 2. + 10., 16.0, WHITE);

    next_frame().await
  }

  Ok(())
}