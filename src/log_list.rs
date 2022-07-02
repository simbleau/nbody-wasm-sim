use winit::event::Event;

pub fn create_log_list() -> web_sys::Element {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let log_header = document.create_element("h2").unwrap();
    log_header.set_text_content(Some("Event Log"));
    body.append_child(&log_header).unwrap();

    let log_list = document.create_element("ul").unwrap();
    body.append_child(&log_list).unwrap();
    log_list
}

pub fn log_event(log_list: &web_sys::Element, event: &Event<()>) {
    //log!("{:?}", event);

    // Getting access to browser logs requires a lot of setup on mobile
    // devices. So we implement this basic logging system into the
    // page to give developers an easy alternative. As a bonus its
    // also kind of handy on desktop.
    if let Event::WindowEvent { event, .. } = &event {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let log = document.create_element("li").unwrap();
        log.set_text_content(Some(&format!("{:?}", event)));
        log_list
            .insert_before(&log, log_list.first_child().as_ref())
            .unwrap();
    }
}
