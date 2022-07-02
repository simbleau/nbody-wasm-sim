mod canvas;
mod fps_counter;
mod log_list;

use instant::Instant;
use wasm_bindgen::prelude::*;
use winit::dpi::PhysicalSize;
use winit::platform::web::WindowBuilderExtWebSys;
use winit::{event::Event, event_loop::EventLoop, window::WindowBuilder};

const FPS_FILTER_PERIOD: f32 = 10.0;

#[wasm_bindgen]
pub fn attach() {
    // Redirect panics to the console (debugging)
    console_error_panic_hook::set_once();

    let canvas = canvas::get_canvas();
    let log_list = log_list::get_log_list();
    let fps_counter = fps_counter::get_fps_counter();

    // Create window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .unwrap();
    // Resize window (the canvas)
    window.set_inner_size(PhysicalSize::new(600.0, 400.0));

    // Run program
    let mut dt_filtered = 0.0;
    let mut last_frame_instant = Instant::now();
    event_loop.run(move |event, _, _control_flow| {
        log_list::log_event(&log_list, &event);

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let now = Instant::now();
                let dt_duration = now - last_frame_instant;
                last_frame_instant = now;

                let dt_raw = dt_duration.as_secs_f32();
                dt_filtered =
                    dt_filtered + (dt_raw - dt_filtered) / FPS_FILTER_PERIOD;

                fps_counter.set_text_content(Some(&format!(
                    "FPS: {:?}",
                    (1.0 / dt_filtered) as i32
                )));
            }
            _ => (),
        }
    });
}
