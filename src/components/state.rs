use super::{planet::Planet, player::Player};



pub struct GameState {
  pub planets: Vec<Planet>,
  pub player: Player,
  pub turn: i32,
}