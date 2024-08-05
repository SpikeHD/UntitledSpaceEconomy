use std::collections::HashMap;
use macroquad::texture::Texture2D;

use super::crew_member::CrewMember;

pub struct Ship {
  pub sprite: Texture2D,
  pub fuel: f32,
  pub inventory: HashMap<i32, i32>,
  pub crew: Vec<CrewMember>,
}