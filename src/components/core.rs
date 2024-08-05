use crate::states::Stage;

use super::{item::Item, state::GameState};


#[derive(Default)]
pub struct Core {
  pub language: Option<serde_json::Value>,
  pub language_file: String,
  pub state: Option<GameState>,
  pub items: Vec<Item>,
  pub current_state: Stage,
}

impl Core {
  pub fn new() -> Core {
    let mut core = Core::default();
    core.language_file = "en".to_string();

    core
  }


}