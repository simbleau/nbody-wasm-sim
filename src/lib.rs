mod canvas;
mod log_list;

use gloo_console::log;
use wasm_bindgen::prelude::*;
use winit::dpi::PhysicalSize;
use winit::platform::web::WindowBuilderExtWebSys;
use winit::{event::Event, event_loop::EventLoop, window::WindowBuilder};

#[wasm_bindgen(start)]
pub fn attach() {
    log!("Test");
    let canvas = canvas::get_canvas();
    canvas::style_canvas(&canvas);
    let log_list = log_list::get_log_list();

    // Create window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .unwrap();

    // Resize canvas
    window.set_inner_size(PhysicalSize::new(600.0, 400.0));

    // Run program
    event_loop.run(move |event, _, _control_flow| {
        log_list::log_event(&log_list, &event);

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
