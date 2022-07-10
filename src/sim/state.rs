use instant::Instant;
use nalgebra::{Vector2, Vector3};
use winit::event::{ElementState, VirtualKeyCode, WindowEvent};

use super::Body;

pub struct State {
    pub mouse_pos: Vector2<f64>,
    pub window_size: Vector2<u32>,
    pub last_frame: Option<Instant>,
    pub bodies: Vec<Body>,
    pub wireframe: bool,
    pub paused: bool,
    pub bg_color: Vector3<f64>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mouse_pos: Vector2::default(),
            window_size: Vector2::default(),
            last_frame: None,
            bodies: vec![Body::default()],
            wireframe: false,
            paused: false,
            bg_color: Vector3::default(),
        }
    }
}

impl State {
    pub fn input(&mut self, event: &WindowEvent) {
        // We have no events to handle currently
        match event {
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::Space)
                    && input.state == ElementState::Released =>
            {
                self.paused = !self.paused;
            }
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::W)
                    && input.state == ElementState::Released =>
            {
                self.wireframe = !self.wireframe;
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = Vector2::new(position.x, position.y);
            }
            WindowEvent::Resized(size) => {
                self.window_size = Vector2::new(size.width, size.height);
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        // Remain paused
        if self.paused {
            self.last_frame = Some(Instant::now());
            self.bg_color = Vector3::default();
            return;
        }

        match self.last_frame {
            Some(last_frame) => {
                let now = Instant::now();
                let dt = now - last_frame;

                // Update background color
                self.bg_color = self
                    .mouse_pos
                    .component_div(&self.window_size.cast::<f64>())
                    .push(0.0);

                // Simulation logic
                let dt_f32 = dt.as_secs_f32();
                for body in self.bodies.iter_mut() {
                    body.update(dt_f32);
                }
                self.last_frame = Some(now);
            }
            None => {
                self.last_frame = Some(Instant::now());
            }
        }
    }
}
