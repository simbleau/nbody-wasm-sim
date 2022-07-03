mod canvas;
mod fps_counter;
mod log_list;
mod state;

use gloo_console::log;
use instant::Instant;
use state::State;
use wasm_bindgen::prelude::*;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::platform::web::WindowBuilderExtWebSys;
use winit::{event::Event, event_loop::EventLoop, window::WindowBuilder};

const FPS_FILTER_PERIOD: f32 = 10.0;

#[wasm_bindgen(start)]
pub async fn run() {
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
        .and_then(|w| {
            // Set attributes
            w.set_inner_size(PhysicalSize::new(600.0, 400.0));
            Ok(w)
        })
        .expect("Could not build window");

    // Connect graphics card to window
    let mut state = State::new(&window).await;
    log!("We connected the graphics card to the surface");

    // Run program
    let mut dt_filtered = 0.0;
    let mut last_frame_instant = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        log_list::log_event(&log_list, &event);

        match event {
            Event::WindowEvent {
                window_id: id,
                event: winevent,
            } if id == window.id() => {
                if !state.input(&winevent) {
                    match winevent {
                        WindowEvent::Resized(physical_size) => {
                            state.resize(physical_size);
                        }
                        WindowEvent::ScaleFactorChanged {
                            new_inner_size,
                            ..
                        } => {
                            // new_inner_size is &&mut so we have to dereference
                            // it twice
                            state.resize(*new_inner_size);
                        }
                        _ => (),
                    }
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {
                        let now = Instant::now();
                        let dt_duration = now - last_frame_instant;
                        last_frame_instant = now;

                        let dt_raw = dt_duration.as_secs_f32();
                        dt_filtered = dt_filtered
                            + (dt_raw - dt_filtered) / FPS_FILTER_PERIOD;

                        fps_counter.set_text_content(Some(&format!(
                            "FPS: {:?}",
                            (1.0 / dt_filtered) as i32
                        )));
                    }
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        // TODO: log this
                        *control_flow = ControlFlow::Exit
                    }
                    // All other errors (Outdated, Timeout) should be resolved
                    // by the next frame
                    Err(e) => eprintln!("{:?}", e), // TODO: log this
                }
            }
            _ => (),
        }
    });
}
