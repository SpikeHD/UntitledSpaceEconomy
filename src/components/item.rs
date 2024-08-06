use std::fs;

use serde::{Deserialize, Serialize};

use crate::util::path::asset_path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
  pub id: i32,
  pub name: String,
  pub description: String,

  /// Lowest price this item can be
  pub low_price: i32,

  /// Highest price this item can be
  pub high_price: i32,

  pub illegal: bool,
}

pub fn get_all_items() -> Result<Vec<Item>, std::io::Error> {
  let asset_path = asset_path();
  let items = fs::read_to_string(asset_path.join("items.dat"))?;

  // Items.dat is a file with a list of items, one per line
  // They are formatted as follows:
  // NAME LOW HIGH ILLEGAL DESCRIPTION_KEY
  // Skipping lines with a # in front of course
  let items = items.lines()
    .filter(|line| !line.starts_with('#'))
    .map(|line| {
      let mut parts = line.split_whitespace();
      let id = parts.next().unwrap_or("0").parse().unwrap();
      let name = parts.next().unwrap_or("Unknown").to_string();
      let low_price = parts.next().unwrap_or("0").parse().unwrap();
      let high_price = parts.next().unwrap_or("0").parse().unwrap();
      let illegal = parts.next().unwrap_or("false").to_lowercase().parse().unwrap();
      let description = parts.next().unwrap_or("UNKNOWN_ITEM").to_string();

      Item {
        id: id,
        name,
        description,
        low_price,
        high_price,
        illegal,
      }
    })
    .collect();

  Ok(items)
}