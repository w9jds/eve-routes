pub mod solar_system;

use solar_system::SolarSystem;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Copy, Clone)]
pub enum RouteType {
  Safest,
  Shortest,
  LessSafe,
}

pub struct Universe {
  pub systems: HashMap<u32, SolarSystem>
}

impl Universe {

  fn weight(&self, id: &u32, weight: RouteType) -> u32 {
    let system = self.systems.get(id).unwrap();
    system.calculate_weight(weight)
  }

  // pub fn add_connections(&self, connections: Vec<(u32, u32)>) {
  //   connections
  //     .into_iter()
  //     .for_each(|connection| {
  //       self.systems.get(&connection.0).unwrap().neighbors.push(connection.1);
  //       self.systems.get(&connection.1).unwrap().neighbors.push(connection.0);
  //     });
  // }

  pub fn distance(&self, start: &u32, end: &u32) -> u32 {
    let left = self.systems.get(start).unwrap().coord();
    let right = self.systems.get(end).unwrap().coord();

    let distance = (((right.0 - left.0).powf(2.0)) + ((right.1 - left.1).powf(2.0)) + ((right.2 - left.2).powf(2.0))).sqrt();

    (distance * 0.0000000001).round() as u32
  }

  pub fn successors(&self, id: &u32, weight: RouteType, avoid: Vec<u32>) -> Vec<(u32, u32)> {
    let system = self.systems.get(id).unwrap();
    system.filter_neighbors(avoid)
      .clone()
      .into_iter()
      .map(|id| (id, self.weight(&id, weight)))
      .collect()
  }
}

pub fn load_universe() -> Universe {
  let jump_map = fs::read_to_string("./data/system_map.json").unwrap();

  Universe{
    systems: serde_json::from_str(&jump_map).unwrap()
  }
}

