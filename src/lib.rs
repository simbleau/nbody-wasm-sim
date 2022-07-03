mod dom;
mod renderer;
mod runtime;
mod state;
mod wgpu_context;

use gloo_console::log;
use wasm_bindgen::prelude::*;
use wgpu_context::WgpuContext;
use winit::dpi::PhysicalSize;
use winit::platform::web::WindowBuilderExtWebSys;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::dom::Dom;
use crate::runtime::Runtime;

#[wasm_bindgen(start)]
pub async fn run() {
    // Redirect panics to the console (debugging)
    console_error_panic_hook::set_once();

    let dom = Dom::new();
    let canvas = dom::get_canvas();

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
    let context = WgpuContext::new(&window).await;
    log!("Acquired graphics context");

    // Run program
    let mut runtime = Runtime::new(context, window, dom);
    event_loop.run(move |event, target, control_flow| {
        runtime.main_loop(event, target, control_flow)
    });
}
