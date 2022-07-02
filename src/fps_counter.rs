pub fn get_fps_counter() -> web_sys::Element {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("f"))
        .expect("FPS counter not found")
}
