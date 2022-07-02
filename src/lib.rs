#![allow(clippy::single_match)]

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, Window};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

#[cfg(target_arch = "wasm32")]
pub fn main() {
    use winit::dpi::PhysicalSize;
    use winit::platform::web::WindowBuilderExtWebSys;

    let event_loop = EventLoop::new();

    let window = web_sys::window().expect_throw("no global `window` exists");
    let document = window
        .document()
        .expect_throw("should have a document on window");
    let canvas = document.get_element_by_id("c").expect_throw("No canvas");
    let canvas = canvas.unchecked_into::<HtmlCanvasElement>();
    let window = WindowBuilder::new()
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .unwrap();
    window.set_inner_size(PhysicalSize::new(400.0, 200.0));

    let log_list = wasm::create_log_list(&window);

    event_loop.run(move |event, _, control_flow| {
        //control_flow.set_wait();

        #[cfg(target_arch = "wasm32")]
        wasm::log_event(&log_list, &event);

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use gloo_console::log;
    use wasm_bindgen::{prelude::*, JsCast};
    use web_sys::HtmlCanvasElement;
    use winit::{event::Event, window::Window};

    #[wasm_bindgen(start)]
    pub fn attach() {
        #[allow(clippy::main_recursion)]
        super::main();
    }

    pub fn create_log_list(window: &Window) -> web_sys::Element {
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        // Set a background color for the canvas to make it easier to tell the
        // where the canvas is for debugging purposes.
        canvas.style().set_css_text("background-color: crimson;");
        body.append_child(&canvas).unwrap();

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
}
