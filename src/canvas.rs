use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub fn get_canvas() -> HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("c").unwrap();
    canvas.unchecked_into::<HtmlCanvasElement>()
}

pub fn style_canvas(canvas: &HtmlCanvasElement) {
    // Set a background color for the canvas to make it easier to tell the
    // where the canvas is for debugging purposes.
    canvas.style().set_css_text("background-color: blue;");
}
