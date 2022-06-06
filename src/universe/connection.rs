
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Connection {
  source: u32,
  target: u32,
}

#[wasm_bindgen]
impl Connection {

  #[wasm_bindgen(constructor)]
  pub fn new(source: u32, target: u32) -> Connection {
    Connection { source, target }
  }

}

impl Connection {

  pub fn to_pair(&self) -> (&u32, &u32) {
    (&self.source, &self.target)
  }

}