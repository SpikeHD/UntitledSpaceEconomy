use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Job {
  Pilot,
  Mechanic,
  Gunner,
  Navigator,
  Doctor,
  Misc
}

#[derive(Serialize, Deserialize)]
pub struct CrewMember {
  pub name: String,
  pub job: Job,
  /// level 1 - 5, with tier comes salary but also effectiveness
  pub tier: i32,
  pub salary: i32,
}