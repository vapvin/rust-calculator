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

#[wasm_bindgen]
pub fn return_total_val() -> String {
    unsafe {
        format!("{}", total_val).into()
    }
}

#[wasm_bindgen]
pub fn change_input_val(stringval: &str) {
    let val = stringval.parse::<f64>().unwrap();

    unsafe {
        input_val = input_val.mul_add(10.0, val);
    }
}

#[wasm_bindgen]
pub fn clear() {
    unsafe {
        input_val = 0.0;
    }
}

#[wasm_bindgen]
pub fn all_clear() {
    unsafe {
        input_val = 0.0;
        total_val = 0.0;
    }
}

#[wasm_bindgen]
pub fn equal() {
    unsafe {
        input_val = total_val;
    }
}

#[wasm_bindgen]
pub fn chage_sig() {
    unsafe {
        input_val = -(input_val);
    }
}