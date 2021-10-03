use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_odd(n: isize) -> bool{
    n % 2 != 0
}
