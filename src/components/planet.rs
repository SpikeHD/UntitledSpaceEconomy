use std::{collections::HashMap, fs};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{log, util::{generators::{generate_name, NameGenerationParams}, path::asset_path, random::{pick_one, pick_x}}};

use super::{core::Core, item::{Item, ITEMS}};

#[derive(Serialize, Deserialize, Clone)]
pub enum SecurityLevel {
  Low,
  Medium,
  High,
}

#[derive(Serialize, Deserialize)]
pub struct Planet {
  pub name: String,
  pub population: i64,
  pub poi: Vec<PointOfInterest>,
  pub x: i32,
  pub y: i32,

  pub security: SecurityLevel,
}

#[derive(Serialize, Deserialize)]
pub struct PointOfInterest {
  pub name: String,
  pub description: String,
  /// Amounts of items
  pub inventory: HashMap<i32, i32>,
  /// Demand for each item, as a percentage (ie 1-100)
  pub demand: HashMap<i32, i32>,
  pub x: i32,
  pub y: i32,

  pub types: Vec<POIType>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct POIType {
  pub name: String,
  pub categories: Vec<String>,
  pub description: String,
}

impl Planet {
  pub fn generate() -> Planet {
    let mut rng = rand::thread_rng();

    // Create 1-4 random POI
    let poi: Vec<PointOfInterest> = (0..rng.gen_range(1..4))
      .map(|_| PointOfInterest::generate())
      .collect();

    log!("Generated planet with {} POI", poi.len());

    let x = rng.gen_range(0..100);
    let y = rng.gen_range(0..100);

    Planet {
      name: generate_name(NameGenerationParams::default()),
      population: rng.gen_range(100_000..10_000_000_000),
      poi,
      x,
      y,
      security: pick_one(vec![SecurityLevel::Low, SecurityLevel::Medium, SecurityLevel::High]),
    }
  }

  pub fn avg_demand(&self) -> HashMap<i32, i32> {
    let mut avg_demand = HashMap::new();
    for poi in &self.poi {
      for (key, value) in &poi.demand {
        let entry = avg_demand.entry(*key).or_insert(0);
        *entry += value;
      }
    }
    for (_, value) in &mut avg_demand {
      *value /= self.poi.len() as i32;
    }
    avg_demand
  }

  pub fn avg_prices(&self, core: &Core) -> HashMap<i32, i32> {
    let mut avg_prices = HashMap::new();
    for poi in &self.poi {
      for (key, _) in &poi.inventory {
        let item = core.items.iter().find(|item| item.id == *key).unwrap();
        let entry = avg_prices.entry(*key).or_insert(0);
        *entry += poi.calculate_price(item.clone());
      }
    }
    for (_, value) in &mut avg_prices {
      *value /= self.poi.len() as i32;
    }
    avg_prices
  }
}

impl PointOfInterest {
  pub fn generate() -> PointOfInterest {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..100);
    let y = rng.gen_range(0..100);

    PointOfInterest {
      name: generate_name(NameGenerationParams::default()),
      description: "A point of interest".to_string(),
      inventory: HashMap::new(),
      demand: HashMap::new(),
      x,
      y,
      types: pick_x(get_all_poi_types().unwrap_or_else(|_| {
        log!("Failed to get POI types");
        Vec::new()
      }), false, 2),
    }
  }

  pub fn calculate_price(&self, item: Item) -> i32 {
    let item_id = item.id;
    let demand = self.demand.get(&item_id).unwrap_or(&0);
    let avg_demand = self.demand.values().sum::<i32>() / self.demand.len() as i32;
    let price = item.low_price + (item.high_price - item.low_price) * demand / avg_demand;

    price
  }

  /// Player buys items from POI
  pub fn buy(&mut self, core: &mut Core, item_id: i32, amount: i32) -> Result<(), std::io::Error> {
    if core.state.is_none() {
      return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Player not found"));
    }

    let state = core.state.as_mut().unwrap();
    let player = &mut state.player;
    let item = core.items.iter().find(|item| item.id == item_id).unwrap();

    // Ensure POI has enough items
    let entry = self.inventory.entry(item_id).or_insert(0);
    if *entry < amount {
      return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Not enough to sell"));
    }

    // Converting to integer is intentional
    let price = self.calculate_price(item.clone()) as i32 * amount;

    if player.money < price {
      return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Not enough money"));
    }

    let entry = self.inventory.entry(item_id).or_insert(0);
    *entry -= amount;

    player.money -= price;

    Ok(())
  }

  pub fn sell(&mut self, core: &mut Core, item_id: i32, amount: i32) -> Result<(), std::io::Error> {
    if core.state.is_none() {
      return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Player not found"));
    }

    let state = core.state.as_mut().unwrap();
    let player = &mut state.player;
    let item = core.items.iter().find(|item| item.id == item_id).unwrap();

    let price = self.calculate_price(item.clone()) as i32 * amount;

    let entry = self.inventory.entry(item_id).or_insert(0);
    if *entry < amount {
      return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Not enough items"));
    }

    *entry += amount;

    player.money += price;

    Ok(())
  }
}

pub fn get_all_poi_types() -> Result<Vec<POIType>, std::io::Error> {
  let asset_path = asset_path();
  let items = fs::read_to_string(asset_path.join("poi_types.dat"))?;

  let items = items.lines()
    .filter(|line| !line.starts_with('#'))
    .map(|line| {
      let mut parts = line.split_whitespace();
      let name = parts.next().unwrap_or("UNKNOWN").to_string();
      let description = parts.next().unwrap_or("UNKNOWN").to_string();
      let categories = parts.map(|part| part.to_string()).collect();

      POIType {
        name,
        description,
        categories,
      }
    })
    .collect();

  Ok(items)
}