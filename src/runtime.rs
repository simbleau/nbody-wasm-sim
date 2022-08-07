use glam::Vec2;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};
use winit::window::Window;

use crate::dom::Dom;
use crate::render::WgpuContext;
use crate::sim::{Simulation, WORLD_RADIUS};

pub struct Runtime {
    context: WgpuContext,
    window: Window,
    dom: Dom,
    sim: Simulation,
}

impl Runtime {
    pub fn new(context: WgpuContext, window: Window, dom: Dom) -> Self {
        let mut sim = Simulation::default();
        // Zoom into sim
        let view_size = Vec2::new(
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        );
        sim.state.zoom = if view_size.y < view_size.x {
            view_size.y / (WORLD_RADIUS * 2.0)
        } else {
            view_size.x / (WORLD_RADIUS * 2.0)
        };

        Self {
            context,
            window,
            dom,
            sim,
        }
    }

    pub fn main_loop<T>(
        &mut self,
        event: Event<()>,
        _target: &EventLoopWindowTarget<T>,
        control_flow: &mut ControlFlow,
    ) {
        // Log every event
        self.dom.log_list.log_event(&event);

        // Update world
        self.sim.update();

        // Handle events
        match event {
            Event::WindowEvent {
                window_id: id,
                event: ref winevent,
            } if id == self.window.id() => {
                self.sim.state.handle_input(winevent);
                match winevent {
                    WindowEvent::Resized(physical_size) => {
                        self.context.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size, ..
                    } => {
                        self.context.resize(**new_inner_size);
                    }
                    _ => (),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                self.window.request_redraw();
            }
            Event::RedrawRequested(window_id)
                if window_id == self.window.id() =>
            {
                match self.context.render(&self.sim) {
                    Ok(_) => {
                        // Update frame count
                        self.dom.fps_counter.update();
                    }
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => {
                        self.context.resize(self.context.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        self.dom.log_list.log_message("Out of memory!");
                        *control_flow = ControlFlow::Exit
                    }
                    Err(e) => {
                        // Error!
                        self.dom.log_list.log_message(&format!("{:?}", e));
                    }
                }
            }
            _ => (),
        }
    }
}
