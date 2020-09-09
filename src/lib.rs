mod utils;

use wasm_bindgen::prelude::*;

static mut input_val: f64 = 0.0;
static mut total_val: f64 = 0.0;
static mut this_method: i32 = 0;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn return_input_val() -> String {
    unsafe {
        format!("{}", input_val).into()
    }
}