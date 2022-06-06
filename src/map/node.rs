


pub struct Node {
  pub name: String,
  pub neighbors: Vec<String>,
}

impl Node {

  pub fn filter_neighbors(&self, avoid: Vec<String>) -> Vec<String> {
    self.neighbors
      .clone()
      .into_iter()
      .filter(|id| !avoid.contains(id))
      .collect()
  }

}
