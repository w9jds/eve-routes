mod universe;

extern crate console_error_panic_hook;
use std::panic;

use futures::{future::{join_all}};
use universe::{get_route_type, Universe};
use wasm_bindgen::prelude::*;

static MATRIX: &str = include_str!("../data/system_map.json");

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn init() {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub async fn calc_route(start: u32, destination: u32, route_type: String, avoid: Vec<u32>) -> String  {
  let map = Universe::new(MATRIX);
  let weight = get_route_type(route_type);

  let route = map.route(start, destination, weight, avoid.clone()).await.unwrap();

  serde_json::to_string(&route).unwrap()
}

#[wasm_bindgen]
pub async fn calc_routes(start: u32, destinations: Vec<u32>, route_type: String, additions: Option<String>, avoid: Option<Vec<u32>>) -> String {
  let mut map = Universe::new(MATRIX);
  let mut futures = vec!();

  let ignore = if avoid.is_some() { avoid.unwrap() } else { vec!() };
  let weight = get_route_type(route_type);

  if additions.is_some() {
    let connections: Vec<(u32, u32)> = serde_json::from_str(additions.unwrap().as_str()).unwrap();
    map.add_connections(&connections);
  }

  for id in destinations {
    futures.push(map.route(start, id, weight, ignore.clone()))
  }

  let routes: Vec<Vec<u32>> = join_all(futures).await
    .into_iter()
    .map(|result: Result<Vec<u32>, ()>| result.unwrap())
    .collect();

  serde_json::to_string(&routes).unwrap()
}
