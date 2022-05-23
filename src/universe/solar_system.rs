use serde::Deserialize;
use crate::universe::RouteType;

#[derive(Deserialize, Debug, Clone)]
pub struct SolarSystem {
  pub name: String,
  pub security: f64,
  pub neighbors: Vec<u32>,
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl SolarSystem {

  pub fn coord(&self) -> (f64, f64, f64) {
    (self.x, self.y, self.z)
  }

  pub fn calculate_weight(&self, weight: RouteType) -> u32 {
    match weight {
      RouteType::Shortest => 1,
      RouteType::Safest => if self.security < 0.45 { 50000 } else { 1 },
      RouteType::LessSafe => if self.security >= 0.45 { 50000 } else { 1 },
    }
  }

  pub fn filter_neighbors(&self, avoid: Vec<u32>) -> Vec<u32> {
    self.neighbors
      .clone()
      .into_iter()
      .filter(|id| !avoid.contains(id))
      .collect()
  }

}
