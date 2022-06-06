pub mod solar_system;
pub mod connection;

use wasm_bindgen::prelude::*;
use std::error::Error;
use std::collections::HashMap;
use solar_system::SolarSystem;
use connection::Connection;
use pathfinding::prelude::{astar, dijkstra};
use serde_json::from_str;

#[wasm_bindgen]
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

  pub async fn new(input: &str, connections: &Option<Vec<Connection>>) -> Result<Universe, Box<dyn Error>> {
    let mut map = Universe{
      systems: serde_json::from_str(input)?
    };

    if connections.is_some() {
      map.add_connections(&connections.as_ref().unwrap());
    }

    Ok(map)
  }

}

impl Universe {
  fn weight(&self, id: &u32, weight: RouteType) -> u32 {
    let system = self.systems.get(id).unwrap();
    system.calculate_weight(weight)
  }

  pub fn add_connection(&mut self, (left, right): (&u32, &u32)) {
    if self.systems.contains_key(&left) {
      let system = self.systems.get_mut(&left).unwrap();
      if !system.neighbors.contains(&right) {
        system.neighbors.push(right.clone());
      }
    }
    if self.systems.contains_key(&right) {
      let system = self.systems.get_mut(&right).unwrap();
      if !system.neighbors.contains(&left) {
        system.neighbors.push(left.clone());
      }
    }
  }

  pub fn add_connections(&mut self, connections: &Vec<Connection>) {
    for connection in connections {
      self.add_connection(connection.to_pair());
    }
  }

  fn distance(&self, start: &u32, end: &u32) -> f64 {
    let left = self.systems.get(start).unwrap().coord();
    let right = self.systems.get(end).unwrap().coord();

    (((right.0 - left.0).powf(2.0)) + ((right.1 - left.1).powf(2.0)) + ((right.2 - left.2).powf(2.0))).sqrt()
  }

  fn successors(&self, id: &u32, weight: RouteType, avoid: Vec<u32>) -> Vec<(u32, u32)> {
    let system = self.systems.get(id).unwrap();
    system.filter_neighbors(avoid)
      .clone()
      .into_iter()
      .map(|id| (id, self.weight(&id, weight)))
      .collect()
  }

  pub async fn calculate_weighted_route(&self, start: u32, end: u32, weight: RouteType, avoid: Vec<u32>) -> Result<Vec<u32>, ()> {
    let optimal_route = self.distance(&start, &end);
    let result = astar(
      &start,
      |id| self.successors(id, weight, avoid.clone()),
      |id| {
        let difference = optimal_route - self.distance(id, &end);
        (format!("{}", difference).chars().count()) as u32
      },
      |id| id == &end
    );

    if result.is_some() {
      Ok(result.unwrap().0)
    } else {
      Ok(vec!())
    }
  }

  pub async fn calculate_route(&self, start: u32, end: u32, weight: RouteType, avoid: Vec<u32>) -> Result<Vec<u32>, ()> {
    let result = dijkstra(
      &start,
      |id| self.successors(id, weight, avoid.clone()),
      |id| id == &end
    );

    if result.is_some() {
      Ok(result.unwrap().0)
    } else {
      Ok(vec!())
    }
  }

}

