mod universe;

use std::sync::Arc;
use futures::{future::{join_all, Future}};
use universe::{get_route_type, RouteType, Universe};
use wasm_bindgen::prelude::*;

static matrix: &str = include_str!("../data/system_map.json");

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub async fn calc_route(start: u32, destination: u32, route_type: String, avoid: Vec<u32>) -> String  {
  let map = Universe::new(matrix);
  let weight = get_route_type(route_type);

  let route = map.route(start, destination, weight, avoid.clone()).await.unwrap();

  serde_json::to_string(&route).unwrap()
}

#[wasm_bindgen]
pub async fn calc_routes(start: u32, destinations: Vec<u32>, route_type: String, avoid: Vec<u32>) -> String {
  let map = Universe::new(matrix);
  let weight = get_route_type(route_type);
  let mut futures = vec!();
  let ends = destinations.to_owned();

  for id in ends {
    futures.push(map.route(start, id, weight, avoid.clone()))
  }

  let routes: Vec<Vec<u32>> = join_all(futures).await
    .into_iter()
    .map(|result: Result<Vec<u32>, ()>| result.unwrap())
    .collect();

  serde_json::to_string(&routes).unwrap()
}
