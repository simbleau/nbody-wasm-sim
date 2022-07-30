use glam::{DVec2, DVec3, Quat, UVec2, Vec2, Vec3, Vec3Swizzles};
use instant::Instant;
use winit::event::{ElementState, VirtualKeyCode, WindowEvent};

use crate::sim::Body;

use super::input::InputController;

pub const INITIAL_VIEW_BOUNDS: Vec2 = Vec2::new(1., 1.);
pub const CAM_PAN_SPEED: f32 = 0.05;

pub struct State<'a> {
    pub mouse_pos: DVec2,
    pub view_size: UVec2,
    pub last_frame: Option<Instant>,
    pub bodies: Vec<Body>,
    pub wireframe: bool,
    pub paused: bool,
    pub bg_color: DVec3,
    pub texture_key: &'a str,
    pub pan: Vec2,
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
            bodies: vec![],
            wireframe: false,
            paused: false,
            bg_color: DVec3::default(),
            texture_key: "moon",
            pan: Vec2::ZERO,
            rotation: 0.0,
            zoom: 100.0,
            input_controller: InputController::default(),
        }
    }
}

impl<'a> State<'a> {
    pub fn new(view_size: Vec2) -> Self {
        // Generate a bunch of bodies
        let mut bodies = Vec::new();
        for _ in 0..1 {
            let body = Body::default();
            bodies.push(body);
        }

        Self {
            pan: Vec2::new(0., 0.),
            zoom: if (view_size.y - INITIAL_VIEW_BOUNDS.y).abs()
                < (view_size.x / INITIAL_VIEW_BOUNDS.x).abs()
            {
                view_size.y / INITIAL_VIEW_BOUNDS.y
            } else {
                view_size.x / INITIAL_VIEW_BOUNDS.x
            },
            bodies,
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

    fn update_camera(&mut self) {
        // Handle input
        // Rotation
        if self.input_controller.is_key_active(VirtualKeyCode::Left) {
            self.rotation += 0.1;
        }
        if self.input_controller.is_key_active(VirtualKeyCode::Right) {
            self.rotation -= 0.1;
        }
        // Scale
        if self.input_controller.is_key_active(VirtualKeyCode::Up) {
            self.zoom += self.zoom * 0.1;
        }
        if self.input_controller.is_key_active(VirtualKeyCode::Down) {
            self.zoom -= self.zoom * 0.1;
        }
        // Translation
        if self.input_controller.is_key_active(VirtualKeyCode::W) {
            self.pan += (Quat::from_rotation_z(self.rotation)
                * (CAM_PAN_SPEED * Vec3::Y))
                .xy();
        }
        if self.input_controller.is_key_active(VirtualKeyCode::A) {
            self.pan -= (Quat::from_rotation_z(self.rotation)
                * (CAM_PAN_SPEED * Vec3::X))
                .xy();
        }
        if self.input_controller.is_key_active(VirtualKeyCode::S) {
            self.pan -= (Quat::from_rotation_z(self.rotation)
                * (CAM_PAN_SPEED * Vec3::Y))
                .xy();
        }
        if self.input_controller.is_key_active(VirtualKeyCode::D) {
            self.pan += (Quat::from_rotation_z(self.rotation)
                * (CAM_PAN_SPEED * Vec3::X))
                .xy();
        }
        // Wireframe
        if self.input_controller.is_key_pressed(VirtualKeyCode::Q) {
            self.wireframe = !self.wireframe;
        }
        // Texture Change
        if self.input_controller.is_key_released(VirtualKeyCode::E) {
            self.texture_key = match self.texture_key {
                "moon" => "cookie",
                _ => "moon",
            };
        }
    }

    pub fn update(&mut self) {
        // Pausing
        if self.input_controller.is_key_pressed(VirtualKeyCode::Space) {
            self.paused = !self.paused;
        }

        // Remain paused
        if self.paused {
            self.last_frame = Some(Instant::now());
            self.bg_color = DVec3::default();
            return;
        }

        // Handle camera input
        self.update_camera();

        // Update sim
        self.bg_color = DVec3::new(0.16, 0.33, 0.16);
        match self.last_frame {
            Some(last_frame) => {
                let now = Instant::now();
                let dt = now - last_frame;

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

        // Reset input controller
        self.input_controller.update();
    }
}
