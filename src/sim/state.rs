use glam::{DVec2, DVec3, UVec2, Vec2};
use instant::Instant;
use winit::event::{ElementState, WindowEvent};

use crate::sim::{input::InputController, WORLD_RADIUS};

pub struct State<'a> {
    pub mouse_pos: DVec2,
    pub view_size: UVec2,
    pub last_frame: Option<Instant>,
    pub wireframe: bool,
    pub paused: bool,
    pub bg_color: DVec3,
    pub texture_key: &'a str,
    pub pan: Vec2,
    pub pan_velocity: Vec2,
    pub rotation: f32,
    pub zoom: f32,
    pub input_controller: InputController,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self {
            mouse_pos: DVec2::default(),
            view_size: UVec2::default(),
            last_frame: None,
            wireframe: false,
            paused: false,
            bg_color: DVec3::default(),
            texture_key: "moon",
            pan: Vec2::ZERO,
            pan_velocity: Vec2::ZERO,
            rotation: 0.0,
            zoom: 100.0,
            input_controller: InputController::default(),
        }
    }
}

impl<'a> State<'a> {
    pub fn new(view_size: Vec2) -> Self {
        let zoom = if view_size.y < view_size.x {
            view_size.y / (WORLD_RADIUS * 2.0)
        } else {
            view_size.x / (WORLD_RADIUS * 2.0)
        };

        Self {
            pan: Vec2::new(0., 0.),
            zoom,
            ..Default::default()
        }
    }

    pub fn handle_input(&mut self, event: &WindowEvent) {
        // We have no events to handle currently
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(key) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            self.input_controller.press(key)
                        }
                        ElementState::Released => {
                            self.input_controller.release(key)
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = DVec2::new(position.x, position.y);
            }
            WindowEvent::Resized(size) => {
                self.view_size = UVec2::new(size.width, size.height);
            }
            _ => {}
        }
    }
}
