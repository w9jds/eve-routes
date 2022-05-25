mod universe;

use futures::{future::join_all};
use universe::{get_route_type, Universe};
use wasm_bindgen::prelude::*;

static matrix: &str = include_str!("../data/system_map.json");

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub async fn calc_route(start: u32, destination: u32, route_type: &str, avoid: Vec<u32>) -> String  {
  let map = Universe::new(matrix);
  let weight = get_route_type(route_type);

  let route = map.route(&start, &destination, weight, avoid.clone()).await.unwrap();

  serde_json::to_string(&route).unwrap()
}

#[wasm_bindgen]
pub async fn calc_routes(start: u32, destinations: Vec<u32>, route_type: &str, avoid: Vec<u32>) -> String {
  let map = Universe::new(matrix);
  let weight = get_route_type(route_type);
  let mut futures = vec!();

  for id in destinations {
    futures.push(map.route(&start, &end, weight, avoid.clone()));
  }

  let routes: Vec<Vec<u32>> = join_all(futures).await
    .into_iter()
    .map(|result| result.unwrap())
    .collect();

  serde_json::to_string(&routes).unwrap()
}

