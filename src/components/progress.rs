use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Progression {
  pub achievments: Vec<i32>,
}

impl Progression {
  pub fn has_achievement(&self, achievement: i32) -> bool {
    self.achievments.contains(&achievement)
  }
}