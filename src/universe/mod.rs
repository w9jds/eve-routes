pub mod solar_system;

use solar_system::SolarSystem;
use std::collections::HashMap;
use pathfinding::prelude::{astar};
use std::error::Error;

#[derive(Debug, Copy, Clone)]
pub enum RouteType {
  Safest,
  Shortest,
  LessSafe,
}

pub fn get_route_type(input: String) -> RouteType {
  match input.as_str() {
    "secure" => RouteType::Safest,
    "less-safe" => RouteType::LessSafe,
    _ => RouteType::Shortest,
  }
}

pub struct Universe {
  pub systems: HashMap<u32, SolarSystem>
}

impl Universe {
  pub fn new(input: &str) -> Universe {
    Universe{
      systems: serde_json::from_str(input).unwrap()
    }
  }

  fn weight(&self, id: &u32, weight: RouteType) -> u32 {
    let system = self.systems.get(id).unwrap();
    system.calculate_weight(weight)
  }

  pub fn add_connections(&mut self, connections: &Vec<(u32, u32)>) {
    for (left, right) in connections {
      if self.systems.contains_key(&left) {
        let system = self.systems.get_mut(&left).unwrap();
        if !system.neighbors.contains(&left) {
          system.neighbors.push(left.clone());
        }
      }
      if self.systems.contains_key(&right) {
        let system = self.systems.get_mut(&right).unwrap();
        if !system.neighbors.contains(&right) {
          system.neighbors.push(right.clone());
        }
      }
    }
  }

  pub fn distance(&self, start: &u32, end: &u32) -> f64 {
    let left = self.systems.get(start).unwrap().coord();
    let right = self.systems.get(end).unwrap().coord();

    (((right.0 - left.0).powf(2.0)) + ((right.1 - left.1).powf(2.0)) + ((right.2 - left.2).powf(2.0))).sqrt()
  }

  pub fn successors(&self, id: &u32, weight: RouteType, avoid: Vec<u32>) -> Vec<(u32, u32)> {
    let system = self.systems.get(id).unwrap();
    system.filter_neighbors(avoid)
      .clone()
      .into_iter()
      .map(|id| (id, self.weight(&id, weight)))
      .collect()
  }

  pub fn calculate_route(&self, start: &u32, end: &u32, weight: RouteType, avoid: Vec<u32>) -> Vec<u32> {
    let map = self.clone();
    let optimal_route = map.distance(start, end);

    let result = astar(start, |id| map.successors(id, weight, avoid.clone()), |id| {
       let difference = optimal_route - map.distance(id, end);
       (format!("{}", difference).chars().count()) as u32
    }, |id| id == end);

    if result.is_some() {
      result.unwrap().0
    } else {
      vec!()
    }
  }

  // pub fn calculate_route_d(&self, start: &u32, end: &u32, weight: RouteType, avoid: Vec<u32>) -> Vec<u32> {
  //   let map = self.clone();


  //   vec!()
  // }

  pub async fn route(&self, start: u32, end: u32, weight: RouteType, avoid: Vec<u32>) -> Result<Vec<u32>, ()> {
    let destination = end.to_owned();
    Ok(self.calculate_route(&start, &destination, weight, avoid))
  }
}

