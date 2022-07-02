use winit::event::Event;

pub fn get_log_list() -> web_sys::Element {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("l"))
        .expect("Log list not found")
}

pub fn log_event(log_list: &web_sys::Element, event: &Event<()>) {
    if let Event::WindowEvent { event, .. } = &event {
        let log = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.create_element("li").ok())
            .expect("Log could not be created");
        log.set_text_content(Some(&format!("{:?}", event)));

        log_list
            .insert_before(&log, log_list.first_child().as_ref())
            .expect("Could not append log");
    }
}
