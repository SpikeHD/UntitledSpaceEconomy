use std::collections::HashMap;

use super::{core::Core, item::Item};

pub struct Planet {
  pub name: String,
  pub population: i32,
  pub poi: Vec<PointOfInterest>,
  pub x: i32,
  pub y: i32,
}

pub struct PointOfInterest {
  pub name: String,
  pub description: String,
  /// Amounts of items
  pub inventory: HashMap<i32, i32>,
  /// Demand for each item, as a percentage (ie 1-100)
  pub demand: HashMap<i32, i32>,
  pub x: i32,
  pub y: i32,
}

impl Planet {
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