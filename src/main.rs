mod universe;

// use std::sync::{Arc,Mutex};
// use wasm_bindgen::prelude::*;
use futures::{future::join_all};
use universe::{Universe, RouteType};

static MATRIX: &str = include_str!("../data/system_map.json");

// #[tokio::main]
fn main() {
  let mut map = Universe::new(MATRIX);

  // "30000142", "30002187", "30003794", "30002053", "30005063", "30002512"

  let start = 31001257;
  let destination = 30002512;
  let ignore = vec!();

  let weight = RouteType::Shortest;

  let additions = "[[30003877,30004617],[30003877,31000542],[31000542,31001756],[31000542,30002518],[31000542,31001257],[31001756,31001464],[31001464,31002326],[31001464,31000129],[31000129,31002146],[31002146,30003231],[31002146,30002639]]";
  let connections: Vec<(u32, u32)> = serde_json::from_str(additions).unwrap();
  map.add_connections(&connections);

  let routes = map.calculate_route(&start, &destination, weight, ignore.clone());

  println!("{:?}", routes);
}


