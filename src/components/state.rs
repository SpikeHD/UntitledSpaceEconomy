use serde::{Deserialize, Serialize};

use super::{planet::Planet, player::Player, ship::Ship};

#[derive(Serialize, Deserialize)]
pub struct GameState {
  pub planets: Vec<Planet>,
  /// Index of the current planet in the planets list
  pub current_planet: i32,
  pub player: Player,
  pub turn: i32,
}

impl GameState {
  pub fn new(name: String, ship: Ship) -> GameState {
    GameState {
      planets: Vec::new(),
      current_planet: 0,
      player: Player::new(name, ship),
      turn: 0,
    }
  }

  pub fn current_planet(&self) -> &Planet {
    &self.planets[self.current_planet as usize]
  }

  pub fn fly_to_planet(&mut self, planet_id: i32) -> bool {
    if planet_id < 0 || planet_id >= self.planets.len() as i32 {
      return false;
    }

    // TODO perform random events and such here

    self.current_planet = planet_id;
    true
  }
}