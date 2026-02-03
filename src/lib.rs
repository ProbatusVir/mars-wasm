use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn make_string(input : &str) -> String {
	format!("Fizz Fizz, Buzz {input}")
}

