// #[macro_use]
// extern crate lazy_static;

mod universe;

// use std::sync::{Arc,Mutex};
// use wasm_bindgen::prelude::*;
// use futures::{future::join_all};
// use universe::{load_universe, Universe, RouteType};

// lazy_static! {
//    pub static ref UNIVERSE: Arc<Mutex<Universe>> = Arc::new(Mutex::new(load_universe()));
// }

// #[tokio::main]
// async fn main() {
//    let routes = vec!(
//       (&30000142, &30005244, RouteType::Shortest, vec![]),
//       (&30000142, &30005244, RouteType::LessSafe, vec![]),
//       (&30000142, &30005244, RouteType::Safest, vec![]),
//    );

//    let results = calc_routes(routes).await;

//    let answers: Vec<Vec<u32>> = results
//       .clone()
//       .into_iter()
//       .map(|result| result.unwrap()
//       )
//       .collect();

//    let names: Vec<Vec<String>> = results
//       .clone()
//       .into_iter()
//       .map(|result| result
//          .unwrap()
//          .into_iter()
//          .map(|id| UNIVERSE.lock().unwrap().systems.get(&id).unwrap().name.clone())
//          .collect()
//       )
//       .collect();

//    println!("{:?}", answers);
//    println!("{:?}", names);
// }

// async fn calc_routes(routes: Vec<(&u32, &u32, RouteType, Vec<u32>)>) -> Vec<Result<Vec<u32>, ()>> {
//    let map = UNIVERSE.lock().unwrap();
//    let mut futures = vec!();

//    for (start, end, weight, avoid) in routes {
//       futures.push(map.calculate_route(start, end, weight, avoid));
//    }

//    join_all(futures).await
// }

fn main() {

}
