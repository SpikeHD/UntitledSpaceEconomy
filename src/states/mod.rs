#[derive(Debug, Default, PartialEq)]
pub enum Stage {
  #[default]
  MainMenu,
  ShipSelect,
  Game,
}

pub mod main_menu;
pub mod game;
pub mod ship_select;