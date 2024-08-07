use std::{collections::HashMap, fs};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{log, util::{generators::{generate_name, NameGenerationParams}, path::asset_path, random::{pick_one, pick_x}}};

use super::{core::Core, item::{Item, ITEMS}};

static INV_MAX_MAIN_TYPE: i32 = 1000;
static INV_MIN_MAIN_TYPE: i32 = 400;
static DEM_MAX_MAIN_TYPE: i32 = 20;
static DEM_MIN_MAIN_TYPE: i32 = 1;

static INV_MAX_SUB_TYPE: i32 = 600;
static INV_MIN_SUB_TYPE: i32 = 200;
static DEM_MAX_SUB_TYPE: i32 = 30;
static DEM_MIN_SUB_TYPE: i32 = 5;

static INV_MAX_ANY: i32 = 300;
static INV_MIN_ANY: i32 = 100;
static DEM_MAX_ANY: i32 = 70;
static DEM_MIN_ANY: i32 = 30;

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
  pub inventory: HashMap<u32, i32>,
  /// Demand for each item, as a percentage (ie 1-100)
  pub demand: HashMap<u32, i32>,
  pub x: i32,
  pub y: i32,

  pub types: (POIType, POIType),
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

  pub fn avg_demand(&self) -> HashMap<u32, i32> {
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

  pub fn avg_prices(&self, core: &Core) -> HashMap<u32, i32> {
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
    let types = pick_x(get_all_poi_types().unwrap_or_else(|_| {
      log!("Failed to get POI types");
      Vec::new()
    }), false, 2);

    let mut poi = PointOfInterest {
      name: generate_name(NameGenerationParams::default()),
      description: "A point of interest".to_string(),
      inventory: HashMap::new(),
      demand: HashMap::new(),
      x,
      y,
      types: (types[0].clone(), types[1].clone()),
    };

    poi.generate_inventory();
    poi.generate_demand();

    poi
  }

  pub fn generate_inventory(&mut self) {
    let mut rng = rand::thread_rng();
    let mut inventory = HashMap::new();

    // First iterate the items that fit the same category as the POI
    ITEMS.iter()
      .for_each(|(id, item)| {
        // Main type, big amount
        if self.types.0.categories.contains(&item.category) {
          inventory.insert(*id, rng.gen_range(INV_MIN_MAIN_TYPE..INV_MAX_MAIN_TYPE));
          return;
        } else if self.types.1.categories.contains(&item.category) {
          inventory.insert(*id, rng.gen_range(INV_MIN_SUB_TYPE..INV_MAX_SUB_TYPE));
          return;
        }

        // Other stuff, not a whole lot
        inventory.insert(*id, rng.gen_range(INV_MIN_ANY..INV_MAX_ANY));
      });

    self.inventory = inventory;
  }

  pub fn generate_demand(&mut self) {
    let mut rng = rand::thread_rng();
    let mut demand = HashMap::new();

    // First iterate the items that fit the same category as the POI
    ITEMS.iter()
      .for_each(|(id, item)| {
        log!("POI Category: {:?} | Item Category: {}", self.types.0.categories, item.category);

        // Main type, big amount
        if self.types.0.categories.contains(&item.category) {
          demand.insert(*id, rng.gen_range(DEM_MIN_MAIN_TYPE..DEM_MAX_MAIN_TYPE));
          return;
        } else if self.types.1.categories.contains(&item.category) {
          demand.insert(*id, rng.gen_range(DEM_MIN_SUB_TYPE..DEM_MAX_SUB_TYPE));
          return;
        }

        // Other stuff, not a whole lot
        demand.insert(*id, rng.gen_range(DEM_MIN_ANY..DEM_MAX_ANY));
      });

    self.demand = demand;
  }

  pub fn calculate_price(&self, item: Item) -> i32 {
    let item_id = item.id;
    let demand = self.demand.get(&item_id).unwrap_or(&0);
    let avg_demand = self.demand.values().sum::<i32>() / self.demand.len() as i32;
    let price = item.low_price + (item.high_price - item.low_price) * demand / avg_demand;

    price
  }

  /// Player buys items from POI
  pub fn buy(&mut self, core: &mut Core, item_id: u32, amount: i32) -> Result<(), std::io::Error> {
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

  pub fn sell(&mut self, core: &mut Core, item_id: u32, amount: i32) -> Result<(), std::io::Error> {
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

  // Laid out as follows:
  // NAME CATEGORY1,CATEGORY2,... DESCRIPTION
  let items = items.lines()
    .filter(|line| !line.starts_with('#'))
    .map(|line| {
      let mut parts = line.split_whitespace();
      let name = parts.next().unwrap_or("UNKNOWN").to_string();
      let categories = parts.next().unwrap_or("UNKNOWN").split(',').map(|s| s.to_string()).collect();
      let description = parts.next().unwrap_or("UNKNOWN").to_string();

      POIType {
        name,
        description,
        categories,
      }
    })
    .collect();

  Ok(items)
}