use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
   if n == 0 || n == 1 {
      return n;
   }

   fib(n - 1) + fib(n - 2)
}

#[wasm_bindgen]
pub struct Product {
    id: i32,
}

#[wasm_bindgen]
impl Product {
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32) -> Product {
        Product { id: val }
    }

    pub fn get(&self) -> i32 {
        self.id
    }

    pub fn set(&mut self, val: i32) {
        self.id = val;
    }
}