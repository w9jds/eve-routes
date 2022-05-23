mod universe;

use universe::{load_universe, Universe, RouteType};
use pathfinding::prelude::{astar};

thread_local!(static universe: Universe = load_universe());

fn main() {
   universe.with(|map| {

      let end = &30003794;
      let weight = RouteType::Shortest;
      let avoid = vec![];

      let result = astar(&30001393, |id| map.successors(id, weight, avoid.clone()), |id| map.distance(id, end), |id| id == end);

      // let names: Vec<String> = result.unwrap().0
      //    .clone()
      //    .into_iter()
      //    .map(|id| map.systems.get(&id).unwrap().name)
      //    .collect();

      println!("Universe Loaded");
   })
}
