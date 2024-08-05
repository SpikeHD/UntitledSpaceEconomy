#[derive(Clone)]
pub struct Item {
  pub id: i32,
  pub name: String,
  pub description: String,

  /// Lowest price this item can be
  pub low_price: i32,

  /// Highest price this item can be
  pub high_price: i32,
}