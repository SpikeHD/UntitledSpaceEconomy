use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::crew_member::CrewMember;

#[derive(Serialize, Deserialize)]
pub struct Ship {
  pub spec: ShipSpecification,
  pub fuel: f32,
  pub inventory: HashMap<i32, i32>,
  pub crew: Vec<CrewMember>,
}

#[derive(Serialize, Deserialize)]
pub struct ShipSpecification {
  pub name: String,
  pub max_fuel: f32,
  pub max_inventory: i32,
  pub max_crew: i32,
}