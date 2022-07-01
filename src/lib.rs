use gloo_console::log;
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
pub fn attach() {
    log!("Hello from WASM");

    // Get the canvas element via WASM
    let window = web_sys::window().expect_throw("no global `window` exists");
    let document = window
        .document()
        .expect_throw("should have a document on window");
}
