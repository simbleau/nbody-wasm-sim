use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub fn get_canvas() -> HtmlCanvasElement {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("c"))
        .map(|e| e.unchecked_into::<HtmlCanvasElement>())
        .expect("Canvas not found")
}
