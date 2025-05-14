mod map;
mod universe;

extern crate console_error_panic_hook;

use std::panic;
use futures::future::join_all;
use universe::{get_route_type, Universe, RouteType, connection::Connection};
use wasm_bindgen::prelude::*;
use js_sys::Uint32Array;
use cfg_if::cfg_if;

static MATRIX: &str = include_str!("../data/system_map.json");

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

pub fn set_panic_hook() {
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}

cfg_if! {
  if #[cfg(feature = "wee_alloc")] {
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
  }
}

#[wasm_bindgen]
pub async fn calc_short_route(start: u32, destination: u32, additions: JsValue) -> Result<Uint32Array, JsValue> {
  let connections: Option<Vec<Connection>> = serde_wasm_bindgen::from_value(additions)?;
  let map = Universe::new(MATRIX, &connections).unwrap();
  let weight = RouteType::Shortest;

  let route = map.calculate_route(start, destination, weight, vec!()).await.unwrap();

  Ok(Uint32Array::from(route.as_ref()))
}

#[wasm_bindgen]
pub async fn calc_weighted_routes(start: u32, destinations: Vec<u32>, route_type: String, additions: JsValue, avoid: Option<Vec<u32>>) -> Result<JsValue, JsValue> {
  let connections: Option<Vec<Connection>> = serde_wasm_bindgen::from_value(additions)?;
  let map = Universe::new(MATRIX, &connections).unwrap();
  let mut futures = vec!();

  let ignore = if avoid.is_some() { avoid.unwrap() } else { vec!() };
  let weight = get_route_type(route_type);

  for id in destinations {
    futures.push(map.calculate_weighted_route(start, id, weight, ignore.clone()))
  }

  let routes: Vec<Vec<u32>> = join_all(futures).await
    .into_iter()
    .map(|result: Result<Vec<u32>, ()>| result.unwrap())
    .collect();

  Ok(serde_wasm_bindgen::to_value(&routes)?)
}
