use nalgebra::Vector2;
use winit::event::WindowEvent;

pub struct State {
    pub mouse_pos: Vector2<f64>,
    pub window_size: Vector2<u32>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mouse_pos: Vector2::default(),
            window_size: Vector2::default(),
        }
    }
}

impl State {
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        // We have no events to handle currently
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = Vector2::new(position.x, position.y);
            }
            WindowEvent::Resized(size) => {
                self.window_size = Vector2::new(size.width, size.height);
            }
            _ => {}
        }
        false
    }

    pub fn update(&mut self) {
        // We have nothing to update currently
    }
}
