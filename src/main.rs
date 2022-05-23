mod universe;

use universe::{load_universe, Universe, RouteType};
use pathfinding::prelude::{astar};

thread_local!(static universe: Universe = load_universe());

fn main() {
   universe.with(|map| {

      let end = &30005244;
      let start = &30000142;
      let weight = RouteType::LessSafe;
      let avoid = vec![];

      let route = calculate_route(map, start, end, weight, avoid);

      let systems: Vec<String> = route
         .clone()
         .into_iter()
         .map(|id| map.systems.get(&id).unwrap().name.clone())
         .collect();

      println!("Universe Loaded");
   })
}

fn calculate_route(map: &Universe, start: &u32, end: &u32, weight: RouteType, avoid: Vec<u32>) -> Vec<u32> {
   let optimal_route = map.distance(start, end);

   let result = astar(start, |id| map.successors(id, weight, avoid.clone()), |id| {
      let difference = optimal_route - map.distance(id, end);
      (format!("{}", difference).chars().count()) as u32
   }, |id| id == end);

   result.unwrap().0
}

fn heuristic() {

}

