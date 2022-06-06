mod node;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use node::{Node};

#[wasm_bindgen]
struct NodeMap {
  nodes: HashMap<String, Node>
}