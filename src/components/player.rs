use serde::{Deserialize, Serialize};

use super::ship::Ship;

#[derive(Serialize, Deserialize)]
pub struct Player {
  pub name: String,
  pub money: i32,
  pub ship: Ship,
}

impl Player {
  pub fn new(name: String, ship: Ship) -> Player {
    Player {
      name,
      ship,
      money: 0,
    }
  }
}