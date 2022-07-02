use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub fn get_canvas() -> HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("c").unwrap();
    canvas.unchecked_into::<HtmlCanvasElement>()
}
