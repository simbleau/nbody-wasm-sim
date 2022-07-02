use winit::event::Event;

pub fn get_log_list() -> web_sys::Element {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let log = document.get_element_by_id("l").unwrap();

    log
}

pub fn log_event(log_list: &web_sys::Element, event: &Event<()>) {
    if let Event::WindowEvent { event, .. } = &event {
        let document = web_sys::window().unwrap().document().unwrap();
        let log = document.create_element("li").unwrap();
        log.set_text_content(Some(&format!("{:?}", event)));

        log_list
            .insert_before(&log, log_list.first_child().as_ref())
            .unwrap();
    }
}
