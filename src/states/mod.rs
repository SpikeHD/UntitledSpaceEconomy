#[derive(Debug, Default, PartialEq)]
pub enum Stage {
  #[default]
  MainMenu,
  ShipSelect,
  Game,

  /// Special state, ideally should never happen
  Error
}

pub mod error;
pub mod main_menu;
pub mod game;
pub mod ship_select;