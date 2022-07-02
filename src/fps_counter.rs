pub fn get_fps_counter() -> web_sys::Element {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let counter = document.get_element_by_id("f").unwrap();

    counter
}
