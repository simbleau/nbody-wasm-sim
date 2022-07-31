mod dom;
mod gpu_primitives;
mod render;
mod runtime;
mod sim;

use gloo_console::log;
use render::WgpuContext;
use wasm_bindgen::prelude::*;
use winit::dpi::LogicalSize;
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
    log!("Acquired DOM elements");

    // Create window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .and_then(|w| {
            // Set attributes
            w.set_inner_size(LogicalSize::new(600.0, 400.0));
            Ok(w)
        })
        .expect("Could not build window");
    log!("Created window");

    // Connect graphics card to window
    let mut context = WgpuContext::new(&window).await;
    log!("Acquired graphics context");

    // Load shaders
    context.add_shader("vert", include_str!("../assets/shaders/vert.wgsl"));
    context.add_shader("frag", include_str!("../assets/shaders/frag.wgsl"));
    context.add_shader(
        "wireframe.vert",
        include_str!("../assets/shaders/wireframe.vert.wgsl"),
    );
    context.add_shader(
        "wireframe.frag",
        include_str!("../assets/shaders/wireframe.frag.wgsl"),
    );
    context.add_shader(
        "world.vert",
        include_str!("../assets/shaders/world.vert.wgsl"),
    );
    context.add_shader(
        "world.frag",
        include_str!("../assets/shaders/world.frag.wgsl"),
    );
    log!("Loaded shaders");

    // Load textures
    context
        .add_texture("cookie", include_bytes!("../assets/textures/cookie.png"));
    context.add_texture("moon", include_bytes!("../assets/textures/moon.jpg"));
    log!("Loaded textures");

    // Run program
    let mut runtime = Runtime::new(context, window, dom);
    log!("Starting...");
    event_loop.run(move |event, target, control_flow| {
        runtime.main_loop(event, target, control_flow)
    });
}
